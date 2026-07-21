use rust_system_design::airbnb::{AirbnbService, SearchQuery, StayRange, UserRole};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut service = AirbnbService::new();
    let host = service.register_user("Ada Host", UserRole::Host)?;
    let guest = service.register_user("Grace Guest", UserRole::Guest)?;
    let listing = service.create_listing(host.id, "guadalajara", 4, 15_000)?;
    let stay = StayRange::new(10, 13)?;

    service.upsert_availability(listing.id, stay, 1)?;
    let results = service.search(SearchQuery {
        city: "guadalajara".to_string(),
        guests: 2,
        stay,
        max_total_price_cents: Some(60_000),
    })?;
    let reservation = service.book(guest.id, results[0].listing_id, stay, 2)?;

    println!(
        "results={} reservation={} total={}",
        results.len(),
        reservation.id,
        reservation.total_price_cents
    );
    Ok(())
}
