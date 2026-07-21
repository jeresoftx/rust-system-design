use rust_system_design::kafka::KafkaService;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut service = KafkaService::new();
    service.create_topic("payments", 3, 100)?;

    let published = service.publish("payments", Some("booking-123"), "paid")?;
    let batch = service.fetch(
        "billing",
        "payments",
        published.partition,
        published.offset,
        10,
    )?;
    service.commit(
        "billing",
        "payments",
        published.partition,
        batch.next_offset,
    )?;

    let lag = service.lag("billing", "payments", published.partition)?;
    println!(
        "partition={} offset={} fetched={} lag={}",
        published.partition,
        published.offset,
        batch.events.len(),
        lag.lag
    );
    Ok(())
}
