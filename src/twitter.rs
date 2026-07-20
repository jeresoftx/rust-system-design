//! Modelo educativo de Twitter.
//!
//! El módulo estudia timelines, fan-out híbrido, ranking simple, notificaciones
//! y consistencia eventual sin implementar HTTP, persistencia ni colas reales.

use std::collections::{BTreeSet, HashMap, HashSet};
use std::error::Error;
use std::fmt;

/// Identificador lógico de usuario.
pub type UserId = u64;
/// Identificador lógico de tweet.
pub type TweetId = u64;

/// Configuración pedagógica del modelo Twitter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TwitterConfig {
    /// Tamaño máximo de un tweet en caracteres Unicode.
    pub max_tweet_chars: usize,
    /// A partir de este número de seguidores, el autor se trata como alto
    /// alcance y se resuelve con fan-out on read.
    pub high_reach_follower_threshold: usize,
    /// Número máximo de entradas devueltas en timeline cuando el usuario no
    /// pide un límite más pequeño.
    pub default_timeline_limit: usize,
    /// Máximo de notificaciones pedagógicas creadas por publicación.
    pub max_notifications_per_publish: usize,
}

impl Default for TwitterConfig {
    fn default() -> Self {
        Self {
            max_tweet_chars: 280,
            high_reach_follower_threshold: 10_000,
            default_timeline_limit: 50,
            max_notifications_per_publish: 1_000,
        }
    }
}

/// Usuario del modelo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    /// Identificador interno.
    pub id: UserId,
    /// Handle visible.
    pub handle: String,
    /// Tiempo lógico de creación.
    pub created_at_tick: u64,
}

/// Tweet publicado por un usuario.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tweet {
    /// Identificador interno.
    pub id: TweetId,
    /// Autor del tweet.
    pub author_id: UserId,
    /// Texto validado.
    pub text: String,
    /// Tiempo lógico de publicación.
    pub created_at_tick: u64,
}

/// Entrada visible en un timeline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimelineEntry {
    /// Tweet mostrado.
    pub tweet_id: TweetId,
    /// Autor del tweet.
    pub author_id: UserId,
    /// Texto del tweet.
    pub text: String,
    /// Score pedagógico de ranking.
    pub score: u64,
    /// Indica si la entrada vino de timeline materializado o merge en lectura.
    pub source: TimelineSource,
}

/// Origen de una entrada de timeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimelineSource {
    /// Entrada escrita al publicar.
    Materialized,
    /// Entrada mezclada al leer por ser autor de alto alcance.
    ReadMerge,
}

/// Notificación pedagógica.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Notification {
    /// Usuario que recibe la notificación.
    pub recipient_id: UserId,
    /// Autor que publicó.
    pub author_id: UserId,
    /// Tweet asociado.
    pub tweet_id: TweetId,
    /// Tiempo lógico de creación.
    pub created_at_tick: u64,
}

/// Métricas mínimas del modelo.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TwitterMetrics {
    /// Usuarios creados.
    pub users_created: u64,
    /// Relaciones follow creadas.
    pub follow_edges_created: u64,
    /// Tweets publicados.
    pub tweets_published: u64,
    /// Trabajos conceptuales de fan-out encolados.
    pub fanout_jobs_enqueued: u64,
    /// Entradas escritas en timelines materializados.
    pub fanout_entries_written: u64,
    /// Lecturas de timeline.
    pub timeline_reads: u64,
    /// Lecturas con timeline materializado disponible.
    pub timeline_materialized_hits: u64,
    /// Autores de alto alcance mezclados durante lectura.
    pub timeline_read_merges: u64,
    /// Notificaciones creadas.
    pub notifications_created: u64,
    /// Retraso lógico observado por fan-out on read.
    pub eventual_lag_ticks: u64,
}

/// Errores esperados del modelo Twitter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TwitterError {
    /// El handle viene vacío.
    EmptyHandle,
    /// El handle ya existe.
    DuplicateHandle { handle: String },
    /// El usuario no existe.
    UnknownUser { user_id: UserId },
    /// Un usuario intentó seguirse a sí mismo.
    SelfFollow { user_id: UserId },
    /// La relación de follow ya existe.
    AlreadyFollowing {
        follower_id: UserId,
        followee_id: UserId,
    },
    /// El texto está vacío.
    EmptyTweet,
    /// El texto excede el máximo configurado.
    TweetTooLong { max_chars: usize },
}

