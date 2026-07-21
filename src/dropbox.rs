//! Modelo educativo de Dropbox.
//!
//! El módulo estudia sincronización de archivos, chunks deduplicados,
//! metadatos versionados, cambios incrementales y conflictos explícitos sin
//! tocar un sistema de archivos real.

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::hash::{Hash, Hasher};

/// Identificador lógico de dispositivo.
pub type DeviceId = u64;
/// Identificador lógico de revisión.
pub type RevisionId = u64;
/// Huella educativa de chunk.
pub type ChunkHash = u64;

/// Configuración pedagógica del modelo Dropbox.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DropboxConfig {
    /// Tamaño fijo de chunk.
    pub chunk_size: usize,
}

impl Default for DropboxConfig {
    fn default() -> Self {
        Self {
            chunk_size: 4 * 1024,
        }
    }
}

/// Dispositivo registrado.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Device {
    /// Identificador interno.
    pub id: DeviceId,
    /// Nombre visible.
    pub name: String,
    /// Última revisión que el dispositivo declaró haber visto.
    pub last_seen_revision: RevisionId,
    /// Tiempo lógico de creación.
    pub created_at_tick: u64,
}

/// Chunk deduplicado.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chunk {
    /// Huella del contenido.
    pub hash: ChunkHash,
    /// Bytes guardados.
    pub bytes: Vec<u8>,
    /// Número de manifiestos que lo referencian.
    pub references: u64,
}

/// Metadatos actuales de un archivo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileMetadata {
    /// Path visible.
    pub path: String,
    /// Revisión actual.
    pub current_revision: RevisionId,
    /// Manifiesto actual.
    pub chunks: Vec<ChunkHash>,
    /// Tamaño original en bytes.
    pub size_bytes: usize,
    /// Tiempo lógico de actualización.
    pub updated_at_tick: u64,
}

/// Revisión histórica de archivo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileRevision {
    /// Identificador global de revisión.
    pub id: RevisionId,
    /// Dispositivo que originó el cambio.
    pub device_id: DeviceId,
    /// Path afectado.
    pub path: String,
    /// Revisión base declarada por el cliente.
    pub base_revision: Option<RevisionId>,
    /// Manifiesto de chunks.
    pub chunks: Vec<ChunkHash>,
    /// Tamaño original en bytes.
    pub size_bytes: usize,
    /// Si la revisión nació como conflicto.
    pub conflict: bool,
    /// Tiempo lógico de creación.
    pub created_at_tick: u64,
}

/// Cambio visible para sincronización.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncChange {
    /// Revisión del cambio.
    pub revision_id: RevisionId,
    /// Path afectado.
    pub path: String,
    /// Tipo pedagógico del cambio.
    pub kind: SyncChangeKind,
    /// Si el cambio creó conflicto.
    pub conflict: bool,
}

/// Tipo de cambio sincronizable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncChangeKind {
    /// Archivo creado.
    Created,
    /// Archivo actualizado.
    Updated,
    /// Copia en conflicto creada.
    ConflictCreated,
}

/// Resultado de una subida.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UploadOutcome {
    /// Path final escrito.
    pub path: String,
    /// Revisión creada.
    pub revision_id: RevisionId,
    /// Si se creó una copia en conflicto.
    pub conflict_created: bool,
    /// Chunks del manifiesto.
    pub chunks: Vec<ChunkHash>,
}

/// Métricas mínimas del modelo.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DropboxMetrics {
    /// Dispositivos registrados.
    pub devices_registered: u64,
    /// Solicitudes de subida.
    pub upload_requests: u64,
    /// Solicitudes de descarga.
    pub download_requests: u64,
    /// Solicitudes de sincronización.
    pub sync_requests: u64,
    /// Bytes recibidos.
    pub bytes_received: u64,
    /// Chunks observados.
    pub chunks_seen: u64,
    /// Chunks nuevos almacenados.
    pub chunks_stored: u64,
    /// Chunks reutilizados.
    pub chunks_reused: u64,
    /// Revisiones creadas.
    pub revisions_created: u64,
    /// Conflictos creados.
    pub conflicts_created: u64,
    /// Cambios devueltos por sync.
    pub changes_returned: u64,
    /// Fallas por chunks faltantes.
    pub missing_chunk_failures: u64,
}

