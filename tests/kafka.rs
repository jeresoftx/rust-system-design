use rust_system_design::kafka::{KafkaError, KafkaService};

#[test]
fn round_robin_publication_uses_all_partitions_without_key() {
    let mut service = KafkaService::new();
    service.create_topic("events", 3, 10).expect("topic");

    let first = service.publish("events", None, "a").expect("first");
    let second = service.publish("events", None, "b").expect("second");
    let third = service.publish("events", None, "c").expect("third");
    let fourth = service.publish("events", None, "d").expect("fourth");

    assert_eq!(first.partition, 0);
    assert_eq!(second.partition, 1);
    assert_eq!(third.partition, 2);
    assert_eq!(fourth.partition, 0);
}

#[test]
fn keyed_publication_keeps_entity_order_in_one_partition() {
    let mut service = KafkaService::new();
    service.create_topic("payments", 8, 10).expect("topic");

    let created = service
        .publish("payments", Some("booking-42"), "created")
        .expect("created");
    let paid = service
        .publish("payments", Some("booking-42"), "paid")
        .expect("paid");
    let cancelled = service
        .publish("payments", Some("booking-42"), "cancelled")
        .expect("cancelled");

    assert_eq!(created.partition, paid.partition);
    assert_eq!(paid.partition, cancelled.partition);
    assert_eq!([created.offset, paid.offset, cancelled.offset], [0, 1, 2]);
}

#[test]
fn fetch_limit_returns_next_offset_for_manual_commit() {
    let mut service = KafkaService::new();
    service.create_topic("payments", 1, 10).expect("topic");
    service
        .publish("payments", None, "created")
        .expect("created");
    service.publish("payments", None, "paid").expect("paid");
    service
        .publish("payments", None, "confirmed")
        .expect("confirmed");

    let batch = service
        .fetch("billing", "payments", 0, 0, 2)
        .expect("fetch");
    service
        .commit("billing", "payments", 0, batch.next_offset)
        .expect("commit");

    assert_eq!(batch.events.len(), 2);
    assert_eq!(batch.next_offset, 2);
    assert_eq!(service.committed_offset("billing", "payments", 0), Some(2));
}

#[test]
fn groups_keep_independent_lag() {
    let mut service = KafkaService::new();
    service.create_topic("payments", 1, 10).expect("topic");
    for payload in ["created", "paid", "confirmed"] {
        service.publish("payments", None, payload).expect("publish");
    }

    service
        .commit("billing", "payments", 0, 3)
        .expect("billing");
    service.commit("audit", "payments", 0, 1).expect("audit");

    assert_eq!(
        service.lag("billing", "payments", 0).expect("billing").lag,
        0
    );
    assert_eq!(service.lag("audit", "payments", 0).expect("audit").lag, 2);
}

#[test]
fn retention_removes_old_events_and_rejects_old_fetches() {
    let mut service = KafkaService::new();
    service.create_topic("events", 1, 2).expect("topic");
    for payload in ["one", "two", "three", "four"] {
        service.publish("events", None, payload).expect("publish");
    }

    let error = service
        .fetch("archive", "events", 0, 1, 10)
        .expect_err("retained offset");
    let batch = service.fetch("archive", "events", 0, 2, 10).expect("fetch");

    assert_eq!(
        error,
        KafkaError::OffsetOutOfRange {
            requested: 1,
            first_available: 2,
            next_offset: 4
        }
    );
    assert_eq!(batch.events.len(), 2);
    assert_eq!(service.metrics().events_removed_by_retention, 2);
}

#[test]
fn future_commit_is_rejected_without_advancing_group() {
    let mut service = KafkaService::new();
    service.create_topic("events", 1, 10).expect("topic");
    service.publish("events", None, "one").expect("publish");

    let error = service
        .commit("worker", "events", 0, 99)
        .expect_err("future commit");

    assert_eq!(
        error,
        KafkaError::CommitBeyondEnd {
            requested: 99,
            next_offset: 1
        }
    );
    assert_eq!(service.committed_offset("worker", "events", 0), None);
    assert_eq!(service.metrics().commits_rejected, 1);
}
