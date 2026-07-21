//! Modelo educativo de Booking Engine.
//!
//! El módulo estudia inventario por noche, cotización, holds temporales,
//! confirmación de reservas y liberación de disponibilidad sin pagos reales ni
//! integraciones hoteleras externas.

use std::collections::HashMap;
use std::error::Error;
use std::fmt;

/// Identificador de hold.
pub type HoldId = u64;

/// Identificador de reserva.
pub type ReservationId = u64;

/// Noche lógica dentro del calendario educativo.
pub type Night = u32;

/// Dinero en centavos para evitar punto flotante.
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
    /// Crea un rango válido de noches.
    ///
    /// # Errors
    ///
    /// Devuelve error si la salida no es posterior a la entrada.
    pub fn new(check_in: Night, check_out: Night) -> Result<Self, BookingError> {
        if check_out <= check_in {
            return Err(BookingError::InvalidStayRange {
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

/// Llave conceptual de inventario.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InventoryKey {
    /// Propiedad, hotel, tour o producto reservable.
    pub property_id: String,
    /// Tipo de unidad reservable.
    pub unit_type_id: String,
}

/// Inventario visible para una noche.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NightInventory {
    /// Capacidad total configurada.
    pub capacity: u32,
    /// Precio base de esa noche en centavos.
    pub price_cents: MoneyCents,
    /// Unidades retenidas por holds activos.
    pub held_units: u32,
    /// Unidades confirmadas como reservas.
    pub reserved_units: u32,
}

impl NightInventory {
    fn available_units(self) -> u32 {
        self.capacity
            .saturating_sub(self.held_units)
            .saturating_sub(self.reserved_units)
    }
}

/// Cotización y disponibilidad para una estancia.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AvailabilityQuote {
    /// Propiedad consultada.
    pub property_id: String,
    /// Tipo de unidad consultado.
    pub unit_type_id: String,
    /// Rango de estancia.
    pub stay: StayRange,
    /// Unidades solicitadas.
    pub requested_units: u32,
    /// Si hay inventario suficiente en todas las noches.
    pub available: bool,
    /// Mínimo disponible dentro del rango.
    pub available_units: u32,
    /// Total para todas las unidades y noches.
    pub total_price_cents: MoneyCents,
}

/// Estado de un hold.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HoldStatus {
    /// Aparta inventario y puede confirmarse.
    Active,
    /// Fue confirmado como reserva.
    Confirmed,
    /// Fue cancelado antes de confirmarse.
    Cancelled,
    /// Expiró antes de confirmarse.
    Expired,
}

/// Hold temporal de inventario.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BookingHold {
    /// Identificador.
    pub id: HoldId,
    /// Inventario retenido.
    pub key: InventoryKey,
    /// Rango retenido.
    pub stay: StayRange,
    /// Unidades retenidas.
    pub units: u32,
    /// Tick de expiración.
    pub expires_at_tick: u64,
    /// Cotización aceptada al crear el hold.
    pub total_price_cents: MoneyCents,
    /// Estado actual.
    pub status: HoldStatus,
}

/// Estado de una reserva.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReservationStatus {
    /// Reserva confirmada.
    Confirmed,
    /// Reserva cancelada.
    Cancelled,
}

/// Reserva confirmada.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BookingReservation {
    /// Identificador.
    pub id: ReservationId,
    /// Hold origen.
    pub hold_id: HoldId,
    /// Inventario reservado.
    pub key: InventoryKey,
    /// Rango reservado.
    pub stay: StayRange,
    /// Unidades reservadas.
    pub units: u32,
    /// Huésped o cliente educativo.
    pub guest_id: String,
    /// Precio total confirmado.
    pub total_price_cents: MoneyCents,
    /// Estado actual.
    pub status: ReservationStatus,
}

