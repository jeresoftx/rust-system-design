//! Modelo educativo de Uber.
//!
//! El módulo estudia matching por proximidad, índice por celdas, asignación
//! exclusiva, eventos y estados del viaje sin mapas reales ni red.

use std::collections::{BTreeSet, HashMap};
use std::error::Error;
use std::fmt;

/// Identificador lógico de rider.
pub type RiderId = u64;
/// Identificador lógico de driver.
pub type DriverId = u64;
/// Identificador lógico de viaje.
pub type RideId = u64;

/// Configuración pedagógica del modelo Uber.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UberConfig {
    /// Tamaño de celda usado para indexar ubicación.
    pub cell_size: i32,
    /// Radio de búsqueda medido en celdas vecinas.
    pub search_radius_cells: i32,
    /// Distancia cuadrada máxima permitida entre pickup y driver.
    pub max_match_distance_sq: i64,
}

impl Default for UberConfig {
    fn default() -> Self {
        Self {
            cell_size: 10,
            search_radius_cells: 1,
            max_match_distance_sq: 400,
        }
    }
}

/// Punto lógico en una cuadrícula educativa.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Location {
    /// Coordenada horizontal.
    pub x: i32,
    /// Coordenada vertical.
    pub y: i32,
}

impl Location {
    /// Crea una ubicación lógica.
    #[must_use]
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn distance_sq(self, other: Self) -> i64 {
        let dx = i64::from(self.x) - i64::from(other.x);
        let dy = i64::from(self.y) - i64::from(other.y);
        dx * dx + dy * dy
    }
}

/// Rider registrado.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rider {
    /// Identificador interno.
    pub id: RiderId,
    /// Nombre visible.
    pub name: String,
    /// Tiempo lógico de registro.
    pub created_at_tick: u64,
}

/// Driver registrado.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Driver {
    /// Identificador interno.
    pub id: DriverId,
    /// Nombre visible.
    pub name: String,
    /// Ubicación más reciente.
    pub location: Location,
    /// Disponibilidad para recibir viajes.
    pub available: bool,
    /// Viaje activo, si existe.
    pub active_ride_id: Option<RideId>,
    /// Tiempo lógico de la última ubicación.
    pub last_location_tick: u64,
}

/// Estado de un viaje.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RideState {
    /// Solicitud creada, antes de asignar driver.
    Requested,
    /// Driver asignado.
    Assigned,
    /// Driver aceptó.
    Accepted,
    /// Viaje en curso.
    InProgress,
    /// Viaje completado.
    Completed,
    /// Viaje cancelado.
    Cancelled,
}

/// Viaje del modelo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ride {
    /// Identificador interno.
    pub id: RideId,
    /// Rider que solicitó.
    pub rider_id: RiderId,
    /// Driver asignado, si existe.
    pub driver_id: Option<DriverId>,
    /// Origen lógico.
    pub pickup: Location,
    /// Destino lógico.
    pub dropoff: Location,
    /// Estado actual.
    pub state: RideState,
    /// Tiempo lógico de creación.
    pub created_at_tick: u64,
}

/// Evento auditable del viaje.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RideEvent {
    /// Viaje relacionado.
    pub ride_id: RideId,
    /// Estado registrado.
    pub state: RideState,
    /// Descripción pedagógica.
    pub message: String,
    /// Tiempo lógico del evento.
    pub created_at_tick: u64,
}

/// Métricas mínimas del modelo.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct UberMetrics {
    /// Riders registrados.
    pub riders_registered: u64,
    /// Drivers registrados.
    pub drivers_registered: u64,
    /// Actualizaciones de ubicación.
    pub driver_location_updates: u64,
    /// Solicitudes de viaje.
    pub ride_requests: u64,
    /// Matches exitosos.
    pub matches_created: u64,
    /// Matches fallidos.
    pub match_failures: u64,
    /// Drivers candidatos revisados.
    pub candidate_drivers_scanned: u64,
    /// Transiciones de estado.
    pub ride_state_transitions: u64,
    /// Viajes completados.
    pub rides_completed: u64,
    /// Viajes cancelados.
    pub rides_cancelled: u64,
}

