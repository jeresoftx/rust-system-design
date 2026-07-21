//! Modelo educativo de Kafka.
//!
//! El módulo estudia logs append-only, particiones, offsets, consumer groups y
//! retención por cantidad de eventos sin red ni protocolo Kafka real.

use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::fmt;

/// Offset dentro de una partición.
pub type KafkaOffset = u64;

/// Evento guardado en una partición.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KafkaEvent {
    /// Offset único dentro de la partición.
    pub offset: KafkaOffset,
    /// Clave opcional usada para particionar.
    pub key: Option<String>,
    /// Payload simplificado.
    pub payload: String,
}

/// Resultado de una publicación aceptada.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishedEvent {
    /// Topic destino.
    pub topic: String,
    /// Partición elegida.
    pub partition: usize,
    /// Offset asignado.
    pub offset: KafkaOffset,
}

/// Lote de lectura para un consumer group.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FetchBatch {
    /// Consumer group solicitante.
    pub group: String,
    /// Topic leído.
    pub topic: String,
    /// Partición leída.
    pub partition: usize,
    /// Offset solicitado.
    pub from_offset: KafkaOffset,
    /// Siguiente offset sugerido para commit.
    pub next_offset: KafkaOffset,
    /// Eventos entregados.
    pub events: Vec<KafkaEvent>,
}

/// Atraso visible de un consumer group en una partición.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartitionLag {
    /// Topic observado.
    pub topic: String,
    /// Partición observada.
    pub partition: usize,
    /// Offset confirmado por el grupo.
    pub committed_offset: KafkaOffset,
    /// Siguiente offset a publicar.
    pub high_watermark: KafkaOffset,
    /// Eventos pendientes desde el commit.
    pub lag: u64,
}

/// Métricas mínimas del modelo.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct KafkaMetrics {
    /// Topics creados.
    pub topics_created: u64,
    /// Eventos publicados.
    pub events_published: u64,
    /// Eventos entregados por fetch.
    pub events_fetched: u64,
    /// Commits aceptados.
    pub offsets_committed: u64,
    /// Eventos eliminados por retención.
    pub events_removed_by_retention: u64,
    /// Fetches rechazados.
    pub fetches_rejected: u64,
    /// Commits rechazados.
    pub commits_rejected: u64,
}

/// Errores esperados del modelo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KafkaError {
    /// Texto requerido vacío.
    EmptyText,
    /// El topic ya existe.
    TopicAlreadyExists { topic: String },
    /// El topic no existe.
    TopicNotFound { topic: String },
    /// Un topic no puede crearse sin particiones.
    TopicWithoutPartitions,
    /// Partición fuera de rango.
    InvalidPartition { topic: String, partition: usize },
    /// Offset eliminado por retención o más allá del final.
    OffsetOutOfRange {
        requested: KafkaOffset,
        first_available: KafkaOffset,
        next_offset: KafkaOffset,
    },
    /// Commit más allá del último offset disponible.
    CommitBeyondEnd {
        requested: KafkaOffset,
        next_offset: KafkaOffset,
    },
}

impl fmt::Display for KafkaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyText => write!(f, "el texto no puede estar vacío"),
            Self::TopicAlreadyExists { topic } => write!(f, "el topic {topic} ya existe"),
            Self::TopicNotFound { topic } => write!(f, "el topic {topic} no existe"),
            Self::TopicWithoutPartitions => write!(f, "el topic necesita al menos una partición"),
            Self::InvalidPartition { topic, partition } => {
                write!(f, "la partición {partition} no existe en el topic {topic}")
            }
            Self::OffsetOutOfRange {
                requested,
                first_available,
                next_offset,
            } => write!(
                f,
                "offset {requested} fuera de rango; primero disponible {first_available}, siguiente {next_offset}"
            ),
            Self::CommitBeyondEnd {
                requested,
                next_offset,
            } => write!(
                f,
                "commit {requested} no puede avanzar más allá del siguiente offset {next_offset}"
            ),
        }
    }
}

impl Error for KafkaError {}

