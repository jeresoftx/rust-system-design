# Revisión pendiente: Uber

Este documento prepara el capítulo Uber para revisión humana posterior. No marca
el capítulo como `reviewed` ni como `published`.

## Estado editorial

- **Capítulo:** Uber
- **Estado actual:** benchmarked
- **Issue de diseño:** #13
- **Issue de implementación:** #14
- **Issue de pruebas, diagramas y benchmarks:** #15
- **Issue de preparación de revisión:** #16
- **Milestone:** S4 · 03 · Uber
- **Revisión humana:** pendiente

## Concepto

Uber ya tiene diseño, modelo Rust, pruebas, diagramas y baseline de benchmark.
La revisión humana debe validar si el capítulo enseña bien matching,
geolocalización pedagógica, asignación exclusiva, eventos y estados del viaje
sin fingir mapas reales ni operación global.

## Problema

Los sistemas de movilidad se prestan a simplificaciones peligrosas. Decir
"elige el driver más cercano" oculta disponibilidad, competencia, ubicación
atrasada, transición de estados, cancelaciones y observabilidad.

## Alternativas consideradas

- **Marcar `reviewed` automáticamente:** rápido, pero contradice RFC-0001 §20.
- **Dejar evidencia solo en PRs:** trazable, pero disperso para revisión
  editorial.
- **Crear hoja de revisión del capítulo:** concentra estado y pendientes sin
  fingir publicación.

## Justificación

Se crea una hoja de revisión porque la IA aceleró el bloque, pero Joel conserva
la decisión final sobre claridad, profundidad, honestidad autoral y publicación.

## Checklist de revisión humana

### Claridad técnica

- [ ] El capítulo explica matching antes de hablar de arquitectura.
- [ ] La diferencia entre búsqueda global, índice por celdas y cola de eventos
      queda clara.
- [ ] Los requisitos funcionales están separados de los no funcionales.
- [ ] Los supuestos de capacidad son explícitos y pedagógicos.
- [ ] El modelo de datos distingue rider, driver, ubicación, viaje y evento.
- [ ] El capítulo no promete representar mapas reales ni operación global.

### Tradeoffs

- [ ] El costo de buscar drivers globalmente queda explicado.
- [ ] El beneficio y costo de celdas vecinas queda explicado.
- [ ] La asignación exclusiva del driver es visible.
- [ ] Las transiciones inválidas tienen consecuencia explícita.
- [ ] La ubicación lógica se reconoce como simplificación educativa.

### Código Rust

- [ ] El módulo `src/uber.rs` es legible y pequeño para el objetivo.
- [ ] El código declara invariantes y modos de error relevantes.
- [ ] No usa `unsafe`.
- [ ] No agrega dependencias externas.
- [ ] El ejemplo `examples/uber.rs` aporta claridad.
- [ ] `cargo fmt --check` pasa.
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` pasa.

### Pruebas y benchmarks

- [ ] Hay pruebas unitarias para reglas pequeñas.
- [ ] Hay pruebas de integración para matching, estados, cancelación y errores.
- [ ] Hay doctest del API principal.
- [ ] `benches/uber_matching_baseline.rs` mide un costo observable.
- [ ] El benchmark no promete rendimiento de producción.
- [ ] La decisión de no agregar Criterion todavía está justificada.

### Diagramas y ejercicios

- [ ] `diagrams/uber-flow.mmd` explica ubicación, matching y asignación.
- [ ] `diagrams/uber-failures.mmd` explica fallas y degradación.
- [ ] Los ejercicios progresan de ajuste local a matching asíncrono.
- [ ] El capítulo no reexplica canónicos de otros cursos.

### Honestidad autoral, licencias y ortografía

- [ ] No se infla experiencia del autor.
- [ ] No se inventan anécdotas, cifras, clientes, incidentes ni autoridad.
- [ ] El contenido educativo respeta `CC BY-SA 4.0`.
- [ ] El código respeta `MIT OR Apache-2.0`.
- [ ] Links y referencias son pertinentes.
- [ ] Español es-MX revisado: acentos, `ñ`, signos de apertura y nombres
      propios.

## Resultado de revisión

- [ ] Aprobado para `reviewed`.
- [ ] Aprobado para `published`.
- [ ] Requiere correcciones.

Notas:

```text
Pendiente de revisión humana por Joel.
```