/// Métricas mínimas del modelo.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BookingMetrics {
    /// Consultas de disponibilidad.
    pub availability_checks: u64,
    /// Holds creados.
    pub holds_created: u64,
    /// Holds rechazados.
    pub holds_rejected: u64,
    /// Holds expirados.
    pub holds_expired: u64,
    /// Holds cancelados.
    pub holds_cancelled: u64,
    /// Reservas confirmadas.
    pub reservations_confirmed: u64,
    /// Reservas canceladas.
    pub reservations_cancelled: u64,
    /// Reservas rechazadas.
    pub reservations_rejected: u64,
    /// Noches retenidas por unidades.
    pub nights_held: u64,
    /// Noches confirmadas por unidades.
    pub nights_reserved: u64,
}

/// Errores esperados del modelo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BookingError {
    /// Texto requerido vacío.
    EmptyText,
    /// Unidades o capacidad inválidas.
    InvalidUnits,
    /// Precio inválido.
    InvalidPrice,
    /// Rango inválido.
    InvalidStayRange { check_in: Night, check_out: Night },
    /// Falta inventario para una noche.
    MissingInventory { night: Night },
    /// No hay unidades suficientes.
    InsufficientAvailability { requested: u32, available: u32 },
    /// Hold desconocido.
    HoldNotFound { hold_id: HoldId },
    /// Hold expirado.
    HoldExpired { hold_id: HoldId },
    /// Hold no está activo.
    HoldNotActive { hold_id: HoldId, status: HoldStatus },
    /// Reserva desconocida.
    ReservationNotFound { reservation_id: ReservationId },
    /// Reserva no está activa.
    ReservationNotActive {
        reservation_id: ReservationId,
        status: ReservationStatus,
    },
}

impl fmt::Display for BookingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyText => write!(f, "el texto no puede estar vacío"),
            Self::InvalidUnits => write!(f, "las unidades deben ser mayores a cero"),
            Self::InvalidPrice => write!(f, "el precio debe ser mayor a cero"),
            Self::InvalidStayRange {
                check_in,
                check_out,
            } => write!(
                f,
                "rango inválido: check-in {check_in}, check-out {check_out}"
            ),
            Self::MissingInventory { night } => write!(f, "falta inventario para la noche {night}"),
            Self::InsufficientAvailability {
                requested,
                available,
            } => write!(
                f,
                "se solicitaron {requested} unidades, pero solo hay {available}"
            ),
            Self::HoldNotFound { hold_id } => write!(f, "hold {hold_id} no existe"),
            Self::HoldExpired { hold_id } => write!(f, "hold {hold_id} expiró"),
            Self::HoldNotActive { hold_id, status } => {
                write!(f, "hold {hold_id} no está activo: {status:?}")
            }
            Self::ReservationNotFound { reservation_id } => {
                write!(f, "reserva {reservation_id} no existe")
            }
            Self::ReservationNotActive {
                reservation_id,
                status,
            } => write!(f, "reserva {reservation_id} no está activa: {status:?}"),
        }
    }
}

impl Error for BookingError {}

/// Servicio en memoria para estudiar decisiones tipo Booking Engine.
///
/// ```
/// use rust_system_design::booking_engine::{BookingService, StayRange};
///
/// let mut service = BookingService::new();
/// let stay = StayRange::new(10, 12).unwrap();
/// service.upsert_inventory("hotel-1", "standard", stay, 2, 10_000).unwrap();
/// let hold = service.create_hold("hotel-1", "standard", stay, 1, 5).unwrap();
/// let reservation = service.confirm_hold(hold.id, "ada").unwrap();
/// assert_eq!(reservation.total_price_cents, 20_000);
/// ```
#[derive(Debug, Clone, Default)]
pub struct BookingService {
    logical_tick: u64,
    next_hold_id: HoldId,
    next_reservation_id: ReservationId,
    calendar: HashMap<(InventoryKey, Night), NightInventory>,
    holds: HashMap<HoldId, BookingHold>,
    reservations: HashMap<ReservationId, BookingReservation>,
    metrics: BookingMetrics,
}

impl BookingService {
    /// Crea un servicio vacío.
    #[must_use]
    pub fn new() -> Self {
        Self {
            next_hold_id: 1,
            next_reservation_id: 1,
            ..Self::default()
        }
    }

