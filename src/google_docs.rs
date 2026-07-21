//! Modelo educativo de Google Docs.
//!
//! El módulo estudia edición colaborativa con operaciones ordenadas por el
//! servidor, transformación simple de posiciones, presencia efímera y
//! sincronización de cambios sin WebSockets ni CRDT completo.

use std::collections::HashMap;
use std::error::Error;
use std::fmt;

/// Identificador lógico de documento.
pub type DocumentId = u64;
/// Identificador lógico de colaborador.
pub type CollaboratorId = u64;
/// Versión monotónica del documento.
pub type DocumentVersion = u64;

/// Documento colaborativo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    /// Identificador interno.
    pub id: DocumentId,
    /// Título visible.
    pub title: String,
    /// Texto actual.
    pub text: String,
    /// Versión actual.
    pub version: DocumentVersion,
    /// Tiempo lógico de creación.
    pub created_at_tick: u64,
    /// Tiempo lógico de actualización.
    pub updated_at_tick: u64,
}

/// Colaborador registrado.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Collaborator {
    /// Identificador interno.
    pub id: CollaboratorId,
    /// Nombre visible.
    pub name: String,
    /// Tiempo lógico de creación.
    pub created_at_tick: u64,
}

/// Tipo de operación de edición.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditKind {
    /// Insertar texto.
    Insert { text: String },
    /// Borrar `len` bytes desde la posición transformada.
    Delete { len: usize },
}

/// Operación enviada por un cliente.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditOperation {
    /// Documento afectado.
    pub document_id: DocumentId,
    /// Colaborador que originó el cambio.
    pub collaborator_id: CollaboratorId,
    /// Versión que el cliente vio al crear la operación.
    pub base_version: DocumentVersion,
    /// Posición local enviada por el cliente.
    pub position: usize,
    /// Cambio solicitado.
    pub kind: EditKind,
}

/// Operación aceptada por el servidor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppliedOperation {
    /// Documento afectado.
    pub document_id: DocumentId,
    /// Colaborador que originó el cambio.
    pub collaborator_id: CollaboratorId,
    /// Versión base enviada por el cliente.
    pub base_version: DocumentVersion,
    /// Versión asignada por el servidor.
    pub version: DocumentVersion,
    /// Posición original enviada por el cliente.
    pub original_position: usize,
    /// Posición después de transformar contra operaciones aceptadas.
    pub transformed_position: usize,
    /// Cambio aplicado.
    pub kind: EditKind,
    /// Tiempo lógico de aceptación.
    pub created_at_tick: u64,
}

/// Presencia efímera de un colaborador.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Presence {
    /// Documento relacionado.
    pub document_id: DocumentId,
    /// Colaborador relacionado.
    pub collaborator_id: CollaboratorId,
    /// Cursor lógico dentro del texto.
    pub cursor: usize,
    /// Tiempo lógico del último latido.
    pub updated_at_tick: u64,
}

/// Métricas mínimas del modelo.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GoogleDocsMetrics {
    /// Documentos creados.
    pub documents_created: u64,
    /// Colaboradores registrados.
    pub collaborators_registered: u64,
    /// Operaciones recibidas.
    pub operations_received: u64,
    /// Operaciones aplicadas.
    pub operations_applied: u64,
    /// Operaciones transformadas por base atrasada.
    pub operations_transformed: u64,
    /// Operaciones rechazadas.
    pub operations_rejected: u64,
    /// Caracteres insertados, medidos en bytes para el modelo.
    pub characters_inserted: u64,
    /// Caracteres borrados, medidos en bytes para el modelo.
    pub characters_deleted: u64,
    /// Actualizaciones de presencia.
    pub presence_updates: u64,
    /// Solicitudes de sincronización.
    pub sync_requests: u64,
    /// Operaciones devueltas por sync.
    pub operations_returned: u64,
}

/// Errores esperados del modelo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GoogleDocsError {
    /// Texto requerido vacío.
    EmptyText,
    /// Documento inexistente.
    UnknownDocument { document_id: DocumentId },
    /// Colaborador inexistente.
    UnknownCollaborator { collaborator_id: CollaboratorId },
    /// Versión base mayor a la versión del servidor.
    FutureVersion {
        base_version: DocumentVersion,
        current_version: DocumentVersion,
    },
    /// Posición inválida.
    InvalidPosition { position: usize, text_len: usize },
    /// Rango de borrado inválido.
    InvalidDeleteRange {
        position: usize,
        len: usize,
        text_len: usize,
    },
}

