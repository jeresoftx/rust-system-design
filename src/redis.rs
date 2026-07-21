//! Modelo educativo de Redis.
//!
//! El módulo estudia estructuras en memoria, expiración de claves, límite de
//! memoria lógico, append-only log, snapshot y replicación por offset sin red ni
//! protocolo RESP real.

use std::collections::HashMap;
use std::error::Error;
use std::fmt;

/// Offset de comando aceptado.
pub type RedisOffset = u64;

/// Configuración pedagógica de Redis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RedisConfig {
    /// Límite lógico de memoria en bytes estimados.
    pub max_memory_bytes: usize,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            max_memory_bytes: 1024 * 1024,
        }
    }
}

/// Valor guardado en memoria.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedisValue {
    /// String binario simplificado.
    String(String),
    /// Lista de strings.
    List(Vec<String>),
}

impl RedisValue {
    fn memory_bytes(&self) -> usize {
        match self {
            Self::String(value) => value.len(),
            Self::List(items) => items.iter().map(String::len).sum(),
        }
    }
}

/// Entrada interna del store.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedisEntry {
    /// Valor guardado.
    pub value: RedisValue,
    /// Expiración lógica, si aplica.
    pub expires_at_tick: Option<u64>,
    /// Tamaño estimado.
    pub memory_bytes: usize,
}

/// Comando mutable aceptado.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedisCommand {
    /// Offset asignado.
    pub offset: RedisOffset,
    /// Tipo de comando.
    pub kind: RedisCommandKind,
}

/// Tipo de comando mutable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedisCommandKind {
    /// Guardar string.
    Set {
        key: String,
        value: String,
        ttl_ticks: Option<u64>,
    },
    /// Insertar al frente de una lista.
    LPush { key: String, value: String },
    /// Borrar clave.
    Del { key: String },
}

/// Snapshot del estado visible.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedisSnapshot {
    /// Tick lógico del snapshot.
    pub created_at_tick: u64,
    /// Entradas visibles.
    pub entries: HashMap<String, RedisValue>,
}

/// Lote de replicación.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplicationBatch {
    /// Offset solicitado.
    pub from_offset: RedisOffset,
    /// Último offset incluido.
    pub last_offset: RedisOffset,
    /// Comandos incluidos.
    pub commands: Vec<RedisCommand>,
}

/// Métricas mínimas del modelo.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RedisMetrics {
    /// Comandos recibidos.
    pub commands_received: u64,
    /// Escrituras aceptadas.
    pub writes_accepted: u64,
    /// Escrituras rechazadas.
    pub writes_rejected: u64,
    /// Lecturas exitosas.
    pub key_hits: u64,
    /// Lecturas fallidas.
    pub key_misses: u64,
    /// Claves expiradas.
    pub keys_expired: u64,
    /// Memoria lógica usada.
    pub memory_used: usize,
    /// Entradas en AOF.
    pub aof_entries: u64,
    /// Lotes de replicación servidos.
    pub replication_batches: u64,
    /// Comandos devueltos para réplica.
    pub replication_commands_returned: u64,
    /// Snapshots creados.
    pub snapshots_created: u64,
}

/// Errores esperados del modelo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedisError {
    /// Clave o valor requerido vacío.
    EmptyText,
    /// Tipo incorrecto para el comando.
    WrongType { key: String },
    /// La escritura excede el límite de memoria.
    MemoryLimitExceeded { requested: usize, limit: usize },
    /// Offset de réplica inválido.
    InvalidOffset {
        requested: RedisOffset,
        last_offset: RedisOffset,
    },
}

impl fmt::Display for RedisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyText => write!(f, "el texto no puede estar vacío"),
            Self::WrongType { key } => write!(f, "tipo incorrecto para la clave {key}"),
            Self::MemoryLimitExceeded { requested, limit } => {
                write!(f, "memoria requerida {requested} supera el límite {limit}")
            }
            Self::InvalidOffset {
                requested,
                last_offset,
            } => write!(
                f,
                "offset solicitado {requested} está fuera del último offset {last_offset}"
            ),
        }
    }
}

