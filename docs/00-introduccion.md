# Introducción

`rust-system-design` es el curso de Jeresoft Academy dedicado a diseñar sistemas
completos: productos con requisitos, restricciones, datos, APIs, capacidad,
fallas, observabilidad y tradeoffs explícitos.

Este repositorio pertenece al Semestre 4 del camino troncal (RFC-0001 §10) y se
estudia junto con `rust-distributed-systems`. La diferencia es deliberada:

- `rust-distributed-systems` estudia mecanismos como consenso, clocks, gossip y
  transacciones distribuidas.
- `rust-system-design` integra esos mecanismos para diseñar sistemas completos
  como TinyURL, Twitter, Redis, Kafka, Booking Engine y Airbnb.

## Estado

Este repositorio está en fase inicial. Los capítulos todavía no están
implementados ni publicados. Antes de tocar código de curso, el plan completo
debe convertirse en milestones e issues de GitHub, siguiendo RFC-0001 §7 y el
flujo descrito en `AGENTS.md`.

## Criterio rector

Un buen diseño de sistema no empieza por tecnologías. Empieza por entender:

1. Qué problema resuelve el sistema.
2. Quién lo usa y qué espera.
3. Cuáles son los requisitos funcionales.
4. Cuáles son los requisitos no funcionales.
5. Qué datos existen y cómo cambian.
6. Qué puede fallar.
7. Qué alternativas se consideraron.
8. Qué tradeoffs se aceptan conscientemente.

El código Rust acompaña el razonamiento. No lo reemplaza.
