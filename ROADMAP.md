# ROADMAP

Estado de avance de `rust-system-design`, repositorio del camino troncal de
Jeresoft Academy para diseño de sistemas en Rust.

No hay fechas límite: este es un proyecto de legado (RFC-0001 §1). Este archivo
orienta el avance, pero no convierte el curso en una carrera por terminar.

## Estado actual

El repositorio ya tiene estructura inicial, licencias, README, AGENTS, crate
Rust mínimo, tabla de capítulos-proyecto planeados, milestones, issues, labels
y asignaciones en GitHub.

Todavía no hay capítulos implementados ni publicados. El siguiente paso natural
es empezar por el issue de gobernanza que define la plantilla canónica de
capítulo-proyecto.

## Capítulos-proyecto planeados

| # | Proyecto | Estado |
|---|----------|--------|
| 01 | TinyURL | benchmarked |
| 02 | Twitter | planned |
| 03 | Uber | planned |
| 04 | Netflix | planned |
| 05 | Dropbox | planned |
| 06 | Google Docs | planned |
| 07 | Redis | planned |
| 08 | Kafka | planned |
| 09 | Booking Engine | planned |
| 10 | Airbnb | planned |

## Alineación RFC-0001

- Este repositorio sigue la plantilla de repositorio de RFC-0001 §15.
- Cada capítulo debe cumplir la anatomía de RFC-0001 §14.
- Cada ejercicio debe seguir los niveles de RFC-0001 §17.
- El mapa global del curso vive en `diagrams/course-map.mmd` y se explica en
  `docs/mapa-global.md`.
- La checklist de revisión humana vive en
  `docs/checklist-revision-capitulo.md`.
- El uso de IA se rige por RFC-0001 §20: la IA acelera, el criterio humano
  decide.
- El modo autónomo con revisión diferida vive en `docs/flujo-autonomo.md` y no
  equivale a publicar contenido.

## Fuera de alcance por ahora

- Preparar respuestas memorizadas para entrevistas sin explicar tradeoffs.
- Construir clones de producción de los sistemas citados.
- Reexplicar desde cero algoritmos, estructuras, redes, concurrencia o sistemas
  distribuidos; esos fundamentos viven en sus cursos canónicos.
- Agregar dependencias externas antes de justificar su valor educativo.
- Publicar capítulos parciales como si estuvieran completos.

## Siguiente paso natural

Revisión humana de TinyURL antes de marcar el capítulo como `reviewed` o
`published`. Después de esa revisión, continuar con el bloque del capítulo
Twitter.
