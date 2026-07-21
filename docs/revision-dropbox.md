# Revisión pendiente: Dropbox

Este documento prepara el capítulo Dropbox para revisión humana posterior. No
marca el capítulo como `reviewed` ni como `published`.

## Estado editorial

- **Capítulo:** Dropbox
- **Estado actual:** benchmarked
- **Issue de diseño:** #21
- **Issue de implementación:** #22
- **Issue de pruebas, diagramas y benchmarks:** #23
- **Issue de preparación de revisión:** #24
- **Milestone:** S4 · 05 · Dropbox
- **Revisión humana:** pendiente

## Concepto

Dropbox ya tiene diseño, modelo Rust, pruebas, diagramas y baseline de
benchmark. La revisión humana debe validar si el capítulo enseña bien
sincronización de archivos, chunks, metadatos, revisiones, conflictos y
deduplicación sin fingir un sistema de archivos real ni almacenamiento global.

## Problema

Los sistemas de sincronización se prestan a una simplificación peligrosa: tratar
cada archivo como una escritura aislada. El capítulo debe mostrar que la parte
difícil está en preservar datos cuando los dispositivos no comparten la misma
realidad al mismo tiempo.

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

- [ ] El capítulo explica sincronización antes de hablar de almacenamiento.
- [ ] La diferencia entre archivo visible, chunks, metadatos y revisión queda
      clara.
- [ ] Los requisitos funcionales están separados de los no funcionales.
- [ ] Los supuestos de capacidad son explícitos y pedagógicos.
- [ ] El modelo de datos distingue dispositivo, archivo, chunk, revisión,
      cambio y conflicto.
- [ ] El capítulo no promete representar filesystem real ni operación global.

### Tradeoffs

- [ ] El costo de subir archivos completos queda explicado.
- [ ] El beneficio y costo de chunks fijos queda explicado.
- [ ] La deduplicación se presenta como optimización medible, no como magia.
- [ ] La detección de conflictos por revisión base queda visible.
- [ ] La copia en conflicto se reconoce como decisión conservadora para no
      perder datos.

### Código Rust

- [ ] El módulo `src/dropbox.rs` es legible y pequeño para el objetivo.
- [ ] El código declara invariantes y modos de error relevantes.
- [ ] No usa `unsafe`.
- [ ] No agrega dependencias externas.
- [ ] El ejemplo `examples/dropbox.rs` aporta claridad.
- [ ] `cargo fmt --check` pasa.
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` pasa.

### Pruebas y benchmarks

- [ ] Hay pruebas unitarias para reglas pequeñas.
- [ ] Hay pruebas de integración para sync, actualización, conflicto,
      deduplicación y errores.
- [ ] Hay doctest del API principal.
- [ ] `benches/dropbox_upload_baseline.rs` mide un costo observable.
- [ ] El benchmark no promete rendimiento de producción.
- [ ] La decisión de no agregar Criterion todavía está justificada.

### Diagramas y ejercicios

- [ ] `diagrams/dropbox-flow.mmd` explica subida, chunking, metadatos, sync y
      descarga.
- [ ] `diagrams/dropbox-failures.mmd` explica fallas y conflictos.
- [ ] Los ejercicios progresan de tamaño de chunk a recolección de basura y
      carpetas compartidas.
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
