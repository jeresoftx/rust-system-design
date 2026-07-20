//! Modelo educativo de TinyURL.
//!
//! Este módulo no implementa HTTP ni almacenamiento persistente. Modela las
//! decisiones centrales del capítulo: validación, generación de códigos,
//! resolución, caché, límites por dueño y métricas.

use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::fmt;

const BASE62: &[u8; 62] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// Configuración pedagógica del acortador.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TinyUrlConfig {
    /// URL base usada para construir enlaces cortos visibles.
    pub base_url: String,
    /// Longitud máxima aceptada para la URL original.
    pub max_url_len: usize,
    /// Capacidad máxima de la caché en memoria.
    pub cache_capacity: usize,
    /// Máximo de enlaces que un dueño puede crear.
    pub max_links_per_owner: usize,
    /// Reutiliza el código cuando el mismo dueño acorta la misma URL.
    pub reuse_existing_per_owner: bool,
}

impl Default for TinyUrlConfig {
    fn default() -> Self {
        Self {
            base_url: "https://jrs.dev".to_string(),
            max_url_len: 2_048,
            cache_capacity: 128,
            max_links_per_owner: 1_000,
            reuse_existing_per_owner: true,
        }
    }
}

/// Enlace corto almacenado por el modelo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortLink {
    /// Código público usado en la redirección.
    pub code: String,
    /// URL corta completa.
    pub short_url: String,
    /// URL original validada.
    pub long_url: String,
    /// Dueño lógico del enlace.
    pub owner: String,
    /// Tiempo lógico de creación.
    pub created_at_tick: u64,
    /// Conteo pedagógico de resoluciones exitosas.
    pub visits: u64,
}

/// Métricas mínimas para razonar sobre operación.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TinyUrlMetrics {
    /// Enlaces nuevos creados.
    pub links_created: u64,
    /// Creaciones que reutilizaron un enlace existente del mismo dueño.
    pub links_reused: u64,
    /// Redirecciones exitosas.
    pub redirect_hits: u64,
    /// Redirecciones fallidas por código inexistente.
    pub redirect_misses: u64,
    /// Resoluciones atendidas desde caché.
    pub cache_hits: u64,
    /// Resoluciones que tuvieron que leer el repositorio.
    pub cache_misses: u64,
    /// Creaciones rechazadas por límite de dueño.
    pub rate_limited_creations: u64,
    /// URLs rechazadas por validación.
    pub invalid_urls: u64,
}

/// Errores esperados del modelo TinyURL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TinyUrlError {
    /// El dueño lógico viene vacío.
    EmptyOwner,
    /// La URL no cumple las reglas mínimas del modelo.
    InvalidUrl,
    /// La URL excede el límite configurado.
    UrlTooLong { max_len: usize },
    /// El dueño superó su cuota de creación.
    RateLimited { owner: String },
    /// El código de redirección es inválido.
    InvalidCode,
    /// El código no existe en el repositorio.
    NotFound { code: String },
    /// Se agotó el espacio de IDs del modelo.
    CapacityExhausted,
}

impl fmt::Display for TinyUrlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyOwner => write!(f, "el dueño no puede estar vacío"),
            Self::InvalidUrl => write!(f, "la URL debe iniciar con http:// o https://"),
            Self::UrlTooLong { max_len } => {
                write!(f, "la URL excede la longitud máxima de {max_len} bytes")
            }
            Self::RateLimited { owner } => {
                write!(f, "el dueño {owner} superó su cuota de creación")
            }
            Self::InvalidCode => write!(f, "el código corto es inválido"),
            Self::NotFound { code } => write!(f, "el código {code} no existe"),
            Self::CapacityExhausted => write!(f, "se agotó el espacio de IDs"),
        }
    }
}

impl Error for TinyUrlError {}

/// Servicio en memoria para estudiar decisiones de TinyURL.
#[derive(Debug, Clone)]
pub struct TinyUrlService {
    config: TinyUrlConfig,
    next_id: u64,
    logical_clock: u64,
    links: HashMap<String, ShortLink>,
    owner_url_index: HashMap<String, String>,
    owner_counts: HashMap<String, usize>,
    cache: RedirectCache,
    metrics: TinyUrlMetrics,
}

impl TinyUrlService {
    /// Crea un servicio con configuración por defecto.
    #[must_use]
    pub fn new() -> Self {
        Self::with_config(TinyUrlConfig::default())
    }

