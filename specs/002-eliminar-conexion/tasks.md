---
description: "Task list for deleting a saved connection"
---

# Tasks: Eliminar una conexión

**Input**: Design documents from `/specs/002-eliminar-conexion/`

**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: No se generan tareas de tests. La spec no pide TDD. La validación de punta a punta está en quickstart.md.

**Organization**: Tareas agrupadas por historia de usuario para implementar y probar cada una por separado.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Puede correr en paralelo (archivos distintos, sin depender de tareas incompletas)
- **[Story]**: Historia a la que pertenece (US1, US2)
- Las fases de setup, fundación y cierre no llevan etiqueta de historia

## Path Conventions

- Núcleo: `crates/inventory/`, `crates/vault/`, `crates/ssh/`
- Shell: `src-tauri/`
- UI: `ui/`

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Usar el workspace que ya trajo `develop`. No se crea un crate nuevo.

- [x] T001 Confirmar en `Cargo.toml` que los miembros siguen siendo `src-tauri`, `crates/ssh`, `crates/vault`, `crates/inventory` y `crates/terminal`, sin agregar un crate para eliminar

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Borrar una fila y una entrada del llavero, que las dos historias usan

**⚠️ CRITICAL**: Ninguna historia empieza antes de cerrar esta fase

- [x] T002 Implementar `eliminar` en `crates/inventory/src/lib.rs`: borra solo la fila de ese `id`; si el `id` no existe, termina bien; no agrega columnas ni escribe contraseña; las demás filas conservan `id`, `nombre_visible`, `host`, `puerto`, `usuario`, `secreto_id` y `huella_host`
- [x] T003 [P] Implementar `borrar` en `crates/vault/src/lib.rs`: un `secreto_id` vacío no llama al llavero; `delete_credential` borra la entrada de servicio `sshuttle` y cuenta `secreto_id`; `NoEntry` es éxito; cualquier otro error es fallo del llavero; el log anota solo la cuenta, nunca la contraseña

**Checkpoint**: La fila y el secreto se pueden quitar por separado

---

## Phase 3: User Story 1 - Quitar un servidor de la lista (Priority: P1) 🎯 MVP

**Goal**: El operador confirma y ese servidor desaparece de la lista. Cancelar no cambia nada. Los demás servidores siguen. No se pregunta por paquetes, Kubernetes ni tmux, y no se toca la máquina remota.

**Independent Test**: Con dos servidores guardados y sin sesión abierta, eliminar uno tras confirmar. Ese desaparece y el otro sigue. Cancelar en el que queda no cambia la lista. Cerrar y abrir la aplicación deja el mismo resultado.

### Implementation for User Story 1

- [x] T004 [P] [US1] Agregar `eliminarServidor` en `ui/src/api.ts` con el `id` y sin enviar contraseña, según `specs/002-eliminar-conexion/contracts/ipc.md`
- [x] T005 [US1] En `ui/src/App.tsx` mostrar Eliminar en el servidor elegido; al pulsarlo pedir Confirmar o Cancelar; Cancelar no llama al comando y deja la lista igual; Confirmar llama `eliminar_servidor` y vuelve a listar; no mostrar campos de paquetes, Kubernetes ni tmux
- [x] T006 [US1] Implementar `eliminar_servidor` en `src-tauri/src/commands.rs` y registrarlo en `src-tauri/src/lib.rs`: si el `id` no está, terminar bien; si está, borrar esa fila con `inventory.eliminar` y no contactar al host

**Checkpoint**: La historia 1 se usa sola, con servidores sin sesión abierta

---

## Phase 4: User Story 2 - Cerrar la sesión y olvidar la contraseña (Priority: P1)

**Goal**: Al confirmar, la sesión o los reintentos se detienen, la contraseña sale del llavero y solo entonces se borra el perfil. Si el llavero falla, el perfil sigue y se ve el error. Un servidor sin contraseña también se puede eliminar.

**Independent Test**: Conectar un servidor, eliminarlo confirmando, y comprobar que la consola se cierra y no pasa a `reintentando`. Agregar de nuevo el mismo host: se pide la contraseña. Repetir con un servidor en `reintentando` y con uno sin contraseña guardada.