impl fmt::Display for TwitterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyHandle => write!(f, "el handle no puede estar vacío"),
            Self::DuplicateHandle { handle } => write!(f, "el handle {handle} ya existe"),
            Self::UnknownUser { user_id } => write!(f, "el usuario {user_id} no existe"),
            Self::SelfFollow { user_id } => write!(f, "el usuario {user_id} no puede seguirse"),
            Self::AlreadyFollowing {
                follower_id,
                followee_id,
            } => write!(
                f,
                "el usuario {follower_id} ya sigue al usuario {followee_id}"
            ),
            Self::EmptyTweet => write!(f, "el tweet no puede estar vacío"),
            Self::TweetTooLong { max_chars } => {
                write!(f, "el tweet excede el máximo de {max_chars} caracteres")
            }
        }
    }
}

impl Error for TwitterError {}

/// Servicio en memoria para estudiar un timeline tipo Twitter.
///
/// ```
/// use rust_system_design::twitter::TwitterService;
///
/// let mut service = TwitterService::new();
/// let reader = service.create_user("reader").unwrap();
/// let author = service.create_user("author").unwrap();
/// service.follow(reader.id, author.id).unwrap();
/// service.publish_tweet(author.id, "hola timeline").unwrap();
///
/// assert_eq!(service.timeline(reader.id, None).unwrap().len(), 1);
/// ```
#[derive(Debug, Clone)]
pub struct TwitterService {
    config: TwitterConfig,
    logical_clock: u64,
    next_user_id: UserId,
    next_tweet_id: TweetId,
    users: HashMap<UserId, User>,
    handles: HashSet<String>,
    followees_by_follower: HashMap<UserId, BTreeSet<UserId>>,
    followers_by_followee: HashMap<UserId, BTreeSet<UserId>>,
    tweets: HashMap<TweetId, Tweet>,
    tweets_by_author: HashMap<UserId, Vec<TweetId>>,
    materialized_timelines: HashMap<UserId, Vec<TweetId>>,
    notifications: HashMap<UserId, Vec<Notification>>,
    metrics: TwitterMetrics,
}

impl TwitterService {
    /// Crea un servicio con configuración por defecto.
    #[must_use]
    pub fn new() -> Self {
        Self::with_config(TwitterConfig::default())
    }

    /// Crea un servicio con configuración explícita.
    #[must_use]
    pub fn with_config(config: TwitterConfig) -> Self {
        Self {
            config,
            logical_clock: 0,
            next_user_id: 1,
            next_tweet_id: 1,
            users: HashMap::new(),
            handles: HashSet::new(),
            followees_by_follower: HashMap::new(),
            followers_by_followee: HashMap::new(),
            tweets: HashMap::new(),
            tweets_by_author: HashMap::new(),
            materialized_timelines: HashMap::new(),
            notifications: HashMap::new(),
            metrics: TwitterMetrics::default(),
        }
    }

    /// Crea un usuario.
    ///
    /// # Errors
    ///
    /// Devuelve error si el handle está vacío o ya existe.
    pub fn create_user(&mut self, handle: &str) -> Result<User, TwitterError> {
        let handle = normalize_handle(handle)?;
        if self.handles.contains(&handle) {
            return Err(TwitterError::DuplicateHandle { handle });
        }

        self.logical_clock += 1;
        let user = User {
            id: self.next_user_id,
            handle: handle.clone(),
            created_at_tick: self.logical_clock,
        };
        self.next_user_id += 1;

        self.handles.insert(handle);
        self.users.insert(user.id, user.clone());
        self.metrics.users_created += 1;

        Ok(user)
    }

    /// Crea una relación de seguimiento.
    ///
    /// # Errors
    ///
    /// Devuelve error si algún usuario no existe, si se intenta self-follow o
    /// si la relación ya existe.
    pub fn follow(&mut self, follower_id: UserId, followee_id: UserId) -> Result<(), TwitterError> {
        self.ensure_user(follower_id)?;
        self.ensure_user(followee_id)?;
        if follower_id == followee_id {
            return Err(TwitterError::SelfFollow {
                user_id: follower_id,
            });
        }

        let followees = self.followees_by_follower.entry(follower_id).or_default();
        if !followees.insert(followee_id) {
            return Err(TwitterError::AlreadyFollowing {
                follower_id,
                followee_id,
            });
        }

        self.followers_by_followee
            .entry(followee_id)
            .or_default()
            .insert(follower_id);
        self.metrics.follow_edges_created += 1;

        Ok(())
    }

