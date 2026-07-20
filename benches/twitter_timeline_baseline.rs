use rust_system_design::twitter::{TwitterConfig, TwitterService};
use std::hint::black_box;
use std::time::Instant;

fn main() {
    let mut service = TwitterService::with_config(TwitterConfig {
        high_reach_follower_threshold: 10,
        default_timeline_limit: 50,
        ..TwitterConfig::default()
    });

    let reader = service.create_user("reader").expect("reader");
    let normal_author = service.create_user("normal").expect("normal");
    let high_author = service.create_user("high").expect("high");

    service.follow(reader.id, normal_author.id).expect("follow");
    service
        .follow(reader.id, high_author.id)
        .expect("follow high");

    for index in 0..10 {
        let follower = service
            .create_user(&format!("follower-{index}"))
            .expect("follower");
        service.follow(follower.id, high_author.id).expect("fanout");
    }

    for index in 0..100 {
        service
            .publish_tweet(normal_author.id, &format!("normal {index}"))
            .expect("normal tweet");
        service
            .publish_tweet(high_author.id, &format!("high {index}"))
            .expect("high tweet");
    }

    let iterations = 1_000;
    let start = Instant::now();
    for _ in 0..iterations {
        let timeline = service
            .timeline(black_box(reader.id), Some(50))
            .expect("timeline");
        black_box(timeline);
    }
    let elapsed = start.elapsed();

    println!("twitter mixed-timeline baseline: {iterations} reads in {elapsed:?}");
}