    /// Registra inventario para todas las noches de un rango.
    ///
    /// # Errors
    ///
    /// Devuelve error si propiedad, unidad, capacidad o precio son inválidos.
    pub fn upsert_inventory(
        &mut self,
        property_id: &str,
        unit_type_id: &str,
        stay: StayRange,
        capacity: u32,
        price_cents: MoneyCents,
    ) -> Result<(), BookingError> {
        let key = inventory_key(property_id, unit_type_id)?;
        if capacity == 0 {
            return Err(BookingError::InvalidUnits);
        }
        if price_cents == 0 {
            return Err(BookingError::InvalidPrice);
        }

        for night in stay.iter() {
            let previous = self.calendar.get(&(key.clone(), night)).copied();
            let held_units = previous.map_or(0, |entry| entry.held_units);
            let reserved_units = previous.map_or(0, |entry| entry.reserved_units);
            self.calendar.insert(
                (key.clone(), night),
                NightInventory {
                    capacity,
                    price_cents,
                    held_units,
                    reserved_units,
                },
            );
        }
        Ok(())
    }

    /// Consulta disponibilidad y cotización.
    ///
    /// # Errors
    ///
    /// Devuelve error si falta inventario o la solicitud es inválida.
    pub fn availability(
        &mut self,
        property_id: &str,
        unit_type_id: &str,
        stay: StayRange,
        units: u32,
    ) -> Result<AvailabilityQuote, BookingError> {
        self.expire_due_holds();
        self.metrics.availability_checks += 1;
        let key = inventory_key(property_id, unit_type_id)?;
        self.quote(&key, stay, units)
    }

    /// Crea un hold temporal.
    ///
    /// # Errors
    ///
    /// Devuelve error si no hay disponibilidad suficiente o la solicitud es
    /// inválida.
    pub fn create_hold(
        &mut self,
        property_id: &str,
        unit_type_id: &str,
        stay: StayRange,
        units: u32,
        ttl_ticks: u64,
    ) -> Result<BookingHold, BookingError> {
        self.expire_due_holds();
        let key = inventory_key(property_id, unit_type_id)?;
        let quote = self.quote(&key, stay, units)?;
        if !quote.available {
            self.metrics.holds_rejected += 1;
            return Err(BookingError::InsufficientAvailability {
                requested: units,
                available: quote.available_units,
            });
        }

        for night in stay.iter() {
            let inventory = self
                .calendar
                .get_mut(&(key.clone(), night))
                .expect("quote validó inventario");
            inventory.held_units += units;
        }

        let hold = BookingHold {
            id: self.next_hold_id,
            key,
            stay,
            units,
            expires_at_tick: self.logical_tick + ttl_ticks.max(1),
            total_price_cents: quote.total_price_cents,
            status: HoldStatus::Active,
        };
        self.next_hold_id += 1;
        self.metrics.holds_created += 1;
        self.metrics.nights_held += u64::from(units) * u64::from(stay.nights());
        self.holds.insert(hold.id, hold.clone());
        Ok(hold)
    }

    /// Confirma un hold como reserva.
    ///
    /// # Errors
    ///
    /// Devuelve error si el hold no existe, expiró o no está activo.
    pub fn confirm_hold(
        &mut self,
        hold_id: HoldId,
        guest_id: &str,
    ) -> Result<BookingReservation, BookingError> {
        self.expire_due_holds();
        let guest_id = normalize_text(guest_id)?;
        let hold = self
            .holds
            .get(&hold_id)
            .cloned()
            .ok_or(BookingError::HoldNotFound { hold_id })?;
        if hold.status == HoldStatus::Expired {
            self.metrics.reservations_rejected += 1;
            return Err(BookingError::HoldExpired { hold_id });
        }
        if hold.status != HoldStatus::Active {
            self.metrics.reservations_rejected += 1;
            return Err(BookingError::HoldNotActive {
                hold_id,
                status: hold.status,
            });
        }

        for night in hold.stay.iter() {
            let inventory = self
                .calendar
                .get_mut(&(hold.key.clone(), night))
                .expect("hold activo debe apuntar a inventario");
            inventory.held_units = inventory.held_units.saturating_sub(hold.units);
            inventory.reserved_units += hold.units;
        }

        let reservation = BookingReservation {
            id: self.next_reservation_id,
            hold_id,
            key: hold.key,
            stay: hold.stay,
            units: hold.units,
            guest_id,
            total_price_cents: hold.total_price_cents,
            status: ReservationStatus::Confirmed,
        };
        self.next_reservation_id += 1;
        self.metrics.reservations_confirmed += 1;
        self.metrics.nights_reserved +=
            u64::from(reservation.units) * u64::from(hold.stay.nights());
        self.reservations
            .insert(reservation.id, reservation.clone());
        if let Some(stored_hold) = self.holds.get_mut(&hold_id) {
            stored_hold.status = HoldStatus::Confirmed;
        }
        Ok(reservation)
    }

