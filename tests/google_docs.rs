use rust_system_design::google_docs::{EditKind, GoogleDocsError, GoogleDocsService};

#[test]
fn sync_returns_operations_after_known_version() {
    let mut service = GoogleDocsService::new();
    let doc = service.create_document("Notas", "").expect("doc");
    let ada = service.register_collaborator("Ada").expect("ada");
    let first = service
        .apply_operation(doc.id, ada.id, 0, 0, EditKind::Insert { text: "A".into() })
        .expect("first");
    let second = service
        .apply_operation(
            doc.id,
            ada.id,
            first.version,
            1,
            EditKind::Insert { text: "B".into() },
        )
        .expect("second");

    let operations = service
        .sync_operations(doc.id, first.version)
        .expect("sync");

    assert_eq!(operations.len(), 1);
    assert_eq!(operations[0].version, second.version);
    assert_eq!(service.metrics().operations_returned, 1);
}

#[test]
fn stale_delete_is_transformed_after_server_insert() {
    let mut service = GoogleDocsService::new();
    let doc = service.create_document("Notas", "abcdef").expect("doc");
    let ada = service.register_collaborator("Ada").expect("ada");
    let alan = service.register_collaborator("Alan").expect("alan");

    service
        .apply_operation(doc.id, ada.id, 0, 0, EditKind::Insert { text: "XX".into() })
        .expect("server insert");
    let stale_delete = service
        .apply_operation(doc.id, alan.id, 0, 2, EditKind::Delete { len: 2 })
        .expect("stale delete");

    assert_eq!(stale_delete.original_position, 2);
    assert_eq!(stale_delete.transformed_position, 4);
    assert_eq!(service.document(doc.id).expect("doc").text, "XXabef");
    assert_eq!(service.metrics().operations_transformed, 1);
}

#[test]
fn active_presence_expires_after_later_activity() {
    let mut service = GoogleDocsService::new();
    let doc = service.create_document("Notas", "Hola").expect("doc");
    let ada = service.register_collaborator("Ada").expect("ada");
    let alan = service.register_collaborator("Alan").expect("alan");

    service.update_presence(doc.id, ada.id, 1).expect("ada");
    service.update_presence(doc.id, alan.id, 2).expect("alan");

    let active = service.active_presence(doc.id, 0).expect("active");

    assert_eq!(active.len(), 1);
    assert_eq!(active[0].collaborator_id, alan.id);
}

#[test]
fn invalid_delete_range_is_rejected_without_advancing_version() {
    let mut service = GoogleDocsService::new();
    let doc = service.create_document("Notas", "Hola").expect("doc");
    let ada = service.register_collaborator("Ada").expect("ada");

    let error = service
        .apply_operation(doc.id, ada.id, 0, 3, EditKind::Delete { len: 9 })
        .expect_err("invalid delete");

    assert_eq!(
        error,
        GoogleDocsError::InvalidDeleteRange {
            position: 3,
            len: 9,
            text_len: 4
        }
    );
    assert_eq!(service.document(doc.id).expect("doc").version, 0);
    assert_eq!(service.metrics().operations_rejected, 1);
}

#[test]
fn unknown_collaborator_is_rejected() {
    let mut service = GoogleDocsService::new();
    let doc = service.create_document("Notas", "").expect("doc");

    let error = service
        .apply_operation(doc.id, 999, 0, 0, EditKind::Insert { text: "x".into() })
        .expect_err("unknown collaborator");

    assert_eq!(
        error,
        GoogleDocsError::UnknownCollaborator {
            collaborator_id: 999
        }
    );
}

#[test]
fn sync_rejects_future_version() {
    let mut service = GoogleDocsService::new();
    let doc = service.create_document("Notas", "").expect("doc");

    let error = service.sync_operations(doc.id, 7).expect_err("future sync");

    assert_eq!(
        error,
        GoogleDocsError::FutureVersion {
            base_version: 7,
            current_version: 0
        }
    );
}
