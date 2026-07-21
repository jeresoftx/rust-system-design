use rust_system_design::google_docs::{EditKind, GoogleDocsService};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut service = GoogleDocsService::new();
    let doc = service.create_document("Notas", "Hola")?;
    let ada = service.register_collaborator("Ada")?;
    let alan = service.register_collaborator("Alan")?;

    service.apply_operation(
        doc.id,
        ada.id,
        0,
        4,
        EditKind::Insert {
            text: ", Rust".into(),
        },
    )?;
    let stale = service.apply_operation(
        doc.id,
        alan.id,
        0,
        0,
        EditKind::Insert {
            text: "Equipo: ".into(),
        },
    )?;

    println!(
        "versión {} texto: {}",
        stale.version,
        service.document(doc.id).expect("documento").text
    );

    Ok(())
}
