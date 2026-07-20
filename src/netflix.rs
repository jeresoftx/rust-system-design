//! Modelo educativo de Netflix.
//!
//! El módulo estudia catálogo regional, recomendaciones explicables, selección
//! de variante de video, capacidad de CDN y sesiones de reproducción sin
//! streaming real ni dependencias externas.

use std::cmp::Reverse;
use std::collections::{BTreeSet, HashMap};
use std::error::Error;
use std::fmt;

/// Identificador lógico de perfil.
pub type ProfileId = u64;
/// Identificador lógico de título.
pub type TitleId = u64;
/// Identificador lógico de nodo CDN.
pub type CdnNodeId = u64;
/// Identificador lógico de sesión.
pub type SessionId = u64;

/// Calidad de una variante de video.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VideoQuality {
    /// Calidad baja para ancho de banda limitado.
    Low,
    /// Calidad media.
    Medium,
    /// Alta definición.
    High,
    /// Ultra alta definición.
    Ultra,
}

/// Dispositivo lógico usado para iniciar reproducción.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Device {
    /// Teléfono o tableta.
    Mobile,
    /// Navegador web.
    Web,
    /// Televisión.
    Tv,
}

/// Estado de una sesión de reproducción.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackState {
    /// Sesión activa y reproduciendo.
    Playing,
    /// Sesión pausada.
    Paused,
    /// Sesión completada.
    Completed,
    /// Sesión cancelada.
    Cancelled,
}

/// Perfil registrado.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Profile {
    /// Identificador interno.
    pub id: ProfileId,
    /// Nombre visible.
    pub name: String,
    /// Región lógica del perfil.
    pub region: String,
    /// Afinidad por género derivada de sesiones completadas.
    pub genre_affinity: HashMap<String, u32>,
    /// Tiempo lógico de creación.
    pub created_at_tick: u64,
}

/// Título del catálogo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Title {
    /// Identificador interno.
    pub id: TitleId,
    /// Nombre visible.
    pub name: String,
    /// Géneros normalizados.
    pub genres: Vec<String>,
    /// Regiones donde el título puede verse.
    pub available_regions: BTreeSet<String>,
    /// Popularidad pedagógica para ordenar recomendaciones.
    pub popularity: u64,
    /// Clasificación de madurez simplificada.
    pub maturity_rating: u8,
}

/// Variante de video asociada a un título.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VideoVariant {
    /// Título al que pertenece.
    pub title_id: TitleId,
    /// Calidad de la variante.
    pub quality: VideoQuality,
    /// Ancho de banda mínimo requerido.
    pub bitrate_kbps: u32,
}

/// Nodo de CDN disponible para entregar contenido.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CdnNode {
    /// Identificador interno.
    pub id: CdnNodeId,
    /// Nombre visible.
    pub name: String,
    /// Región que atiende.
    pub region: String,
    /// Sesiones simultáneas soportadas en el modelo.
    pub capacity: u32,
    /// Sesiones activas asignadas.
    pub active_sessions: u32,
    /// Salud operativa.
    pub healthy: bool,
}

/// Sesión de reproducción.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaybackSession {
    /// Identificador interno.
    pub id: SessionId,
    /// Perfil que reproduce.
    pub profile_id: ProfileId,
    /// Título reproducido.
    pub title_id: TitleId,
    /// Variante elegida.
    pub quality: VideoQuality,
    /// Bitrate elegido.
    pub bitrate_kbps: u32,
    /// Dispositivo de reproducción.
    pub device: Device,
    /// CDN asignada.
    pub cdn_node_id: CdnNodeId,
    /// Estado actual.
    pub state: PlaybackState,
    /// Tiempo lógico de creación.
    pub created_at_tick: u64,
}

/// Recomendación explicable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recommendation {
    /// Título recomendado.
    pub title_id: TitleId,
    /// Puntaje determinista usado para ordenar.
    pub score: u64,
    /// Razón pedagógica visible.
    pub reason: String,
}

