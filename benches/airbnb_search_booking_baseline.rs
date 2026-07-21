use rust_system_design::airbnb::{AirbnbService, SearchQuery, StayRange, UserRole};
use std::time::Instant;

fn main() {
    let mut service = AirbnbService::new();
    let host = service
        .register_user("benchmark host", UserRole::Host)
        .expect("host");
    let guest = service
        .register_user("benchmark guest", UserRole::Guest)
        .expect("guest");
    let stay = StayRange::new(10, 13).expect("stay");

    for index in 0..1_000 {
        let city = if index % 2 == 0 {
            "guadalajara"
        } else {
            "oaxaca"
        };
        let listing = service
            .create_listing(host.id, city, 4, 10_000 + (index % 10) * 1_000)
            .expect("listing");
        service
            .upsert_availability(listing.id, stay, 500)
            .expect("availability");
    }

    let started = Instant::now();
    for _ in 0..500 {
        let results = service
            .search(SearchQuery {
                city: "guadalajara".to_string(),
                guests: 2,
                stay,
                max_total_price_cents: Some(50_000),
            })
            .expect("search");
        service
            .book(guest.id, results[0].listing_id, stay, 2)
            .expect("book");
    }
    let elapsed = started.elapsed();

    println!("airbnb search/booking baseline: 500 searches and bookings in {elapsed:?}");
}