impl Error for RedisError {}

/// Servicio en memoria para estudiar decisiones tipo Redis.
///
/// ```
/// use rust_system_design::redis::RedisService;
///
/// let mut service = RedisService::new();
/// service.set("token", "abc", Some(5)).unwrap();
/// assert_eq!(service.get("token").unwrap(), Some("abc".to_string()));
/// ```
#[derive(Debug, Clone)]
pub struct RedisService {
    config: RedisConfig,
    logical_tick: u64,
    next_offset: RedisOffset,
    entries: HashMap<String, RedisEntry>,
    aof: Vec<RedisCommand>,
    metrics: RedisMetrics,
}

impl RedisService {
    /// Crea un servicio con configuración por defecto.
    #[must_use]
    pub fn new() -> Self {
        Self::with_config(RedisConfig::default())
    }

    /// Crea un servicio con configuración explícita.
    #[must_use]
    pub fn with_config(config: RedisConfig) -> Self {
        Self {
            config,
            logical_tick: 0,
            next_offset: 1,
            entries: HashMap::new(),
            aof: Vec::new(),
            metrics: RedisMetrics::default(),
        }
    }

    /// Guarda un string.
    ///
    /// # Errors
    ///
    /// Devuelve error si clave o valor están vacíos, o si excede memoria.
    pub fn set(
        &mut self,
        key: &str,
        value: &str,
        ttl_ticks: Option<u64>,
    ) -> Result<RedisOffset, RedisError> {
        let key = normalize_text(key)?;
        let value = normalize_text(value)?;
        self.metrics.commands_received += 1;
        self.advance_tick();
        let expires_at_tick = ttl_ticks.map(|ttl| self.logical_tick + ttl);
        let redis_value = RedisValue::String(value.clone());
        self.write_value(
            key.clone(),
            redis_value,
            expires_at_tick,
            RedisCommandKind::Set {
                key,
                value,
                ttl_ticks,
            },
        )
    }

    /// Lee un string.
    ///
    /// # Errors
    ///
    /// Devuelve error si la clave existe con otro tipo.
    pub fn get(&mut self, key: &str) -> Result<Option<String>, RedisError> {
        let key = normalize_text(key)?;
        self.expire_if_needed(&key);
        let Some(entry) = self.entries.get(&key) else {
            self.metrics.key_misses += 1;
            return Ok(None);
        };
        match &entry.value {
            RedisValue::String(value) => {
                self.metrics.key_hits += 1;
                Ok(Some(value.clone()))
            }
            RedisValue::List(_) => Err(RedisError::WrongType { key }),
        }
    }

    /// Inserta un elemento al frente de una lista.
    ///
    /// # Errors
    ///
    /// Devuelve error si clave o valor están vacíos, si el tipo no es lista o
    /// si excede memoria.
    pub fn lpush(&mut self, key: &str, value: &str) -> Result<(RedisOffset, usize), RedisError> {
        let key = normalize_text(key)?;
        let value = normalize_text(value)?;
        self.metrics.commands_received += 1;
        self.advance_tick();
        self.expire_if_needed(&key);

        let mut items = match self.entries.get(&key) {
            Some(entry) => match &entry.value {
                RedisValue::List(items) => items.clone(),
                RedisValue::String(_) => return Err(RedisError::WrongType { key }),
            },
            None => Vec::new(),
        };
        items.insert(0, value.clone());
        let len = items.len();
        let offset = self.write_value(
            key.clone(),
            RedisValue::List(items),
            None,
            RedisCommandKind::LPush { key, value },
        )?;
        Ok((offset, len))
    }

