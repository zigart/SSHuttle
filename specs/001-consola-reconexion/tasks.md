---
description: "Task list for the reconnecting SSH console"
---

# Tasks: Consola con reconexión

**Input**: Design documents from `/specs/001-consola-reconexion/`

**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: No se generan tareas de tests. La spec no pide TDD. La validación de punta a punta está en quickstart.md.

**Organization**: Tareas agrupadas por historia de usuario para implementar y probar cada una por separado.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Puede correr en paralelo (archivos distintos, sin depender de tareas incompletas)
- **[Story]**: Historia a la que pertenece (US1, US2, US3, US4, US5)
- Las fases de setup, fundación y cierre no llevan etiqueta de historia

## Path Conventions

- Núcleo: `crates/ssh/`, `crates/vault/`, `crates/inventory/`, `crates/terminal/`
- Shell: `src-tauri/`
- UI: `ui/`

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Crear el esqueleto Tauri y los crates de este hito

- [x] T001 Crear el workspace Cargo en `Cargo.toml` con miembros `src-tauri`, `crates/ssh`, `crates/vault`, `crates/inventory` y `crates/terminal`
- [x] T002 Inicializar Tauri 2 en `src-tauri/Cargo.toml` y la UI React con xterm.js en `ui/package.json`
- [x] T003 [P] Configurar formato y clippy en `rustfmt.toml` y `Cargo.toml`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Tipos y persistencia que todas las historias comparten

**⚠️ CRITICAL**: Ninguna historia empieza antes de cerrar esta fase

- [x] T004 Crear el esquema del perfil en `crates/inventory/src/schema.rs` con `id`, `nombre_visible`, `host` obligatorio y sin vacío, `puerto` entero 1–65535 con valor 22 si no se cambia, `usuario` obligatorio, `secreto_id` vacío hasta autenticar y `huella_host` vacía hasta la primera autenticación exitosa, sin columna de contraseña
- [x] T005 [P] Implementar el llavero en `crates/vault/src/lib.rs` con servicio `sshuttle` y cuenta igual a `secreto_id`, sin registrar la contraseña
- [x] T006 [P] Definir `SessionHandle` en `crates/ssh/src/session.rs` de modo que solo ese crate abra sockets `russh`
- [x] T007 [P] Definir el indicador `activa`, `inactiva` o `reintentando` y el motivo opcional en `crates/ssh/src/indicador.rs`
- [x] T008 Configurar el registro en `src-tauri/src/lib.rs` para no imprimir contraseñas ni el contenido del llavero
- [x] T009 Declarar los comandos y eventos de `specs/001-consola-reconexion/contracts/ipc.md` en `src-tauri/src/commands.rs`

**Checkpoint**: La fundación está lista y las historias pueden empezar

---

## Phase 3: User Story 1 - Dar de alta un servidor y usar la consola (Priority: P1) 🎯 MVP

**Goal**: Con host, puerto, usuario y contraseña el operador escribe en una consola. El puerto vacío queda en 22. No se pregunta por paquetes, Kubernetes ni tmux.

**Independent Test**: Dejar el puerto vacío, completar el alta y escribir un comando. El indicador dice `activa` y no hay pantallas extra. Al reabrir la app el servidor sigue en la lista.

### Implementation for User Story 1

- [x] T010 [P] [US1] Implementar `guardar_servidor` y `listar_servidores` en `crates/inventory/src/lib.rs`, con puerto omitido guardado como 22, `nombre_visible` copiado del host y sin campos de paquetes, Kubernetes ni tmux
- [x] T011 [US1] Autenticar con usuario y contraseña y abrir un PTY en `crates/ssh/src/session.rs`
- [x] T012 [US1] Publicar la salida del PTY y aplicar columnas y filas de la ventana en `crates/terminal/src/pty.rs`
- [x] T013 [US1] Conectar `conectar`, `escribir_consola`, `redimensionar_consola` y `salida_consola` en `src-tauri/src/commands.rs`
- [x] T014 [P] [US1] Crear el formulario de alta solo con host, puerto, usuario y contraseña en `ui/src/servers/ServerForm.tsx`
- [x] T015 [US1] Mostrar xterm.js y el indicador `activa` cuando la consola acepta escritura en `ui/src/console/Console.tsx`
- [x] T016 [US1] Mantener una sesión por id de servidor en `crates/ssh/src/supervisor.rs`

