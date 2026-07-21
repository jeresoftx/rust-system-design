use rust_system_design::dropbox::DropboxService;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut service = DropboxService::new();
    let laptop = service.register_device("laptop")?;
    let phone = service.register_device("phone")?;

    let first = service.upload_file(laptop.id, "/docs/rust.md", None, b"version uno")?;
    service.upload_file(
        laptop.id,
        "/docs/rust.md",
        Some(first.revision_id),
        b"version dos",
    )?;
    let conflict = service.upload_file(
        phone.id,
        "/docs/rust.md",
        Some(first.revision_id),
        b"version desde telefono",
    )?;

    println!(
        "revision {} escrita en {} conflict={}",
        conflict.revision_id, conflict.path, conflict.conflict_created
    );

    Ok(())
}
