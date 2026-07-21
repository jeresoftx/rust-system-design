//! Modelo educativo de Airbnb.
//!
//! El módulo estudia marketplace de dos lados: usuarios, listings, búsqueda,
//! disponibilidad, reservas, reseñas y confianza simple sin pagos reales,
//! antifraude ni motor de búsqueda distribuido.

use std::collections::HashMap;
use std::error::Error;
use std::fmt;

/// Identificador de usuario.
pub type UserId = u64;
/// Identificador de listing.
pub type ListingId = u64;
/// Identificador de reserva.
pub type AirbnbReservationId = u64;
/// Identificador de reseña.
pub type ReviewId = u64;
/// Noche lógica.
pub type Night = u32;
/// Dinero en centavos.
pub type MoneyCents = u64;

/// Rango de estancia: incluye `check_in` y excluye `check_out`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StayRange {
    /// Noche de entrada.
    pub check_in: Night,
    /// Noche de salida.
    pub check_out: Night,
}

impl StayRange {
    /// Crea un rango válido.
    ///
    /// # Errors
    ///
    /// Devuelve error si `check_out` no es posterior a `check_in`.
    pub fn new(check_in: Night, check_out: Night) -> Result<Self, AirbnbError> {
        if check_out <= check_in {
            return Err(AirbnbError::InvalidStayRange {
                check_in,
                check_out,
            });
        }
        Ok(Self {
            check_in,
            check_out,
        })
    }

    /// Cantidad de noches.
    #[must_use]
    pub fn nights(self) -> u32 {
        self.check_out - self.check_in
    }

    fn iter(self) -> impl Iterator<Item = Night> {
        self.check_in..self.check_out
    }
}

/// Rol de usuario.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserRole {
    /// Publica alojamientos.
    Host,
    /// Reserva alojamientos.
    Guest,
}

/// Estado de usuario.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserStatus {
    /// Puede operar.
    Active,
    /// Queda fuera de búsqueda/reserva nuevas.
    Suspended,
}

/// Usuario del marketplace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AirbnbUser {
    /// Identificador.
    pub id: UserId,
    /// Nombre visible.
    pub name: String,
    /// Rol principal.
    pub role: UserRole,
    /// Estado operativo.
    pub status: UserStatus,
}

/// Estado de listing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListingStatus {
    /// Visible y reservable.
    Active,
    /// Oculto y no reservable.
    Suspended,
}

/// Listing publicado por un anfitrión.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Listing {
    /// Identificador.
    pub id: ListingId,
    /// Anfitrión dueño del listing.
    pub host_id: UserId,
    /// Ciudad normalizada.
    pub city: String,
    /// Capacidad máxima de huéspedes.
    pub capacity: u32,
    /// Precio por noche en centavos.
    pub price_per_night_cents: MoneyCents,
    /// Estado operativo.
    pub status: ListingStatus,
}

/// Calendario de una noche.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ListingNight {
    /// Unidades disponibles para reservar.
    pub available_units: u32,
}

/// Consulta de búsqueda.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchQuery {
    /// Ciudad solicitada.
    pub city: String,
    /// Huéspedes solicitados.
    pub guests: u32,
    /// Rango de estancia.
    pub stay: StayRange,
    /// Precio máximo total opcional.
    pub max_total_price_cents: Option<MoneyCents>,
}

/// Resultado de búsqueda.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchResult {
    /// Listing candidato.
    pub listing_id: ListingId,
    /// Ciudad del listing.
    pub city: String,
    /// Precio total del rango.
    pub total_price_cents: MoneyCents,
    /// Score simple de confianza.
    pub trust_score: u8,
}

/// Estado de reserva.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AirbnbReservationStatus {
    /// Reserva confirmada.
    Confirmed,
    /// Reserva cancelada.
    Cancelled,
}

/// Reserva confirmada.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AirbnbReservation {
    /// Identificador.
    pub id: AirbnbReservationId,
    /// Huésped.
    pub guest_id: UserId,
    /// Listing reservado.
    pub listing_id: ListingId,
    /// Rango reservado.
    pub stay: StayRange,
    /// Huéspedes incluidos.
    pub guests: u32,
    /// Precio total confirmado.
    pub total_price_cents: MoneyCents,
    /// Estado.
    pub status: AirbnbReservationStatus,
}

