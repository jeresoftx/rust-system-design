use rust_system_design::uber::{Location, RideState, UberConfig, UberError, UberService};

#[test]
fn updates_driver_location_and_uses_new_cell_for_matching() {
    let mut service = UberService::with_config(UberConfig {
        search_radius_cells: 0,
        max_match_distance_sq: 25,
        ..UberConfig::default()
    });
    let rider = service.register_rider("Ada").expect("rider");
    let driver = service
        .register_driver("Grace", Location::new(100, 100))
        .expect("driver");

    service
        .update_driver_location(driver.id, Location::new(1, 1))
        .expect("update location");
    let ride = service
        .request_ride(rider.id, Location::new(0, 0), Location::new(10, 10))
        .expect("match cercano");

    assert_eq!(ride.driver_id, Some(driver.id));
    assert_eq!(service.metrics().driver_location_updates, 1);
}

#[test]
fn busy_driver_is_not_assigned_twice() {
    let mut service = UberService::new();
    let first_rider = service.register_rider("Ada").expect("first");
    let second_rider = service.register_rider("Alan").expect("second");
    let driver = service
        .register_driver("Grace", Location::new(0, 0))
        .expect("driver");

    let first_ride = service
        .request_ride(first_rider.id, Location::new(0, 1), Location::new(9, 9))
        .expect("first ride");
    let second = service.request_ride(second_rider.id, Location::new(0, 1), Location::new(9, 9));

    assert_eq!(first_ride.driver_id, Some(driver.id));
    assert_eq!(
        second.expect_err("driver ocupado"),
        UberError::NoDriversAvailable
    );
}

#[test]
fn cancelling_assigned_ride_releases_driver() {
    let mut service = UberService::new();
    let rider = service.register_rider("Ada").expect("rider");
    let driver = service
        .register_driver("Grace", Location::new(0, 0))
        .expect("driver");
    let ride = service
        .request_ride(rider.id, Location::new(1, 1), Location::new(5, 5))
        .expect("ride");

    let cancelled = service.cancel_ride(ride.id).expect("cancelled");

    assert_eq!(cancelled.state, RideState::Cancelled);
    assert!(service.driver(driver.id).expect("driver").available);
    assert_eq!(service.metrics().rides_cancelled, 1);
}

#[test]
fn ride_events_record_assignment_and_transitions() {
    let mut service = UberService::new();
    let rider = service.register_rider("Ada").expect("rider");
    service
        .register_driver("Grace", Location::new(0, 0))
        .expect("driver");
    let ride = service
        .request_ride(rider.id, Location::new(0, 0), Location::new(10, 10))
        .expect("ride");

    service.accept_ride(ride.id).expect("accepted");
    service.start_ride(ride.id).expect("started");

    let states: Vec<_> = service
        .events(ride.id)
        .iter()
        .map(|event| event.state)
        .collect();

    assert_eq!(
        states,
        vec![
            RideState::Assigned,
            RideState::Accepted,
            RideState::InProgress
        ]
    );
}

#[test]
fn matching_respects_max_distance() {
    let mut service = UberService::with_config(UberConfig {
        search_radius_cells: 10,
        max_match_distance_sq: 4,
        ..UberConfig::default()
    });
    let rider = service.register_rider("Ada").expect("rider");
    service
        .register_driver("Grace", Location::new(10, 10))
        .expect("driver");

    let error = service
        .request_ride(rider.id, Location::new(0, 0), Location::new(1, 1))
        .expect_err("demasiado lejos");

    assert_eq!(error, UberError::NoDriversAvailable);
}

#[test]
fn unknown_rider_is_rejected() {
    let mut service = UberService::new();
    service
        .register_driver("Grace", Location::new(0, 0))
        .expect("driver");

    let error = service
        .request_ride(999, Location::new(0, 0), Location::new(1, 1))
        .expect_err("rider inexistente");

    assert_eq!(error, UberError::UnknownRider { rider_id: 999 });
}