/// Errores esperados del modelo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UberError {
    /// Nombre vacío.
    EmptyName,
    /// Rider inexistente.
    UnknownRider { rider_id: RiderId },
    /// Driver inexistente.
    UnknownDriver { driver_id: DriverId },
    /// Viaje inexistente.
    UnknownRide { ride_id: RideId },
    /// No hay driver disponible para la solicitud.
    NoDriversAvailable,
    /// Transición de estado inválida.
    InvalidTransition {
        ride_id: RideId,
        from: RideState,
        to: RideState,
    },
}

impl fmt::Display for UberError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "el nombre no puede estar vacío"),
            Self::UnknownRider { rider_id } => write!(f, "el rider {rider_id} no existe"),
            Self::UnknownDriver { driver_id } => write!(f, "el driver {driver_id} no existe"),
            Self::UnknownRide { ride_id } => write!(f, "el viaje {ride_id} no existe"),
            Self::NoDriversAvailable => write!(f, "no hay drivers disponibles"),
            Self::InvalidTransition { ride_id, from, to } => write!(
                f,
                "transición inválida para viaje {ride_id}: {from:?} -> {to:?}"
            ),
        }
    }
}

impl Error for UberError {}

/// Servicio en memoria para estudiar matching tipo Uber.
///
/// ```
/// use rust_system_design::uber::{Location, RideState, UberService};
///
/// let mut service = UberService::new();
/// let rider = service.register_rider("Ada").unwrap();
/// let driver = service.register_driver("Grace", Location::new(0, 0)).unwrap();
/// let ride = service
///     .request_ride(rider.id, Location::new(1, 1), Location::new(10, 10))
///     .unwrap();
///
/// assert_eq!(ride.driver_id, Some(driver.id));
/// assert_eq!(ride.state, RideState::Assigned);
/// ```
#[derive(Debug, Clone)]
pub struct UberService {
    config: UberConfig,
    logical_clock: u64,
    next_rider_id: RiderId,
    next_driver_id: DriverId,
    next_ride_id: RideId,
    riders: HashMap<RiderId, Rider>,
    drivers: HashMap<DriverId, Driver>,
    rides: HashMap<RideId, Ride>,
    events: HashMap<RideId, Vec<RideEvent>>,
    available_by_cell: HashMap<(i32, i32), BTreeSet<DriverId>>,
    metrics: UberMetrics,
}

impl UberService {
    /// Crea un servicio con configuración por defecto.
    #[must_use]
    pub fn new() -> Self {
        Self::with_config(UberConfig::default())
    }

    /// Crea un servicio con configuración explícita.
    #[must_use]
    pub fn with_config(config: UberConfig) -> Self {
        Self {
            config,
            logical_clock: 0,
            next_rider_id: 1,
            next_driver_id: 1,
            next_ride_id: 1,
            riders: HashMap::new(),
            drivers: HashMap::new(),
            rides: HashMap::new(),
            events: HashMap::new(),
            available_by_cell: HashMap::new(),
            metrics: UberMetrics::default(),
        }
    }

    /// Registra un rider.
    ///
    /// # Errors
    ///
    /// Devuelve error si el nombre está vacío.
    pub fn register_rider(&mut self, name: &str) -> Result<Rider, UberError> {
        let name = normalize_name(name)?;
        self.logical_clock += 1;
        let rider = Rider {
            id: self.next_rider_id,
            name,
            created_at_tick: self.logical_clock,
        };
        self.next_rider_id += 1;
        self.riders.insert(rider.id, rider.clone());
        self.metrics.riders_registered += 1;
        Ok(rider)
    }