#[derive(Debug, Clone)]
struct Topic {
    partitions: Vec<Partition>,
    retention_events: usize,
    next_round_robin_partition: usize,
}

#[derive(Debug, Clone)]
struct Partition {
    events: VecDeque<KafkaEvent>,
    next_offset: KafkaOffset,
    first_available_offset: KafkaOffset,
}

impl Partition {
    fn new() -> Self {
        Self {
            events: VecDeque::new(),
            next_offset: 0,
            first_available_offset: 0,
        }
    }

    fn high_watermark(&self) -> KafkaOffset {
        self.next_offset
    }
}

/// Servicio en memoria para estudiar decisiones tipo Kafka.
///
/// ```
/// use rust_system_design::kafka::KafkaService;
///
/// let mut service = KafkaService::new();
/// service.create_topic("payments", 2, 100).unwrap();
/// let published = service.publish("payments", Some("booking-1"), "paid").unwrap();
/// let batch = service.fetch("billing", "payments", published.partition, 0, 10).unwrap();
/// assert_eq!(batch.events.len(), 1);
/// ```
#[derive(Debug, Clone, Default)]
pub struct KafkaService {
    topics: HashMap<String, Topic>,
    committed_offsets: HashMap<(String, String, usize), KafkaOffset>,
    metrics: KafkaMetrics,
}

impl KafkaService {
    /// Crea un servicio vacío.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Crea un topic con particiones fijas y retención por cantidad de eventos.
    ///
    /// # Errors
    ///
    /// Devuelve error si el nombre está vacío, si no hay particiones o si el
    /// topic ya existe.
    pub fn create_topic(
        &mut self,
        topic: &str,
        partitions: usize,
        retention_events: usize,
    ) -> Result<(), KafkaError> {
        let topic = normalize_text(topic)?;
        if partitions == 0 {
            return Err(KafkaError::TopicWithoutPartitions);
        }
        if self.topics.contains_key(&topic) {
            return Err(KafkaError::TopicAlreadyExists { topic });
        }
        let partitions = (0..partitions).map(|_| Partition::new()).collect();
        self.topics.insert(
            topic,
            Topic {
                partitions,
                retention_events: retention_events.max(1),
                next_round_robin_partition: 0,
            },
        );
        self.metrics.topics_created += 1;
        Ok(())
    }

    /// Publica un evento y devuelve partición y offset asignados.
    ///
    /// # Errors
    ///
    /// Devuelve error si el topic no existe o si el payload está vacío.
    pub fn publish(
        &mut self,
        topic: &str,
        key: Option<&str>,
        payload: &str,
    ) -> Result<PublishedEvent, KafkaError> {
        let topic_name = normalize_text(topic)?;
        let payload = normalize_text(payload)?;
        let key = key.map(normalize_text).transpose()?;
        let (partition_index, offset, removed_by_retention) = {
            let topic = self.topic_mut(&topic_name)?;
            let partition_index = choose_partition(topic, key.as_deref());
            let partition = &mut topic.partitions[partition_index];
            let offset = partition.next_offset;
            partition.next_offset += 1;
            partition.events.push_back(KafkaEvent {
                offset,
                key,
                payload,
            });
            let removed_by_retention = apply_retention(partition, topic.retention_events);
            (partition_index, offset, removed_by_retention)
        };
        self.metrics.events_removed_by_retention += removed_by_retention;
        self.metrics.events_published += 1;

        Ok(PublishedEvent {
            topic: topic_name,
            partition: partition_index,
            offset,
        })
    }

