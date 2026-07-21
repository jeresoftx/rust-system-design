use rust_system_design::dropbox::{DropboxConfig, DropboxError, DropboxService, SyncChangeKind};

#[test]
fn sync_returns_changes_after_known_revision() {
    let mut service = DropboxService::new();
    let laptop = service.register_device("laptop").expect("laptop");
    let phone = service.register_device("phone").expect("phone");
    let first = service
        .upload_file(laptop.id, "/docs/a.md", None, b"a")
        .expect("first");
    let second = service
        .upload_file(laptop.id, "/docs/b.md", None, b"b")
        .expect("second");

    let changes = service
        .sync_changes(phone.id, first.revision_id)
        .expect("sync changes");

    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].revision_id, second.revision_id);
    assert_eq!(changes[0].kind, SyncChangeKind::Created);
    assert_eq!(service.metrics().changes_returned, 1);
}

#[test]
fn update_with_current_base_replaces_visible_file() {
    let mut service = DropboxService::new();
    let laptop = service.register_device("laptop").expect("laptop");
    let first = service
        .upload_file(laptop.id, "/docs/rust.md", None, b"v1")
        .expect("first");

    let second = service
        .upload_file(laptop.id, "/docs/rust.md", Some(first.revision_id), b"v2")
        .expect("second");
    let metadata = service.file_metadata("/docs/rust.md").expect("metadata");

    assert!(!second.conflict_created);
    assert_eq!(metadata.current_revision, second.revision_id);
    assert_eq!(
        service.download_file("/docs/rust.md").expect("download"),
        b"v2"
    );
}

#[test]
fn stale_base_creates_conflict_change_without_losing_current_file() {
    let mut service = DropboxService::new();
    let laptop = service.register_device("laptop").expect("laptop");
    let phone = service.register_device("phone").expect("phone");
    let first = service
        .upload_file(laptop.id, "/docs/rust.md", None, b"v1")
        .expect("first");
    service
        .upload_file(laptop.id, "/docs/rust.md", Some(first.revision_id), b"v2")
        .expect("second");

    let conflict = service
        .upload_file(phone.id, "/docs/rust.md", Some(first.revision_id), b"phone")
        .expect("conflict");
    let changes = service
        .sync_changes(laptop.id, first.revision_id)
        .expect("changes");

    assert!(conflict.conflict_created);
    assert_eq!(
        service.download_file("/docs/rust.md").expect("current"),
        b"v2"
    );
    assert_eq!(
        service
            .download_file(&conflict.path)
            .expect("conflict bytes"),
        b"phone"
    );
    assert!(changes
        .iter()
        .any(|change| change.kind == SyncChangeKind::ConflictCreated));
}

#[test]
fn fixed_chunks_make_partial_reuse_visible() {
    let mut service =
        DropboxService::with_config(DropboxConfig { chunk_size: 4 }).expect("service");
    let laptop = service.register_device("laptop").expect("laptop");
    let first = service
        .upload_file(laptop.id, "/docs/rust.md", None, b"aaaabbbbcccc")
        .expect("first");

    service
        .upload_file(
            laptop.id,
            "/docs/rust.md",
            Some(first.revision_id),
            b"aaaabbbbdddd",
        )
        .expect("second");

    assert_eq!(service.metrics().chunks_seen, 6);
    assert_eq!(service.metrics().chunks_reused, 2);
    assert_eq!(service.metrics().chunks_stored, 4);
}

#[test]
fn sync_rejects_unknown_revision() {
    let mut service = DropboxService::new();
    let laptop = service.register_device("laptop").expect("laptop");

    let error = service
        .sync_changes(laptop.id, 999)
        .expect_err("unknown revision");

    assert_eq!(error, DropboxError::UnknownRevision { revision_id: 999 });
}

#[test]
fn empty_path_is_rejected_before_uploading_chunks() {
    let mut service = DropboxService::new();
    let laptop = service.register_device("laptop").expect("laptop");

    let error = service
        .upload_file(laptop.id, "   ", None, b"data")
        .expect_err("empty path");

    assert_eq!(error, DropboxError::EmptyText);
    assert_eq!(service.metrics().chunks_seen, 0);
}