    /// Publica un tweet y aplica fan-out según el alcance del autor.
    ///
    /// # Errors
    ///
    /// Devuelve error si el autor no existe o el texto es inválido.
    pub fn publish_tweet(&mut self, author_id: UserId, text: &str) -> Result<Tweet, TwitterError> {
        self.ensure_user(author_id)?;
        let text = self.validate_tweet_text(text)?;

        self.logical_clock += 1;
        let tweet = Tweet {
            id: self.next_tweet_id,
            author_id,
            text,
            created_at_tick: self.logical_clock,
        };
        self.next_tweet_id += 1;

        self.tweets.insert(tweet.id, tweet.clone());
        self.tweets_by_author
            .entry(author_id)
            .or_default()
            .push(tweet.id);
        self.metrics.tweets_published += 1;

        let followers = self.followers_of(author_id);
        if !self.is_high_reach_author(author_id) {
            self.metrics.fanout_jobs_enqueued += 1;
            for follower_id in &followers {
                self.materialized_timelines
                    .entry(*follower_id)
                    .or_default()
                    .push(tweet.id);
                self.metrics.fanout_entries_written += 1;
            }
        }

        self.create_notifications(author_id, tweet.id, &followers);

        Ok(tweet)
    }

    /// Lee el timeline de un usuario.
    ///
    /// # Errors
    ///
    /// Devuelve error si el usuario no existe.
    pub fn timeline(
        &mut self,
        user_id: UserId,
        limit: Option<usize>,
    ) -> Result<Vec<TimelineEntry>, TwitterError> {
        self.ensure_user(user_id)?;
        self.metrics.timeline_reads += 1;

        let mut candidate_ids = Vec::new();
        if let Some(materialized) = self.materialized_timelines.get(&user_id) {
            if !materialized.is_empty() {
                self.metrics.timeline_materialized_hits += 1;
            }
            candidate_ids.extend(materialized.iter().copied());
        }

        for followee_id in self.followees_of(user_id) {
            if self.is_high_reach_author(followee_id) {
                self.metrics.timeline_read_merges += 1;
                if let Some(tweet_ids) = self.tweets_by_author.get(&followee_id) {
                    candidate_ids.extend(tweet_ids.iter().copied());
                    if let Some(last_tweet_id) = tweet_ids.last() {
                        if let Some(tweet) = self.tweets.get(last_tweet_id) {
                            self.metrics.eventual_lag_ticks +=
                                self.logical_clock.saturating_sub(tweet.created_at_tick);
                        }
                    }
                }
            }
        }

        let mut seen = HashSet::new();
        let mut entries = Vec::new();
        for tweet_id in candidate_ids {
            if !seen.insert(tweet_id) {
                continue;
            }
            if let Some(tweet) = self.tweets.get(&tweet_id) {
                let source = if self.is_high_reach_author(tweet.author_id) {
                    TimelineSource::ReadMerge
                } else {
                    TimelineSource::Materialized
                };
                entries.push(TimelineEntry {
                    tweet_id: tweet.id,
                    author_id: tweet.author_id,
                    text: tweet.text.clone(),
                    score: tweet.created_at_tick,
                    source,
                });
            }
        }

        entries.sort_by(|left, right| {
            right
                .score
                .cmp(&left.score)
                .then_with(|| right.tweet_id.cmp(&left.tweet_id))
        });
        entries.truncate(limit.unwrap_or(self.config.default_timeline_limit));

        Ok(entries)
    }

    /// Devuelve notificaciones acumuladas para un usuario.
    ///
    /// # Errors
    ///
    /// Devuelve error si el usuario no existe.
    pub fn notifications(&self, user_id: UserId) -> Result<&[Notification], TwitterError> {
        self.ensure_user(user_id)?;
        Ok(self
            .notifications
            .get(&user_id)
            .map(Vec::as_slice)
            .unwrap_or(&[]))
    }

    /// Devuelve una copia del tweet si existe.
    #[must_use]
    pub fn tweet(&self, tweet_id: TweetId) -> Option<&Tweet> {
        self.tweets.get(&tweet_id)
    }

    /// Cantidad de seguidores de un usuario.
    #[must_use]
    pub fn follower_count(&self, user_id: UserId) -> usize {
        self.followers_by_followee
            .get(&user_id)
            .map(BTreeSet::len)
            .unwrap_or_default()
    }

    /// Indica si el autor cruza el umbral de alto alcance.
    #[must_use]
    pub fn is_high_reach_author(&self, user_id: UserId) -> bool {
        self.follower_count(user_id) >= self.config.high_reach_follower_threshold
    }