    /// Registra un driver disponible.
    ///
    /// # Errors
    ///
    /// Devuelve error si el nombre está vacío.
    pub fn register_driver(&mut self, name: &str, location: Location) -> Result<Driver, UberError> {
        let name = normalize_name(name)?;
        self.logical_clock += 1;
        let driver = Driver {
            id: self.next_driver_id,
            name,
            location,
            available: true,
            active_ride_id: None,
            last_location_tick: self.logical_clock,
        };
        self.next_driver_id += 1;
        self.index_available_driver(driver.id, driver.location);
        self.drivers.insert(driver.id, driver.clone());
        self.metrics.drivers_registered += 1;
        Ok(driver)
    }

    /// Actualiza ubicación de driver.
    ///
    /// # Errors
    ///
    /// Devuelve error si el driver no existe.
    pub fn update_driver_location(
        &mut self,
        driver_id: DriverId,
        location: Location,
    ) -> Result<(), UberError> {
        let old_location = self
            .drivers
            .get(&driver_id)
            .ok_or(UberError::UnknownDriver { driver_id })?
            .location;
        self.remove_available_driver(driver_id, old_location);
        self.logical_clock += 1;
        let driver = self
            .drivers
            .get_mut(&driver_id)
            .ok_or(UberError::UnknownDriver { driver_id })?;
        driver.location = location;
        driver.last_location_tick = self.logical_clock;
        if driver.available {
            self.index_available_driver(driver_id, location);
        }
        self.metrics.driver_location_updates += 1;
        Ok(())
    }

    /// Solicita un viaje y asigna driver si hay candidato cercano.
    ///
    /// # Errors
    ///
    /// Devuelve error si el rider no existe o si no hay drivers disponibles.
    pub fn request_ride(
        &mut self,
        rider_id: RiderId,
        pickup: Location,
        dropoff: Location,
    ) -> Result<Ride, UberError> {
        self.ensure_rider(rider_id)?;
        self.metrics.ride_requests += 1;

        let Some(driver_id) = self.find_nearest_available_driver(pickup) else {
            self.metrics.match_failures += 1;
            return Err(UberError::NoDriversAvailable);
        };

        self.logical_clock += 1;
        let ride = Ride {
            id: self.next_ride_id,
            rider_id,
            driver_id: Some(driver_id),
            pickup,
            dropoff,
            state: RideState::Assigned,
            created_at_tick: self.logical_clock,
        };
        self.next_ride_id += 1;

        let driver_location = self
            .drivers
            .get(&driver_id)
            .expect("driver candidato debe existir")
            .location;
        self.remove_available_driver(driver_id, driver_location);
        let driver = self
            .drivers
            .get_mut(&driver_id)
            .expect("driver candidato debe existir");
        driver.available = false;
        driver.active_ride_id = Some(ride.id);

        self.rides.insert(ride.id, ride.clone());
        self.metrics.matches_created += 1;
        self.record_event(ride.id, RideState::Assigned, "driver asignado");
        Ok(ride)
    }

    /// Acepta un viaje asignado.
    ///
    /// # Errors
    ///
    /// Devuelve error si el viaje no existe o la transición es inválida.
    pub fn accept_ride(&mut self, ride_id: RideId) -> Result<Ride, UberError> {
        self.transition_ride(ride_id, RideState::Accepted)
    }

    /// Inicia un viaje aceptado.
    ///
    /// # Errors
    ///
    /// Devuelve error si el viaje no existe o la transición es inválida.
    pub fn start_ride(&mut self, ride_id: RideId) -> Result<Ride, UberError> {
        self.transition_ride(ride_id, RideState::InProgress)
    }

    /// Completa un viaje en curso y libera el driver.
    ///
    /// # Errors
    ///
    /// Devuelve error si el viaje no existe o la transición es inválida.
    pub fn complete_ride(&mut self, ride_id: RideId) -> Result<Ride, UberError> {
        let ride = self.transition_ride(ride_id, RideState::Completed)?;
        if let Some(driver_id) = ride.driver_id {
            self.release_driver(driver_id);
        }
        self.metrics.rides_completed += 1;
        Ok(ride)
    }

