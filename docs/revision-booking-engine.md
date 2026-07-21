# Revisión pendiente: Booking Engine

Este documento prepara el capítulo Booking Engine para revisión humana posterior.
No marca el capítulo como `reviewed` ni como `published`.

## Estado editorial

- **Capítulo:** Booking Engine
- **Estado actual:** benchmarked
- **Issue de diseño:** #37
- **Issue de implementación:** #38
- **Issue de pruebas, diagramas y benchmarks:** #39
- **Issue de preparación de revisión:** #40
- **Milestone:** S4 · 09 · Booking Engine
- **Revisión humana:** pendiente

## Concepto

Booking Engine ya tiene diseño, modelo Rust, pruebas, diagramas y baseline de
benchmark. La revisión humana debe validar si el capítulo enseña bien
disponibilidad, inventario por noche, holds temporales, expiración, cotización,
confirmación y cancelación sin fingir un motor hotelero real, pagos reales,
channel manager ni integración con OTAs.

## Problema

Los sistemas de reserva suelen contarse como "crear una orden". El capítulo debe
mostrar que la reserva es una promesa temporal sobre inventario limitado: cada
noche debe estar disponible, el hold puede expirar, el precio debe ser trazable y
la confirmación no debe vender más de lo existente.

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

- [ ] El capítulo explica disponibilidad antes de hablar de reservas.
- [ ] La diferencia entre inventario visible, hold, reserva, expiración y
      cancelación queda clara.
- [ ] Los requisitos funcionales están separados de los no funcionales.
- [ ] Los supuestos de capacidad son explícitos y pedagógicos.
- [ ] El modelo de datos distingue calendario, rango de estancia, cotización,
      hold y reserva.
- [ ] El capítulo no promete representar PMS, channel manager, pagos reales ni
      regulación de viajes.

### Tradeoffs

- [ ] El riesgo de reservar directamente queda explicado.
- [ ] El hold temporal se presenta como protección con costo operativo.
- [ ] El inventario por noche se reconoce como más preciso que un contador
      global.
- [ ] La expiración de holds queda ligada a liberación de disponibilidad.
- [ ] La revalidación se explica como defensa educativa contra overselling.

### Código Rust

- [ ] El módulo `src/booking_engine.rs` es legible y pequeño para el objetivo.
- [ ] El código declara invariantes y modos de error relevantes.
- [ ] No usa `unsafe`.
- [ ] No agrega dependencias externas.
- [ ] El ejemplo `examples/booking_engine.rs` aporta claridad.
- [ ] `cargo fmt --check` pasa.
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` pasa.

### Pruebas y benchmarks

- [ ] Hay pruebas unitarias para reglas pequeñas.
- [ ] Hay pruebas de integración para cotización, overselling, hold expirado,
      cancelación de hold, cancelación de reserva e inventario faltante.
- [ ] Hay doctest del API principal.
- [ ] `benches/booking_engine_hold_baseline.rs` mide un costo observable.
- [ ] El benchmark no promete rendimiento de producción.
- [ ] La decisión de no agregar Criterion todavía está justificada.

### Diagramas y ejercicios

- [ ] `diagrams/booking-engine-flow.mmd` explica disponibilidad, calendario,
      precios, hold, TTL, reserva y métricas.
- [ ] `diagrams/booking-engine-failures.mmd` explica rango inválido, inventario
      faltante, overselling, expiración y transiciones inválidas.
- [ ] Los ejercicios progresan hacia pagos como saga, temporadas y canales
      externos sin prometer integración real.
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