/// Reseña posterior a una reserva.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Review {
    /// Identificador.
    pub id: ReviewId,
    /// Reserva reseñada.
    pub reservation_id: AirbnbReservationId,
    /// Listing reseñado.
    pub listing_id: ListingId,
    /// Calificación 1..=5.
    pub rating: u8,
}

/// Métricas mínimas del modelo.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AirbnbMetrics {
    /// Usuarios registrados.
    pub users_registered: u64,
    /// Listings creados.
    pub listings_created: u64,
    /// Búsquedas realizadas.
    pub searches_performed: u64,
    /// Resultados devueltos.
    pub search_results_returned: u64,
    /// Reservas confirmadas.
    pub bookings_confirmed: u64,
    /// Reservas rechazadas.
    pub bookings_rejected: u64,
    /// Reservas canceladas.
    pub bookings_cancelled: u64,
    /// Reseñas creadas.
    pub reviews_created: u64,
    /// Suspensiones aplicadas.
    pub suspensions_applied: u64,
    /// Noches reservadas por unidad.
    pub nights_reserved: u64,
}

/// Errores esperados del modelo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AirbnbError {
    /// Texto requerido vacío.
    EmptyText,
    /// Rango inválido.
    InvalidStayRange { check_in: Night, check_out: Night },
    /// Cantidad inválida.
    InvalidAmount,
    /// Usuario desconocido.
    UserNotFound { user_id: UserId },
    /// Listing desconocido.
    ListingNotFound { listing_id: ListingId },
    /// Reserva desconocida.
    ReservationNotFound { reservation_id: AirbnbReservationId },
    /// Usuario con rol incorrecto.
    WrongRole { user_id: UserId, expected: UserRole },
    /// Usuario suspendido.
    UserSuspended { user_id: UserId },
    /// Listing suspendido.
    ListingSuspended { listing_id: ListingId },
    /// Falta disponibilidad publicada.
    MissingAvailability { listing_id: ListingId, night: Night },
    /// Capacidad insuficiente.
    CapacityExceeded { requested: u32, capacity: u32 },
    /// Disponibilidad insuficiente.
    InsufficientAvailability { requested: u32, available: u32 },
    /// Precio fuera del filtro.
    PriceAboveLimit {
        total: MoneyCents,
        limit: MoneyCents,
    },
    /// Reseña duplicada.
    DuplicateReview { reservation_id: AirbnbReservationId },
    /// Rating inválido.
    InvalidRating { rating: u8 },
    /// Reserva no está confirmada.
    ReservationNotConfirmed {
        reservation_id: AirbnbReservationId,
        status: AirbnbReservationStatus,
    },
}

impl fmt::Display for AirbnbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyText => write!(f, "el texto no puede estar vacío"),
            Self::InvalidStayRange {
                check_in,
                check_out,
            } => write!(
                f,
                "rango inválido: check-in {check_in}, check-out {check_out}"
            ),
            Self::InvalidAmount => write!(f, "la cantidad debe ser mayor a cero"),
            Self::UserNotFound { user_id } => write!(f, "usuario {user_id} no existe"),
            Self::ListingNotFound { listing_id } => write!(f, "listing {listing_id} no existe"),
            Self::ReservationNotFound { reservation_id } => {
                write!(f, "reserva {reservation_id} no existe")
            }
            Self::WrongRole { user_id, expected } => {
                write!(f, "usuario {user_id} no tiene rol {expected:?}")
            }
            Self::UserSuspended { user_id } => write!(f, "usuario {user_id} está suspendido"),
            Self::ListingSuspended { listing_id } => {
                write!(f, "listing {listing_id} está suspendido")
            }
            Self::MissingAvailability { listing_id, night } => {
                write!(
                    f,
                    "listing {listing_id} no tiene disponibilidad en noche {night}"
                )
            }
            Self::CapacityExceeded {
                requested,
                capacity,
            } => write!(
                f,
                "se solicitaron {requested} huéspedes, capacidad {capacity}"
            ),
            Self::InsufficientAvailability {
                requested,
                available,
            } => write!(
                f,
                "se solicitaron {requested} unidades, pero solo hay {available}"
            ),
            Self::PriceAboveLimit { total, limit } => {
                write!(f, "precio total {total} supera el límite {limit}")
            }
            Self::DuplicateReview { reservation_id } => {
                write!(f, "la reserva {reservation_id} ya tiene reseña")
            }
            Self::InvalidRating { rating } => write!(f, "rating {rating} fuera de 1..=5"),
            Self::ReservationNotConfirmed {
                reservation_id,
                status,
            } => write!(f, "reserva {reservation_id} no está confirmada: {status:?}"),
        }
    }
}

