# Revisión pendiente: Google Docs

Este documento prepara el capítulo Google Docs para revisión humana posterior.
No marca el capítulo como `reviewed` ni como `published`.

## Estado editorial

- **Capítulo:** Google Docs
- **Estado actual:** benchmarked
- **Issue de diseño:** #25
- **Issue de implementación:** #26
- **Issue de pruebas, diagramas y benchmarks:** #27
- **Issue de preparación de revisión:** #28
- **Milestone:** S4 · 06 · Google Docs
- **Revisión humana:** pendiente

## Concepto

Google Docs ya tiene diseño, modelo Rust, pruebas, diagramas y baseline de
benchmark. La revisión humana debe validar si el capítulo enseña bien edición
colaborativa, operaciones concurrentes, transformación de posiciones, presencia,
sync incremental y persistencia de operaciones sin fingir un editor real ni un
CRDT completo.

## Problema

Los sistemas colaborativos suelen contarse como "varias personas escribiendo al
mismo tiempo". El capítulo debe hacer visible lo que sostiene esa experiencia:
orden canónico, versiones, transformación de operaciones, presencia efímera y
degradación cuando un cliente llega atrasado o inválido.

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

- [ ] El capítulo explica colaboración antes de hablar de algoritmos.
- [ ] La diferencia entre texto visible, operación, versión y presencia queda
      clara.
- [ ] Los requisitos funcionales están separados de los no funcionales.
- [ ] Los supuestos de capacidad son explícitos y pedagógicos.
- [ ] El modelo de datos distingue documento, colaborador, operación aceptada y
      presencia.
- [ ] El capítulo no promete representar un editor real ni un CRDT completo.

### Tradeoffs

- [ ] El costo de bloquear documentos queda explicado.
- [ ] El riesgo de última escritura gana queda explicado.
- [ ] OT educativo se presenta como simplificación, no como solución completa.
- [ ] La presencia efímera queda separada del contenido persistente.
- [ ] El log de operaciones se reconoce como auditable pero costoso de
      reproducir si crece sin snapshots.

### Código Rust

- [ ] El módulo `src/google_docs.rs` es legible y pequeño para el objetivo.
- [ ] El código declara invariantes y modos de error relevantes.
- [ ] No usa `unsafe`.
- [ ] No agrega dependencias externas.
- [ ] El ejemplo `examples/google_docs.rs` aporta claridad.
- [ ] `cargo fmt --check` pasa.
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` pasa.

### Pruebas y benchmarks

- [ ] Hay pruebas unitarias para reglas pequeñas.
- [ ] Hay pruebas de integración para sync, transformación, presencia,
      rechazos y versiones futuras.
- [ ] Hay doctest del API principal.
- [ ] `benches/google_docs_operations_baseline.rs` mide un costo observable.
- [ ] El benchmark no promete rendimiento de producción.
- [ ] La decisión de no agregar Criterion todavía está justificada.

### Diagramas y ejercicios

- [ ] `diagrams/google-docs-flow.mmd` explica API, transformación, estado, log,
      presencia y sync.
- [ ] `diagrams/google-docs-failures.mmd` explica rechazos, base atrasada,
      presencia caducada y sync.
- [ ] Los ejercicios progresan de snapshots a CRDT completo.
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
