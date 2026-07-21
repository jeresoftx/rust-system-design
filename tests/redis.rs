use rust_system_design::redis::{RedisConfig, RedisError, RedisService, RedisValue};

#[test]
fn lpush_creates_list_and_preserves_order() {
    let mut service = RedisService::new();

    service.lpush("jobs", "a").expect("first");
    let (_, len) = service.lpush("jobs", "b").expect("second");

    assert_eq!(len, 2);
    assert_eq!(
        service.list("jobs").expect("list"),
        Some(vec!["b".to_string(), "a".to_string()])
    );
}

#[test]
fn snapshot_omits_expired_keys() {
    let mut service = RedisService::new();
    service.set("short", "gone", Some(1)).expect("short");
    service.set("long", "visible", None).expect("long");
    let _ = service.advance_time(1);

    let snapshot = service.snapshot();

    assert!(!snapshot.entries.contains_key("short"));
    assert_eq!(
        snapshot.entries.get("long"),
        Some(&RedisValue::String("visible".to_string()))
    );
}

#[test]
fn replay_reconstructs_visible_state_from_aof() {
    let mut service = RedisService::new();
    service.set("name", "ada", None).expect("set");
    service.lpush("jobs", "render").expect("lpush");
    service.del("name").expect("del");
    let commands = service.aof().to_vec();

    let mut rebuilt = RedisService::replay(RedisConfig::default(), &commands).expect("replay");

    assert_eq!(rebuilt.get("name").expect("name"), None);
    assert_eq!(
        rebuilt.list("jobs").expect("jobs"),
        Some(vec!["render".to_string()])
    );
}

#[test]
fn replicate_since_returns_only_newer_commands() {
    let mut service = RedisService::new();
    let first = service.set("a", "1", None).expect("first");
    service.set("b", "2", None).expect("second");
    service.lpush("jobs", "x").expect("third");

    let batch = service.replicate_since(first).expect("replica");

    assert_eq!(batch.from_offset, first);
    assert_eq!(batch.commands.len(), 2);
    assert_eq!(batch.last_offset, service.last_offset());
    assert_eq!(service.metrics().replication_commands_returned, 2);
}

#[test]
fn replicate_rejects_future_offset() {
    let mut service = RedisService::new();
    service.set("a", "1", None).expect("first");

    let error = service.replicate_since(99).expect_err("future offset");

    assert_eq!(
        error,
        RedisError::InvalidOffset {
            requested: 99,
            last_offset: 1
        }
    );
}

#[test]
fn replacing_value_updates_memory_used() {
    let mut service = RedisService::with_config(RedisConfig {
        max_memory_bytes: 64,
    });
    service.set("key", "1234567890", None).expect("large");
    let after_large = service.metrics().memory_used;
    service.set("key", "1", None).expect("small");

    assert!(service.metrics().memory_used < after_large);
}
