# Revisión pendiente: Twitter

Este documento prepara el capítulo Twitter para revisión humana posterior. No
marca el capítulo como `reviewed` ni como `published`.

## Estado editorial

- **Capítulo:** Twitter
- **Estado actual:** benchmarked
- **Issue de diseño:** #9
- **Issue de implementación:** #10
- **Issue de pruebas, diagramas y benchmarks:** #11
- **Issue de preparación de revisión:** #12
- **Milestone:** S4 · 02 · Twitter
- **Revisión humana:** pendiente

## Concepto

Twitter ya tiene diseño, modelo Rust, pruebas, diagramas y baseline de
benchmark. La revisión humana debe validar si el capítulo enseña bien fan-out,
timeline híbrido, ranking simple, notificaciones y consistencia eventual sin
convertirse en una respuesta de entrevista memorizada.

## Problema

Los sistemas de feeds suelen explicarse con recetas rápidas: "usa cola",
"precalcula timeline", "usa caché". Este capítulo debe mostrar por qué cada
decisión existe, qué costo mueve y qué límite acepta.

## Alternativas consideradas

- **Marcar `reviewed` automáticamente:** rápido, pero contradice RFC-0001 §20.
- **Dejar evidencia solo en PRs:** trazable, pero incómodo para revisión
  editorial.
- **Crear hoja de revisión del capítulo:** concentra estado y pendientes sin
  fingir publicación.

## Justificación

Se crea una hoja de revisión porque la IA aceleró el bloque, pero Joel conserva
la decisión final sobre claridad, profundidad, honestidad autoral y publicación.

## Checklist de revisión humana

### Claridad técnica

- [ ] El capítulo explica fan-out antes de hablar de arquitectura.
- [ ] La diferencia entre fan-out on write, fan-out on read e híbrido queda
      clara.
- [ ] Los requisitos funcionales están separados de los no funcionales.
- [ ] Los supuestos de capacidad son explícitos y pedagógicos.
- [ ] El modelo de datos distingue usuario, follow, tweet, timeline y
      notificación.
- [ ] El capítulo no promete representar la plataforma real.

### Tradeoffs

- [ ] El costo de publicar para autores grandes queda explicado.
- [ ] El costo de leer timelines mezclados queda explicado.
- [ ] La consistencia eventual se presenta como decisión, no como accidente.
- [ ] Las notificaciones no bloqueantes tienen límite y consecuencia.
- [ ] El ranking simple se reconoce como simplificación educativa.

### Código Rust

- [ ] El módulo `src/twitter.rs` es legible y pequeño para el objetivo.
- [ ] El código declara invariantes y modos de error relevantes.
- [ ] No usa `unsafe`.
- [ ] No agrega dependencias externas.
- [ ] El ejemplo `examples/twitter.rs` aporta claridad.
- [ ] `cargo fmt --check` pasa.
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` pasa.

### Pruebas y benchmarks

- [ ] Hay pruebas unitarias para reglas pequeñas.
- [ ] Hay pruebas de integración para timeline, notificaciones y errores.
- [ ] Hay doctest del API principal.
- [ ] `benches/twitter_timeline_baseline.rs` mide un costo observable.
- [ ] El benchmark no promete rendimiento de producción.
- [ ] La decisión de no agregar Criterion todavía está justificada.

### Diagramas y ejercicios

- [ ] `diagrams/twitter-flow.mmd` explica publicación, fan-out y lectura.
- [ ] `diagrams/twitter-failures.mmd` explica fallas y degradación.
- [ ] Los ejercicios progresan de ajuste local a diseño multi-región.
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
