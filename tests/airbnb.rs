use rust_system_design::airbnb::{
    AirbnbError, AirbnbReservationStatus, AirbnbService, SearchQuery, StayRange, UserRole,
};

fn setup_listing(
    city: &str,
    capacity: u32,
    price: u64,
) -> (AirbnbService, u64, u64, u64, StayRange) {
    let mut service = AirbnbService::new();
    let host = service
        .register_user("Ada Host", UserRole::Host)
        .expect("host");
    let guest = service
        .register_user("Grace Guest", UserRole::Guest)
        .expect("guest");
    let listing = service
        .create_listing(host.id, city, capacity, price)
        .expect("listing");
    let stay = StayRange::new(10, 13).expect("stay");
    service
        .upsert_availability(listing.id, stay, 1)
        .expect("availability");
    (service, host.id, guest.id, listing.id, stay)
}

#[test]
fn search_filters_by_price_capacity_and_city() {
    let (mut service, host_id, _guest_id, _listing_id, stay) =
        setup_listing("guadalajara", 4, 10_000);
    let expensive = service
        .create_listing(host_id, "guadalajara", 4, 50_000)
        .expect("expensive");
    service
        .upsert_availability(expensive.id, stay, 1)
        .expect("expensive availability");

    let results = service
        .search(SearchQuery {
            city: "guadalajara".to_string(),
            guests: 3,
            stay,
            max_total_price_cents: Some(40_000),
        })
        .expect("search");

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].total_price_cents, 30_000);
}

#[test]
fn suspended_host_removes_listing_from_search() {
    let (mut service, host_id, _guest_id, _listing_id, stay) = setup_listing("puebla", 2, 10_000);

    service.suspend_user(host_id).expect("suspend");
    let results = service
        .search(SearchQuery {
            city: "puebla".to_string(),
            guests: 2,
            stay,
            max_total_price_cents: None,
        })
        .expect("search");

    assert!(results.is_empty());
    assert_eq!(service.metrics().suspensions_applied, 1);
}

#[test]
fn suspended_guest_cannot_book() {
    let (mut service, _host_id, guest_id, listing_id, stay) = setup_listing("oaxaca", 2, 10_000);
    service.suspend_user(guest_id).expect("suspend");

    let error = service
        .book(guest_id, listing_id, stay, 2)
        .expect_err("book");

    assert_eq!(error, AirbnbError::UserSuspended { user_id: guest_id });
    assert_eq!(service.metrics().bookings_rejected, 1);
}

#[test]
fn booking_revalidates_stale_search_results() {
    let (mut service, _host_id, guest_id, listing_id, stay) = setup_listing("vallarta", 2, 10_000);
    let other_guest = service
        .register_user("Linus Guest", UserRole::Guest)
        .expect("other guest");
    let results = service
        .search(SearchQuery {
            city: "vallarta".to_string(),
            guests: 2,
            stay,
            max_total_price_cents: None,
        })
        .expect("search");

    service
        .book(guest_id, results[0].listing_id, stay, 2)
        .expect("first");
    let error = service
        .book(other_guest.id, listing_id, stay, 2)
        .expect_err("stale");

    assert_eq!(
        error,
        AirbnbError::InsufficientAvailability {
            requested: 1,
            available: 0
        }
    );
}

#[test]
fn cancelling_reservation_releases_listing_calendar() {
    let (mut service, _host_id, guest_id, listing_id, stay) = setup_listing("merida", 2, 10_000);
    let reservation = service.book(guest_id, listing_id, stay, 2).expect("book");

    service
        .cancel_reservation(reservation.id)
        .expect("cancel reservation");
    let results = service
        .search(SearchQuery {
            city: "merida".to_string(),
            guests: 2,
            stay,
            max_total_price_cents: None,
        })
        .expect("search");

    assert_eq!(
        service.reservation(reservation.id).unwrap().status,
        AirbnbReservationStatus::Cancelled
    );
    assert_eq!(results.len(), 1);
    assert_eq!(service.metrics().bookings_cancelled, 1);
}

#[test]
fn capacity_is_checked_on_direct_booking() {
    let (mut service, _host_id, guest_id, listing_id, stay) = setup_listing("queretaro", 2, 10_000);

    let error = service
        .book(guest_id, listing_id, stay, 3)
        .expect_err("capacity");

    assert_eq!(
        error,
        AirbnbError::CapacityExceeded {
            requested: 3,
            capacity: 2
        }
    );
}
