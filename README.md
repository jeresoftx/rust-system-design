# Rust System Design

Repositorio del camino troncal de Jeresoft Academy para estudiar diseño de
sistemas en Rust. Pertenece al Semestre 4 del plan de estudios junto con
`rust-distributed-systems` (RFC-0001 §10).

El objetivo no es memorizar entrevistas ni repetir recetas. El objetivo es
aprender a diseñar sistemas explicando requisitos, capacidad, datos,
consistencia, fallas, tradeoffs y evolución operativa.

## Qué contiene

- Capítulos-proyecto en Markdown compatibles con mdBook.
- Modelos Rust pequeños para representar decisiones de diseño.
- Ejemplos progresivos: básico, intermedio, avanzado y caso real.
- Tests unitarios, tests de integración y doctests.
- Benchmarks cuando una decisión de diseño tenga costo observable.
- Diagramas Mermaid y recursos visuales.
- Ejercicios graduados con soluciones para niveles 1 a 3.

## Lugar en el camino

Este curso vive en el Semestre 4. Recibe fundamentos de `rust-algorithms`,
`rust-data-structures`, `rust-networking`, `rust-operating-systems`,
`rust-database-internals`, `rust-concurrency` y `rust-distributed-systems`.

Alimenta `rust-software-architecture`, `rust-cloud`, `rust-projects` y los
dominios aplicados como Travel Tech, e-commerce, pasarelas de pago, mensajería,
redes sociales y video.

## Capítulos-proyecto

| # | Proyecto | Módulo sugerido | Estado |
|---|----------|-----------------|--------|
| 01 | TinyURL | `src/tiny_url.rs` | benchmarked |
| 02 | Twitter | `src/twitter.rs` | benchmarked |
| 03 | Uber | `src/uber.rs` | benchmarked |
| 04 | Netflix | `src/netflix.rs` | benchmarked |
| 05 | Dropbox | `src/dropbox.rs` | benchmarked |
| 06 | Google Docs | `src/google_docs.rs` | benchmarked |
| 07 | Redis | `src/redis.rs` | benchmarked |
| 08 | Kafka | `src/kafka.rs` | planned |
| 09 | Booking Engine | `src/booking_engine.rs` | planned |
| 10 | Airbnb | `src/airbnb.rs` | planned |

Estados posibles: `planned`, `draft`, `implemented`, `tested`,
`benchmarked`, `reviewed`, `published`.

## Estructura

```text
AGENTS.md
ROADMAP.md
LICENSE.md
LICENSE-MIT
LICENSE-APACHE
LICENSE-CC-BY-SA-4.0.md
docs/
  SUMMARY.md
src/
  lib.rs
examples/
tests/
benches/
diagrams/
assets/
```

## Cómo usarlo

Ejecutar pruebas:

```bash
cargo test
```

Formatear:

```bash
cargo fmt
```

Verificación completa:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test --doc
```

## Gobernanza

- `AGENTS.md` es la guía de arranque para humanos e IA en este repositorio.
- `course.manifest.json` expone el mapa estructurado del curso para
  `academy-web`.
- `docs/revision-tinyurl.md` concentra el estado de revisión pendiente del
  primer capítulo-proyecto.
- `docs/revision-twitter.md` concentra el estado de revisión pendiente del
  capítulo-proyecto Twitter.
- `docs/revision-uber.md` concentra el estado de revisión pendiente del
  capítulo-proyecto Uber.
- `docs/revision-netflix.md` concentra el estado de revisión pendiente del
  capítulo-proyecto Netflix.
- `docs/revision-dropbox.md` concentra el estado de revisión pendiente del
  capítulo-proyecto Dropbox.
- `docs/revision-google-docs.md` concentra el estado de revisión pendiente del
  capítulo-proyecto Google Docs.
- `docs/revision-redis.md` concentra el estado de revisión pendiente del
  capítulo-proyecto Redis.
- `diagrams/course-map.mmd` muestra prerequisitos, capítulos-proyecto y salidas
  hacia cursos posteriores o dominios aplicados.
- `docs/flujo-autonomo.md` define el modo autónomo con revisión diferida
  permitido por RFC-0001 §20.
- `docs/checklist-revision-capitulo.md` define la revisión humana requerida
  antes de marcar capítulos como `reviewed` o `published`.
- `docs/plantilla-capitulo-proyecto.md` define la anatomía obligatoria de cada
  capítulo-proyecto del curso.
- `ROADMAP.md` registra el avance del curso sin convertirlo en una fecha
  límite.
- Antes de tocar código de curso, el plan completo debe existir como milestones
  e issues de GitHub.
- `LICENSE.md` resume la doble licencia: código bajo `MIT OR Apache-2.0`;
  contenido educativo bajo `CC BY-SA 4.0`.

## Filosofía

Este repositorio debe poder leerse como un libro de ingeniería. La claridad
gana sobre el ingenio, la calidad gana sobre la velocidad, y ningún capítulo se
considera publicable hasta cumplir la anatomía completa de RFC-0001 §14.