    /// Crea un servicio con configuración explícita.
    #[must_use]
    pub fn with_config(config: TinyUrlConfig) -> Self {
        let cache = RedirectCache::new(config.cache_capacity);

        Self {
            config,
            next_id: 1,
            logical_clock: 0,
            links: HashMap::new(),
            owner_url_index: HashMap::new(),
            owner_counts: HashMap::new(),
            cache,
            metrics: TinyUrlMetrics::default(),
        }
    }

    /// Crea o reutiliza un enlace corto para el mismo dueño y URL.
    ///
    /// # Errors
    ///
    /// Devuelve error si el dueño está vacío, la URL es inválida, la URL excede
    /// el máximo configurado, el dueño agotó su cuota o el contador interno se
    /// desbordó.
    pub fn create_link(&mut self, owner: &str, long_url: &str) -> Result<ShortLink, TinyUrlError> {
        let owner = normalize_owner(owner)?;
        let long_url = self.validate_url(long_url)?;
        let owner_url_key = owner_url_key(&owner, &long_url);

        if self.config.reuse_existing_per_owner {
            if let Some(code) = self.owner_url_index.get(&owner_url_key) {
                let link = self
                    .links
                    .get(code)
                    .expect("owner_url_index debe apuntar a un enlace existente");
                self.metrics.links_reused += 1;
                return Ok(link.clone());
            }
        }

        self.ensure_owner_can_create(&owner)?;

        let id = self.next_id;
        self.next_id = self
            .next_id
            .checked_add(1)
            .ok_or(TinyUrlError::CapacityExhausted)?;
        self.logical_clock += 1;

        let code = encode_base62(id);
        let short_url = build_short_url(&self.config.base_url, &code);
        let link = ShortLink {
            code: code.clone(),
            short_url,
            long_url: long_url.clone(),
            owner: owner.clone(),
            created_at_tick: self.logical_clock,
            visits: 0,
        };

        self.links.insert(code.clone(), link.clone());
        self.owner_url_index.insert(owner_url_key, code);
        *self.owner_counts.entry(owner).or_default() += 1;
        self.metrics.links_created += 1;

        Ok(link)
    }

    /// Resuelve un código corto hacia su URL original.
    ///
    /// # Errors
    ///
    /// Devuelve error si el código está vacío, contiene caracteres fuera de
    /// base62 o no existe.
    pub fn resolve(&mut self, code: &str) -> Result<String, TinyUrlError> {
        validate_code(code)?;

        if let Some(long_url) = self.cache.get(code) {
            self.metrics.cache_hits += 1;
            self.metrics.redirect_hits += 1;
            if let Some(link) = self.links.get_mut(code) {
                link.visits += 1;
            }
            return Ok(long_url);
        }

        self.metrics.cache_misses += 1;
        let Some(link) = self.links.get_mut(code) else {
            self.metrics.redirect_misses += 1;
            return Err(TinyUrlError::NotFound {
                code: code.to_string(),
            });
        };

        link.visits += 1;
        self.metrics.redirect_hits += 1;
        let long_url = link.long_url.clone();
        self.cache.insert(code.to_string(), long_url.clone());

        Ok(long_url)
    }

    /// Devuelve un enlace por código sin modificar métricas.
    #[must_use]
    pub fn link(&self, code: &str) -> Option<&ShortLink> {
        self.links.get(code)
    }

    /// Número de enlaces almacenados.
    #[must_use]
    pub fn link_count(&self) -> usize {
        self.links.len()
    }

    /// Número de entradas actualmente guardadas en caché.
    #[must_use]
    pub fn cache_len(&self) -> usize {
        self.cache.len()
    }

    /// Métricas acumuladas.
    #[must_use]
    pub fn metrics(&self) -> TinyUrlMetrics {
        self.metrics
    }

    fn validate_url(&mut self, long_url: &str) -> Result<String, TinyUrlError> {
        let long_url = long_url.trim();
        if long_url.len() > self.config.max_url_len {
            self.metrics.invalid_urls += 1;
            return Err(TinyUrlError::UrlTooLong {
                max_len: self.config.max_url_len,
            });
        }

        if long_url.is_empty()
            || long_url.chars().any(char::is_whitespace)
            || !(long_url.starts_with("http://") || long_url.starts_with("https://"))
        {
            self.metrics.invalid_urls += 1;
            return Err(TinyUrlError::InvalidUrl);
        }

        Ok(long_url.to_string())
    }

