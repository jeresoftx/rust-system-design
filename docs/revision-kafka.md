# Revisión pendiente: Kafka

Este documento prepara el capítulo Kafka para revisión humana posterior. No
marca el capítulo como `reviewed` ni como `published`.

## Estado editorial

- **Capítulo:** Kafka
- **Estado actual:** benchmarked
- **Issue de diseño:** #33
- **Issue de implementación:** #34
- **Issue de pruebas, diagramas y benchmarks:** #35
- **Issue de preparación de revisión:** #36
- **Milestone:** S4 · 08 · Kafka
- **Revisión humana:** pendiente

## Concepto

Kafka ya tiene diseño, modelo Rust, pruebas, diagramas y baseline de benchmark.
La revisión humana debe validar si el capítulo enseña bien logs append-only,
topics, particiones, offsets, consumer groups, commits manuales, lag y retención
sin fingir brokers múltiples, protocolo Kafka real ni exactly-once real.

## Problema

Kafka suele explicarse como "mensajería". El capítulo debe mostrar que su valor
educativo está en conservar historia ordenada por partición: productores que
agregan eventos, consumidores que avanzan a su ritmo y una política explícita
que decide cuándo olvidar.

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

- [ ] El capítulo explica log append-only antes de hablar de mensajería.
- [ ] La diferencia entre topic, partición, evento, offset, commit y lag queda
      clara.
- [ ] Los requisitos funcionales están separados de los no funcionales.
- [ ] Los supuestos de capacidad son explícitos y pedagógicos.
- [ ] El modelo de datos distingue evento publicado, lote leído y offset
      confirmado.
- [ ] El capítulo no promete representar Kafka real, brokers múltiples ni
      exactly-once real.

### Tradeoffs

- [ ] El costo de conservar historia queda explicado.
- [ ] La partición por clave se presenta como orden por entidad, no como orden
      global.
- [ ] El round-robin se reconoce como distribución simple con pérdida de orden
      por entidad.
- [ ] El commit manual se explica como control con riesgo de duplicados.
- [ ] La retención se presenta como decisión operativa que limita el replay.

### Código Rust

- [ ] El módulo `src/kafka.rs` es legible y pequeño para el objetivo.
- [ ] El código declara invariantes y modos de error relevantes.
- [ ] No usa `unsafe`.
- [ ] No agrega dependencias externas.
- [ ] El ejemplo `examples/kafka.rs` aporta claridad.
- [ ] `cargo fmt --check` pasa.
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` pasa.

### Pruebas y benchmarks

- [ ] Hay pruebas unitarias para reglas pequeñas.
- [ ] Hay pruebas de integración para round-robin, claves, fetch, commits,
      lag, retención y commits futuros.
- [ ] Hay doctest del API principal.
- [ ] `benches/kafka_publish_fetch_baseline.rs` mide un costo observable.
- [ ] El benchmark no promete rendimiento de producción.
- [ ] La decisión de no agregar Criterion todavía está justificada.

### Diagramas y ejercicios

- [ ] `diagrams/kafka-flow.mmd` explica productores, particiones, retención,
      fetch, offsets, lag y métricas.
- [ ] `diagrams/kafka-failures.mmd` explica topic inexistente, payload inválido,
      offsets retenidos y commits futuros.
- [ ] Los ejercicios progresan de compaction a rebalances y retención por
      tiempo lógico.
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