    /// Lee una lista.
    ///
    /// # Errors
    ///
    /// Devuelve error si la clave existe con otro tipo.
    pub fn list(&mut self, key: &str) -> Result<Option<Vec<String>>, RedisError> {
        let key = normalize_text(key)?;
        self.expire_if_needed(&key);
        let Some(entry) = self.entries.get(&key) else {
            self.metrics.key_misses += 1;
            return Ok(None);
        };
        match &entry.value {
            RedisValue::List(items) => {
                self.metrics.key_hits += 1;
                Ok(Some(items.clone()))
            }
            RedisValue::String(_) => Err(RedisError::WrongType { key }),
        }
    }

    /// Borra una clave.
    ///
    /// # Errors
    ///
    /// Devuelve error si la clave está vacía.
    pub fn del(&mut self, key: &str) -> Result<RedisOffset, RedisError> {
        let key = normalize_text(key)?;
        self.metrics.commands_received += 1;
        self.advance_tick();
        self.remove_entry(&key);
        let offset = self.append_command(RedisCommandKind::Del { key });
        self.metrics.writes_accepted += 1;
        Ok(offset)
    }

    /// Avanza tiempo lógico y ejecuta expiración perezosa global.
    #[must_use]
    pub fn advance_time(&mut self, ticks: u64) -> u64 {
        self.logical_tick += ticks;
        self.logical_tick
    }

    /// Barre claves expiradas explícitamente.
    #[must_use]
    pub fn sweep_expired(&mut self) -> usize {
        let expired: Vec<_> = self
            .entries
            .iter()
            .filter(|(_, entry)| {
                entry
                    .expires_at_tick
                    .is_some_and(|tick| tick <= self.logical_tick)
            })
            .map(|(key, _)| key.clone())
            .collect();
        let count = expired.len();
        for key in expired {
            self.remove_entry(&key);
            self.metrics.keys_expired += 1;
        }
        count
    }

    /// Crea snapshot del estado visible.
    #[must_use]
    pub fn snapshot(&mut self) -> RedisSnapshot {
        let _ = self.sweep_expired();
        self.metrics.snapshots_created += 1;
        RedisSnapshot {
            created_at_tick: self.logical_tick,
            entries: self
                .entries
                .iter()
                .map(|(key, entry)| (key.clone(), entry.value.clone()))
                .collect(),
        }
    }

    /// Devuelve comandos posteriores a un offset.
    ///
    /// # Errors
    ///
    /// Devuelve error si el offset solicitado está en el futuro.
    pub fn replicate_since(
        &mut self,
        from_offset: RedisOffset,
    ) -> Result<ReplicationBatch, RedisError> {
        let last_offset = self.last_offset();
        if from_offset > last_offset {
            return Err(RedisError::InvalidOffset {
                requested: from_offset,
                last_offset,
            });
        }
        let commands: Vec<_> = self
            .aof
            .iter()
            .filter(|command| command.offset > from_offset)
            .cloned()
            .collect();
        self.metrics.replication_batches += 1;
        self.metrics.replication_commands_returned += commands.len() as u64;
        Ok(ReplicationBatch {
            from_offset,
            last_offset,
            commands,
        })
    }

    /// Reconstruye un servicio desde un log de comandos.
    ///
    /// # Errors
    ///
    /// Devuelve error si algún comando replayeado falla por límites o tipos.
    pub fn replay(config: RedisConfig, commands: &[RedisCommand]) -> Result<Self, RedisError> {
        let mut service = Self::with_config(config);
        for command in commands {
            match &command.kind {
                RedisCommandKind::Set {
                    key,
                    value,
                    ttl_ticks,
                } => {
                    service.set(key, value, *ttl_ticks)?;
                }
                RedisCommandKind::LPush { key, value } => {
                    service.lpush(key, value)?;
                }
                RedisCommandKind::Del { key } => {
                    service.del(key)?;
                }
            }
        }
        Ok(service)
    }

    /// Último offset aceptado.
    #[must_use]
    pub fn last_offset(&self) -> RedisOffset {
        self.next_offset.saturating_sub(1)
    }