    /// Cancela un viaje activo y libera el driver si estaba asignado.
    ///
    /// # Errors
    ///
    /// Devuelve error si el viaje no existe o la transición es inválida.
    pub fn cancel_ride(&mut self, ride_id: RideId) -> Result<Ride, UberError> {
        let ride = self.transition_ride(ride_id, RideState::Cancelled)?;
        if let Some(driver_id) = ride.driver_id {
            self.release_driver(driver_id);
        }
        self.metrics.rides_cancelled += 1;
        Ok(ride)
    }

    /// Devuelve un viaje si existe.
    #[must_use]
    pub fn ride(&self, ride_id: RideId) -> Option<&Ride> {
        self.rides.get(&ride_id)
    }

    /// Devuelve un driver si existe.
    #[must_use]
    pub fn driver(&self, driver_id: DriverId) -> Option<&Driver> {
        self.drivers.get(&driver_id)
    }

    /// Devuelve eventos de un viaje.
    #[must_use]
    pub fn events(&self, ride_id: RideId) -> &[RideEvent] {
        self.events.get(&ride_id).map(Vec::as_slice).unwrap_or(&[])
    }

    /// Métricas acumuladas.
    #[must_use]
    pub fn metrics(&self) -> UberMetrics {
        self.metrics
    }

    fn transition_ride(
        &mut self,
        ride_id: RideId,
        next_state: RideState,
    ) -> Result<Ride, UberError> {
        let current = self
            .rides
            .get(&ride_id)
            .ok_or(UberError::UnknownRide { ride_id })?
            .state;
        if !is_valid_transition(current, next_state) {
            return Err(UberError::InvalidTransition {
                ride_id,
                from: current,
                to: next_state,
            });
        }

        self.logical_clock += 1;
        let ride = self
            .rides
            .get_mut(&ride_id)
            .ok_or(UberError::UnknownRide { ride_id })?;
        ride.state = next_state;
        let ride = ride.clone();
        self.metrics.ride_state_transitions += 1;
        self.record_event(ride_id, next_state, "transición de estado");
        Ok(ride)
    }

    fn release_driver(&mut self, driver_id: DriverId) {
        let Some(driver) = self.drivers.get_mut(&driver_id) else {
            return;
        };
        driver.available = true;
        driver.active_ride_id = None;
        let location = driver.location;
        self.index_available_driver(driver_id, location);
    }

    fn find_nearest_available_driver(&mut self, pickup: Location) -> Option<DriverId> {
        let pickup_cell = self.cell_for(pickup);
        let mut best: Option<(DriverId, i64)> = None;
        for dx in -self.config.search_radius_cells..=self.config.search_radius_cells {
            for dy in -self.config.search_radius_cells..=self.config.search_radius_cells {
                let cell = (pickup_cell.0 + dx, pickup_cell.1 + dy);
                let driver_ids: Vec<_> = self
                    .available_by_cell
                    .get(&cell)
                    .map(|ids| ids.iter().copied().collect())
                    .unwrap_or_default();
                for driver_id in driver_ids {
                    self.metrics.candidate_drivers_scanned += 1;
                    let Some(driver) = self.drivers.get(&driver_id) else {
                        continue;
                    };
                    if !driver.available {
                        continue;
                    }
                    let distance = pickup.distance_sq(driver.location);
                    if distance > self.config.max_match_distance_sq {
                        continue;
                    }
                    if best
                        .map(|(best_id, best_distance)| {
                            distance < best_distance
                                || (distance == best_distance && driver_id < best_id)
                        })
                        .unwrap_or(true)
                    {
                        best = Some((driver_id, distance));
                    }
                }
            }
        }

        best.map(|(driver_id, _)| driver_id)
    }

    fn ensure_rider(&self, rider_id: RiderId) -> Result<(), UberError> {
        if self.riders.contains_key(&rider_id) {
            Ok(())
        } else {
            Err(UberError::UnknownRider { rider_id })
        }
    }

    fn record_event(&mut self, ride_id: RideId, state: RideState, message: &str) {
        self.events.entry(ride_id).or_default().push(RideEvent {
            ride_id,
            state,
            message: message.to_string(),
            created_at_tick: self.logical_clock,
        });
    }