    /// Métricas acumuladas.
    #[must_use]
    pub fn metrics(&self) -> TwitterMetrics {
        self.metrics
    }

    fn create_notifications(&mut self, author_id: UserId, tweet_id: TweetId, followers: &[UserId]) {
        for recipient_id in followers
            .iter()
            .take(self.config.max_notifications_per_publish)
        {
            self.notifications
                .entry(*recipient_id)
                .or_default()
                .push(Notification {
                    recipient_id: *recipient_id,
                    author_id,
                    tweet_id,
                    created_at_tick: self.logical_clock,
                });
            self.metrics.notifications_created += 1;
        }
    }

    fn followers_of(&self, user_id: UserId) -> Vec<UserId> {
        self.followers_by_followee
            .get(&user_id)
            .map(|followers| followers.iter().copied().collect())
            .unwrap_or_default()
    }

    fn followees_of(&self, user_id: UserId) -> Vec<UserId> {
        self.followees_by_follower
            .get(&user_id)
            .map(|followees| followees.iter().copied().collect())
            .unwrap_or_default()
    }

    fn ensure_user(&self, user_id: UserId) -> Result<(), TwitterError> {
        if self.users.contains_key(&user_id) {
            Ok(())
        } else {
            Err(TwitterError::UnknownUser { user_id })
        }
    }

    fn validate_tweet_text(&self, text: &str) -> Result<String, TwitterError> {
        let text = text.trim();
        if text.is_empty() {
            return Err(TwitterError::EmptyTweet);
        }
        if text.chars().count() > self.config.max_tweet_chars {
            return Err(TwitterError::TweetTooLong {
                max_chars: self.config.max_tweet_chars,
            });
        }

        Ok(text.to_string())
    }
}

impl Default for TwitterService {
    fn default() -> Self {
        Self::new()
    }
}

fn normalize_handle(handle: &str) -> Result<String, TwitterError> {
    let handle = handle.trim().trim_start_matches('@');
    if handle.is_empty() {
        return Err(TwitterError::EmptyHandle);
    }

    Ok(handle.to_ascii_lowercase())
}

#[cfg(test)]
mod tests {
    use super::{TimelineSource, TwitterConfig, TwitterError, TwitterService};

    #[test]
    fn publishes_normal_author_into_materialized_timeline() {
        let mut service = TwitterService::new();
        let reader = service.create_user("reader").expect("reader");
        let author = service.create_user("author").expect("author");
        service.follow(reader.id, author.id).expect("follow");

        let tweet = service
            .publish_tweet(author.id, "hola desde el sistema")
            .expect("tweet");
        let timeline = service.timeline(reader.id, None).expect("timeline");

        assert_eq!(timeline.len(), 1);
        assert_eq!(timeline[0].tweet_id, tweet.id);
        assert_eq!(timeline[0].source, TimelineSource::Materialized);
        assert_eq!(service.metrics().fanout_entries_written, 1);
    }

    #[test]
    fn reads_high_reach_author_by_merge() {
        let mut service = TwitterService::with_config(TwitterConfig {
            high_reach_follower_threshold: 2,
            ..TwitterConfig::default()
        });
        let reader = service.create_user("reader").expect("reader");
        let second_reader = service.create_user("second").expect("second");
        let author = service.create_user("author").expect("author");
        service.follow(reader.id, author.id).expect("follow reader");
        service
            .follow(second_reader.id, author.id)
            .expect("follow second");

        service
            .publish_tweet(author.id, "tweet de alto alcance")
            .expect("tweet");
        let timeline = service.timeline(reader.id, None).expect("timeline");

        assert_eq!(timeline.len(), 1);
        assert_eq!(timeline[0].source, TimelineSource::ReadMerge);
        assert_eq!(service.metrics().fanout_entries_written, 0);
        assert_eq!(service.metrics().timeline_read_merges, 1);
    }

    #[test]
    fn rejects_invalid_follow() {
        let mut service = TwitterService::new();
        let user = service.create_user("reader").expect("reader");

        let error = service.follow(user.id, user.id).expect_err("self-follow");

        assert_eq!(error, TwitterError::SelfFollow { user_id: user.id });
    }

    #[test]
    fn rejects_empty_tweet() {
        let mut service = TwitterService::new();
        let author = service.create_user("author").expect("author");

        let error = service
            .publish_tweet(author.id, "   ")
            .expect_err("tweet vacío");

        assert_eq!(error, TwitterError::EmptyTweet);
    }
}
