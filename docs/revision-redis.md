# Revisión pendiente: Redis

Este documento prepara el capítulo Redis para revisión humana posterior. No
marca el capítulo como `reviewed` ni como `published`.

## Estado editorial

- **Capítulo:** Redis
- **Estado actual:** benchmarked
- **Issue de diseño:** #29
- **Issue de implementación:** #30
- **Issue de pruebas, diagramas y benchmarks:** #31
- **Issue de preparación de revisión:** #32
- **Milestone:** S4 · 07 · Redis
- **Revisión humana:** pendiente

## Concepto

Redis ya tiene diseño, modelo Rust, pruebas, diagramas y baseline de benchmark.
La revisión humana debe validar si el capítulo enseña bien un almacén en
memoria con expiración, límites de capacidad, persistencia append-only,
snapshot, replay y replicación por offsets sin fingir rendimiento real de Redis
ni red distribuida.

## Problema

Redis suele explicarse solo como "caché rápida". El capítulo debe mostrar que
esa velocidad nace de decisiones explícitas: memoria limitada, expiración
oportunista, errores por tipo incorrecto, escritura durable de comandos y
recuperación reproducible.

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

- [ ] El capítulo explica memoria antes de hablar de rendimiento.
- [ ] La diferencia entre valor visible, comando mutable, AOF, snapshot y
      réplica queda clara.
- [ ] Los requisitos funcionales están separados de los no funcionales.
- [ ] Los supuestos de capacidad son explícitos y pedagógicos.
- [ ] El modelo de datos distingue entrada, valor, TTL, offset y métricas.
- [ ] El capítulo no promete representar Redis real ni replicación distribuida.

### Tradeoffs

- [ ] El costo de mantener todo en memoria queda explicado.
- [ ] La expiración oportunista se presenta como simplificación consciente.
- [ ] El límite de memoria se enseña como presión operativa, no como detalle
      accidental.
- [ ] AOF se reconoce como auditable pero costoso si crece sin compactación.
- [ ] Snapshot y replay se explican como caminos complementarios de
      recuperación.

### Código Rust

- [ ] El módulo `src/redis.rs` es legible y pequeño para el objetivo.
- [ ] El código declara invariantes y modos de error relevantes.
- [ ] No usa `unsafe`.
- [ ] No agrega dependencias externas.
- [ ] El ejemplo `examples/redis.rs` aporta claridad.
- [ ] `cargo fmt --check` pasa.
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` pasa.

### Pruebas y benchmarks

- [ ] Hay pruebas unitarias para reglas pequeñas.
- [ ] Hay pruebas de integración para TTL, memoria, snapshot, replay,
      replicación y AOF.
- [ ] Hay doctest del API principal.
- [ ] `benches/redis_hot_path_baseline.rs` mide un costo observable.
- [ ] El benchmark no promete rendimiento de producción.
- [ ] La decisión de no agregar Criterion todavía está justificada.

### Diagramas y ejercicios

- [ ] `diagrams/redis-flow.mmd` explica cliente, memoria, TTL, AOF, snapshot y
      réplica.
- [ ] `diagrams/redis-failures.mmd` explica expiración, memoria agotada, tipo
      incorrecto y replay.
- [ ] Los ejercicios progresan de TTL a eviction y replicación más explícita.
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