    fn index_available_driver(&mut self, driver_id: DriverId, location: Location) {
        let cell = self.cell_for(location);
        self.available_by_cell
            .entry(cell)
            .or_default()
            .insert(driver_id);
    }

    fn remove_available_driver(&mut self, driver_id: DriverId, location: Location) {
        let cell = self.cell_for(location);
        if let Some(drivers) = self.available_by_cell.get_mut(&cell) {
            drivers.remove(&driver_id);
        }
    }

    fn cell_for(&self, location: Location) -> (i32, i32) {
        (
            location.x.div_euclid(self.config.cell_size),
            location.y.div_euclid(self.config.cell_size),
        )
    }
}

impl Default for UberService {
    fn default() -> Self {
        Self::new()
    }
}

fn normalize_name(name: &str) -> Result<String, UberError> {
    let name = name.trim();
    if name.is_empty() {
        return Err(UberError::EmptyName);
    }

    Ok(name.to_string())
}

fn is_valid_transition(from: RideState, to: RideState) -> bool {
    matches!(
        (from, to),
        (RideState::Assigned, RideState::Accepted)
            | (RideState::Assigned, RideState::Cancelled)
            | (RideState::Accepted, RideState::InProgress)
            | (RideState::Accepted, RideState::Cancelled)
            | (RideState::InProgress, RideState::Completed)
            | (RideState::InProgress, RideState::Cancelled)
    )
}

#[cfg(test)]
mod tests {
    use super::{Location, RideState, UberError, UberService};

    #[test]
    fn assigns_nearest_available_driver() {
        let mut service = UberService::new();
        let rider = service.register_rider("Ada").expect("rider");
        let far = service
            .register_driver("Far", Location::new(10, 10))
            .expect("far");
        let near = service
            .register_driver("Near", Location::new(1, 1))
            .expect("near");

        let ride = service
            .request_ride(rider.id, Location::new(0, 0), Location::new(20, 20))
            .expect("ride");

        assert_eq!(ride.driver_id, Some(near.id));
        assert_eq!(ride.state, RideState::Assigned);
        assert!(!service.driver(near.id).expect("near driver").available);
        assert!(service.driver(far.id).expect("far driver").available);
    }

    #[test]
    fn rejects_request_when_no_driver_available() {
        let mut service = UberService::new();
        let rider = service.register_rider("Ada").expect("rider");

        let error = service
            .request_ride(rider.id, Location::new(0, 0), Location::new(1, 1))
            .expect_err("sin drivers");

        assert_eq!(error, UberError::NoDriversAvailable);
        assert_eq!(service.metrics().match_failures, 1);
    }

    #[test]
    fn completes_ride_and_releases_driver() {
        let mut service = UberService::new();
        let rider = service.register_rider("Ada").expect("rider");
        let driver = service
            .register_driver("Grace", Location::new(0, 0))
            .expect("driver");
        let ride = service
            .request_ride(rider.id, Location::new(0, 1), Location::new(5, 5))
            .expect("ride");

        service.accept_ride(ride.id).expect("accepted");
        service.start_ride(ride.id).expect("started");
        let completed = service.complete_ride(ride.id).expect("completed");

        assert_eq!(completed.state, RideState::Completed);
        assert!(service.driver(driver.id).expect("driver").available);
        assert_eq!(service.metrics().rides_completed, 1);
    }

    #[test]
    fn rejects_invalid_transition() {
        let mut service = UberService::new();
        let rider = service.register_rider("Ada").expect("rider");
        service
            .register_driver("Grace", Location::new(0, 0))
            .expect("driver");
        let ride = service
            .request_ride(rider.id, Location::new(0, 1), Location::new(5, 5))
            .expect("ride");

        let error = service
            .complete_ride(ride.id)
            .expect_err("no puede completar sin iniciar");

        assert_eq!(
            error,
            UberError::InvalidTransition {
                ride_id: ride.id,
                from: RideState::Assigned,
                to: RideState::Completed
            }
        );
    }
}