    /// Cancela un hold activo.
    ///
    /// # Errors
    ///
    /// Devuelve error si el hold no existe o ya no está activo.
    pub fn cancel_hold(&mut self, hold_id: HoldId) -> Result<(), BookingError> {
        self.expire_due_holds();
        let hold = self
            .holds
            .get(&hold_id)
            .cloned()
            .ok_or(BookingError::HoldNotFound { hold_id })?;
        if hold.status != HoldStatus::Active {
            return Err(BookingError::HoldNotActive {
                hold_id,
                status: hold.status,
            });
        }
        self.release_hold_units(&hold);
        if let Some(stored_hold) = self.holds.get_mut(&hold_id) {
            stored_hold.status = HoldStatus::Cancelled;
        }
        self.metrics.holds_cancelled += 1;
        Ok(())
    }

    /// Cancela una reserva confirmada.
    ///
    /// # Errors
    ///
    /// Devuelve error si la reserva no existe o ya fue cancelada.
    pub fn cancel_reservation(
        &mut self,
        reservation_id: ReservationId,
    ) -> Result<(), BookingError> {
        self.expire_due_holds();
        let reservation = self
            .reservations
            .get(&reservation_id)
            .cloned()
            .ok_or(BookingError::ReservationNotFound { reservation_id })?;
        if reservation.status != ReservationStatus::Confirmed {
            return Err(BookingError::ReservationNotActive {
                reservation_id,
                status: reservation.status,
            });
        }
        for night in reservation.stay.iter() {
            let inventory = self
                .calendar
                .get_mut(&(reservation.key.clone(), night))
                .expect("reserva confirmada debe apuntar a inventario");
            inventory.reserved_units = inventory.reserved_units.saturating_sub(reservation.units);
        }
        if let Some(stored_reservation) = self.reservations.get_mut(&reservation_id) {
            stored_reservation.status = ReservationStatus::Cancelled;
        }
        self.metrics.reservations_cancelled += 1;
        Ok(())
    }

    /// Avanza el reloj lógico y expira holds vencidos.
    #[must_use]
    pub fn advance_time(&mut self, ticks: u64) -> u64 {
        self.logical_tick += ticks;
        self.expire_due_holds()
    }

    /// Devuelve una copia de un hold.
    #[must_use]
    pub fn hold(&self, hold_id: HoldId) -> Option<&BookingHold> {
        self.holds.get(&hold_id)
    }

    /// Devuelve una copia de una reserva.
    #[must_use]
    pub fn reservation(&self, reservation_id: ReservationId) -> Option<&BookingReservation> {
        self.reservations.get(&reservation_id)
    }

    /// Métricas actuales.
    #[must_use]
    pub fn metrics(&self) -> BookingMetrics {
        self.metrics
    }

    fn quote(
        &self,
        key: &InventoryKey,
        stay: StayRange,
        units: u32,
    ) -> Result<AvailabilityQuote, BookingError> {
        if units == 0 {
            return Err(BookingError::InvalidUnits);
        }

        let mut min_available = u32::MAX;
        let mut nightly_price = 0;
        for night in stay.iter() {
            let inventory = self
                .calendar
                .get(&(key.clone(), night))
                .ok_or(BookingError::MissingInventory { night })?;
            min_available = min_available.min(inventory.available_units());
            nightly_price += inventory.price_cents;
        }

        Ok(AvailabilityQuote {
            property_id: key.property_id.clone(),
            unit_type_id: key.unit_type_id.clone(),
            stay,
            requested_units: units,
            available: min_available >= units,
            available_units: min_available,
            total_price_cents: nightly_price * u64::from(units),
        })
    }

