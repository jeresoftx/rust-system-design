use rust_system_design::google_docs::{EditKind, GoogleDocsService};
use std::time::Instant;

fn main() {
    let mut service = GoogleDocsService::new();
    let doc = service.create_document("Benchmark", "").expect("doc");
    let ada = service.register_collaborator("Ada").expect("ada");

    let started = Instant::now();
    for _ in 0..1_000 {
        service
            .apply_operation(doc.id, ada.id, 0, 0, EditKind::Insert { text: "x".into() })
            .expect("operation");
    }
    let elapsed = started.elapsed();
    println!("google docs operations baseline: 1000 stale inserts in {elapsed:?}");
}