    /// Métricas acumuladas.
    #[must_use]
    pub fn metrics(&self) -> RedisMetrics {
        self.metrics
    }

    /// Log de comandos aceptados.
    #[must_use]
    pub fn aof(&self) -> &[RedisCommand] {
        &self.aof
    }

    fn write_value(
        &mut self,
        key: String,
        value: RedisValue,
        expires_at_tick: Option<u64>,
        command: RedisCommandKind,
    ) -> Result<RedisOffset, RedisError> {
        let new_size = key.len() + value.memory_bytes();
        let old_size = self
            .entries
            .get(&key)
            .map(|entry| entry.memory_bytes)
            .unwrap_or(0);
        let requested = self.metrics.memory_used - old_size + new_size;
        if requested > self.config.max_memory_bytes {
            self.metrics.writes_rejected += 1;
            return Err(RedisError::MemoryLimitExceeded {
                requested,
                limit: self.config.max_memory_bytes,
            });
        }

        self.remove_entry(&key);
        self.entries.insert(
            key,
            RedisEntry {
                value,
                expires_at_tick,
                memory_bytes: new_size,
            },
        );
        self.metrics.memory_used = requested;
        let offset = self.append_command(command);
        self.metrics.writes_accepted += 1;
        Ok(offset)
    }

    fn append_command(&mut self, kind: RedisCommandKind) -> RedisOffset {
        let offset = self.next_offset;
        self.next_offset += 1;
        self.aof.push(RedisCommand { offset, kind });
        self.metrics.aof_entries = self.aof.len() as u64;
        offset
    }

    fn advance_tick(&mut self) {
        self.logical_tick += 1;
    }

    fn expire_if_needed(&mut self, key: &str) {
        let expired = self
            .entries
            .get(key)
            .and_then(|entry| entry.expires_at_tick)
            .is_some_and(|tick| tick <= self.logical_tick);
        if expired {
            self.remove_entry(key);
            self.metrics.keys_expired += 1;
        }
    }

    fn remove_entry(&mut self, key: &str) {
        if let Some(entry) = self.entries.remove(key) {
            self.metrics.memory_used = self.metrics.memory_used.saturating_sub(entry.memory_bytes);
        }
    }
}

impl Default for RedisService {
    fn default() -> Self {
        Self::new()
    }
}

fn normalize_text(text: &str) -> Result<String, RedisError> {
    let text = text.trim();
    if text.is_empty() {
        return Err(RedisError::EmptyText);
    }
    Ok(text.to_string())
}

#[cfg(test)]
mod tests {
    use super::{RedisConfig, RedisError, RedisService};

    #[test]
    fn sets_and_gets_string() {
        let mut service = RedisService::new();

        let offset = service.set("token", "abc", None).expect("set");
        let value = service.get("token").expect("get");

        assert_eq!(offset, 1);
        assert_eq!(value, Some("abc".to_string()));
        assert_eq!(service.metrics().key_hits, 1);
    }

    #[test]
    fn expires_key_with_ttl() {
        let mut service = RedisService::new();

        service.set("token", "abc", Some(1)).expect("set");
        let _ = service.advance_time(1);
        let value = service.get("token").expect("get expired");

        assert_eq!(value, None);
        assert_eq!(service.metrics().keys_expired, 1);
    }

    #[test]
    fn rejects_wrong_type() {
        let mut service = RedisService::new();
        service.lpush("queue", "a").expect("lpush");

        let error = service.get("queue").expect_err("wrong type");

        assert_eq!(
            error,
            RedisError::WrongType {
                key: "queue".to_string()
            }
        );
    }

    #[test]
    fn rejects_memory_limit() {
        let mut service = RedisService::with_config(RedisConfig {
            max_memory_bytes: 4,
        });

        let error = service
            .set("key", "value-too-large", None)
            .expect_err("memory");

        assert!(matches!(error, RedisError::MemoryLimitExceeded { .. }));
        assert_eq!(service.metrics().writes_rejected, 1);
    }
}