/// Evento auditable de reproducción.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaybackEvent {
    /// Sesión relacionada.
    pub session_id: SessionId,
    /// Estado registrado.
    pub state: PlaybackState,
    /// Descripción pedagógica.
    pub message: String,
    /// Tiempo lógico del evento.
    pub created_at_tick: u64,
}

/// Métricas mínimas del modelo.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NetflixMetrics {
    /// Perfiles registrados.
    pub profiles_registered: u64,
    /// Títulos registrados.
    pub titles_registered: u64,
    /// Nodos CDN registrados.
    pub cdn_nodes_registered: u64,
    /// Lecturas de catálogo.
    pub catalog_reads: u64,
    /// Lecturas de recomendaciones.
    pub recommendation_reads: u64,
    /// Solicitudes de reproducción.
    pub playback_requests: u64,
    /// Sesiones iniciadas.
    pub sessions_started: u64,
    /// Fallas de reproducción.
    pub playback_failures: u64,
    /// Asignaciones de CDN.
    pub cdn_assignments: u64,
    /// Rechazos por capacidad o salud de CDN.
    pub cdn_capacity_rejections: u64,
    /// Degradaciones de calidad por ancho de banda.
    pub quality_downgrades: u64,
    /// Transiciones de sesión.
    pub session_state_transitions: u64,
    /// Sesiones completadas.
    pub sessions_completed: u64,
    /// Sesiones canceladas.
    pub sessions_cancelled: u64,
}

/// Errores esperados del modelo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetflixError {
    /// Texto requerido vacío.
    EmptyText,
    /// Lista requerida vacía.
    EmptyList,
    /// Capacidad inválida para CDN.
    ZeroCapacity,
    /// Ancho de banda inválido.
    InvalidBandwidth,
    /// Perfil inexistente.
    UnknownProfile { profile_id: ProfileId },
    /// Título inexistente.
    UnknownTitle { title_id: TitleId },
    /// CDN inexistente.
    UnknownCdnNode { cdn_node_id: CdnNodeId },
    /// Sesión inexistente.
    UnknownSession { session_id: SessionId },
    /// Título no disponible para la región del perfil.
    TitleUnavailableInRegion { title_id: TitleId, region: String },
    /// No existe variante compatible con el ancho de banda.
    NoCompatibleVariant {
        title_id: TitleId,
        bandwidth_kbps: u32,
    },
    /// No hay CDN saludable con capacidad para la región.
    NoCdnCapacity { region: String },
    /// Transición de sesión inválida.
    InvalidTransition {
        session_id: SessionId,
        from: PlaybackState,
        to: PlaybackState,
    },
}

impl fmt::Display for NetflixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyText => write!(f, "el texto no puede estar vacío"),
            Self::EmptyList => write!(f, "la lista no puede estar vacía"),
            Self::ZeroCapacity => write!(f, "la capacidad debe ser mayor que cero"),
            Self::InvalidBandwidth => write!(f, "el ancho de banda debe ser mayor que cero"),
            Self::UnknownProfile { profile_id } => {
                write!(f, "el perfil {profile_id} no existe")
            }
            Self::UnknownTitle { title_id } => write!(f, "el título {title_id} no existe"),
            Self::UnknownCdnNode { cdn_node_id } => {
                write!(f, "el nodo CDN {cdn_node_id} no existe")
            }
            Self::UnknownSession { session_id } => {
                write!(f, "la sesión {session_id} no existe")
            }
            Self::TitleUnavailableInRegion { title_id, region } => {
                write!(f, "el título {title_id} no está disponible en {region}")
            }
            Self::NoCompatibleVariant {
                title_id,
                bandwidth_kbps,
            } => write!(
                f,
                "el título {title_id} no tiene variante para {bandwidth_kbps} kbps"
            ),
            Self::NoCdnCapacity { region } => {
                write!(f, "no hay CDN saludable con capacidad en {region}")
            }
            Self::InvalidTransition {
                session_id,
                from,
                to,
            } => write!(
                f,
                "transición inválida para sesión {session_id}: {from:?} -> {to:?}"
            ),
        }
    }
}

impl Error for NetflixError {}

