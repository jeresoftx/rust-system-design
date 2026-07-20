use rust_system_design::uber::{Location, UberService};

fn main() {
    let mut service = UberService::new();

    let rider = service.register_rider("Ada").expect("rider válido");
    let driver = service
        .register_driver("Grace", Location::new(0, 0))
        .expect("driver válido");

    let ride = service
        .request_ride(rider.id, Location::new(1, 1), Location::new(12, 12))
        .expect("driver cercano disponible");

    println!(
        "ride={} rider={} driver={:?} matched_driver={}",
        ride.id, ride.rider_id, ride.driver_id, driver.id
    );
}