/// Errores esperados del modelo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DropboxError {
    /// Texto requerido vacío.
    EmptyText,
    /// Tamaño de chunk inválido.
    InvalidChunkSize,
    /// Dispositivo inexistente.
    UnknownDevice { device_id: DeviceId },
    /// Archivo inexistente.
    UnknownFile { path: String },
    /// Revisión inexistente.
    UnknownRevision { revision_id: RevisionId },
    /// Chunk faltante durante reconstrucción.
    MissingChunk { hash: ChunkHash },
}

impl fmt::Display for DropboxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyText => write!(f, "el texto no puede estar vacío"),
            Self::InvalidChunkSize => write!(f, "el tamaño de chunk debe ser mayor que cero"),
            Self::UnknownDevice { device_id } => {
                write!(f, "el dispositivo {device_id} no existe")
            }
            Self::UnknownFile { path } => write!(f, "el archivo {path} no existe"),
            Self::UnknownRevision { revision_id } => {
                write!(f, "la revisión {revision_id} no existe")
            }
            Self::MissingChunk { hash } => write!(f, "el chunk {hash} no existe"),
        }
    }
}

impl Error for DropboxError {}

/// Servicio en memoria para estudiar sincronización tipo Dropbox.
///
/// ```
/// use rust_system_design::dropbox::DropboxService;
///
/// let mut service = DropboxService::new();
/// let device = service.register_device("laptop").unwrap();
/// let upload = service
///     .upload_file(device.id, "/docs/rust.md", None, b"hola rust")
///     .unwrap();
/// let bytes = service.download_file("/docs/rust.md").unwrap();
///
/// assert_eq!(upload.revision_id, 1);
/// assert_eq!(bytes, b"hola rust");
/// ```
#[derive(Debug, Clone)]
pub struct DropboxService {
    config: DropboxConfig,
    logical_clock: u64,
    next_device_id: DeviceId,
    next_revision_id: RevisionId,
    devices: HashMap<DeviceId, Device>,
    chunks: HashMap<ChunkHash, Chunk>,
    files: HashMap<String, FileMetadata>,
    revisions: HashMap<RevisionId, FileRevision>,
    revision_order: Vec<RevisionId>,
    metrics: DropboxMetrics,
}

impl DropboxService {
    /// Crea un servicio con configuración por defecto.
    #[must_use]
    pub fn new() -> Self {
        Self::with_config(DropboxConfig::default()).expect("configuración por defecto válida")
    }

    /// Crea un servicio con configuración explícita.
    ///
    /// # Errors
    ///
    /// Devuelve error si `chunk_size` es cero.
    pub fn with_config(config: DropboxConfig) -> Result<Self, DropboxError> {
        if config.chunk_size == 0 {
            return Err(DropboxError::InvalidChunkSize);
        }

        Ok(Self {
            config,
            logical_clock: 0,
            next_device_id: 1,
            next_revision_id: 1,
            devices: HashMap::new(),
            chunks: HashMap::new(),
            files: HashMap::new(),
            revisions: HashMap::new(),
            revision_order: Vec::new(),
            metrics: DropboxMetrics::default(),
        })
    }

    /// Registra un dispositivo.
    ///
    /// # Errors
    ///
    /// Devuelve error si el nombre está vacío.
    pub fn register_device(&mut self, name: &str) -> Result<Device, DropboxError> {
        let name = normalize_text(name)?;
        self.logical_clock += 1;
        let device = Device {
            id: self.next_device_id,
            name,
            last_seen_revision: 0,
            created_at_tick: self.logical_clock,
        };
        self.next_device_id += 1;
        self.devices.insert(device.id, device.clone());
        self.metrics.devices_registered += 1;
        Ok(device)
    }