impl fmt::Display for GoogleDocsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyText => write!(f, "el texto no puede estar vacío"),
            Self::UnknownDocument { document_id } => {
                write!(f, "el documento {document_id} no existe")
            }
            Self::UnknownCollaborator { collaborator_id } => {
                write!(f, "el colaborador {collaborator_id} no existe")
            }
            Self::FutureVersion {
                base_version,
                current_version,
            } => write!(
                f,
                "la versión base {base_version} está adelante de la versión actual {current_version}"
            ),
            Self::InvalidPosition { position, text_len } => {
                write!(f, "posición inválida {position} para texto de {text_len} bytes")
            }
            Self::InvalidDeleteRange {
                position,
                len,
                text_len,
            } => write!(
                f,
                "rango de borrado inválido: posición {position}, longitud {len}, texto {text_len}"
            ),
        }
    }
}

impl Error for GoogleDocsError {}

/// Servicio en memoria para estudiar edición colaborativa tipo Google Docs.
///
/// ```
/// use rust_system_design::google_docs::{EditKind, GoogleDocsService};
///
/// let mut service = GoogleDocsService::new();
/// let doc = service.create_document("Notas", "Hola").unwrap();
/// let ada = service.register_collaborator("Ada").unwrap();
/// let applied = service
///     .apply_operation(doc.id, ada.id, doc.version, 4, EditKind::Insert { text: ", Rust".into() })
///     .unwrap();
///
/// assert_eq!(applied.version, 1);
/// assert_eq!(service.document(doc.id).unwrap().text, "Hola, Rust");
/// ```
#[derive(Debug, Clone)]
pub struct GoogleDocsService {
    logical_clock: u64,
    next_document_id: DocumentId,
    next_collaborator_id: CollaboratorId,
    documents: HashMap<DocumentId, Document>,
    collaborators: HashMap<CollaboratorId, Collaborator>,
    operations_by_document: HashMap<DocumentId, Vec<AppliedOperation>>,
    presence: HashMap<(DocumentId, CollaboratorId), Presence>,
    metrics: GoogleDocsMetrics,
}

impl GoogleDocsService {
    /// Crea un servicio vacío.
    #[must_use]
    pub fn new() -> Self {
        Self {
            logical_clock: 0,
            next_document_id: 1,
            next_collaborator_id: 1,
            documents: HashMap::new(),
            collaborators: HashMap::new(),
            operations_by_document: HashMap::new(),
            presence: HashMap::new(),
            metrics: GoogleDocsMetrics::default(),
        }
    }

    /// Crea un documento.
    ///
    /// # Errors
    ///
    /// Devuelve error si el título está vacío.
    pub fn create_document(
        &mut self,
        title: &str,
        initial_text: &str,
    ) -> Result<Document, GoogleDocsError> {
        let title = normalize_text(title)?;
        self.logical_clock += 1;
        let document = Document {
            id: self.next_document_id,
            title,
            text: initial_text.to_string(),
            version: 0,
            created_at_tick: self.logical_clock,
            updated_at_tick: self.logical_clock,
        };
        self.next_document_id += 1;
        self.documents.insert(document.id, document.clone());
        self.metrics.documents_created += 1;
        Ok(document)
    }

    /// Registra un colaborador.
    ///
    /// # Errors
    ///
    /// Devuelve error si el nombre está vacío.
    pub fn register_collaborator(&mut self, name: &str) -> Result<Collaborator, GoogleDocsError> {
        let name = normalize_text(name)?;
        self.logical_clock += 1;
        let collaborator = Collaborator {
            id: self.next_collaborator_id,
            name,
            created_at_tick: self.logical_clock,
        };
        self.next_collaborator_id += 1;
        self.collaborators
            .insert(collaborator.id, collaborator.clone());
        self.metrics.collaborators_registered += 1;
        Ok(collaborator)
    }

    /// Aplica una operación de edición.
    ///
    /// # Errors
    ///
    /// Devuelve error si documento, colaborador, versión o posición son
    /// inválidos.
    pub fn apply_operation(
        &mut self,
        document_id: DocumentId,
        collaborator_id: CollaboratorId,
        base_version: DocumentVersion,
        position: usize,
        kind: EditKind,
    ) -> Result<AppliedOperation, GoogleDocsError> {
        self.metrics.operations_received += 1;
        if let Err(error) = self.ensure_collaborator(collaborator_id) {
            self.metrics.operations_rejected += 1;
            return Err(error);
        }

        let current_version = match self.documents.get(&document_id) {
            Some(document) => document.version,
            None => {
                self.metrics.operations_rejected += 1;
                return Err(GoogleDocsError::UnknownDocument { document_id });
            }
        };
        if base_version > current_version {
            self.metrics.operations_rejected += 1;
            return Err(GoogleDocsError::FutureVersion {
                base_version,
                current_version,
            });
        }

        let transformed_position = self.transform_position(document_id, base_version, position);
        if transformed_position != position || base_version < current_version {
            self.metrics.operations_transformed += 1;
        }

        self.validate_operation(document_id, transformed_position, &kind)?;
        self.logical_clock += 1;
        let document = self
            .documents
            .get_mut(&document_id)
            .expect("el documento validado debe existir");
        apply_kind(&mut document.text, transformed_position, &kind);
        document.version += 1;
        document.updated_at_tick = self.logical_clock;
        let applied = AppliedOperation {
            document_id,
            collaborator_id,
            base_version,
            version: document.version,
            original_position: position,
            transformed_position,
            kind: kind.clone(),
            created_at_tick: self.logical_clock,
        };
        self.operations_by_document
            .entry(document_id)
            .or_default()
            .push(applied.clone());
        self.metrics.operations_applied += 1;
        match kind {
            EditKind::Insert { text } => self.metrics.characters_inserted += text.len() as u64,
            EditKind::Delete { len } => self.metrics.characters_deleted += len as u64,
        }
        Ok(applied)
    }