**Checkpoint**: La historia 1 se usa sola, sin llavero ni reconexión

---

## Phase 4: User Story 2 - Guardar la contraseña una sola vez (Priority: P1)

**Goal**: La contraseña se guarda en el llavero solo si el servidor la acepta. Después no se vuelve a pedir. Sin secreto, el indicador queda `inactiva` y no hay reintento.

**Independent Test**: Conectar, cerrar y volver a abrir el mismo servidor sin formulario de contraseña. Una contraseña rechazada no se guarda. Quitar el secreto del llavero deja el indicador en `inactiva`.

### Implementation for User Story 2

- [x] T017 [US2] Escribir la contraseña en el llavero solo después de una autenticación aceptada en `crates/ssh/src/session.rs`
- [x] T018 [US2] Guardar `secreto_id` sin la contraseña en `crates/inventory/src/lib.rs`
- [x] T019 [US2] Omitir la contraseña en `conectar` cuando el llavero ya tiene el secreto en `src-tauri/src/commands.rs`
- [x] T020 [US2] Emitir `inactiva` con motivo `falta_secreto` y no reintentar en `crates/ssh/src/supervisor.rs`
- [x] T021 [US2] Emitir `inactiva` con motivo `clave_rechazada` y no guardar la contraseña rechazada en `crates/ssh/src/session.rs`
- [x] T022 [US2] No volver a mostrar la contraseña después de enviarla en `ui/src/servers/ServerForm.tsx`

**Checkpoint**: Las historias 1 y 2 funcionan. La segunda conexión no pide contraseña

---

## Phase 5: User Story 3 - Reconectar sola si se corta la red (Priority: P1)

**Goal**: Un corte pasa el indicador a `reintentando` y la app reintenta sin credenciales hasta volver a `activa` o hasta que el operador cierre.

**Independent Test**: Con el indicador `activa`, cortar la red. Pasa a `reintentando` sin formulario. Al volver la red pasa a `activa`. Cerrar durante los reintentos lo deja en `inactiva`.

### Implementation for User Story 3

- [x] T023 [US3] Enviar keepalive cada 15 segundos en `crates/ssh/src/session.rs`
- [x] T024 [P] [US3] Implementar la espera 1, 2, 4, 8, 16 y luego 30 segundos en `crates/ssh/src/retry.rs`
- [x] T025 [US3] Mapear el corte de red a `reintentando` y la autenticación exitosa a `activa` en `crates/ssh/src/indicador.rs`
- [x] T026 [US3] Implementar `cerrar` para dejar `inactiva` y cancelar el temporizador en `crates/ssh/src/supervisor.rs`
- [x] T027 [US3] Publicar el evento `indicador` con un solo valor `activa`, `inactiva` o `reintentando` en `src-tauri/src/commands.rs`
- [x] T028 [US3] Mostrar exactamente `activa`, `inactiva` o `reintentando` en `ui/src/console/Indicator.tsx`

**Checkpoint**: El corte de red se recupera solo y el indicador no tiene un cuarto valor

---

## Phase 6: User Story 4 - Confirmar la identidad del servidor (Priority: P2)

**Goal**: La primera conexión guarda la huella SHA256. Si cambia, el indicador queda `inactiva` hasta que el operador confirme.

**Independent Test**: La segunda conexión con la misma huella entra. Una huella distinta no abre consola hasta confirmar.

### Implementation for User Story 4

- [x] T029 [US4] Guardar la huella SHA256 de la clave pública en la primera autenticación exitosa en `crates/ssh/src/hostkey.rs` y `crates/inventory/src/lib.rs`
- [x] T030 [US4] Cortar la sesión y emitir `inactiva` con motivo `huella_distinta` si la huella no coincide en `crates/ssh/src/session.rs`
- [x] T031 [US4] Implementar `confirmar_huella` para reemplazar la huella guardada y permitir un intento nuevo en `src-tauri/src/commands.rs`
- [x] T032 [US4] Pedir confirmación antes de aceptar la identidad nueva en `ui/src/console/HostKeyConfirm.tsx`

**Checkpoint**: Un host distinto no se acepta en silencio

---

## Phase 7: User Story 5 - Recuperar la consola si el servidor tiene tmux (Priority: P2)

**Goal**: Con tmux se crea o se retoma la sesión `sshuttle`. Sin tmux se abre un shell nuevo y se avisa que el trabajo quedó interrumpido. La app no reenvía lo ya escrito.