    /// Sube un archivo y crea una revisión.
    ///
    /// Si `base_revision` no coincide con la revisión actual del path, el
    /// servicio conserva el archivo actual y crea una copia en conflicto.
    ///
    /// # Errors
    ///
    /// Devuelve error si el dispositivo o la revisión base no existen, o si el
    /// path está vacío.
    pub fn upload_file(
        &mut self,
        device_id: DeviceId,
        path: &str,
        base_revision: Option<RevisionId>,
        bytes: &[u8],
    ) -> Result<UploadOutcome, DropboxError> {
        self.ensure_device(device_id)?;
        if let Some(revision_id) = base_revision {
            self.ensure_revision(revision_id)?;
        }

        let path = normalize_path(path)?;
        self.metrics.upload_requests += 1;
        self.metrics.bytes_received += bytes.len() as u64;

        let current_revision = self
            .files
            .get(&path)
            .map(|metadata| metadata.current_revision);
        let conflict_created = current_revision.is_some() && base_revision != current_revision;
        let final_path = if conflict_created {
            self.conflict_path(&path, device_id)
        } else {
            path
        };

        let chunks = self.store_chunks(bytes);
        let kind = if conflict_created {
            SyncChangeKind::ConflictCreated
        } else if current_revision.is_some() {
            SyncChangeKind::Updated
        } else {
            SyncChangeKind::Created
        };
        let revision_id = self.create_revision(
            device_id,
            final_path.clone(),
            base_revision,
            chunks.clone(),
            bytes.len(),
            conflict_created,
        );
        self.files.insert(
            final_path.clone(),
            FileMetadata {
                path: final_path.clone(),
                current_revision: revision_id,
                chunks: chunks.clone(),
                size_bytes: bytes.len(),
                updated_at_tick: self.logical_clock,
            },
        );
        self.revision_order.push(revision_id);
        if conflict_created {
            self.metrics.conflicts_created += 1;
        }
        self.metrics.revisions_created += 1;
        self.record_kind(revision_id, kind);

        Ok(UploadOutcome {
            path: final_path,
            revision_id,
            conflict_created,
            chunks,
        })
    }

    /// Descarga y reconstruye un archivo desde chunks.
    ///
    /// # Errors
    ///
    /// Devuelve error si el archivo o algún chunk no existen.
    pub fn download_file(&mut self, path: &str) -> Result<Vec<u8>, DropboxError> {
        let path = normalize_path(path)?;
        let chunks = self
            .files
            .get(&path)
            .ok_or_else(|| DropboxError::UnknownFile { path: path.clone() })?
            .chunks
            .clone();
        self.metrics.download_requests += 1;
        self.assemble_chunks(&chunks)
    }

    /// Sincroniza cambios posteriores a una revisión conocida.
    ///
    /// # Errors
    ///
    /// Devuelve error si el dispositivo o la revisión `since_revision` no
    /// existen. La revisión cero representa "no he visto nada".
    pub fn sync_changes(
        &mut self,
        device_id: DeviceId,
        since_revision: RevisionId,
    ) -> Result<Vec<SyncChange>, DropboxError> {
        self.ensure_device(device_id)?;
        if since_revision != 0 {
            self.ensure_revision(since_revision)?;
        }

        self.metrics.sync_requests += 1;
        let changes: Vec<_> = self
            .revision_order
            .iter()
            .copied()
            .filter(|revision_id| *revision_id > since_revision)
            .filter_map(|revision_id| self.change_for_revision(revision_id))
            .collect();
        self.metrics.changes_returned += changes.len() as u64;
        if let Some(last) = changes.last() {
            let device = self
                .devices
                .get_mut(&device_id)
                .expect("el dispositivo validado debe existir");
            device.last_seen_revision = last.revision_id;
        }
        Ok(changes)
    }

    /// Devuelve metadatos actuales de un archivo.
    #[must_use]
    pub fn file_metadata(&self, path: &str) -> Option<&FileMetadata> {
        self.files.get(path)
    }

    /// Devuelve un chunk si existe.
    #[must_use]
    pub fn chunk(&self, hash: ChunkHash) -> Option<&Chunk> {
        self.chunks.get(&hash)
    }

    /// Métricas acumuladas.
    #[must_use]
    pub fn metrics(&self) -> DropboxMetrics {
        self.metrics
    }

    fn ensure_device(&self, device_id: DeviceId) -> Result<(), DropboxError> {
        if self.devices.contains_key(&device_id) {
            Ok(())
        } else {
            Err(DropboxError::UnknownDevice { device_id })
        }
    }

    fn ensure_revision(&self, revision_id: RevisionId) -> Result<(), DropboxError> {
        if self.revisions.contains_key(&revision_id) {
            Ok(())
        } else {
            Err(DropboxError::UnknownRevision { revision_id })
        }
    }

    fn store_chunks(&mut self, bytes: &[u8]) -> Vec<ChunkHash> {
        bytes
            .chunks(self.config.chunk_size)
            .map(|chunk_bytes| {
                self.metrics.chunks_seen += 1;
                let hash = hash_chunk(chunk_bytes);
                if let Some(chunk) = self.chunks.get_mut(&hash) {
                    chunk.references += 1;
                    self.metrics.chunks_reused += 1;
                } else {
                    self.chunks.insert(
                        hash,
                        Chunk {
                            hash,
                            bytes: chunk_bytes.to_vec(),
                            references: 1,
                        },
                    );
                    self.metrics.chunks_stored += 1;
                }
                hash
            })
            .collect()
    }