/// Servicio en memoria para estudiar un sistema tipo Netflix.
///
/// ```
/// use rust_system_design::netflix::{Device, NetflixService, VideoQuality};
///
/// let mut service = NetflixService::new();
/// let profile = service.register_profile("Ada", "mx").unwrap();
/// let title = service
///     .register_title("Rust at Scale", &["tech"], &["mx"], 100, 12)
///     .unwrap();
/// service.add_variant(title.id, VideoQuality::Low, 900).unwrap();
/// service.add_variant(title.id, VideoQuality::High, 4500).unwrap();
/// service.register_cdn_node("mx-edge-1", "mx", 10).unwrap();
///
/// let session = service
///     .start_playback(profile.id, title.id, 5_000, Device::Tv)
///     .unwrap();
///
/// assert_eq!(session.quality, VideoQuality::High);
/// ```
#[derive(Debug, Clone)]
pub struct NetflixService {
    logical_clock: u64,
    next_profile_id: ProfileId,
    next_title_id: TitleId,
    next_cdn_node_id: CdnNodeId,
    next_session_id: SessionId,
    profiles: HashMap<ProfileId, Profile>,
    titles: HashMap<TitleId, Title>,
    variants_by_title: HashMap<TitleId, Vec<VideoVariant>>,
    cdn_nodes: HashMap<CdnNodeId, CdnNode>,
    sessions: HashMap<SessionId, PlaybackSession>,
    events: HashMap<SessionId, Vec<PlaybackEvent>>,
    metrics: NetflixMetrics,
}

impl NetflixService {
    /// Crea un servicio vacío.
    #[must_use]
    pub fn new() -> Self {
        Self {
            logical_clock: 0,
            next_profile_id: 1,
            next_title_id: 1,
            next_cdn_node_id: 1,
            next_session_id: 1,
            profiles: HashMap::new(),
            titles: HashMap::new(),
            variants_by_title: HashMap::new(),
            cdn_nodes: HashMap::new(),
            sessions: HashMap::new(),
            events: HashMap::new(),
            metrics: NetflixMetrics::default(),
        }
    }

    /// Registra un perfil.
    ///
    /// # Errors
    ///
    /// Devuelve error si nombre o región están vacíos.
    pub fn register_profile(&mut self, name: &str, region: &str) -> Result<Profile, NetflixError> {
        let name = normalize_text(name)?;
        let region = normalize_text(region)?;
        self.logical_clock += 1;
        let profile = Profile {
            id: self.next_profile_id,
            name,
            region,
            genre_affinity: HashMap::new(),
            created_at_tick: self.logical_clock,
        };
        self.next_profile_id += 1;
        self.profiles.insert(profile.id, profile.clone());
        self.metrics.profiles_registered += 1;
        Ok(profile)
    }

    /// Registra un título del catálogo.
    ///
    /// # Errors
    ///
    /// Devuelve error si nombre, géneros o regiones son inválidos.
    pub fn register_title(
        &mut self,
        name: &str,
        genres: &[&str],
        regions: &[&str],
        popularity: u64,
        maturity_rating: u8,
    ) -> Result<Title, NetflixError> {
        let name = normalize_text(name)?;
        let genres = normalize_list(genres)?;
        let available_regions = normalize_list(regions)?.into_iter().collect();
        self.logical_clock += 1;
        let title = Title {
            id: self.next_title_id,
            name,
            genres,
            available_regions,
            popularity,
            maturity_rating,
        };
        self.next_title_id += 1;
        self.titles.insert(title.id, title.clone());
        self.metrics.titles_registered += 1;
        Ok(title)
    }

    /// Agrega una variante de video a un título.
    ///
    /// # Errors
    ///
    /// Devuelve error si el título no existe o el bitrate es inválido.
    pub fn add_variant(
        &mut self,
        title_id: TitleId,
        quality: VideoQuality,
        bitrate_kbps: u32,
    ) -> Result<VideoVariant, NetflixError> {
        self.ensure_title(title_id)?;
        if bitrate_kbps == 0 {
            return Err(NetflixError::InvalidBandwidth);
        }
        let variant = VideoVariant {
            title_id,
            quality,
            bitrate_kbps,
        };
        self.variants_by_title
            .entry(title_id)
            .or_default()
            .push(variant);
        self.variants_by_title
            .get_mut(&title_id)
            .expect("la variante recién insertada debe existir")
            .sort_by_key(|variant| (variant.quality, variant.bitrate_kbps));
        Ok(variant)
    }