impl Error for AirbnbError {}

/// Servicio en memoria para estudiar decisiones tipo Airbnb.
///
/// ```
/// use rust_system_design::airbnb::{AirbnbService, SearchQuery, StayRange, UserRole};
///
/// let mut service = AirbnbService::new();
/// let host = service.register_user("Ada Host", UserRole::Host).unwrap();
/// let guest = service.register_user("Grace Guest", UserRole::Guest).unwrap();
/// let listing = service.create_listing(host.id, "guadalajara", 2, 10_000).unwrap();
/// let stay = StayRange::new(1, 3).unwrap();
/// service.upsert_availability(listing.id, stay, 1).unwrap();
/// let results = service.search(SearchQuery {
///     city: "guadalajara".to_string(),
///     guests: 2,
///     stay,
///     max_total_price_cents: None,
/// }).unwrap();
/// assert_eq!(results.len(), 1);
/// service.book(guest.id, listing.id, stay, 2).unwrap();
/// ```
#[derive(Debug, Clone, Default)]
pub struct AirbnbService {
    next_user_id: UserId,
    next_listing_id: ListingId,
    next_reservation_id: AirbnbReservationId,
    next_review_id: ReviewId,
    users: HashMap<UserId, AirbnbUser>,
    listings: HashMap<ListingId, Listing>,
    calendar: HashMap<(ListingId, Night), ListingNight>,
    reservations: HashMap<AirbnbReservationId, AirbnbReservation>,
    reviews: HashMap<ReviewId, Review>,
    reviews_by_reservation: HashMap<AirbnbReservationId, ReviewId>,
    metrics: AirbnbMetrics,
}

impl AirbnbService {
    /// Crea un marketplace vacío.
    #[must_use]
    pub fn new() -> Self {
        Self {
            next_user_id: 1,
            next_listing_id: 1,
            next_reservation_id: 1,
            next_review_id: 1,
            ..Self::default()
        }
    }

    /// Registra un usuario.
    ///
    /// # Errors
    ///
    /// Devuelve error si el nombre está vacío.
    pub fn register_user(&mut self, name: &str, role: UserRole) -> Result<AirbnbUser, AirbnbError> {
        let user = AirbnbUser {
            id: self.next_user_id,
            name: normalize_text(name)?,
            role,
            status: UserStatus::Active,
        };
        self.next_user_id += 1;
        self.metrics.users_registered += 1;
        self.users.insert(user.id, user.clone());
        Ok(user)
    }

    /// Crea un listing activo.
    ///
    /// # Errors
    ///
    /// Devuelve error si el anfitrión no existe, no es host, está suspendido o
    /// los datos del listing son inválidos.
    pub fn create_listing(
        &mut self,
        host_id: UserId,
        city: &str,
        capacity: u32,
        price_per_night_cents: MoneyCents,
    ) -> Result<Listing, AirbnbError> {
        self.require_active_role(host_id, UserRole::Host)?;
        if capacity == 0 || price_per_night_cents == 0 {
            return Err(AirbnbError::InvalidAmount);
        }
        let listing = Listing {
            id: self.next_listing_id,
            host_id,
            city: normalize_text(city)?.to_lowercase(),
            capacity,
            price_per_night_cents,
            status: ListingStatus::Active,
        };
        self.next_listing_id += 1;
        self.metrics.listings_created += 1;
        self.listings.insert(listing.id, listing.clone());
        Ok(listing)
    }

    /// Publica disponibilidad para todas las noches del rango.
    ///
    /// # Errors
    ///
    /// Devuelve error si el listing no existe o las unidades son inválidas.
    pub fn upsert_availability(
        &mut self,
        listing_id: ListingId,
        stay: StayRange,
        available_units: u32,
    ) -> Result<(), AirbnbError> {
        self.listing(listing_id)?;
        if available_units == 0 {
            return Err(AirbnbError::InvalidAmount);
        }
        for night in stay.iter() {
            self.calendar
                .insert((listing_id, night), ListingNight { available_units });
        }
        Ok(())
    }