    fn create_revision(
        &mut self,
        device_id: DeviceId,
        path: String,
        base_revision: Option<RevisionId>,
        chunks: Vec<ChunkHash>,
        size_bytes: usize,
        conflict: bool,
    ) -> RevisionId {
        self.logical_clock += 1;
        let revision_id = self.next_revision_id;
        self.next_revision_id += 1;
        self.revisions.insert(
            revision_id,
            FileRevision {
                id: revision_id,
                device_id,
                path,
                base_revision,
                chunks,
                size_bytes,
                conflict,
                created_at_tick: self.logical_clock,
            },
        );
        revision_id
    }

    fn assemble_chunks(&mut self, chunks: &[ChunkHash]) -> Result<Vec<u8>, DropboxError> {
        let mut bytes = Vec::new();
        for hash in chunks {
            let Some(chunk) = self.chunks.get(hash) else {
                self.metrics.missing_chunk_failures += 1;
                return Err(DropboxError::MissingChunk { hash: *hash });
            };
            bytes.extend_from_slice(&chunk.bytes);
        }
        Ok(bytes)
    }

    fn change_for_revision(&self, revision_id: RevisionId) -> Option<SyncChange> {
        let revision = self.revisions.get(&revision_id)?;
        Some(SyncChange {
            revision_id,
            path: revision.path.clone(),
            kind: if revision.conflict {
                SyncChangeKind::ConflictCreated
            } else if revision.base_revision.is_some() {
                SyncChangeKind::Updated
            } else {
                SyncChangeKind::Created
            },
            conflict: revision.conflict,
        })
    }

    fn conflict_path(&self, path: &str, device_id: DeviceId) -> String {
        format!(
            "{path} (conflicto dispositivo {device_id} rev {})",
            self.next_revision_id
        )
    }

    fn record_kind(&mut self, _revision_id: RevisionId, _kind: SyncChangeKind) {}
}

impl Default for DropboxService {
    fn default() -> Self {
        Self::new()
    }
}

fn normalize_text(text: &str) -> Result<String, DropboxError> {
    let text = text.trim();
    if text.is_empty() {
        return Err(DropboxError::EmptyText);
    }
    Ok(text.to_string())
}

fn normalize_path(path: &str) -> Result<String, DropboxError> {
    let path = path.trim();
    if path.is_empty() {
        return Err(DropboxError::EmptyText);
    }
    Ok(path.to_string())
}

fn hash_chunk(bytes: &[u8]) -> ChunkHash {
    let mut hasher = DefaultHasher::new();
    bytes.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::{DropboxConfig, DropboxError, DropboxService};

    #[test]
    fn uploads_and_downloads_file() {
        let mut service = DropboxService::new();
        let device = service.register_device("laptop").expect("device");

        let upload = service
            .upload_file(device.id, "/docs/rust.md", None, b"hola rust")
            .expect("upload");
        let bytes = service
            .download_file("/docs/rust.md")
            .expect("download file");

        assert_eq!(upload.revision_id, 1);
        assert_eq!(bytes, b"hola rust");
        assert_eq!(service.metrics().revisions_created, 1);
    }

    #[test]
    fn reuses_identical_chunks() {
        let mut service =
            DropboxService::with_config(DropboxConfig { chunk_size: 4 }).expect("service");
        let device = service.register_device("laptop").expect("device");

        service
            .upload_file(device.id, "/a.txt", None, b"aaaabbbb")
            .expect("first");
        service
            .upload_file(device.id, "/b.txt", None, b"aaaabbbb")
            .expect("second");

        assert_eq!(service.metrics().chunks_stored, 2);
        assert_eq!(service.metrics().chunks_reused, 2);
    }

    #[test]
    fn creates_conflict_when_base_revision_is_stale() {
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
            .upload_file(
                phone.id,
                "/docs/rust.md",
                Some(first.revision_id),
                b"phone v2",
            )
            .expect("conflict");

        assert!(conflict.conflict_created);
        assert!(conflict.path.contains("conflicto"));
        assert_eq!(service.metrics().conflicts_created, 1);
        assert_eq!(
            service.download_file("/docs/rust.md").expect("current"),
            b"v2"
        );
    }

    #[test]
    fn rejects_unknown_device() {
        let mut service = DropboxService::new();

        let error = service
            .upload_file(999, "/docs/rust.md", None, b"hola")
            .expect_err("unknown device");

        assert_eq!(error, DropboxError::UnknownDevice { device_id: 999 });
    }
}