    /// Registra presencia para un colaborador.
    ///
    /// # Errors
    ///
    /// Devuelve error si documento, colaborador o cursor son inválidos.
    pub fn update_presence(
        &mut self,
        document_id: DocumentId,
        collaborator_id: CollaboratorId,
        cursor: usize,
    ) -> Result<Presence, GoogleDocsError> {
        self.ensure_collaborator(collaborator_id)?;
        let text_len = self
            .documents
            .get(&document_id)
            .ok_or(GoogleDocsError::UnknownDocument { document_id })?
            .text
            .len();
        if cursor > text_len {
            return Err(GoogleDocsError::InvalidPosition {
                position: cursor,
                text_len,
            });
        }

        self.logical_clock += 1;
        let presence = Presence {
            document_id,
            collaborator_id,
            cursor,
            updated_at_tick: self.logical_clock,
        };
        self.presence
            .insert((document_id, collaborator_id), presence.clone());
        self.metrics.presence_updates += 1;
        Ok(presence)
    }

    /// Devuelve presencia activa dentro de una ventana de ticks.
    ///
    /// # Errors
    ///
    /// Devuelve error si el documento no existe.
    pub fn active_presence(
        &self,
        document_id: DocumentId,
        max_age_ticks: u64,
    ) -> Result<Vec<Presence>, GoogleDocsError> {
        self.ensure_document(document_id)?;
        let mut active: Vec<_> = self
            .presence
            .values()
            .filter(|presence| presence.document_id == document_id)
            .filter(|presence| {
                self.logical_clock.saturating_sub(presence.updated_at_tick) <= max_age_ticks
            })
            .cloned()
            .collect();
        active.sort_by_key(|presence| presence.collaborator_id);
        Ok(active)
    }

