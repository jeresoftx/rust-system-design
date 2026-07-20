use rust_system_design::tiny_url::TinyUrlService;
use std::hint::black_box;
use std::time::Instant;

fn main() {
    let mut service = TinyUrlService::new();
    let link = service
        .create_link("academy", "https://example.com/hot-cache")
        .expect("link válido");

    service.resolve(&link.code).expect("calienta caché");

    let iterations = 10_000;
    let start = Instant::now();
    for _ in 0..iterations {
        let resolved = service
            .resolve(black_box(&link.code))
            .expect("código existente");
        black_box(resolved);
    }
    let elapsed = start.elapsed();

    assert_eq!(service.metrics().cache_hits, iterations);
    println!("tiny_url hot-cache baseline: {iterations} resolves in {elapsed:?}");
}
