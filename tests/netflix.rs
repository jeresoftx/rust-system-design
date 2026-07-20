use rust_system_design::netflix::{
    Device, NetflixError, NetflixService, PlaybackState, VideoQuality,
};

fn seeded_service() -> NetflixService {
    let mut service = NetflixService::new();
    let rust = service
        .register_title("Rust at Scale", &["tech", "sistemas"], &["mx"], 1_000, 12)
        .expect("rust title");
    service
        .add_variant(rust.id, VideoQuality::Low, 900)
        .expect("rust low");
    service
        .add_variant(rust.id, VideoQuality::Medium, 2_500)
        .expect("rust medium");
    service
        .add_variant(rust.id, VideoQuality::High, 5_000)
        .expect("rust high");

    let architecture = service
        .register_title("Architecture Notes", &["arquitectura"], &["mx"], 950, 12)
        .expect("architecture title");
    service
        .add_variant(architecture.id, VideoQuality::Low, 800)
        .expect("architecture low");

    service
        .register_cdn_node("mx-edge-1", "mx", 1)
        .expect("cdn one");
    service
        .register_cdn_node("mx-edge-2", "mx", 10)
        .expect("cdn two");
    service
}

#[test]
fn recommendations_prefer_learned_genre_over_raw_popularity() {
    let mut service = seeded_service();
    let profile = service.register_profile("Ada", "mx").expect("profile");
    let low_popularity = service
        .visible_catalog(profile.id)
        .expect("catalog")
        .into_iter()
        .find(|title| title.name == "architecture notes")
        .expect("architecture");
    let session = service
        .start_playback(profile.id, low_popularity.id, 1_000, Device::Web)
        .expect("session");
    service
        .complete_session(session.id)
        .expect("complete learning session");

    let recommendations = service
        .recommendations(profile.id, 2)
        .expect("recommendations");

    assert_eq!(recommendations[0].title_id, low_popularity.id);
    assert!(recommendations[0].reason.contains("afinidad"));
}

#[test]
fn cdn_capacity_spills_to_next_healthy_node() {
    let mut service = seeded_service();
    let first_profile = service.register_profile("Ada", "mx").expect("first");
    let second_profile = service.register_profile("Alan", "mx").expect("second");
    let title = service
        .visible_catalog(first_profile.id)
        .expect("catalog")
        .into_iter()
        .find(|title| title.name == "rust at scale")
        .expect("title");

    let first_session = service
        .start_playback(first_profile.id, title.id, 5_000, Device::Tv)
        .expect("first session");
    let second_session = service
        .start_playback(second_profile.id, title.id, 5_000, Device::Tv)
        .expect("second session");

    assert_ne!(first_session.cdn_node_id, second_session.cdn_node_id);
    assert_eq!(service.metrics().cdn_assignments, 2);
}

#[test]
fn unhealthy_cdn_is_skipped_for_new_sessions() {
    let mut service = NetflixService::new();
    let profile = service.register_profile("Ada", "mx").expect("profile");
    let title = service
        .register_title("Rust", &["tech"], &["mx"], 100, 12)
        .expect("title");
    service
        .add_variant(title.id, VideoQuality::Low, 900)
        .expect("variant");
    let unhealthy = service
        .register_cdn_node("mx-edge-1", "mx", 10)
        .expect("unhealthy");
    let healthy = service
        .register_cdn_node("mx-edge-2", "mx", 10)
        .expect("healthy");
    service
        .set_cdn_health(unhealthy.id, false)
        .expect("mark unhealthy");

    let session = service
        .start_playback(profile.id, title.id, 1_000, Device::Mobile)
        .expect("session");

    assert_eq!(session.cdn_node_id, healthy.id);
}

#[test]
fn playback_fails_when_capacity_is_exhausted() {
    let mut service = NetflixService::new();
    let first = service.register_profile("Ada", "mx").expect("first");
    let second = service.register_profile("Alan", "mx").expect("second");
    let title = service
        .register_title("Rust", &["tech"], &["mx"], 100, 12)
        .expect("title");
    service
        .add_variant(title.id, VideoQuality::Low, 900)
        .expect("variant");
    service.register_cdn_node("mx-edge", "mx", 1).expect("cdn");
    service
        .start_playback(first.id, title.id, 1_000, Device::Mobile)
        .expect("first session");

    let error = service
        .start_playback(second.id, title.id, 1_000, Device::Mobile)
        .expect_err("capacity exhausted");

    assert_eq!(
        error,
        NetflixError::NoCdnCapacity {
            region: "mx".to_string()
        }
    );
    assert_eq!(service.metrics().cdn_capacity_rejections, 1);
}

#[test]
fn low_bandwidth_chooses_lower_variant_and_records_downgrade() {
    let mut service = seeded_service();
    let profile = service.register_profile("Ada", "mx").expect("profile");
    let title = service
        .visible_catalog(profile.id)
        .expect("catalog")
        .into_iter()
        .find(|title| title.name == "rust at scale")
        .expect("title");

    let session = service
        .start_playback(profile.id, title.id, 2_600, Device::Mobile)
        .expect("session");

    assert_eq!(session.quality, VideoQuality::Medium);
    assert_eq!(service.metrics().quality_downgrades, 1);
}

#[test]
fn session_events_record_pause_resume_and_completion() {
    let mut service = seeded_service();
    let profile = service.register_profile("Ada", "mx").expect("profile");
    let title = service
        .visible_catalog(profile.id)
        .expect("catalog")
        .into_iter()
        .find(|title| title.name == "rust at scale")
        .expect("title");
    let session = service
        .start_playback(profile.id, title.id, 5_000, Device::Tv)
        .expect("session");

    service.pause_session(session.id).expect("pause");
    service.resume_session(session.id).expect("resume");
    service.complete_session(session.id).expect("complete");

    let states: Vec<_> = service
        .events(session.id)
        .iter()
        .map(|event| event.state)
        .collect();

    assert_eq!(
        states,
        vec![
            PlaybackState::Playing,
            PlaybackState::Paused,
            PlaybackState::Playing,
            PlaybackState::Completed
        ]
    );
}