### Implementation for User Story 2

- [x] T007 [P] [US2] Agregar en `crates/ssh/src/supervisor.rs` un cierre que envía `Cerrar` y espera a que el lazo de ese `id` termine, también si está `reintentando` o esperando confirmar la huella; si no hay sesión, termina enseguida
- [x] T008 [US2] Extender `eliminar_servidor` en `src-tauri/src/commands.rs` para que, antes de `inventory.eliminar`, cierre la sesión con el supervisor y llame a `vault.borrar` cuando hay `secreto_id`; si el llavero falla, no borrar la fila y devolver el error sin la contraseña; `secreto_id` vacío o `NoEntry` siguen hacia el borrado de la fila
- [x] T009 [US2] En `ui/src/App.tsx`, si `eliminar_servidor` falla, dejar ese servidor en la lista y mostrar que no se pudo eliminar; si sale bien, quitar su consola para que no siga en `activa` ni en `reintentando`

**Checkpoint**: Las historias 1 y 2 funcionan. Eliminar cierra la sesión y olvida la contraseña

---

## Phase 5: Polish & Cross-Cutting Concerns

**Purpose**: Cierre sin sumar capacidades fuera de la spec

- [x] T010 [P] Revisar que `src-tauri/src/commands.rs` y `crates/vault/src/lib.rs` no registran la contraseña ni el contenido del llavero al eliminar
- [x] T011 Ejecutar la validación de `specs/002-eliminar-conexion/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: sin dependencias
- **Foundational (Phase 2)**: depende del setup y bloquea todas las historias
- **User Stories (Phase 3+)**: dependen de la fundación
- **Polish (Phase 5)**: depende de las historias que se quieran cerrar

### User Story Dependencies

- **User Story 1 (P1)**: empieza después de la fundación. No depende de US2
- **User Story 2 (P1)**: el cierre del supervisor puede adelantarse en paralelo con US1; el comando y la pantalla dependen de `eliminar_servidor` y de la confirmación de US1

### Within Each User Story

- El inventario y el llavero van antes del comando Tauri
- El comando Tauri va antes de la pantalla que muestra su error
- La historia se prueba en su checkpoint antes de pasar a la siguiente

### Parallel Opportunities

- T003 puede ir en paralelo con T002 después de T001
- T004 puede ir en paralelo con T002 y T003 en cuanto existan los nombres del contrato, y en paralelo con T007
- T007 puede ir en paralelo con T004, T005 y T006 porque toca solo `crates/ssh/src/supervisor.rs`
- T008 espera T003, T006 y T007
- T010 puede ir en paralelo con la preparación del servidor de T011

---

## Parallel Example: User Story 1

```bash
Task: "Agregar eliminarServidor en ui/src/api.ts"
Task: "Agregar el cierre que espera al lazo en crates/ssh/src/supervisor.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Completar Phase 1: Setup
2. Completar Phase 2: Foundational
3. Completar Phase 3: User Story 1
4. **STOP and VALIDATE**: confirmar, cancelar y que el otro servidor sigue
5. Seguir con US2 antes de dar por cerrada la contraseña y la sesión

### Incremental Delivery

1. Setup + Foundational
2. US1 → el servidor sale de la lista
3. US2 → se cierra la sesión y se olvida la contraseña
4. Polish → quickstart.md

### Parallel Team Strategy

Con más de una persona, después de la fundación:

1. Una persona cierra la lista y la confirmación de US1
2. Otra adelanta el cierre del supervisor en `crates/ssh/src/supervisor.rs`
3. El comando que borra el secreto espera a las dos

---

## Notes

- [P] marca archivos distintos, sin depender de una tarea incompleta
- La etiqueta de historia sirve para rastrear US1 y US2
- No crear `crates/discover`, `crates/files`, `crates/updates`, `crates/kube` ni `crates/assistant`
- Cancelar no llama a `eliminar_servidor`
- Si el llavero falla, la fila no se borra
- Parar en cualquier checkpoint para probar esa historia sola
