# Plantilla canónica de capítulo-proyecto

Esta plantilla define la anatomía mínima de cada capítulo-proyecto de
`rust-system-design`. Existe para que el curso avance con consistencia entre
capítulos sin convertir el diseño de sistemas en recetas de entrevista.

## Concepto

Cada capítulo-proyecto enseña a diseñar un sistema completo a partir de
requisitos, restricciones, capacidad, datos, fallas y tradeoffs. El código Rust
sirve como modelo ejecutable de las decisiones, no como sustituto del
razonamiento.

## Problema

Sin una plantilla común, cada capítulo puede explicar cosas distintas con una
profundidad desigual. Eso rompería el doble propósito del repositorio:
material educativo y evidencia profesional del criterio de ingeniería
(RFC-0001 §7).

## Alternativas consideradas

- **Capítulos libres:** dan flexibilidad, pero dificultan comparar decisiones
  entre proyectos.
- **Plantilla de entrevista:** es fácil de seguir, pero empuja a memorizar
  respuestas en vez de diseñar sistemas.
- **Plantilla de capítulo-proyecto:** conserva estructura, deja espacio para el
  criterio y conecta teoría, diseño, código, pruebas y ejercicios.

## Justificación

Se adopta la plantilla de capítulo-proyecto porque respeta el lugar del curso en
el Semestre 4 (RFC-0001 §10), cumple la anatomía de capítulo de RFC-0001 §14,
mantiene la estructura de repositorio de RFC-0001 §15 y permite trabajar con IA
sin perder revisión ni responsabilidad humana (RFC-0001 §20).

## Metadatos del capítulo

Cada capítulo debe iniciar con una ficha breve:

```markdown
# {Nombre del sistema}

- **Curso:** rust-system-design
- **Semestre:** 4
- **Estado:** planned | draft | implemented | tested | benchmarked | reviewed | published
- **Issue:** #{número}
- **Milestone:** {nombre}
- **Módulo Rust:** `src/{modulo}.rs`
- **Ejemplo principal:** `examples/{modulo}.rs`
- **Benchmarks:** aplica | no aplica, con justificación
```

El estado `reviewed` o `published` solo puede marcarse después de revisión
humana explícita.

## Secciones obligatorias

### 1. Contexto y propósito

Explicar qué problema resuelve el sistema, quién lo usa y por qué vale la pena
estudiarlo. Debe quedar claro qué parte del diseño es educativa y qué parte es
una simplificación.

### 2. Requisitos

Separar requisitos funcionales y no funcionales.

La sección queda completa cuando declara:

- Operaciones principales del sistema.
- Usuarios o actores.
- Restricciones de latencia, disponibilidad, durabilidad o consistencia.
- Lo que queda deliberadamente fuera del alcance.

### 3. Estimación de capacidad

Estimar carga, tamaño de datos y límites razonables antes de elegir
arquitectura.

La sección queda completa cuando declara:

- Lecturas y escrituras esperadas.
- Tamaño aproximado de datos.
- Crecimiento esperado.
- Supuestos explícitos y verificables.
- Qué números son pedagógicos y no promesas de producción.

### 4. Modelo de datos

Definir entidades, relaciones, identificadores, índices y cambios de estado.

La sección queda completa cuando incluye:

- Entidades principales.
- Campos mínimos y sus invariantes.
- Relaciones y cardinalidad.
- Decisiones sobre índices o claves.
- Riesgos de consistencia o duplicación.

### 5. APIs y contratos

Describir la frontera pública del sistema antes de implementar detalles
internos.

La sección queda completa cuando incluye:

- Operaciones principales.
- Entradas, salidas y errores.
- Idempotencia cuando aplique.
- Reglas de validación.
- Ejemplos de uso.

### 6. Arquitectura

Presentar los componentes y sus responsabilidades. Debe explicar por qué están
separados y qué alternativa se descartó.

La sección queda completa cuando incluye:

- Diagrama Mermaid de componentes o flujo.
- Responsabilidad de cada componente.
- Dependencias entre componentes.
- Puntos de escalamiento.
- Límites de la simplificación educativa.

