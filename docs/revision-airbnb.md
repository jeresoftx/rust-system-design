# Revisión pendiente: Airbnb

- **Curso:** rust-system-design
- **Semestre:** 4
- **Estado actual:** benchmarked
- **Milestone:** S4 · 10 · Airbnb
- **Issues:** #41, #42, #43, #44
- **Módulo Rust:** `src/airbnb.rs`
- **Ejemplo:** `examples/airbnb.rs`
- **Tests de integración:** `tests/airbnb.rs`
- **Benchmark:** `benches/airbnb_search_booking_baseline.rs`
- **Diagramas:** `diagrams/airbnb-flow.mmd`,
  `diagrams/airbnb-failures.mmd`

## Concepto

Airbnb se usa como capítulo-proyecto para estudiar marketplace de hospedaje:
usuarios con roles, listings, calendario por noche, búsqueda filtrada,
reservas, cancelaciones, suspensiones, reseñas, confianza y métricas.

El objetivo no es clonar Airbnb real ni simular pagos, antifraude, impuestos,
ranking comercial o mensajería. El objetivo educativo es hacer visible cómo
varios subsistemas pequeños se coordinan antes de confirmar una reserva.

## Problema de revisión

El bloque autónomo ya dejó diseño, modelo Rust, ejemplo, pruebas, diagramas y
benchmark. Eso demuestra avance técnico, pero no equivale a revisión editorial
ni autorización de publicación.

Antes de pasar a `reviewed` o `published`, una persona debe validar que el
capítulo explica con claridad sus límites: resultados de búsqueda obsoletos,
revalidación de disponibilidad, suspensiones, reseñas duplicadas, capacidad,
calendario y señales de confianza.

## Alternativas consideradas

- Marcar el capítulo como `reviewed` automáticamente después de pruebas verdes.
- Dejar la evidencia repartida entre issues y PRs.
- Crear esta hoja de revisión humana antes de cualquier publicación.

## Justificación

La tercera alternativa mantiene RFC-0001 §20: la IA acelera, pero el criterio
humano decide. El capítulo queda listo para lectura crítica sin fingir que ya
fue aprobado editorialmente.

## Checklist de revisión humana

- [ ] El capítulo explica el problema antes de presentar el modelo.
- [ ] Los tradeoffs entre búsqueda, calendario, reserva y confianza son claros.
- [ ] El código Rust es legible para estudiantes del semestre 4.
- [ ] Las pruebas cubren casos felices, fallas y límites relevantes.
- [ ] El benchmark se entiende como baseline educativo, no como promesa de
      rendimiento productivo.
- [ ] Los diagramas ayudan a razonar el sistema sin decorar de más.
- [ ] El texto evita inflar experiencia o afirmar operación real de Airbnb.
- [ ] Ortografía, acentos y nombres propios están revisados.
- [ ] El capítulo no se marca como `reviewed` ni `published` hasta que Joel lo
      apruebe.
