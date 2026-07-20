# Checklist de revisión humana por capítulo-proyecto

Este checklist se usa antes de marcar un capítulo como `reviewed` o
`published`. La IA puede acelerar borradores, diagramas, pruebas y andamiaje,
pero el criterio humano decide qué queda aprobado (RFC-0001 §20).

## Concepto

La revisión humana es una compuerta editorial y técnica. No existe para frenar
el trabajo, sino para asegurar que cada capítulo represente con honestidad el
estándar de Jeresoft Academy.

## Problema

Un capítulo de diseño de sistemas puede verse completo aunque tenga huecos
graves: tradeoffs débiles, código no verificable, benchmarks decorativos,
diagramas confusos, links rotos, ortografía descuidada o experiencia del autor
inflada. La checklist fuerza una revisión explícita antes de publicar.

## Alternativas consideradas

- **Confiar solo en CI:** valida código, pero no criterio editorial.
- **Revisión libre:** permite juicio humano, pero olvida detalles repetibles.
- **Checklist humana:** conserva criterio, reduce omisiones y deja evidencia de
  qué se revisó.

## Justificación

Se adopta una checklist humana porque el curso es material educativo y evidencia
profesional. La calidad técnica, la claridad, la honestidad autoral y la
trazabilidad pesan tanto como que `cargo test` pase.

## Identificación

- Capítulo:
- Issue:
- PR:
- Milestone:
- Revisor humano:
- Fecha de revisión:

## Claridad técnica

- [ ] El capítulo explica el problema antes de presentar arquitectura.
- [ ] Los requisitos funcionales están separados de los no funcionales.
- [ ] Los supuestos de capacidad son explícitos y no se presentan como hechos
      universales.
- [ ] El modelo de datos declara entidades, identificadores, relaciones e
      invariantes.
- [ ] Las APIs tienen entradas, salidas, errores y reglas de validación.
- [ ] El capítulo diferencia decisiones educativas de decisiones de producción.

## Tradeoffs

- [ ] Hay al menos una alternativa más simple.
- [ ] Hay al menos una alternativa más escalable o robusta.
- [ ] Se explica qué se gana y qué se sacrifica con la decisión elegida.
- [ ] No se presentan tecnologías como respuesta mágica.
- [ ] Las limitaciones quedan documentadas con honestidad.

## Código Rust

- [ ] El módulo Rust es pequeño, legible y coherente con el capítulo.
- [ ] El código declara invariantes, límites y errores relevantes.
- [ ] No usa `unsafe`.
- [ ] No agrega dependencias externas sin justificación escrita.
- [ ] Los ejemplos ejecutables aportan claridad y no son relleno.
- [ ] `cargo fmt --check` pasa.
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` pasa.

## Pruebas

- [ ] Hay pruebas unitarias para reglas pequeñas.
- [ ] Hay pruebas de integración cuando el flujo cruza componentes.
- [ ] Hay doctests si el API público lo amerita.
- [ ] Se prueban errores, límites y casos incómodos.
- [ ] `cargo test --all-targets` pasa.
- [ ] `cargo test --doc` pasa.

## Benchmarks

- [ ] El capítulo declara si los benchmarks aplican.
- [ ] Si aplican, miden una decisión con costo observable.
- [ ] Si no aplican, la decisión está justificada por escrito.
- [ ] Los resultados no prometen rendimiento de producción.

## Diagramas

- [ ] Los diagramas Mermaid explican arquitectura, flujo o datos.
- [ ] El diagrama no duplica texto sin aportar claridad.
- [ ] Los nombres son consistentes con el capítulo y el código.
- [ ] El diagrama evita reexplicar canónicos que viven en otros cursos.

## Ejercicios

- [ ] Hay ejercicios de nivel 1, 2, 3 y 4.
- [ ] Los ejercicios progresan de comprensión a diseño abierto.
- [ ] Las soluciones de niveles 1 a 3, cuando existan, no eliminan el
      aprendizaje.
- [ ] El nivel 4 deja espacio para criterio y discusión.

## Honestidad autoral

- [ ] El capítulo no infla experiencia del autor.
- [ ] Los dominios aplicados se presentan solo donde hay experiencia real o se
      declaran como modelos educativos.
- [ ] No se inventan anécdotas, cifras, clientes, incidentes ni autoridad.
- [ ] Las referencias externas se distinguen de criterio propio.

## Licencias, links y referencias

- [ ] El contenido educativo respeta `CC BY-SA 4.0`.
- [ ] El código respeta `MIT OR Apache-2.0`.
- [ ] Los links funcionan.
- [ ] Las referencias son pertinentes y de calidad.
- [ ] No hay material copiado sin atribución o licencia compatible.

## Ortografía y español es-MX

- [ ] Acentos correctos.
- [ ] Uso correcto de `ñ`.
- [ ] Signos de apertura cuando aplican.
- [ ] Nombres propios bien escritos.
- [ ] Redacción clara, sin anglicismos innecesarios.

## Publicación

- [ ] El capítulo no está marcado como `reviewed` sin revisión humana.
- [ ] El capítulo no está marcado como `published` sin aprobación explícita.
- [ ] El manifiesto del curso refleja el estado correcto.
- [ ] `academy-web` solo consume el capítulo si su estado editorial lo permite.

## Resultado

- [ ] Aprobado para `reviewed`.
- [ ] Aprobado para `published`.
- [ ] Requiere correcciones.

Notas:

```text
{observaciones de revisión}
```
