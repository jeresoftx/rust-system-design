use rust_system_design::kafka::KafkaService;
use std::time::Instant;

fn main() {
    let mut service = KafkaService::new();
    service
        .create_topic("events", 8, 20_000)
        .expect("create topic");

    let started = Instant::now();
    for index in 0..10_000 {
        let key = format!("booking:{}", index % 256);
        service
            .publish("events", Some(&key), "paid")
            .expect("publish");
    }
    for partition in 0..service.partition_count("events").expect("partitions") {
        let batch = service
            .fetch("billing", "events", partition, 0, 10_000)
            .expect("fetch");
        service
            .commit("billing", "events", partition, batch.next_offset)
            .expect("commit");
    }
    let elapsed = started.elapsed();

    println!("kafka publish/fetch baseline: 10000 events in {elapsed:?}");
}
