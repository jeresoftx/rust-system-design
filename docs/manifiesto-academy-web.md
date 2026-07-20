# Manifiesto para academy-web

`course.manifest.json` es el contrato mínimo para que `academy-web` pueda leer
el curso sin duplicar información editorial.

## Concepto

El manifiesto separa datos estructurados de prosa larga. El sitio puede usarlo
para listar capítulos, estados, rutas, relaciones y assets; los capítulos siguen
viviendo como contenido educativo en Markdown y Rust.

## Problema

Si el sitio copia títulos, estados o rutas de capítulos manualmente, el
ecosistema termina con dos fuentes de verdad. El manifiesto reduce esa
duplicación y deja que cada repositorio de curso declare su propio mapa.

## Alternativas consideradas

- **Leer solo README:** fácil para humanos, frágil para el sitio.
- **Mantener datos duplicados en academy-web:** rápido al inicio, costoso cuando
  cambien capítulos o estados.
- **Manifiesto JSON en el repo del curso:** simple de consumir, versionado con
  el curso y suficiente para una primera integración.

## Justificación

Se adopta un manifiesto JSON porque `academy-web` puede consumirlo sin depender
de una base de datos ni de un parser de Markdown. El archivo vive junto al curso,
así que cada cambio queda trazado por issue, commit y PR conforme a RFC-0001
§7, §15 y §20.

## Estructura

- `schemaVersion`: versión del contrato del manifiesto.
- `course`: metadatos estables del curso.
- `assets`: ruta esperada para assets visuales del sitio.
- `chapters`: lista ordenada de capítulos-proyecto.

Cada capítulo declara:

- `number`: posición dentro del curso.
- `id`: identificador estable para integraciones.
- `title`: nombre visible.
- `slug`: ruta sugerida para el sitio.
- `status`: estado editorial inicial.
- `milestone`: milestone de GitHub que gobierna el capítulo.
- `module`: módulo Rust esperado.
- `example`: ejemplo ejecutable esperado.
- `themes`: temas que ayudan al sitio a mostrar badges o filtros.
- `bridges`: cursos o dominios relacionados.

## Consumo esperado en academy-web

`academy-web` debe tratar este manifiesto como entrada de catálogo, no como
contenido publicable por sí mismo.

Uso recomendado:

1. Leer `course.manifest.json` desde el repositorio de curso.
2. Validar `schemaVersion`.
3. Mostrar solo capítulos con estados permitidos por el sitio.
4. Usar `assets.basePath` como ruta base de imágenes cuando existan.
5. Mantener el estado `published` bajo revisión humana explícita.

## Estado inicial

Todos los capítulos quedan en `planned`. Ese estado significa que el curso tiene
intención y estructura, pero todavía no hay contenido listo para publicación.