    /// Registra un nodo CDN.
    ///
    /// # Errors
    ///
    /// Devuelve error si nombre, región o capacidad son inválidos.
    pub fn register_cdn_node(
        &mut self,
        name: &str,
        region: &str,
        capacity: u32,
    ) -> Result<CdnNode, NetflixError> {
        if capacity == 0 {
            return Err(NetflixError::ZeroCapacity);
        }
        let name = normalize_text(name)?;
        let region = normalize_text(region)?;
        let node = CdnNode {
            id: self.next_cdn_node_id,
            name,
            region,
            capacity,
            active_sessions: 0,
            healthy: true,
        };
        self.next_cdn_node_id += 1;
        self.cdn_nodes.insert(node.id, node.clone());
        self.metrics.cdn_nodes_registered += 1;
        Ok(node)
    }

    /// Cambia la salud de un nodo CDN.
    ///
    /// # Errors
    ///
    /// Devuelve error si el nodo no existe.
    pub fn set_cdn_health(
        &mut self,
        cdn_node_id: CdnNodeId,
        healthy: bool,
    ) -> Result<(), NetflixError> {
        let node = self
            .cdn_nodes
            .get_mut(&cdn_node_id)
            .ok_or(NetflixError::UnknownCdnNode { cdn_node_id })?;
        node.healthy = healthy;
        Ok(())
    }

    /// Devuelve el catálogo visible para un perfil.
    ///
    /// # Errors
    ///
    /// Devuelve error si el perfil no existe.
    pub fn visible_catalog(&mut self, profile_id: ProfileId) -> Result<Vec<Title>, NetflixError> {
        let region = self.ensure_profile(profile_id)?.region.clone();
        self.metrics.catalog_reads += 1;
        let mut titles: Vec<_> = self
            .titles
            .values()
            .filter(|title| title.available_regions.contains(&region))
            .cloned()
            .collect();
        titles.sort_by_key(|title| (Reverse(title.popularity), title.id));
        Ok(titles)
    }

    /// Devuelve recomendaciones explicables para un perfil.
    ///
    /// # Errors
    ///
    /// Devuelve error si el perfil no existe.
    pub fn recommendations(
        &mut self,
        profile_id: ProfileId,
        limit: usize,
    ) -> Result<Vec<Recommendation>, NetflixError> {
        let profile = self.ensure_profile(profile_id)?.clone();
        self.metrics.recommendation_reads += 1;
        let mut recommendations: Vec<_> = self
            .titles
            .values()
            .filter(|title| title.available_regions.contains(&profile.region))
            .map(|title| recommendation_for(title, &profile))
            .collect();
        recommendations.sort_by_key(|item| (Reverse(item.score), item.title_id));
        recommendations.truncate(limit);
        Ok(recommendations)
    }