    /// Busca listings disponibles.
    ///
    /// # Errors
    ///
    /// Devuelve error si la consulta es inválida.
    pub fn search(&mut self, query: SearchQuery) -> Result<Vec<SearchResult>, AirbnbError> {
        let city = normalize_text(&query.city)?.to_lowercase();
        if query.guests == 0 {
            return Err(AirbnbError::InvalidAmount);
        }
        self.metrics.searches_performed += 1;
        let mut results = Vec::new();

        for listing in self.listings.values() {
            if listing.city != city || listing.capacity < query.guests {
                continue;
            }
            if !self.listing_and_host_are_active(listing) {
                continue;
            }
            let Ok(total_price_cents) = self.ensure_bookable(listing, query.stay, query.guests)
            else {
                continue;
            };
            if query
                .max_total_price_cents
                .is_some_and(|limit| total_price_cents > limit)
            {
                continue;
            }
            results.push(SearchResult {
                listing_id: listing.id,
                city: listing.city.clone(),
                total_price_cents,
                trust_score: self.trust_score(listing.id),
            });
        }
        results.sort_by_key(|result| {
            (
                std::cmp::Reverse(result.trust_score),
                result.total_price_cents,
            )
        });
        self.metrics.search_results_returned += results.len() as u64;
        Ok(results)
    }

    /// Confirma una reserva y bloquea noches.
    ///
    /// # Errors
    ///
    /// Devuelve error si huésped, listing, capacidad, disponibilidad o estado
    /// son inválidos.
    pub fn book(
        &mut self,
        guest_id: UserId,
        listing_id: ListingId,
        stay: StayRange,
        guests: u32,
    ) -> Result<AirbnbReservation, AirbnbError> {
        if let Err(error) = self.try_book(guest_id, listing_id, stay, guests) {
            self.metrics.bookings_rejected += 1;
            return Err(error);
        }
        let listing = self.listing(listing_id)?.clone();
        let total_price_cents = self.ensure_bookable(&listing, stay, guests)?;
        for night in stay.iter() {
            let inventory = self
                .calendar
                .get_mut(&(listing_id, night))
                .expect("ensure_bookable validó calendario");
            inventory.available_units -= 1;
        }
        let reservation = AirbnbReservation {
            id: self.next_reservation_id,
            guest_id,
            listing_id,
            stay,
            guests,
            total_price_cents,
            status: AirbnbReservationStatus::Confirmed,
        };
        self.next_reservation_id += 1;
        self.metrics.bookings_confirmed += 1;
        self.metrics.nights_reserved += u64::from(stay.nights());
        self.reservations
            .insert(reservation.id, reservation.clone());
        Ok(reservation)
    }

    /// Cancela una reserva y libera calendario.
    ///
    /// # Errors
    ///
    /// Devuelve error si la reserva no existe o ya no está confirmada.
    pub fn cancel_reservation(
        &mut self,
        reservation_id: AirbnbReservationId,
    ) -> Result<(), AirbnbError> {
        let reservation = self
            .reservations
            .get(&reservation_id)
            .cloned()
            .ok_or(AirbnbError::ReservationNotFound { reservation_id })?;
        if reservation.status != AirbnbReservationStatus::Confirmed {
            return Err(AirbnbError::ReservationNotConfirmed {
                reservation_id,
                status: reservation.status,
            });
        }
        for night in reservation.stay.iter() {
            let inventory = self
                .calendar
                .get_mut(&(reservation.listing_id, night))
                .expect("reserva confirmada debe apuntar a calendario");
            inventory.available_units += 1;
        }
        if let Some(stored) = self.reservations.get_mut(&reservation_id) {
            stored.status = AirbnbReservationStatus::Cancelled;
        }
        self.metrics.bookings_cancelled += 1;
        Ok(())
    }

