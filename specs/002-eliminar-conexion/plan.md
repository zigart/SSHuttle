# Implementation Plan: Eliminar una conexión

**Branch**: `feature/eliminar-conexion` | **Date**: 2026-10-20 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/002-eliminar-conexion/spec.md`

## Summary

El operador elige un servidor de la lista y confirma. La app cierra la sesión o los reintentos de ese servidor, borra la contraseña del llavero y recién entonces quita el perfil. Si el llavero no puede borrar la contraseña, el perfil se queda y se muestra el fallo. Cancelar no llama al núcleo. No se toca la máquina remota ni se pregunta por paquetes, Kubernetes o tmux.

## Technical Context

**Language/Version**: Rust 2021 en el núcleo. TypeScript en la UI.

**Primary Dependencies**: Tauri 2, `rusqlite` para el perfil, `keyring` para borrar la entrada, el supervisor ya existente en `crates/ssh` para cerrar la sesión. No se abre SSH para eliminar.

**Storage**: La fila del perfil sale de SQLite. La contraseña sale del llavero del sistema, servicio `sshuttle` y cuenta `secreto_id`. No se agrega columna.

**Testing**: `cargo test` para borrar un perfil sin tocar los demás, tratar un secreto ausente como ya borrado, y no quitar el perfil si el llavero falla. El recorrido de la lista se valida con [quickstart.md](quickstart.md).

**Target Platform**: El mismo escritorio que la consola con reconexión. Sin instaladores nuevos.

**Project Type**: Aplicación de escritorio. La confirmación vive en la UI. El núcleo recibe un solo comando después de confirmar.

**Performance Goals**: La lista se actualiza en cuanto el borrado termina. No hay espera de red: eliminar no contacta al servidor.

**Constraints**: Confirmación antes de borrar. Si el llavero falla, el perfil permanece. La contraseña no se imprime. Solo el supervisor cierra la sesión. Un servidor sin `secreto_id` se puede borrar igual.

**Scale/Scope**: Un operador borra un servidor por vez. No hay borrado masivo ni deshacer.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principio | Antes de investigar | Después del diseño |
| --- | --- | --- |
| I. Arquitectura vigente | Pasa. Sigue inventario, llavero y supervisor. No cambia la pila. | Pasa. El contrato no abre otro transporte ni otra base. |
| II. Spec antes de código | Pasa. Este paso solo escribe el plan. | Pasa. No hay código de la app. |
| III. Orden de fases | Pasa. Es inventario y llavero de la fase 1, sobre la consola ya integrada en `develop`. | Pasa. No aparecen archivos, actualizaciones, pods ni asistente. |
| IV. Límites de seguridad | Pasa. La contraseña sigue solo en el llavero y borrar pide confirmación. | Pasa. El comando no devuelve la contraseña. Un fallo del llavero no deja un perfil sin secreto. |
| V. Una sola vía de SSH | Pasa. Eliminar no abre sockets. Cierra la sesión que el supervisor ya tiene. | Pasa. `crates/inventory` y `crates/vault` no conectan. |

## Project Structure

### Documentation (this feature)

```text
specs/002-eliminar-conexion/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── ipc.md
└── tasks.md              # lo escribe /speckit-tasks
```

### Source Code (repository root)

```text
src-tauri/src/commands.rs # comando eliminar_servidor
crates/ssh/               # cerrar la sesión y esperar a que el lazo termine
crates/vault/             # borrar la entrada del llavero
crates/inventory/         # borrar la fila del perfil
ui/                       # confirmar o cancelar en la lista
```

**Structure Decision**: No se crean crates. La confirmación queda en `ui/` y no llama al núcleo si el operador cancela. El comando, ya confirmado, para el supervisor, borra el secreto y después la fila.

## Complexity Tracking

No hay desviaciones de la constitución.
