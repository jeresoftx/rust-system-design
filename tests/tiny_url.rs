use rust_system_design::tiny_url::{TinyUrlConfig, TinyUrlError, TinyUrlService};

#[test]
fn cache_records_miss_then_hit_for_existing_code() {
    let mut service = TinyUrlService::new();
    let link = service
        .create_link("academy", "https://example.com/cache")
        .expect("link válido");

    assert_eq!(
        service.resolve(&link.code).expect("primer resolve"),
        link.long_url
    );
    assert_eq!(
        service.resolve(&link.code).expect("segundo resolve"),
        link.long_url
    );

    let metrics = service.metrics();
    assert_eq!(metrics.cache_misses, 1);
    assert_eq!(metrics.cache_hits, 1);
    assert_eq!(metrics.redirect_hits, 2);
    assert_eq!(service.link(&link.code).expect("link existente").visits, 2);
}

#[test]
fn cache_capacity_evicts_oldest_entry() {
    let mut service = TinyUrlService::with_config(TinyUrlConfig {
        cache_capacity: 1,
        ..TinyUrlConfig::default()
    });
    let first = service
        .create_link("academy", "https://example.com/first")
        .expect("primer link");
    let second = service
        .create_link("academy", "https://example.com/second")
        .expect("segundo link");

    service.resolve(&first.code).expect("cachea primero");
    service.resolve(&second.code).expect("expulsa primero");
    service
        .resolve(&first.code)
        .expect("vuelve a consultar repositorio");

    assert_eq!(service.cache_len(), 1);
    assert_eq!(service.metrics().cache_misses, 3);
}

#[test]
fn same_url_from_different_owners_gets_different_codes() {
    let mut service = TinyUrlService::new();

    let academy = service
        .create_link("academy", "https://example.com/shared")
        .expect("link academy");
    let travel = service
        .create_link("travel", "https://example.com/shared")
        .expect("link travel");

    assert_ne!(academy.code, travel.code);
    assert_eq!(service.link_count(), 2);
}

#[test]
fn rejects_empty_owner_without_touching_storage() {
    let mut service = TinyUrlService::new();

    let error = service
        .create_link("   ", "https://example.com")
        .expect_err("dueño vacío");

    assert_eq!(error, TinyUrlError::EmptyOwner);
    assert_eq!(service.link_count(), 0);
}

#[test]
fn rejects_codes_outside_base62() {
    let mut service = TinyUrlService::new();

    let error = service.resolve("abc-123").expect_err("código inválido");

    assert_eq!(error, TinyUrlError::InvalidCode);
}

#[test]
fn rejects_too_long_url() {
    let mut service = TinyUrlService::with_config(TinyUrlConfig {
        max_url_len: 20,
        ..TinyUrlConfig::default()
    });

    let error = service
        .create_link("academy", "https://example.com/too-long")
        .expect_err("URL demasiado larga");

    assert_eq!(error, TinyUrlError::UrlTooLong { max_len: 20 });
    assert_eq!(service.metrics().invalid_urls, 1);
}

#[test]
fn trims_base_url_before_building_short_url() {
    let mut service = TinyUrlService::with_config(TinyUrlConfig {
        base_url: "https://short.example/".to_string(),
        ..TinyUrlConfig::default()
    });

    let link = service
        .create_link("academy", "https://example.com")
        .expect("link válido");

    assert_eq!(link.short_url, "https://short.example/1");
}