    /// Crea una reseña para una reserva confirmada.
    ///
    /// # Errors
    ///
    /// Devuelve error si la reserva no existe, ya tiene reseña o rating es
    /// inválido.
    pub fn review(
        &mut self,
        reservation_id: AirbnbReservationId,
        rating: u8,
    ) -> Result<Review, AirbnbError> {
        if !(1..=5).contains(&rating) {
            return Err(AirbnbError::InvalidRating { rating });
        }
        if self.reviews_by_reservation.contains_key(&reservation_id) {
            return Err(AirbnbError::DuplicateReview { reservation_id });
        }
        let reservation = self
            .reservations
            .get(&reservation_id)
            .ok_or(AirbnbError::ReservationNotFound { reservation_id })?;
        if reservation.status != AirbnbReservationStatus::Confirmed {
            return Err(AirbnbError::ReservationNotConfirmed {
                reservation_id,
                status: reservation.status,
            });
        }
        let review = Review {
            id: self.next_review_id,
            reservation_id,
            listing_id: reservation.listing_id,
            rating,
        };
        self.next_review_id += 1;
        self.metrics.reviews_created += 1;
        self.reviews_by_reservation
            .insert(reservation_id, review.id);
        self.reviews.insert(review.id, review.clone());
        Ok(review)
    }

    /// Suspende un usuario.
    ///
    /// # Errors
    ///
    /// Devuelve error si el usuario no existe.
    pub fn suspend_user(&mut self, user_id: UserId) -> Result<(), AirbnbError> {
        let user = self
            .users
            .get_mut(&user_id)
            .ok_or(AirbnbError::UserNotFound { user_id })?;
        user.status = UserStatus::Suspended;
        self.metrics.suspensions_applied += 1;
        Ok(())
    }

    /// Suspende un listing.
    ///
    /// # Errors
    ///
    /// Devuelve error si el listing no existe.
    pub fn suspend_listing(&mut self, listing_id: ListingId) -> Result<(), AirbnbError> {
        let listing = self
            .listings
            .get_mut(&listing_id)
            .ok_or(AirbnbError::ListingNotFound { listing_id })?;
        listing.status = ListingStatus::Suspended;
        self.metrics.suspensions_applied += 1;
        Ok(())
    }

    /// Score simple de confianza para un listing.
    #[must_use]
    pub fn trust_score(&self, listing_id: ListingId) -> u8 {
        let ratings: Vec<u8> = self
            .reviews
            .values()
            .filter(|review| review.listing_id == listing_id)
            .map(|review| review.rating)
            .collect();
        if ratings.is_empty() {
            return 80;
        }
        let sum: u32 = ratings.iter().map(|rating| u32::from(*rating)).sum();
        let average = sum / ratings.len() as u32;
        (50 + average * 10).min(100) as u8
    }

    /// Métricas actuales.
    #[must_use]
    pub fn metrics(&self) -> AirbnbMetrics {
        self.metrics
    }

    /// Devuelve una reserva, si existe.
    #[must_use]
    pub fn reservation(&self, reservation_id: AirbnbReservationId) -> Option<&AirbnbReservation> {
        self.reservations.get(&reservation_id)
    }

    fn try_book(
        &self,
        guest_id: UserId,
        listing_id: ListingId,
        stay: StayRange,
        guests: u32,
    ) -> Result<(), AirbnbError> {
        self.require_active_role(guest_id, UserRole::Guest)?;
        let listing = self.listing(listing_id)?;
        if !self.listing_and_host_are_active(listing) {
            if listing.status == ListingStatus::Suspended {
                return Err(AirbnbError::ListingSuspended { listing_id });
            }
            return Err(AirbnbError::UserSuspended {
                user_id: listing.host_id,
            });
        }
        self.ensure_bookable(listing, stay, guests)?;
        Ok(())
    }

    fn ensure_bookable(
        &self,
        listing: &Listing,
        stay: StayRange,
        guests: u32,
    ) -> Result<MoneyCents, AirbnbError> {
        if guests == 0 {
            return Err(AirbnbError::InvalidAmount);
        }
        if guests > listing.capacity {
            return Err(AirbnbError::CapacityExceeded {
                requested: guests,
                capacity: listing.capacity,
            });
        }
        let mut min_available = u32::MAX;
        for night in stay.iter() {
            let inventory = self.calendar.get(&(listing.id, night)).ok_or(
                AirbnbError::MissingAvailability {
                    listing_id: listing.id,
                    night,
                },
            )?;
            min_available = min_available.min(inventory.available_units);
        }
        if min_available == 0 {
            return Err(AirbnbError::InsufficientAvailability {
                requested: 1,
                available: 0,
            });
        }
        Ok(listing.price_per_night_cents * u64::from(stay.nights()))
    }