    /// Lee eventos desde un offset explícito.
    ///
    /// # Errors
    ///
    /// Devuelve error si topic, partición u offset son inválidos.
    pub fn fetch(
        &mut self,
        group: &str,
        topic: &str,
        partition: usize,
        offset: KafkaOffset,
        limit: usize,
    ) -> Result<FetchBatch, KafkaError> {
        let group = normalize_text(group)?;
        let topic_name = normalize_text(topic)?;
        let partition_ref = match self.partition(&topic_name, partition) {
            Ok(partition_ref) => partition_ref,
            Err(error) => {
                self.metrics.fetches_rejected += 1;
                return Err(error);
            }
        };
        let first_available = partition_ref.first_available_offset;
        let next_available = partition_ref.next_offset;
        if offset < first_available || offset > next_available {
            self.metrics.fetches_rejected += 1;
            return Err(KafkaError::OffsetOutOfRange {
                requested: offset,
                first_available,
                next_offset: next_available,
            });
        }

        let events: Vec<KafkaEvent> = partition_ref
            .events
            .iter()
            .filter(|event| event.offset >= offset)
            .take(limit)
            .cloned()
            .collect();
        let next_offset = events
            .last()
            .map_or(offset, |event| event.offset.saturating_add(1));
        self.metrics.events_fetched += events.len() as u64;

        Ok(FetchBatch {
            group,
            topic: topic_name,
            partition,
            from_offset: offset,
            next_offset,
            events,
        })
    }

    /// Confirma el siguiente offset que un consumer group debe leer.
    ///
    /// # Errors
    ///
    /// Devuelve error si topic, partición u offset son inválidos.
    pub fn commit(
        &mut self,
        group: &str,
        topic: &str,
        partition: usize,
        offset: KafkaOffset,
    ) -> Result<(), KafkaError> {
        let group = normalize_text(group)?;
        let topic_name = normalize_text(topic)?;
        let partition_ref = match self.partition(&topic_name, partition) {
            Ok(partition_ref) => partition_ref,
            Err(error) => {
                self.metrics.commits_rejected += 1;
                return Err(error);
            }
        };
        let first_available = partition_ref.first_available_offset;
        let next_available = partition_ref.next_offset;
        if offset > next_available {
            self.metrics.commits_rejected += 1;
            return Err(KafkaError::CommitBeyondEnd {
                requested: offset,
                next_offset: next_available,
            });
        }
        if offset < first_available {
            self.metrics.commits_rejected += 1;
            return Err(KafkaError::OffsetOutOfRange {
                requested: offset,
                first_available,
                next_offset: next_available,
            });
        }

        self.committed_offsets
            .insert((group, topic_name, partition), offset);
        self.metrics.offsets_committed += 1;
        Ok(())
    }

    /// Devuelve el offset confirmado por un consumer group, si existe.
    #[must_use]
    pub fn committed_offset(
        &self,
        group: &str,
        topic: &str,
        partition: usize,
    ) -> Option<KafkaOffset> {
        self.committed_offsets
            .get(&(group.to_string(), topic.to_string(), partition))
            .copied()
    }

    /// Calcula el atraso de un consumer group en una partición.
    ///
    /// # Errors
    ///
    /// Devuelve error si topic o partición son inválidos.
    pub fn lag(
        &self,
        group: &str,
        topic: &str,
        partition: usize,
    ) -> Result<PartitionLag, KafkaError> {
        let group = normalize_text(group)?;
        let topic_name = normalize_text(topic)?;
        let partition_ref = self.partition(&topic_name, partition)?;
        let committed_offset = self
            .committed_offsets
            .get(&(group, topic_name.clone(), partition))
            .copied()
            .unwrap_or(partition_ref.first_available_offset);
        let high_watermark = partition_ref.high_watermark();

        Ok(PartitionLag {
            topic: topic_name,
            partition,
            committed_offset,
            high_watermark,
            lag: high_watermark.saturating_sub(committed_offset),
        })
    }

    /// Cantidad de particiones de un topic.
    ///
    /// # Errors
    ///
    /// Devuelve error si el topic no existe.
    pub fn partition_count(&self, topic: &str) -> Result<usize, KafkaError> {
        let topic = normalize_text(topic)?;
        Ok(self.topic(&topic)?.partitions.len())
    }

    /// Métricas actuales.
    #[must_use]
    pub fn metrics(&self) -> KafkaMetrics {
        self.metrics
    }

