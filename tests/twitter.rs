use rust_system_design::twitter::{TimelineSource, TwitterConfig, TwitterError, TwitterService};

#[test]
fn duplicate_handles_are_rejected_case_insensitively() {
    let mut service = TwitterService::new();

    service.create_user("@Jeresoft").expect("primer usuario");
    let error = service
        .create_user("jeresoft")
        .expect_err("handle duplicado");

    assert_eq!(
        error,
        TwitterError::DuplicateHandle {
            handle: "jeresoft".to_string()
        }
    );
}

#[test]
fn timeline_orders_newer_tweets_first() {
    let mut service = TwitterService::new();
    let reader = service.create_user("reader").expect("reader");
    let author = service.create_user("author").expect("author");
    service.follow(reader.id, author.id).expect("follow");

    let older = service.publish_tweet(author.id, "primero").expect("older");
    let newer = service.publish_tweet(author.id, "segundo").expect("newer");

    let timeline = service.timeline(reader.id, None).expect("timeline");

    assert_eq!(timeline[0].tweet_id, newer.id);
    assert_eq!(timeline[1].tweet_id, older.id);
}

#[test]
fn timeline_limit_truncates_results() {
    let mut service = TwitterService::new();
    let reader = service.create_user("reader").expect("reader");
    let author = service.create_user("author").expect("author");
    service.follow(reader.id, author.id).expect("follow");

    service.publish_tweet(author.id, "uno").expect("uno");
    service.publish_tweet(author.id, "dos").expect("dos");
    service.publish_tweet(author.id, "tres").expect("tres");

    let timeline = service.timeline(reader.id, Some(2)).expect("timeline");

    assert_eq!(timeline.len(), 2);
}

#[test]
fn mixed_timeline_contains_materialized_and_read_merge_entries() {
    let mut service = TwitterService::with_config(TwitterConfig {
        high_reach_follower_threshold: 2,
        ..TwitterConfig::default()
    });
    let reader = service.create_user("reader").expect("reader");
    let other_reader = service.create_user("other").expect("other");
    let normal_author = service.create_user("normal").expect("normal");
    let high_reach_author = service.create_user("high").expect("high");

    service
        .follow(reader.id, normal_author.id)
        .expect("follow normal");
    service
        .follow(reader.id, high_reach_author.id)
        .expect("follow high");
    service
        .follow(other_reader.id, high_reach_author.id)
        .expect("high reach threshold");

    service
        .publish_tweet(normal_author.id, "materializado")
        .expect("normal tweet");
    service
        .publish_tweet(high_reach_author.id, "merge en lectura")
        .expect("high tweet");

    let timeline = service.timeline(reader.id, None).expect("timeline");

    assert!(timeline
        .iter()
        .any(|entry| entry.source == TimelineSource::Materialized));
    assert!(timeline
        .iter()
        .any(|entry| entry.source == TimelineSource::ReadMerge));
}

#[test]
fn notifications_are_limited_per_publish() {
    let mut service = TwitterService::with_config(TwitterConfig {
        max_notifications_per_publish: 1,
        ..TwitterConfig::default()
    });
    let first_reader = service.create_user("first").expect("first");
    let second_reader = service.create_user("second").expect("second");
    let author = service.create_user("author").expect("author");
    service
        .follow(first_reader.id, author.id)
        .expect("follow first");
    service
        .follow(second_reader.id, author.id)
        .expect("follow second");

    let tweet = service
        .publish_tweet(author.id, "notificación limitada")
        .expect("tweet");

    let first_notifications = service
        .notifications(first_reader.id)
        .expect("notificaciones");
    let second_notifications = service
        .notifications(second_reader.id)
        .expect("notificaciones");

    assert_eq!(first_notifications.len(), 1);
    assert_eq!(first_notifications[0].tweet_id, tweet.id);
    assert!(second_notifications.is_empty());
    assert_eq!(service.metrics().notifications_created, 1);
}

#[test]
fn rejects_too_long_tweet() {
    let mut service = TwitterService::with_config(TwitterConfig {
        max_tweet_chars: 5,
        ..TwitterConfig::default()
    });
    let author = service.create_user("author").expect("author");

    let error = service
        .publish_tweet(author.id, "demasiado largo")
        .expect_err("tweet largo");

    assert_eq!(error, TwitterError::TweetTooLong { max_chars: 5 });
}
