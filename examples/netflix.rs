use rust_system_design::netflix::{Device, NetflixService, VideoQuality};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut service = NetflixService::new();
    let profile = service.register_profile("Ada", "mx")?;
    let title = service.register_title(
        "Rust at Scale",
        &["sistemas", "tecnología"],
        &["mx", "co", "es"],
        900,
        12,
    )?;
    service.add_variant(title.id, VideoQuality::Low, 900)?;
    service.add_variant(title.id, VideoQuality::Medium, 2_500)?;
    service.add_variant(title.id, VideoQuality::High, 5_000)?;
    service.register_cdn_node("mx-edge-1", "mx", 100)?;

    let session = service.start_playback(profile.id, title.id, 3_000, Device::Tv)?;
    println!(
        "sesión {} reproduce título {} en {:?} desde CDN {}",
        session.id, session.title_id, session.quality, session.cdn_node_id
    );

    Ok(())
}