    fn topic(&self, topic: &str) -> Result<&Topic, KafkaError> {
        self.topics
            .get(topic)
            .ok_or_else(|| KafkaError::TopicNotFound {
                topic: topic.to_string(),
            })
    }

    fn topic_mut(&mut self, topic: &str) -> Result<&mut Topic, KafkaError> {
        self.topics
            .get_mut(topic)
            .ok_or_else(|| KafkaError::TopicNotFound {
                topic: topic.to_string(),
            })
    }

    fn partition(&self, topic: &str, partition: usize) -> Result<&Partition, KafkaError> {
        self.topic(topic)?
            .partitions
            .get(partition)
            .ok_or_else(|| KafkaError::InvalidPartition {
                topic: topic.to_string(),
                partition,
            })
    }
}

fn choose_partition(topic: &mut Topic, key: Option<&str>) -> usize {
    if let Some(key) = key {
        return stable_hash(key.as_bytes()) as usize % topic.partitions.len();
    }

    let partition = topic.next_round_robin_partition;
    topic.next_round_robin_partition = (partition + 1) % topic.partitions.len();
    partition
}

fn apply_retention(partition: &mut Partition, retention_events: usize) -> u64 {
    let mut removed = 0;
    while partition.events.len() > retention_events {
        partition.events.pop_front();
        partition.first_available_offset += 1;
        removed += 1;
    }
    removed
}

fn normalize_text(value: &str) -> Result<String, KafkaError> {
    let normalized = value.trim();
    if normalized.is_empty() {
        return Err(KafkaError::EmptyText);
    }
    Ok(normalized.to_string())
}

fn stable_hash(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::{KafkaError, KafkaService};

    #[test]
    fn publishes_and_fetches_event() {
        let mut service = KafkaService::new();
        service.create_topic("payments", 2, 10).unwrap();

        let published = service
            .publish("payments", Some("booking-1"), "paid")
            .unwrap();
        let batch = service
            .fetch("billing", "payments", published.partition, 0, 10)
            .unwrap();

        assert_eq!(batch.events.len(), 1);
        assert_eq!(batch.events[0].offset, published.offset);
        assert_eq!(batch.next_offset, 1);
        assert_eq!(service.metrics().events_published, 1);
    }

    #[test]
    fn keyed_events_keep_partition_order() {
        let mut service = KafkaService::new();
        service.create_topic("payments", 4, 10).unwrap();

        let first = service
            .publish("payments", Some("booking-7"), "created")
            .unwrap();
        let second = service
            .publish("payments", Some("booking-7"), "paid")
            .unwrap();

        assert_eq!(first.partition, second.partition);
        assert_eq!(first.offset, 0);
        assert_eq!(second.offset, 1);
    }

    #[test]
    fn consumer_groups_commit_independent_offsets() {
        let mut service = KafkaService::new();
        service.create_topic("payments", 1, 10).unwrap();
        service.publish("payments", None, "created").unwrap();
        service.publish("payments", None, "paid").unwrap();

        service.commit("billing", "payments", 0, 2).unwrap();
        service.commit("audit", "payments", 0, 1).unwrap();

        assert_eq!(service.committed_offset("billing", "payments", 0), Some(2));
        assert_eq!(service.committed_offset("audit", "payments", 0), Some(1));
        assert_eq!(service.lag("billing", "payments", 0).unwrap().lag, 0);
        assert_eq!(service.lag("audit", "payments", 0).unwrap().lag, 1);
    }

    #[test]
    fn retention_rejects_removed_offsets() {
        let mut service = KafkaService::new();
        service.create_topic("payments", 1, 2).unwrap();
        service.publish("payments", None, "one").unwrap();
        service.publish("payments", None, "two").unwrap();
        service.publish("payments", None, "three").unwrap();

        let error = service.fetch("billing", "payments", 0, 0, 10).unwrap_err();

        assert_eq!(
            error,
            KafkaError::OffsetOutOfRange {
                requested: 0,
                first_available: 1,
                next_offset: 3
            }
        );
        assert_eq!(service.metrics().events_removed_by_retention, 1);
    }
}