    /// Devuelve operaciones posteriores a una versión.
    ///
    /// # Errors
    ///
    /// Devuelve error si el documento no existe o si la versión está en el
    /// futuro.
    pub fn sync_operations(
        &mut self,
        document_id: DocumentId,
        after_version: DocumentVersion,
    ) -> Result<Vec<AppliedOperation>, GoogleDocsError> {
        let current_version = self.ensure_document(document_id)?.version;
        if after_version > current_version {
            return Err(GoogleDocsError::FutureVersion {
                base_version: after_version,
                current_version,
            });
        }
        self.metrics.sync_requests += 1;
        let operations: Vec<_> = self
            .operations_by_document
            .get(&document_id)
            .map(|operations| {
                operations
                    .iter()
                    .filter(|operation| operation.version > after_version)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default();
        self.metrics.operations_returned += operations.len() as u64;
        Ok(operations)
    }

    /// Devuelve un documento si existe.
    #[must_use]
    pub fn document(&self, document_id: DocumentId) -> Option<&Document> {
        self.documents.get(&document_id)
    }

    /// Métricas acumuladas.
    #[must_use]
    pub fn metrics(&self) -> GoogleDocsMetrics {
        self.metrics
    }

    fn ensure_document(&self, document_id: DocumentId) -> Result<&Document, GoogleDocsError> {
        self.documents
            .get(&document_id)
            .ok_or(GoogleDocsError::UnknownDocument { document_id })
    }

    fn ensure_collaborator(&self, collaborator_id: CollaboratorId) -> Result<(), GoogleDocsError> {
        if self.collaborators.contains_key(&collaborator_id) {
            Ok(())
        } else {
            Err(GoogleDocsError::UnknownCollaborator { collaborator_id })
        }
    }

    fn transform_position(
        &self,
        document_id: DocumentId,
        base_version: DocumentVersion,
        mut position: usize,
    ) -> usize {
        let Some(operations) = self.operations_by_document.get(&document_id) else {
            return position;
        };

        for operation in operations
            .iter()
            .filter(|operation| operation.version > base_version)
        {
            match &operation.kind {
                EditKind::Insert { text } if operation.transformed_position <= position => {
                    position += text.len();
                }
                EditKind::Delete { len } if operation.transformed_position < position => {
                    let deleted_before_position =
                        (*len).min(position - operation.transformed_position);
                    position -= deleted_before_position;
                }
                _ => {}
            }
        }
        position
    }

    fn validate_operation(
        &mut self,
        document_id: DocumentId,
        position: usize,
        kind: &EditKind,
    ) -> Result<(), GoogleDocsError> {
        let text = &self
            .documents
            .get(&document_id)
            .expect("el documento validado debe existir")
            .text;
        let text_len = text.len();
        let position_valid = position <= text_len && text.is_char_boundary(position);
        if !position_valid {
            self.metrics.operations_rejected += 1;
            return Err(GoogleDocsError::InvalidPosition { position, text_len });
        }

        match kind {
            EditKind::Insert { text } if text.is_empty() => {
                self.metrics.operations_rejected += 1;
                Err(GoogleDocsError::EmptyText)
            }
            EditKind::Insert { .. } => Ok(()),
            EditKind::Delete { len } if *len == 0 || position + *len > text_len => {
                self.metrics.operations_rejected += 1;
                Err(GoogleDocsError::InvalidDeleteRange {
                    position,
                    len: *len,
                    text_len,
                })
            }
            EditKind::Delete { len } if !text.is_char_boundary(position + *len) => {
                self.metrics.operations_rejected += 1;
                Err(GoogleDocsError::InvalidDeleteRange {
                    position,
                    len: *len,
                    text_len,
                })
            }
            EditKind::Delete { .. } => Ok(()),
        }
    }
}

impl Default for GoogleDocsService {
    fn default() -> Self {
        Self::new()
    }
}

fn normalize_text(text: &str) -> Result<String, GoogleDocsError> {
    let text = text.trim();
    if text.is_empty() {
        return Err(GoogleDocsError::EmptyText);
    }
    Ok(text.to_string())
}

fn apply_kind(text: &mut String, position: usize, kind: &EditKind) {
    match kind {
        EditKind::Insert { text: inserted } => text.insert_str(position, inserted),
        EditKind::Delete { len } => {
            text.replace_range(position..position + *len, "");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{EditKind, GoogleDocsError, GoogleDocsService};

    #[test]
    fn applies_insert_and_delete() {
        let mut service = GoogleDocsService::new();
        let doc = service.create_document("Notas", "Hola Rust").expect("doc");
        let ada = service.register_collaborator("Ada").expect("ada");

        service
            .apply_operation(
                doc.id,
                ada.id,
                doc.version,
                4,
                EditKind::Insert { text: ",".into() },
            )
            .expect("insert");
        service
            .apply_operation(doc.id, ada.id, 1, 4, EditKind::Delete { len: 1 })
            .expect("delete");

        assert_eq!(service.document(doc.id).expect("doc").text, "Hola Rust");
        assert_eq!(service.metrics().operations_applied, 2);
    }

    #[test]
    fn transforms_stale_insert_after_server_insert() {
        let mut service = GoogleDocsService::new();
        let doc = service.create_document("Notas", "Hola").expect("doc");
        let ada = service.register_collaborator("Ada").expect("ada");
        let alan = service.register_collaborator("Alan").expect("alan");

        service
            .apply_operation(doc.id, ada.id, 0, 0, EditKind::Insert { text: "A ".into() })
            .expect("first");
        let stale = service
            .apply_operation(doc.id, alan.id, 0, 4, EditKind::Insert { text: "!".into() })
            .expect("stale");

        assert_eq!(stale.original_position, 4);
        assert_eq!(stale.transformed_position, 6);
        assert_eq!(service.document(doc.id).expect("doc").text, "A Hola!");
    }

    #[test]
    fn rejects_future_version() {
        let mut service = GoogleDocsService::new();
        let doc = service.create_document("Notas", "").expect("doc");
        let ada = service.register_collaborator("Ada").expect("ada");

        let error = service
            .apply_operation(doc.id, ada.id, 99, 0, EditKind::Insert { text: "x".into() })
            .expect_err("future version");

        assert_eq!(
            error,
            GoogleDocsError::FutureVersion {
                base_version: 99,
                current_version: 0
            }
        );
    }

    #[test]
    fn records_active_presence() {
        let mut service = GoogleDocsService::new();
        let doc = service.create_document("Notas", "Hola").expect("doc");
        let ada = service.register_collaborator("Ada").expect("ada");

        let presence = service
            .update_presence(doc.id, ada.id, 2)
            .expect("presence");
        let active = service.active_presence(doc.id, 0).expect("active");

        assert_eq!(active, vec![presence]);
        assert_eq!(service.metrics().presence_updates, 1);
    }
}
