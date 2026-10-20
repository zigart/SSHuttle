# Implementation Plan: Consola con reconexión

**Branch**: `feature/consola-reconexion` | **Date**: 2026-10-20 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/001-consola-reconexion/spec.md`

## Summary

El operador da de alta un servidor con host, puerto, usuario y contraseña, y obtiene una consola en cuanto el servidor lo acepta. La contraseña queda en el llavero del sistema. Si la red se corta, el indicador pasa a `reintentando` y la sesión vuelve sola, con espera creciente hasta 30 segundos, sin pedir credenciales. Cada servidor muestra solo `activa`, `inactiva` o `reintentando`. Si hay tmux, se crea o se retoma la sesión `sshuttle`. Si la huella del host cambia, el indicador queda `inactiva` hasta una confirmación.

## Technical Context

**Language/Version**: Rust 2021 en el núcleo. TypeScript en la UI.

**Primary Dependencies**: Tauri 2, `russh` para SSH y PTY, `keyring` para el llavero, `rusqlite` para los perfiles, React y xterm.js para la consola.

**Storage**: SQLite para perfiles y huella. Llavero del sistema para la contraseña. El perfil no guarda la contraseña.

**Testing**: `cargo test` para la espera entre reintentos, el mapeo del indicador, la comparación de huellas y la ausencia de contraseña en el perfil. La consola de punta a punta se valida con [quickstart.md](quickstart.md).

**Target Platform**: Escritorio Windows, Linux y macOS en este hito. El código no crea un proceso en segundo plano, para no contradecir el comportamiento en el teléfono. Los instaladores quedan fuera.

**Project Type**: Aplicación de escritorio. La UI habla con el núcleo por comandos y eventos de Tauri.

**Performance Goals**: La consola acepta escritura en cuanto la autenticación termina. El indicador cambia en el mismo momento en que cambia la conexión. El keepalive cada 15 segundos evita un corte por inactividad breve.

**Constraints**: Una conexión SSH por servidor, abierta solo por `crates/ssh`. Espera entre reintentos de 1, 2, 4, 8, 16 y luego 30 segundos. Sin contraseña en SQLite ni en logs. Sin archivos, pods, actualizaciones ni asistente.

**Scale/Scope**: Un operador y los servidores que guarde en su máquina. Seis historias de GitHub, de la [#1](https://github.com/zigart/SSHuttle/issues/1) a la [#6](https://github.com/zigart/SSHuttle/issues/6).

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principio | Antes de investigar | Después del diseño |
| --- | --- | --- |
| I. Arquitectura vigente | Pasa. Pila Tauri, Rust y React. SSH solo con `russh`. | Pasa. Los contratos no abren otro transporte. |
| II. Spec antes de código | Pasa. Este paso solo escribe el plan. | Pasa. No hay código de la app. |
| III. Orden de fases | Pasa con la excepción de abajo. Este hito une la conexión y la consola, y aplaza el sondeo de paquetes, archivos y pods. | Pasa. `crates/discover`, `files`, `updates`, `kube` y `assistant` no entran. |
| IV. Límites de seguridad | Pasa. Alta mínima, secreto en el llavero, huella con confirmación. | Pasa. El contrato no devuelve la contraseña. |
| V. Una sola vía de SSH | Pasa. La UI y la consola no abren sockets. | Pasa. `crates/terminal` recibe la sesión ya abierta. |

## Project Structure

### Documentation (this feature)

```text
specs/001-consola-reconexion/
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
src-tauri/                # comandos y eventos de este contrato
crates/ssh/               # sesión, keepalive, reintentos, huella
crates/vault/             # llavero; la contraseña no sale en logs
crates/inventory/         # SQLite de perfiles, sin secreto
crates/terminal/          # PTY, tamaño, tmux sshuttle, aviso de corte
ui/                       # alta, lista, indicador, xterm.js
```

**Structure Decision**: Se usan solo los crates que esta consola necesita. El sondeo general y las capacidades de archivos, actualizaciones, pods y asistente no se crean en este hito. `crates/terminal` no abre sockets: pide el PTY a la sesión que le entrega `crates/ssh`.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
| --- | --- | --- |
| El hito 1 adelanta la consola y aplaza el sondeo completo de la fase 2 | La spec aprobada define el hito como la consola con reconexión y excluye archivos, pods, actualizaciones y asistente | Hacer antes el sondeo de paquetes y Kubernetes no aporta la consola y contradice FR-027 |
