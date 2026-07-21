use rust_system_design::redis::RedisService;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut service = RedisService::new();
    service.set("session:1", "ada", Some(10))?;
    service.lpush("jobs", "send-email")?;
    service.lpush("jobs", "render-report")?;

    let session = service.get("session:1")?;
    let jobs = service.list("jobs")?;
    let replica = service.replicate_since(0)?;

    println!(
        "session={session:?} jobs={jobs:?} comandos={}",
        replica.commands.len()
    );
    Ok(())
}
