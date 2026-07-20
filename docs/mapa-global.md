# Mapa global del curso

Este mapa muestra cómo `rust-system-design` integra fundamentos previos, recorre
diez capítulos-proyecto y entrega criterio para arquitectura, cloud, proyectos
integradores y dominios aplicados.

```mermaid
flowchart LR
    subgraph prereq["Fundamentos que llegan al curso"]
        algorithms["rust-algorithms"]
        data_structures["rust-data-structures"]
        networking["rust-networking"]
        operating_systems["rust-operating-systems"]
        database_internals["rust-database-internals"]
        concurrency["rust-concurrency"]
        distributed_systems["rust-distributed-systems"]
    end

    subgraph course["rust-system-design · Semestre 4"]
        tinyurl["01 · TinyURL"]
        twitter["02 · Twitter"]
        uber["03 · Uber"]
        netflix["04 · Netflix"]
        dropbox["05 · Dropbox"]
        google_docs["06 · Google Docs"]
        redis["07 · Redis"]
        kafka["08 · Kafka"]
        booking["09 · Booking Engine"]
        airbnb["10 · Airbnb"]
    end

    subgraph outcomes["Cursos y dominios que reciben criterio de diseño"]
        software_architecture["rust-software-architecture"]
        cloud["rust-cloud"]
        projects["rust-projects"]
        travel["rust-travel · Travel Tech"]
        applied_domains["Dominios aplicados"]
    end

    algorithms --> tinyurl
    data_structures --> redis
    networking --> twitter
    operating_systems --> netflix
    database_internals --> dropbox
    concurrency --> google_docs
    distributed_systems --> kafka

    tinyurl --> twitter
    twitter --> uber
    uber --> netflix
    netflix --> dropbox
    dropbox --> google_docs
    google_docs --> redis
    redis --> kafka
    kafka --> booking
    booking --> airbnb

    course --> software_architecture
    course --> cloud
    course --> projects
    booking --> travel
    airbnb --> travel
    travel --> applied_domains
```

## Criterio de lectura

El mapa no reexplica los canónicos de otros cursos. Solo muestra qué
fundamentos alimentan cada tipo de problema:

- `rust-distributed-systems` aporta mecanismos como replicación, consenso,
  particiones y coordinación.
- `rust-system-design` usa esos mecanismos para decidir requisitos, capacidad,
  datos, APIs, fallas, observabilidad y tradeoffs de sistemas completos.
- `Booking Engine` y `Airbnb` son puentes deliberados hacia Travel Tech porque
  conectan inventario, disponibilidad, reservas, pagos, confianza y operación.

El orden de capítulos es una ruta de lectura, no una fecha límite.
