use rust_system_design::booking_engine::{BookingService, StayRange};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut service = BookingService::new();
    let stay = StayRange::new(10, 13)?;

    service.upsert_inventory("hotel-1", "standard", stay, 4, 12_000)?;
    let quote = service.availability("hotel-1", "standard", stay, 2)?;
    let hold = service.create_hold("hotel-1", "standard", stay, 2, 5)?;
    let reservation = service.confirm_hold(hold.id, "ada")?;

    println!(
        "available={} quote={} reservation={}",
        quote.available, quote.total_price_cents, reservation.id
    );
    Ok(())
}