**Independent Test**: Con tmux, el texto de `sshuttle` sigue visible después del corte y no hay aviso de interrupción. Sin tmux hay consola nueva, el comando no sigue solo y el aviso aparece una vez.

### Implementation for User Story 5

- [x] T033 [P] [US5] Detectar tmux con `command -v tmux` y usar `tmux new-session -A -s sshuttle` en `crates/terminal/src/tmux.rs`
- [x] T034 [US5] Abrir el shell del usuario cuando tmux no existe en `crates/terminal/src/pty.rs`
- [x] T035 [US5] No reenviar la entrada previa y emitir `trabajo_interrumpido` una sola vez al volver sin tmux en `crates/terminal/src/pty.rs`
- [x] T036 [US5] Mostrar el aviso de trabajo interrumpido solo cuando no se reenganchó `sshuttle` en `ui/src/console/Console.tsx`

**Checkpoint**: La consola se recupera con tmux y no relanza comandos por su cuenta

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Cierre del hito sin sumar capacidades fuera de la spec

- [x] T037 [P] Revisar que `src-tauri/src/lib.rs` y `crates/vault/src/lib.rs` no registran la contraseña ni el payload del llavero
- [x] T038 Ejecutar la validación de `specs/001-consola-reconexion/quickstart.md` contra un servidor SSH de prueba
- [x] T039 Cumplir FR-003: permitir editar `nombre_visible` desde el panel (`renombrar_servidor` / UI); al actualizar host/puerto/usuario sin nombre nuevo, conservar el nombre ya guardado

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: sin dependencias
- **Foundational (Phase 2)**: depende del setup y bloquea todas las historias
- **User Stories (Phase 3+)**: dependen de la fundación
- **Polish (Phase 8)**: depende de las historias que se quieran cerrar

### User Story Dependencies

- **User Story 1 (P1)**: empieza después de la fundación. No depende de otras historias
- **User Story 2 (P1)**: depende de la conexión de US1 para guardar el secreto tras autenticar
- **User Story 3 (P1)**: depende de la sesión de US1 y del secreto de US2 para reconectar sin pedir contraseña
- **User Story 4 (P2)**: depende de la autenticación de US1 para guardar la huella
- **User Story 5 (P2)**: depende del PTY de US1 y del regreso de US3 para distinguir recuperación e interrupción

### Within Each User Story

- El modelo o el contrato van antes del comando Tauri
- El comando Tauri va antes de la pantalla que lo usa
- La historia se prueba en su checkpoint antes de pasar a la siguiente

### Parallel Opportunities

- T003 puede ir en paralelo con T002 después de T001
- T005, T006 y T007 pueden ir en paralelo después de T004
- T010 y T014 pueden ir en paralelo
- T024 puede ir en paralelo con T023
- T033 puede ir en paralelo con el resto de US5 solo después de que exista el PTY de US1
- T037 puede ir en paralelo con la preparación del servidor de T038

---

## Parallel Example: User Story 1

```bash
Task: "Implementar guardar_servidor y listar_servidores en crates/inventory/src/lib.rs"
Task: "Crear el formulario de alta en ui/src/servers/ServerForm.tsx"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Completar Phase 1: Setup
2. Completar Phase 2: Foundational
3. Completar Phase 3: User Story 1
4. **STOP and VALIDATE**: alta, consola e indicador `activa`
5. Seguir con US2 y US3 antes de dar por cerrada la reconexión

### Incremental Delivery

1. Setup + Foundational
2. US1 → consola usable
3. US2 → la contraseña se pide una sola vez
4. US3 → corte de red con indicador `reintentando`
5. US4 → huella con confirmación
6. US5 → sesión `sshuttle` o aviso de interrupción
7. Polish → quickstart.md

### Parallel Team Strategy

Con más de una persona, después de la fundación:

1. Una persona cierra US1
2. Otra puede adelantar el formulario y el llavero en archivos distintos
3. US3, US4 y US5 esperan la sesión de US1

---

## Notes

- [P] marca archivos distintos, sin depender de una tarea incompleta
- La etiqueta de historia sirve para rastrear US1–US5
- No crear `crates/discover`, `crates/files`, `crates/updates`, `crates/kube` ni `crates/assistant` en este hito
- Cada servidor muestra un solo indicador: `activa`, `inactiva` o `reintentando`
- Parar en cualquier checkpoint para probar esa historia sola