    fn ensure_owner_can_create(&mut self, owner: &str) -> Result<(), TinyUrlError> {
        let current = self.owner_counts.get(owner).copied().unwrap_or_default();
        if current >= self.config.max_links_per_owner {
            self.metrics.rate_limited_creations += 1;
            return Err(TinyUrlError::RateLimited {
                owner: owner.to_string(),
            });
        }

        Ok(())
    }
}

impl Default for TinyUrlService {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
struct RedirectCache {
    capacity: usize,
    order: VecDeque<String>,
    entries: HashMap<String, String>,
}

impl RedirectCache {
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            order: VecDeque::new(),
            entries: HashMap::new(),
        }
    }

    fn get(&self, code: &str) -> Option<String> {
        self.entries.get(code).cloned()
    }

    fn insert(&mut self, code: String, long_url: String) {
        if self.capacity == 0 {
            return;
        }

        if let Entry::Occupied(mut entry) = self.entries.entry(code.clone()) {
            entry.insert(long_url);
            return;
        }

        while self.entries.len() >= self.capacity {
            if let Some(oldest) = self.order.pop_front() {
                self.entries.remove(&oldest);
            } else {
                break;
            }
        }

        self.order.push_back(code.clone());
        self.entries.insert(code, long_url);
    }

    fn len(&self) -> usize {
        self.entries.len()
    }
}

fn normalize_owner(owner: &str) -> Result<String, TinyUrlError> {
    let owner = owner.trim();
    if owner.is_empty() {
        return Err(TinyUrlError::EmptyOwner);
    }

    Ok(owner.to_string())
}

fn validate_code(code: &str) -> Result<(), TinyUrlError> {
    if code.is_empty() || !code.bytes().all(|byte| BASE62.contains(&byte)) {
        return Err(TinyUrlError::InvalidCode);
    }

    Ok(())
}

fn owner_url_key(owner: &str, long_url: &str) -> String {
    format!("{owner}\0{long_url}")
}

fn build_short_url(base_url: &str, code: &str) -> String {
    format!("{}/{code}", base_url.trim_end_matches('/'))
}

fn encode_base62(mut value: u64) -> String {
    if value == 0 {
        return "0".to_string();
    }

    let mut chars = Vec::new();
    while value > 0 {
        let index = (value % 62) as usize;
        chars.push(BASE62[index] as char);
        value /= 62;
    }

    chars.iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::{TinyUrlConfig, TinyUrlError, TinyUrlService};

    #[test]
    fn creates_and_resolves_link() {
        let mut service = TinyUrlService::new();

        let link = service
            .create_link("academy", "https://example.com/articles/system-design")
            .expect("link válido");

        assert_eq!(link.code, "1");
        assert_eq!(link.short_url, "https://jrs.dev/1");
        assert_eq!(
            service.resolve(&link.code).expect("redirección existente"),
            "https://example.com/articles/system-design"
        );
        assert_eq!(service.link(&link.code).expect("link guardado").visits, 1);
    }

    #[test]
    fn rejects_invalid_url() {
        let mut service = TinyUrlService::new();

        let error = service
            .create_link("academy", "ftp://example.com")
            .expect_err("solo http y https");

        assert_eq!(error, TinyUrlError::InvalidUrl);
        assert_eq!(service.metrics().invalid_urls, 1);
    }

    #[test]
    fn reuses_same_url_for_same_owner() {
        let mut service = TinyUrlService::new();

        let first = service
            .create_link("academy", "https://example.com/rust")
            .expect("primer enlace");
        let second = service
            .create_link("academy", "https://example.com/rust")
            .expect("reutilización");

        assert_eq!(first.code, second.code);
        assert_eq!(service.link_count(), 1);
        assert_eq!(service.metrics().links_reused, 1);
    }

    #[test]
    fn limits_creations_per_owner() {
        let mut service = TinyUrlService::with_config(TinyUrlConfig {
            max_links_per_owner: 1,
            ..TinyUrlConfig::default()
        });

        service
            .create_link("academy", "https://example.com/one")
            .expect("primer enlace");

        let error = service
            .create_link("academy", "https://example.com/two")
            .expect_err("límite por dueño");

        assert_eq!(
            error,
            TinyUrlError::RateLimited {
                owner: "academy".to_string()
            }
        );
        assert_eq!(service.metrics().rate_limited_creations, 1);
    }
}