    /// Inicia reproducción.
    ///
    /// # Errors
    ///
    /// Devuelve error si perfil, título, variante o CDN no pueden resolverse.
    pub fn start_playback(
        &mut self,
        profile_id: ProfileId,
        title_id: TitleId,
        bandwidth_kbps: u32,
        device: Device,
    ) -> Result<PlaybackSession, NetflixError> {
        if bandwidth_kbps == 0 {
            return Err(NetflixError::InvalidBandwidth);
        }
        self.metrics.playback_requests += 1;
        let region = self.ensure_profile(profile_id)?.region.clone();
        let title = self.ensure_title(title_id)?;
        if !title.available_regions.contains(&region) {
            self.metrics.playback_failures += 1;
            return Err(NetflixError::TitleUnavailableInRegion { title_id, region });
        }

        let Some(variant) = self.best_variant(title_id, bandwidth_kbps) else {
            self.metrics.playback_failures += 1;
            return Err(NetflixError::NoCompatibleVariant {
                title_id,
                bandwidth_kbps,
            });
        };
        if self.is_quality_downgraded(title_id, variant.quality) {
            self.metrics.quality_downgrades += 1;
        }

        let Some(cdn_node_id) = self.assign_cdn(&region) else {
            self.metrics.playback_failures += 1;
            self.metrics.cdn_capacity_rejections += 1;
            return Err(NetflixError::NoCdnCapacity { region });
        };

        self.logical_clock += 1;
        let session = PlaybackSession {
            id: self.next_session_id,
            profile_id,
            title_id,
            quality: variant.quality,
            bitrate_kbps: variant.bitrate_kbps,
            device,
            cdn_node_id,
            state: PlaybackState::Playing,
            created_at_tick: self.logical_clock,
        };
        self.next_session_id += 1;
        self.sessions.insert(session.id, session.clone());
        self.metrics.sessions_started += 1;
        self.metrics.cdn_assignments += 1;
        self.record_event(session.id, PlaybackState::Playing, "sesión iniciada");
        Ok(session)
    }

    /// Pausa una sesión activa.
    ///
    /// # Errors
    ///
    /// Devuelve error si la sesión no existe o la transición es inválida.
    pub fn pause_session(
        &mut self,
        session_id: SessionId,
    ) -> Result<PlaybackSession, NetflixError> {
        self.transition_session(session_id, PlaybackState::Paused)
    }

    /// Reanuda una sesión pausada.
    ///
    /// # Errors
    ///
    /// Devuelve error si la sesión no existe o la transición es inválida.
    pub fn resume_session(
        &mut self,
        session_id: SessionId,
    ) -> Result<PlaybackSession, NetflixError> {
        self.transition_session(session_id, PlaybackState::Playing)
    }

    /// Completa una sesión y libera capacidad CDN.
    ///
    /// # Errors
    ///
    /// Devuelve error si la sesión no existe o la transición es inválida.
    pub fn complete_session(
        &mut self,
        session_id: SessionId,
    ) -> Result<PlaybackSession, NetflixError> {
        let session = self.transition_session(session_id, PlaybackState::Completed)?;
        self.release_cdn(session.cdn_node_id);
        self.learn_from_completed_session(&session);
        self.metrics.sessions_completed += 1;
        Ok(session)
    }

    /// Cancela una sesión y libera capacidad CDN.
    ///
    /// # Errors
    ///
    /// Devuelve error si la sesión no existe o la transición es inválida.
    pub fn cancel_session(
        &mut self,
        session_id: SessionId,
    ) -> Result<PlaybackSession, NetflixError> {
        let session = self.transition_session(session_id, PlaybackState::Cancelled)?;
        self.release_cdn(session.cdn_node_id);
        self.metrics.sessions_cancelled += 1;
        Ok(session)
    }

    /// Devuelve un título si existe.
    #[must_use]
    pub fn title(&self, title_id: TitleId) -> Option<&Title> {
        self.titles.get(&title_id)
    }

    /// Devuelve un nodo CDN si existe.
    #[must_use]
    pub fn cdn_node(&self, cdn_node_id: CdnNodeId) -> Option<&CdnNode> {
        self.cdn_nodes.get(&cdn_node_id)
    }

    /// Devuelve una sesión si existe.
    #[must_use]
    pub fn session(&self, session_id: SessionId) -> Option<&PlaybackSession> {
        self.sessions.get(&session_id)
    }

