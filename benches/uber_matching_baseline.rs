use rust_system_design::uber::{Location, UberConfig, UberService};
use std::hint::black_box;
use std::time::Instant;

fn main() {
    let mut service = UberService::with_config(UberConfig {
        cell_size: 10,
        search_radius_cells: 2,
        max_match_distance_sq: 2_500,
    });

    for index in 0..1_000 {
        let x = (index % 100) * 2;
        let y = (index / 100) * 2;
        service
            .register_driver(&format!("driver-{index}"), Location::new(x, y))
            .expect("driver válido");
    }

    let iterations = 200;
    let start = Instant::now();
    for index in 0..iterations {
        let rider = service
            .register_rider(&format!("rider-{index}"))
            .expect("rider válido");
        let pickup = Location::new(index % 50, index % 20);
        let result = service.request_ride(rider.id, black_box(pickup), Location::new(200, 200));
        let _ = black_box(result);
    }
    let elapsed = start.elapsed();

    println!("uber matching baseline: {iterations} requests in {elapsed:?}");
}
