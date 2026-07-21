use rust_system_design::booking_engine::{
    BookingError, BookingService, HoldStatus, ReservationStatus, StayRange,
};

#[test]
fn availability_quotes_price_across_multiple_nights() {
    let mut service = BookingService::new();
    let full = StayRange::new(10, 13).expect("range");
    service
        .upsert_inventory(
            "hotel",
            "standard",
            StayRange::new(10, 11).unwrap(),
            3,
            10_000,
        )
        .expect("night 10");
    service
        .upsert_inventory(
            "hotel",
            "standard",
            StayRange::new(11, 12).unwrap(),
            3,
            12_000,
        )
        .expect("night 11");
    service
        .upsert_inventory(
            "hotel",
            "standard",
            StayRange::new(12, 13).unwrap(),
            3,
            14_000,
        )
        .expect("night 12");

    let quote = service
        .availability("hotel", "standard", full, 2)
        .expect("quote");

    assert!(quote.available);
    assert_eq!(quote.available_units, 3);
    assert_eq!(quote.total_price_cents, 72_000);
}

#[test]
fn hold_prevents_overselling_last_units() {
    let mut service = BookingService::new();
    let stay = StayRange::new(1, 4).expect("range");
    service
        .upsert_inventory("hotel", "standard", stay, 2, 10_000)
        .expect("inventory");

    service
        .create_hold("hotel", "standard", stay, 2, 10)
        .expect("first hold");
    let error = service
        .create_hold("hotel", "standard", stay, 1, 10)
        .expect_err("oversell");

    assert_eq!(
        error,
        BookingError::InsufficientAvailability {
            requested: 1,
            available: 0
        }
    );
    assert_eq!(service.metrics().holds_rejected, 1);
}

#[test]
fn expired_hold_cannot_be_confirmed() {
    let mut service = BookingService::new();
    let stay = StayRange::new(5, 7).expect("range");
    service
        .upsert_inventory("hotel", "suite", stay, 1, 20_000)
        .expect("inventory");
    let hold = service
        .create_hold("hotel", "suite", stay, 1, 1)
        .expect("hold");
    assert_eq!(service.advance_time(1), 1);

    let error = service.confirm_hold(hold.id, "ada").expect_err("expired");

    assert_eq!(error, BookingError::HoldExpired { hold_id: hold.id });
    assert_eq!(service.hold(hold.id).unwrap().status, HoldStatus::Expired);
    assert_eq!(service.metrics().reservations_rejected, 1);
}

#[test]
fn cancelling_hold_releases_inventory() {
    let mut service = BookingService::new();
    let stay = StayRange::new(3, 5).expect("range");
    service
        .upsert_inventory("hotel", "standard", stay, 1, 10_000)
        .expect("inventory");
    let hold = service
        .create_hold("hotel", "standard", stay, 1, 10)
        .expect("hold");

    service.cancel_hold(hold.id).expect("cancel");
    let quote = service
        .availability("hotel", "standard", stay, 1)
        .expect("availability");

    assert!(quote.available);
    assert_eq!(service.hold(hold.id).unwrap().status, HoldStatus::Cancelled);
    assert_eq!(service.metrics().holds_cancelled, 1);
}

#[test]
fn cancelling_reservation_releases_future_inventory() {
    let mut service = BookingService::new();
    let stay = StayRange::new(20, 22).expect("range");
    service
        .upsert_inventory("hotel", "standard", stay, 1, 10_000)
        .expect("inventory");
    let hold = service
        .create_hold("hotel", "standard", stay, 1, 10)
        .expect("hold");
    let reservation = service.confirm_hold(hold.id, "ada").expect("reservation");

    service
        .cancel_reservation(reservation.id)
        .expect("cancel reservation");
    let quote = service
        .availability("hotel", "standard", stay, 1)
        .expect("availability");

    assert!(quote.available);
    assert_eq!(
        service.reservation(reservation.id).unwrap().status,
        ReservationStatus::Cancelled
    );
    assert_eq!(service.metrics().reservations_cancelled, 1);
}

#[test]
fn missing_inventory_is_rejected_explicitly() {
    let mut service = BookingService::new();
    let stay = StayRange::new(1, 3).expect("range");
    service
        .upsert_inventory(
            "hotel",
            "standard",
            StayRange::new(1, 2).unwrap(),
            1,
            10_000,
        )
        .expect("partial inventory");

    let error = service
        .availability("hotel", "standard", stay, 1)
        .expect_err("missing night");

    assert_eq!(error, BookingError::MissingInventory { night: 2 });
}
