use rust_system_design::netflix::{Device, NetflixService, VideoQuality};
use std::time::Instant;

fn main() {
    let mut service = NetflixService::new();
    let title = service
        .register_title("Rust at Scale", &["tech", "sistemas"], &["mx"], 10_000, 12)
        .expect("title");
    service
        .add_variant(title.id, VideoQuality::Low, 900)
        .expect("low");
    service
        .add_variant(title.id, VideoQuality::Medium, 2_500)
        .expect("medium");
    service
        .add_variant(title.id, VideoQuality::High, 5_000)
        .expect("high");
    service
        .register_cdn_node("mx-edge", "mx", 10_000)
        .expect("cdn");

    let profile_ids: Vec<_> = (0..1_000)
        .map(|index| {
            service
                .register_profile(&format!("profile-{index}"), "mx")
                .expect("profile")
                .id
        })
        .collect();

    let started = Instant::now();
    for profile_id in profile_ids {
        service
            .start_playback(profile_id, title.id, 5_000, Device::Tv)
            .expect("playback");
    }
    let elapsed = started.elapsed();
    println!("netflix playback baseline: 1000 starts in {elapsed:?}");
}
