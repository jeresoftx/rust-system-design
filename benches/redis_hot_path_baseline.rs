use rust_system_design::redis::RedisService;
use std::time::Instant;

fn main() {
    let mut service = RedisService::new();
    for index in 0..10_000 {
        service
            .set(&format!("key:{index}"), "value", None)
            .expect("set");
    }

    let started = Instant::now();
    for index in 0..10_000 {
        let key = format!("key:{index}");
        let _ = service.get(&key).expect("get");
    }
    let elapsed = started.elapsed();
    println!("redis hot path baseline: 10000 gets in {elapsed:?}");
}
