use rust_system_design::dropbox::{DropboxConfig, DropboxService};
use std::time::Instant;

fn main() {
    let mut service =
        DropboxService::with_config(DropboxConfig { chunk_size: 1024 }).expect("service");
    let device = service.register_device("laptop").expect("device");
    let bytes = vec![42_u8; 64 * 1024];

    let started = Instant::now();
    for index in 0..500 {
        let path = format!("/bench/file-{index}.bin");
        service
            .upload_file(device.id, &path, None, &bytes)
            .expect("upload");
    }
    let elapsed = started.elapsed();
    println!("dropbox upload baseline: 500 uploads of 64KiB in {elapsed:?}");
}