    /// Devuelve eventos de una sesión.
    #[must_use]
    pub fn events(&self, session_id: SessionId) -> &[PlaybackEvent] {
        self.events
            .get(&session_id)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    /// Métricas acumuladas.
    #[must_use]
    pub fn metrics(&self) -> NetflixMetrics {
        self.metrics
    }

    fn ensure_profile(&self, profile_id: ProfileId) -> Result<&Profile, NetflixError> {
        self.profiles
            .get(&profile_id)
            .ok_or(NetflixError::UnknownProfile { profile_id })
    }

    fn ensure_title(&self, title_id: TitleId) -> Result<&Title, NetflixError> {
        self.titles
            .get(&title_id)
            .ok_or(NetflixError::UnknownTitle { title_id })
    }

    fn best_variant(&self, title_id: TitleId, bandwidth_kbps: u32) -> Option<VideoVariant> {
        self.variants_by_title
            .get(&title_id)?
            .iter()
            .copied()
            .filter(|variant| variant.bitrate_kbps <= bandwidth_kbps)
            .max_by_key(|variant| (variant.quality, variant.bitrate_kbps))
    }

    fn is_quality_downgraded(&self, title_id: TitleId, selected: VideoQuality) -> bool {
        self.variants_by_title
            .get(&title_id)
            .and_then(|variants| variants.iter().map(|variant| variant.quality).max())
            .map(|best| selected < best)
            .unwrap_or(false)
    }

    fn assign_cdn(&mut self, region: &str) -> Option<CdnNodeId> {
        let candidate_id = self
            .cdn_nodes
            .values()
            .filter(|node| {
                node.region == region && node.healthy && node.active_sessions < node.capacity
            })
            .min_by_key(|node| (node.active_sessions, node.id))
            .map(|node| node.id)?;
        let node = self
            .cdn_nodes
            .get_mut(&candidate_id)
            .expect("el candidato CDN debe existir");
        node.active_sessions += 1;
        Some(candidate_id)
    }

    fn transition_session(
        &mut self,
        session_id: SessionId,
        next_state: PlaybackState,
    ) -> Result<PlaybackSession, NetflixError> {
        let current = self
            .sessions
            .get(&session_id)
            .ok_or(NetflixError::UnknownSession { session_id })?
            .state;
        if !is_valid_transition(current, next_state) {
            return Err(NetflixError::InvalidTransition {
                session_id,
                from: current,
                to: next_state,
            });
        }
        self.logical_clock += 1;
        let session = self
            .sessions
            .get_mut(&session_id)
            .ok_or(NetflixError::UnknownSession { session_id })?;
        session.state = next_state;
        let session = session.clone();
        self.metrics.session_state_transitions += 1;
        self.record_event(session_id, next_state, "transición de sesión");
        Ok(session)
    }

    fn release_cdn(&mut self, cdn_node_id: CdnNodeId) {
        if let Some(node) = self.cdn_nodes.get_mut(&cdn_node_id) {
            node.active_sessions = node.active_sessions.saturating_sub(1);
        }
    }

    fn learn_from_completed_session(&mut self, session: &PlaybackSession) {
        let Some(title) = self.titles.get(&session.title_id) else {
            return;
        };
        let Some(profile) = self.profiles.get_mut(&session.profile_id) else {
            return;
        };
        for genre in &title.genres {
            *profile.genre_affinity.entry(genre.clone()).or_default() += 1;
        }
    }

    fn record_event(&mut self, session_id: SessionId, state: PlaybackState, message: &str) {
        self.events
            .entry(session_id)
            .or_default()
            .push(PlaybackEvent {
                session_id,
                state,
                message: message.to_string(),
                created_at_tick: self.logical_clock,
            });
    }
}

impl Default for NetflixService {
    fn default() -> Self {
        Self::new()
    }
}

fn recommendation_for(title: &Title, profile: &Profile) -> Recommendation {
    let affinity_bonus: u64 = title
        .genres
        .iter()
        .map(|genre| u64::from(*profile.genre_affinity.get(genre).unwrap_or(&0)) * 100)
        .sum();
    let score = title.popularity + affinity_bonus;
    let reason = if affinity_bonus > 0 {
        "popularidad regional + afinidad por género".to_string()
    } else {
        "popularidad regional".to_string()
    };
    Recommendation {
        title_id: title.id,
        score,
        reason,
    }
}

fn normalize_text(text: &str) -> Result<String, NetflixError> {
    let text = text.trim().to_lowercase();
    if text.is_empty() {
        return Err(NetflixError::EmptyText);
    }
    Ok(text)
}

fn normalize_list(items: &[&str]) -> Result<Vec<String>, NetflixError> {
    if items.is_empty() {
        return Err(NetflixError::EmptyList);
    }
    let mut normalized = Vec::with_capacity(items.len());
    for item in items {
        normalized.push(normalize_text(item)?);
    }
    normalized.sort();
    normalized.dedup();
    if normalized.is_empty() {
        Err(NetflixError::EmptyList)
    } else {
        Ok(normalized)
    }
}

fn is_valid_transition(from: PlaybackState, to: PlaybackState) -> bool {
    matches!(
        (from, to),
        (PlaybackState::Playing, PlaybackState::Paused)
            | (PlaybackState::Playing, PlaybackState::Completed)
            | (PlaybackState::Playing, PlaybackState::Cancelled)
            | (PlaybackState::Paused, PlaybackState::Playing)
            | (PlaybackState::Paused, PlaybackState::Cancelled)
    )
}

#[cfg(test)]
mod tests {
    use super::{Device, NetflixError, NetflixService, PlaybackState, VideoQuality};

