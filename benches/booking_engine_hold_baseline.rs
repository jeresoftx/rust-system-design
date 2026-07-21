use rust_system_design::booking_engine::{BookingService, StayRange};
use std::time::Instant;

fn main() {
    let mut service = BookingService::new();
    let calendar = StayRange::new(1, 366).expect("calendar");
    service
        .upsert_inventory("hotel", "standard", calendar, 50_000, 12_000)
        .expect("inventory");

    let started = Instant::now();
    for index in 0..5_000 {
        let check_in = 1 + (index % 300);
        let stay = StayRange::new(check_in, check_in + 3).expect("stay");
        let quote = service
            .availability("hotel", "standard", stay, 1)
            .expect("availability");
        if quote.available {
            let hold = service
                .create_hold("hotel", "standard", stay, 1, 10)
                .expect("hold");
            service.confirm_hold(hold.id, "guest").expect("confirm");
        }
    }
    let elapsed = started.elapsed();

    println!("booking engine hold baseline: 5000 quote/hold/confirm in {elapsed:?}");
}