### 7. Fallas y recuperación

Diseñar para fallas normales: datos inválidos, duplicados, timeouts,
particiones, saturación, pérdida de eventos o conflictos.

La sección queda completa cuando declara:

- Modos de falla principales.
- Cómo se detectan.
- Qué se recupera automáticamente.
- Qué requiere intervención humana.
- Qué tradeoff se acepta.

### 8. Tradeoffs

Comparar alternativas reales y justificar la elegida.

La sección queda completa cuando incluye al menos:

- Una alternativa más simple.
- Una alternativa más escalable.
- Una alternativa descartada por costo o complejidad.
- La decisión final y su consecuencia.

### 9. Observabilidad

Definir qué señales permiten saber si el sistema funciona o se está degradando.

La sección queda completa cuando incluye:

- Métricas principales.
- Logs o eventos útiles.
- Alertas pedagógicas.
- Preguntas operativas que esas señales responden.

### 10. Modelo Rust

Implementar un modelo pequeño, legible y verificable de las decisiones de
diseño.

La sección queda completa cuando:

- El módulo vive en `src/{modulo}.rs`.
- El ejemplo vive en `examples/{modulo}.rs` cuando aporte claridad.
- El código evita dependencias externas no justificadas.
- No usa `unsafe`.
- Declara invariantes, límites y errores.

### 11. Pruebas

Verificar comportamiento, invariantes y errores.

La sección queda completa cuando incluye:

- Tests unitarios para reglas pequeñas.
- Tests de integración cuando el flujo cruce componentes.
- Doctests si el API público lo amerita.
- Casos de error y límites.

### 12. Benchmarks

Decidir si el capítulo necesita benchmarks y documentar la decisión.

Aplica benchmark cuando una decisión enseñe un costo observable, por ejemplo:
sharding, índices, colas, compactación, hashing, búsqueda, caché, límites de
capacidad o simulación de carga.

No aplica benchmark cuando el capítulo sea puramente estructural o el costo no
aporte aprendizaje. En ese caso, la sección debe decirlo explícitamente.

### 13. Ejercicios

Cada capítulo debe cerrar con ejercicios progresivos.

La sección queda completa cuando incluye:

- Nivel 1: comprensión y modificación pequeña.
- Nivel 2: extensión con una restricción nueva.
- Nivel 3: diseño alternativo con tradeoff explicado.
- Nivel 4: reto abierto sin solución única.

### 14. Checklist de revisión

Cada capítulo debe cerrar con un checklist verificable:

```markdown
## Checklist

- [ ] Requisitos funcionales y no funcionales documentados.
- [ ] Estimación de capacidad con supuestos explícitos.
- [ ] Modelo de datos con invariantes.
- [ ] APIs y contratos documentados.
- [ ] Arquitectura con diagrama Mermaid.
- [ ] Fallas, recuperación y tradeoffs documentados.
- [ ] Observabilidad mínima definida.
- [ ] Modelo Rust implementado sin `unsafe`.
- [ ] Tests unitarios, integración o doctests según aplique.
- [ ] Benchmarks agregados o decisión de no aplicar documentada.
- [ ] Ejercicios en cuatro niveles.
- [ ] `cargo fmt --check` pasa.
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` pasa.
- [ ] `cargo test --all-targets` pasa.
- [ ] `cargo test --doc` pasa.
- [ ] Revisión humana realizada antes de marcar `reviewed` o `published`.
```

## Flujo de trabajo

Cada capítulo se trabaja con el flujo documentado en RFC-0001 §7 y
`AGENTS.md`:

1. Un issue específico.
2. Una rama desde `main`.
3. Un commit principal.
4. Un PR hacia `main`.
5. Trazabilidad de issue, milestone, labels y assignee.
6. Verificaciones completas.
7. Revisión humana o revisión diferida autorizada por RFC-0001 §20.

La revisión diferida permite fusionar trabajo acotado, pero no permite publicar
contenido ni marcarlo como final sin revisión humana.