    fn expire_due_holds(&mut self) -> u64 {
        let due_holds: Vec<BookingHold> = self
            .holds
            .values()
            .filter(|hold| {
                hold.status == HoldStatus::Active && hold.expires_at_tick <= self.logical_tick
            })
            .cloned()
            .collect();

        for hold in &due_holds {
            self.release_hold_units(hold);
            if let Some(stored_hold) = self.holds.get_mut(&hold.id) {
                stored_hold.status = HoldStatus::Expired;
            }
        }
        self.metrics.holds_expired += due_holds.len() as u64;
        due_holds.len() as u64
    }

    fn release_hold_units(&mut self, hold: &BookingHold) {
        for night in hold.stay.iter() {
            let inventory = self
                .calendar
                .get_mut(&(hold.key.clone(), night))
                .expect("hold activo debe apuntar a inventario");
            inventory.held_units = inventory.held_units.saturating_sub(hold.units);
        }
    }
}

fn inventory_key(property_id: &str, unit_type_id: &str) -> Result<InventoryKey, BookingError> {
    Ok(InventoryKey {
        property_id: normalize_text(property_id)?,
        unit_type_id: normalize_text(unit_type_id)?,
    })
}

fn normalize_text(value: &str) -> Result<String, BookingError> {
    let normalized = value.trim();
    if normalized.is_empty() {
        return Err(BookingError::EmptyText);
    }
    Ok(normalized.to_string())
}

#[cfg(test)]
mod tests {
    use super::{BookingError, BookingService, HoldStatus, StayRange};

    #[test]
    fn creates_hold_and_reduces_visible_availability() {
        let mut service = BookingService::new();
        let stay = StayRange::new(1, 3).unwrap();
        service
            .upsert_inventory("hotel", "standard", stay, 2, 10_000)
            .unwrap();

        service
            .create_hold("hotel", "standard", stay, 1, 5)
            .unwrap();
        let quote = service
            .availability("hotel", "standard", stay, 2)
            .expect("availability");

        assert!(!quote.available);
        assert_eq!(quote.available_units, 1);
    }

    #[test]
    fn expires_hold_and_releases_inventory() {
        let mut service = BookingService::new();
        let stay = StayRange::new(1, 2).unwrap();
        service
            .upsert_inventory("hotel", "standard", stay, 1, 10_000)
            .unwrap();
        let hold = service
            .create_hold("hotel", "standard", stay, 1, 1)
            .unwrap();

        let expired = service.advance_time(1);
        let quote = service
            .availability("hotel", "standard", stay, 1)
            .expect("availability");

        assert_eq!(expired, 1);
        assert_eq!(service.hold(hold.id).unwrap().status, HoldStatus::Expired);
        assert!(quote.available);
    }

    #[test]
    fn confirms_hold_into_reservation() {
        let mut service = BookingService::new();
        let stay = StayRange::new(10, 13).unwrap();
        service
            .upsert_inventory("hotel", "suite", stay, 2, 15_000)
            .unwrap();
        let hold = service.create_hold("hotel", "suite", stay, 2, 5).unwrap();

        let reservation = service.confirm_hold(hold.id, "ada").unwrap();

        assert_eq!(reservation.total_price_cents, 90_000);
        assert_eq!(service.metrics().reservations_confirmed, 1);
        assert_eq!(service.hold(hold.id).unwrap().status, HoldStatus::Confirmed);
    }

    #[test]
    fn rejects_invalid_stay_range() {
        assert_eq!(
            StayRange::new(3, 3).unwrap_err(),
            BookingError::InvalidStayRange {
                check_in: 3,
                check_out: 3
            }
        );
    }
}
