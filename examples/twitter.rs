use rust_system_design::twitter::{TimelineSource, TwitterConfig, TwitterService};

fn main() {
    let mut service = TwitterService::with_config(TwitterConfig {
        high_reach_follower_threshold: 2,
        ..TwitterConfig::default()
    });

    let reader = service.create_user("reader").expect("reader válido");
    let second_reader = service.create_user("second").expect("segundo lector");
    let author = service.create_user("jeresoft").expect("autor válido");

    service.follow(reader.id, author.id).expect("follow");
    service
        .follow(second_reader.id, author.id)
        .expect("segundo follow");

    service
        .publish_tweet(author.id, "Diseñar timelines es diseñar tradeoffs.")
        .expect("tweet válido");

    for entry in service.timeline(reader.id, Some(10)).expect("timeline") {
        let source = match entry.source {
            TimelineSource::Materialized => "materialized",
            TimelineSource::ReadMerge => "read-merge",
        };
        println!("[{source}] @{}: {}", entry.author_id, entry.text);
    }
}
