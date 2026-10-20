# Implementation Plan: Barra de fichas

**Branch**: `feature/barra-fichas` | **Date**: 2026-10-20 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/003-barra-fichas/spec.md`

## Summary

La lista de conexiones deja de ser una columna de nombres y pasa a una barra vertical de fichas: iniciales del nombre visible, punto de estado (`activa` / `inactiva` / `reintentando`) y la elegida destacada. Un casillero vacío al final abre el alta (host, puerto, usuario, contraseña). Eliminar sigue pidiendo confirmación. No hay comandos IPC nuevos ni cambios de inventario o llavero.

## Technical Context

**Language/Version**: TypeScript en la UI. El núcleo Rust no cambia en esta fase.

**Primary Dependencies**: React en `ui/`, eventos `indicador` y comandos ya existentes (`listar_servidores`, `guardar_servidor`, `conectar`, `eliminar_servidor`). Sin crates nuevos.

**Storage**: N/A. Las fichas leen el perfil y el estado en memoria que ya entrega la consola.

**Testing**: Función pura de iniciales con pruebas en la UI o módulo hermano sin `#[test]` en Rust (no hay código de producto Rust nuevo). El recorrido visual se valida con [quickstart.md](quickstart.md).

**Target Platform**: El mismo escritorio Tauri 2 que la consola.

**Project Type**: Aplicación de escritorio. Solo cambia presentación y el punto de entrada del alta.

**Performance Goals**: Hasta ~diez fichas en la barra; la elegida se reconoce de un vistazo. El punto de estado refleja el evento `indicador` sin recargar la app.

**Constraints**: Un solo casillero de alta. Sin íconos de inicio, ajustes ni listado aparte. Sin preguntar paquetes, Kubernetes ni tmux. Eliminar no pierde la confirmación.

**Scale/Scope**: Reemplazo del panel de lista en `ui/src/App.tsx` (y estilos). Sin tocar SSH, vault ni SQLite.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principio | Antes de investigar | Después del diseño |
| --- | --- | --- |
| I. Arquitectura vigente | Pasa. No contradice inventario ni alta mínima. El plan anota una línea en `docs/arquitectura.md` sobre la barra de fichas. | Pasa. El contrato UI no agrega capacidades ni pantallas de kube/gestor/tmux. |
| II. Spec antes de código | Pasa. Este paso solo escribe el plan. | Pasa. No hay código de la app. |
| III. Orden de fases | Pasa. Presentación de la lista sobre inventario y consola ya en `develop`. No salta a archivos, pods ni asistente. | Pasa. Fuera de alcance explícito en la spec. |
| IV. Límites de seguridad | Pasa. El alta sigue pidiendo solo host, puerto, usuario y contraseña. Eliminar sigue con confirmación. | Pasa. Sin comandos nuevos que expongan secretos. |
| V. Una sola vía de SSH | Pasa. La UI no abre sockets. | Pasa. Sin cambios en el supervisor. |

## Project Structure

### Documentation (this feature)

```text
specs/003-barra-fichas/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── ui.md
└── tasks.md              # lo escribe /speckit-tasks
```

### Source Code (repository root)

```text
ui/src/App.tsx                 # barra en lugar de la lista de nombres
ui/src/App.css                 # rail, ficha, casillero, destacada
ui/src/servers/                # iniciales (función pura) y, si hace falta, componente de ficha
ui/src/console/Indicator.tsx   # reutilizar el valor del punto; no inventar un cuarto estado
docs/arquitectura.md           # una línea: la lista se muestra como barra de fichas
```

**Structure Decision**: Solo UI y un ajuste mínimo de arquitectura documentada. Sin crates, sin IPC nuevo. Las iniciales viven en un módulo de `ui/` testeable. El casillero vacío reutiliza `ServerForm` con `seleccion === null`.

## Complexity Tracking

No hay desviaciones de la constitución.