    #[test]
    fn filters_catalog_by_profile_region() {
        let mut service = NetflixService::new();
        let profile = service.register_profile("Ada", "mx").expect("profile");
        let visible = service
            .register_title("Rust", &["tech"], &["mx"], 10, 12)
            .expect("visible");
        service
            .register_title("Distributed Systems", &["tech"], &["us"], 100, 12)
            .expect("hidden");

        let catalog = service.visible_catalog(profile.id).expect("catalog");

        assert_eq!(catalog, vec![visible]);
        assert_eq!(service.metrics().catalog_reads, 1);
    }

    #[test]
    fn starts_playback_with_best_compatible_variant_and_cdn() {
        let mut service = NetflixService::new();
        let profile = service.register_profile("Ada", "mx").expect("profile");
        let title = service
            .register_title("Rust", &["tech"], &["mx"], 10, 12)
            .expect("title");
        service
            .add_variant(title.id, VideoQuality::Low, 1_000)
            .expect("low");
        service
            .add_variant(title.id, VideoQuality::High, 5_000)
            .expect("high");
        let cdn = service.register_cdn_node("mx-edge", "mx", 2).expect("cdn");

        let session = service
            .start_playback(profile.id, title.id, 5_000, Device::Tv)
            .expect("session");

        assert_eq!(session.quality, VideoQuality::High);
        assert_eq!(session.cdn_node_id, cdn.id);
        assert_eq!(service.cdn_node(cdn.id).expect("cdn").active_sessions, 1);
        assert_eq!(service.metrics().sessions_started, 1);
    }

    #[test]
    fn rejects_title_unavailable_in_region() {
        let mut service = NetflixService::new();
        let profile = service.register_profile("Ada", "mx").expect("profile");
        let title = service
            .register_title("Rust", &["tech"], &["us"], 10, 12)
            .expect("title");

        let error = service
            .start_playback(profile.id, title.id, 5_000, Device::Web)
            .expect_err("region incorrecta");

        assert_eq!(
            error,
            NetflixError::TitleUnavailableInRegion {
                title_id: title.id,
                region: "mx".to_string()
            }
        );
    }

    #[test]
    fn completing_session_releases_cdn_and_updates_affinity() {
        let mut service = NetflixService::new();
        let profile = service.register_profile("Ada", "mx").expect("profile");
        let title = service
            .register_title("Rust", &["tech"], &["mx"], 10, 12)
            .expect("title");
        service
            .add_variant(title.id, VideoQuality::Low, 1_000)
            .expect("variant");
        let cdn = service.register_cdn_node("mx-edge", "mx", 1).expect("cdn");
        let session = service
            .start_playback(profile.id, title.id, 1_500, Device::Mobile)
            .expect("session");

        let completed = service
            .complete_session(session.id)
            .expect("completed session");

        assert_eq!(completed.state, PlaybackState::Completed);
        assert_eq!(service.cdn_node(cdn.id).expect("cdn").active_sessions, 0);
        let recommendation = service
            .recommendations(profile.id, 1)
            .expect("recommendations")
            .pop()
            .expect("recommendation");
        assert!(recommendation.reason.contains("afinidad"));
    }
}