    fn listing(&self, listing_id: ListingId) -> Result<&Listing, AirbnbError> {
        self.listings
            .get(&listing_id)
            .ok_or(AirbnbError::ListingNotFound { listing_id })
    }

    fn require_active_role(&self, user_id: UserId, expected: UserRole) -> Result<(), AirbnbError> {
        let user = self
            .users
            .get(&user_id)
            .ok_or(AirbnbError::UserNotFound { user_id })?;
        if user.role != expected {
            return Err(AirbnbError::WrongRole { user_id, expected });
        }
        if user.status == UserStatus::Suspended {
            return Err(AirbnbError::UserSuspended { user_id });
        }
        Ok(())
    }

    fn listing_and_host_are_active(&self, listing: &Listing) -> bool {
        listing.status == ListingStatus::Active
            && self
                .users
                .get(&listing.host_id)
                .is_some_and(|user| user.status == UserStatus::Active)
    }
}

fn normalize_text(value: &str) -> Result<String, AirbnbError> {
    let normalized = value.trim();
    if normalized.is_empty() {
        return Err(AirbnbError::EmptyText);
    }
    Ok(normalized.to_string())
}

#[cfg(test)]
mod tests {
    use super::{AirbnbError, AirbnbService, SearchQuery, StayRange, UserRole};

    #[test]
    fn searches_available_listing() {
        let mut service = AirbnbService::new();
        let host = service.register_user("Ada", UserRole::Host).unwrap();
        let listing = service
            .create_listing(host.id, "Guadalajara", 4, 15_000)
            .unwrap();
        let stay = StayRange::new(10, 13).unwrap();
        service.upsert_availability(listing.id, stay, 1).unwrap();

        let results = service
            .search(SearchQuery {
                city: "guadalajara".to_string(),
                guests: 2,
                stay,
                max_total_price_cents: Some(50_000),
            })
            .unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].total_price_cents, 45_000);
    }

    #[test]
    fn booking_reduces_availability() {
        let mut service = AirbnbService::new();
        let host = service.register_user("Ada", UserRole::Host).unwrap();
        let guest = service.register_user("Grace", UserRole::Guest).unwrap();
        let listing = service
            .create_listing(host.id, "Monterrey", 2, 10_000)
            .unwrap();
        let stay = StayRange::new(1, 3).unwrap();
        service.upsert_availability(listing.id, stay, 1).unwrap();

        service.book(guest.id, listing.id, stay, 2).unwrap();
        let results = service
            .search(SearchQuery {
                city: "monterrey".to_string(),
                guests: 2,
                stay,
                max_total_price_cents: None,
            })
            .unwrap();

        assert!(results.is_empty());
    }

    #[test]
    fn suspended_listing_is_rejected() {
        let mut service = AirbnbService::new();
        let host = service.register_user("Ada", UserRole::Host).unwrap();
        let guest = service.register_user("Grace", UserRole::Guest).unwrap();
        let listing = service
            .create_listing(host.id, "Puebla", 2, 10_000)
            .unwrap();
        let stay = StayRange::new(1, 2).unwrap();
        service.upsert_availability(listing.id, stay, 1).unwrap();
        service.suspend_listing(listing.id).unwrap();

        let error = service.book(guest.id, listing.id, stay, 2).unwrap_err();

        assert_eq!(
            error,
            AirbnbError::ListingSuspended {
                listing_id: listing.id
            }
        );
    }

    #[test]
    fn review_updates_trust_score_once() {
        let mut service = AirbnbService::new();
        let host = service.register_user("Ada", UserRole::Host).unwrap();
        let guest = service.register_user("Grace", UserRole::Guest).unwrap();
        let listing = service
            .create_listing(host.id, "Oaxaca", 2, 10_000)
            .unwrap();
        let stay = StayRange::new(1, 2).unwrap();
        service.upsert_availability(listing.id, stay, 1).unwrap();
        let reservation = service.book(guest.id, listing.id, stay, 2).unwrap();

        service.review(reservation.id, 5).unwrap();
        let duplicate = service.review(reservation.id, 4).unwrap_err();

        assert_eq!(service.trust_score(listing.id), 100);
        assert_eq!(
            duplicate,
            AirbnbError::DuplicateReview {
                reservation_id: reservation.id
            }
        );
    }
}
