//! Modelos educativos de System Design para Jeresoft Academy.
//!
//! Este crate acompaña el curso `rust-system-design`. Su propósito inicial es
//! exponer metadatos verificables del curso mientras los proyectos se
//! planifican como milestones e issues antes de tocar código de curso.

pub mod netflix;
pub mod tiny_url;
pub mod twitter;
pub mod uber;

/// Un proyecto canónico del curso de System Design.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemDesignProject {
    /// Número de orden dentro del curso.
    pub number: u8,
    /// Nombre público del proyecto.
    pub title: &'static str,
    /// Enfoque principal del proyecto.
    pub focus: &'static str,
}

/// Nombre canónico del curso dentro de Jeresoft Academy.
#[must_use]
pub fn course_name() -> &'static str {
    "rust-system-design"
}

/// Proyectos planeados para el curso, según RFC-0001 §10.
#[must_use]
pub fn planned_projects() -> &'static [SystemDesignProject] {
    &[
        SystemDesignProject {
            number: 1,
            title: "TinyURL",
            focus: "identificadores, redirecciones, almacenamiento y caché",
        },
        SystemDesignProject {
            number: 2,
            title: "Twitter",
            focus: "feeds, fan-out, ranking y consistencia eventual",
        },
        SystemDesignProject {
            number: 3,
            title: "Uber",
            focus: "geolocalización, matching, eventos y tiempo real",
        },
        SystemDesignProject {
            number: 4,
            title: "Netflix",
            focus: "streaming, catálogo, recomendaciones y entrega de contenido",
        },
        SystemDesignProject {
            number: 5,
            title: "Dropbox",
            focus: "sincronización, metadatos, conflictos y almacenamiento",
        },
        SystemDesignProject {
            number: 6,
            title: "Google Docs",
            focus: "colaboración, edición concurrente y resolución de conflictos",
        },
        SystemDesignProject {
            number: 7,
            title: "Redis",
            focus: "memoria, estructuras, persistencia y replicación",
        },
        SystemDesignProject {
            number: 8,
            title: "Kafka",
            focus: "logs distribuidos, particiones, consumidores y retención",
        },
        SystemDesignProject {
            number: 9,
            title: "Booking Engine",
            focus: "inventario, disponibilidad, reservas y consistencia",
        },
        SystemDesignProject {
            number: 10,
            title: "Airbnb",
            focus: "búsqueda, disponibilidad, confianza y marketplace",
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::{course_name, planned_projects};

    #[test]
    fn exposes_course_name() {
        assert_eq!(course_name(), "rust-system-design");
    }

    #[test]
    fn exposes_ten_planned_projects() {
        assert_eq!(planned_projects().len(), 10);
    }

    #[test]
    fn keeps_booking_engine_as_travel_tech_bridge() {
        let project = planned_projects()
            .iter()
            .find(|project| project.title == "Booking Engine")
            .expect("Booking Engine debe existir como puente hacia Travel Tech");

        assert!(project.focus.contains("reservas"));
    }
}
