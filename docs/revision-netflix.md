# Revisión pendiente: Netflix

Este documento prepara el capítulo Netflix para revisión humana posterior. No
marca el capítulo como `reviewed` ni como `published`.

## Estado editorial

- **Capítulo:** Netflix
- **Estado actual:** benchmarked
- **Issue de diseño:** #17
- **Issue de implementación:** #18
- **Issue de pruebas, diagramas y benchmarks:** #19
- **Issue de preparación de revisión:** #20
- **Milestone:** S4 · 04 · Netflix
- **Revisión humana:** pendiente

## Concepto

Netflix ya tiene diseño, modelo Rust, pruebas, diagramas y baseline de
benchmark. La revisión humana debe validar si el capítulo enseña bien catálogo
regional, recomendaciones explicables, variantes de video, selección de CDN,
capacidad, sesiones y degradación sin fingir streaming real ni operación
global.

## Problema

Los sistemas de video bajo demanda parecen una lectura directa de contenido,
pero esconden decisiones de disponibilidad regional, ranking, ancho de banda,
capacidad de CDN, salud de nodos y estado de sesión. El capítulo debe hacer
visibles esas decisiones sin convertirse en un curso de multimedia.

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

- [ ] El capítulo explica catálogo, reproducción y CDN antes de hablar de
      arquitectura.
- [ ] La diferencia entre catálogo visible, recomendación y sesión queda clara.
- [ ] Los requisitos funcionales están separados de los no funcionales.
- [ ] Los supuestos de capacidad son explícitos y pedagógicos.
- [ ] El modelo de datos distingue perfil, título, variante, CDN, sesión y
      evento.
- [ ] El capítulo no promete representar streaming real ni operación global.

### Tradeoffs

- [ ] La recomendación por popularidad regional se explica como fallback.
- [ ] La afinidad por género se reconoce como simplificación educativa.
- [ ] La selección de variante por ancho de banda queda visible.
- [ ] La capacidad de CDN se explica como recurso finito.
- [ ] La región lógica se reconoce como simplificación de latencia y contratos.

### Código Rust

- [ ] El módulo `src/netflix.rs` es legible y pequeño para el objetivo.
- [ ] El código declara invariantes y modos de error relevantes.
- [ ] No usa `unsafe`.
- [ ] No agrega dependencias externas.
- [ ] El ejemplo `examples/netflix.rs` aporta claridad.
- [ ] `cargo fmt --check` pasa.
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` pasa.

### Pruebas y benchmarks

- [ ] Hay pruebas unitarias para reglas pequeñas.
- [ ] Hay pruebas de integración para catálogo, recomendación, CDN, ancho de
      banda, eventos y errores.
- [ ] Hay doctest del API principal.
- [ ] `benches/netflix_playback_baseline.rs` mide un costo observable.
- [ ] El benchmark no promete rendimiento de producción.
- [ ] La decisión de no agregar Criterion todavía está justificada.

### Diagramas y ejercicios

- [ ] `diagrams/netflix-flow.mmd` explica catálogo, recomendación, variante,
      CDN y sesión.
- [ ] `diagrams/netflix-failures.mmd` explica fallas y degradación.
- [ ] Los ejercicios progresan de reglas locales a decisiones de caché y
      experiencia.
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
