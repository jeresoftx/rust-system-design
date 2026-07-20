# Flujo autónomo con revisión diferida

Este flujo aplica cuando Joel autoriza explícitamente a la IA a trabajar con
mayor autonomía en `rust-system-design`, conforme a RFC-0001 §20.

## Concepto

La IA puede preparar, verificar y fusionar PRs propios dentro de un bloque de
trabajo ya planeado. La revisión humana no desaparece: se mueve al cierre del
bloque y conserva autoridad editorial y técnica.

## Problema

El curso necesita avanzar con trazabilidad sin convertir cada paso pequeño en
una pausa operativa. Al mismo tiempo, Jeresoft Academy no debe publicar ni
marcar contenido como final sin revisión humana.

## Alternativas

- **Revisión humana por cada PR:** máxima supervisión, pero frena trabajo
  mecánico y cambios acotados ya descritos en issues.
- **Autoaceptación sin límites:** rápida, pero contradice la regla rectora de
  RFC-0001 §20 y aumenta el riesgo de publicar material no revisado.
- **Revisión diferida con límites:** permite avanzar por issues pequeños,
  mantener historial verificable y reservar la revisión humana para cerrar el
  bloque.

## Justificación

Se adopta revisión diferida con límites porque conserva la regla "la IA acelera,
el criterio humano decide" y reduce fricción en trabajo cotidiano ya aprobado.
Fusionar a `main` bajo este modo no equivale a publicar en el sitio ni a marcar
capítulos como `reviewed` o `published`.

## Condiciones obligatorias

Antes de fusionar un PR propio en modo autónomo, deben cumplirse todas:

- El issue existe, está asignado a `jeresoftx`, tiene milestone y labels.
- El PR resuelve un solo issue.
- El PR tiene un solo commit principal.
- El PR está asignado a `jeresoftx`, ligado al mismo milestone del issue y
  etiquetado.
- Pasan todas las verificaciones aplicables.
- El cambio está dentro del plan aprobado.
- El cambio no modifica currículum, licencias, gobernanza del ecosistema,
  arquitectura general ni decisiones de RFC-0001.
- El cambio no usa `unsafe`.
- El cambio no agrega dependencias externas no triviales.
- El cambio no marca capítulos como `reviewed` ni `published`.
- El resumen del PR declara que fue fusionado en modo de revisión diferida.

## Comandos de verificación

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test --doc
```

## Uso esperado

1. Tomar el siguiente issue planeado.
2. Crear una rama corta desde `main`.
3. Implementar solo el alcance de ese issue.
4. Hacer un commit convencional y descriptivo.
5. Ejecutar las verificaciones.
6. Abrir PR hacia `main` con la plantilla completa.
7. Asignar el PR a `jeresoftx`, milestone y labels correspondientes.
8. Si todo cumple las condiciones, fusionar con revisión diferida.
9. Crear issues nuevos para cualquier hallazgo que quede fuera del alcance.

## Límites

La IA debe pedir revisión antes de fusionar cuando el cambio:

- Cambia el plan del curso.
- Cambia una decisión documentada en RFC-0001.
- Introduce una dependencia externa no trivial.
- Requiere `unsafe`.
- Afecta licencias, gobernanza o publicación.
- Marca contenido como `reviewed` o `published`.
- No tiene pruebas o verificaciones suficientes.
