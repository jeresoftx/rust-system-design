# Revisión pendiente: TinyURL

Este documento prepara el capítulo TinyURL para revisión humana posterior. No
marca el capítulo como `reviewed` ni como `published`.

## Estado editorial

- **Capítulo:** TinyURL
- **Estado actual:** benchmarked
- **Issue de diseño:** #5
- **Issue de implementación:** #6
- **Issue de pruebas, diagramas y benchmarks:** #7
- **Issue de preparación de revisión:** #8
- **Milestone:** S4 · 01 · TinyURL
- **Revisión humana:** pendiente

## Concepto

TinyURL ya tiene diseño, modelo Rust, pruebas, diagramas y baseline de
benchmark. El siguiente paso no es publicar: es revisar con criterio humano si
el capítulo enseña bien el problema, respeta su frontera educativa y no promete
más de lo que implementa.

## Problema

El modo autónomo permite avanzar más rápido, pero puede dar falsa sensación de
cierre. Este documento deja explícito qué está listo para revisar y qué no debe
tratarse todavía como contenido final del curso.

## Alternativas consideradas

- **Marcar `reviewed` automáticamente:** rápido, pero contradice RFC-0001 §20.
- **Dejar solo los PRs como evidencia:** trazable, pero disperso para revisión
  editorial.
- **Crear una hoja de revisión del capítulo:** concentra estado, checklist y
  pendientes sin fingir publicación.

## Justificación

Se crea una hoja de revisión porque la IA aceleró el bloque, pero el criterio
humano sigue decidiendo si TinyURL queda aprobado para el curso y para
`academy-web`.

## Checklist de revisión humana

### Claridad técnica

- [ ] El capítulo explica el problema antes de presentar arquitectura.
- [ ] Los requisitos funcionales están separados de los no funcionales.
- [ ] Los supuestos de capacidad son explícitos y no se presentan como hechos
      universales.
- [ ] El modelo de datos declara entidades, identificadores, relaciones e
      invariantes.
- [ ] Las APIs tienen entradas, salidas, errores y reglas de validación.
- [ ] El capítulo diferencia decisiones educativas de decisiones de producción.

### Tradeoffs

- [ ] Hay al menos una alternativa más simple.
- [ ] Hay al menos una alternativa más escalable o robusta.
- [ ] Se explica qué se gana y qué se sacrifica con la decisión elegida.
- [ ] No se presentan tecnologías como respuesta mágica.
- [ ] Las limitaciones quedan documentadas con honestidad.

### Código Rust

- [ ] El módulo Rust es pequeño, legible y coherente con el capítulo.
- [ ] El código declara invariantes, límites y errores relevantes.
- [ ] No usa `unsafe`.
- [ ] No agrega dependencias externas sin justificación escrita.
- [ ] Los ejemplos ejecutables aportan claridad y no son relleno.
- [ ] `cargo fmt --check` pasa.
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` pasa.

### Pruebas y benchmarks

- [ ] Hay pruebas unitarias para reglas pequeñas.
- [ ] Hay pruebas de integración para comportamiento público.
- [ ] Hay doctest del API principal.
- [ ] Se prueban errores, límites y casos incómodos.
- [ ] El benchmark baseline no promete rendimiento de producción.
- [ ] La decisión de no agregar Criterion todavía está justificada.

### Diagramas y ejercicios

- [ ] `diagrams/tinyurl-flow.mmd` explica el flujo principal.
- [ ] `diagrams/tinyurl-failures.mmd` explica fallas y degradación.
- [ ] Los ejercicios progresan de comprensión a diseño abierto.
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
