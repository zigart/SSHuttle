---
description: "Task list for connection chip rail UI"
---

# Tasks: Barra de fichas

**Input**: Design documents from `/specs/003-barra-fichas/`

**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: No se generan tareas de tests. La spec no pide TDD. La validación de punta a punta está en quickstart.md.

**Organization**: Tareas agrupadas por historia de usuario para implementar y probar cada una por separado.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Puede correr en paralelo (archivos distintos, sin depender de tareas incompletas)
- **[Story]**: Historia a la que pertenece (US1, US2, US3)
- Las fases de setup, fundación y cierre no llevan etiqueta de historia

## Path Conventions

- UI: `ui/src/`
- Docs: `docs/arquitectura.md`
- Spec: `specs/003-barra-fichas/`

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Usar el workspace que ya trajo `develop`. No se crea un crate ni un comando Tauri.

- [x] T001 Confirmar en `Cargo.toml` y `ui/package.json` que no se agregan crates ni dependencias nuevas solo para la barra; el trabajo queda en `ui/` y en `docs/arquitectura.md`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Función de iniciales según el contrato, compartida por todas las fichas

**⚠️ CRITICAL**: Ninguna historia empieza antes de cerrar esta fase

- [x] T002 Implementar `iniciales(nombre_visible: string): string` en `ui/src/servers/iniciales.ts` según `specs/003-barra-fichas/contracts/ui.md`: `trim` y partir por `/\s+/` descartando vacíos; si hay dos o más palabras, primer carácter de las dos primeras; si hay una, hasta dos primeros caracteres; si no hay palabras, `""`; resultado en mayúsculas; cubrir los ejemplos `bastion prod` → `BP`, `bastion` → `BA`, `a` → `A`, `  web  api  ` → `WA`

**Checkpoint**: Las iniciales se pueden calcular sin montar la barra

---

## Phase 3: User Story 1 - Ver las conexiones como fichas (Priority: P1) 🎯 MVP

**Goal**: La lista de nombres pasa a una barra vertical de fichas con iniciales, punto de estado (`activa` | `inactiva` | `reintentando`) y la elegida destacada; el panel muestra el `nombre_visible` completo.

**Independent Test**: Con tres servidores guardados, la barra muestra tres fichas con iniciales y un punto. Al elegir una, solo esa se destaca, se ve su nombre completo y su consola. El punto cambia con el evento `indicador` sin reiniciar la app.

### Implementation for User Story 1

- [x] T003 [P] [US1] Crear el componente de ficha en `ui/src/servers/Ficha.tsx`: muestra `iniciales(nombre_visible)`, un punto cuyo valor es exactamente `activa` | `inactiva` | `reintentando` (default `inactiva` si no hay evento), estilo distinto solo si `elegida`; al activarla llama con el `id`; no muestra el nombre completo dentro de la ficha
- [x] T004 [P] [US1] Estilos del rail y de la ficha en `ui/src/App.css`: barra vertical angosta; ficha redondeada; punto de estado; ficha elegida distinguible; sin layout de lista de nombres completos en ese rail
- [x] T005 [US1] En `ui/src/App.tsx` reemplazar la `<ul className="lista">` de nombres por la barra de fichas: una `Ficha` por servidor de `listar_servidores` en ese orden; al elegir, `seleccion = id`; con ficha elegida mostrar `nombre_visible` completo en el panel (fuera de la ficha); reutilizar el mapa `estados` del evento `indicador` sin inventar un cuarto valor; no agregar íconos de inicio, ajustes ni listado aparte

**Checkpoint**: La historia 1 se usa sola; el alta puede seguir por el botón existente hasta US2

---

## Phase 4: User Story 2 - Agregar desde un casillero vacío (Priority: P1)

**Goal**: Un solo casillero vacío al final de la barra abre el alta (host, puerto, usuario, contraseña). No es un servidor. Reemplaza el botón “Nuevo servidor”.

**Independent Test**: Elegir el casillero, completar host, usuario y contraseña con puerto vacío. Puerto 22, sin preguntas de paquetes/Kubernetes/tmux, y aparece una ficha nueva. Ninguna ficha queda marcada al elegir el casillero.

### Implementation for User Story 2

- [x] T006 [P] [US2] Agregar el casillero vacío en `ui/src/servers/CasilleroAlta.tsx` (o equivalente en `ui/src/servers/`): sin iniciales ni punto; distinto visualmente de una ficha; al elegirlo no llama a inventario
- [x] T007 [US2] En `ui/src/App.tsx` y `ui/src/App.css`: colocar un solo casillero al final de la barra; al elegirlo `seleccion = null` (ninguna ficha destacada) y mostrar `ServerForm` pidiendo contraseña; quitar el botón “Nuevo servidor”; tras alta exitosa, listar de nuevo, mostrar la ficha nueva y dejarla elegida; el formulario no pide paquetes, Kubernetes ni tmux

**Checkpoint**: Alta solo desde el casillero; la barra vacía muestra únicamente el casillero

---

## Phase 5: User Story 3 - Eliminar sigue pidiendo confirmación (Priority: P2)

**Goal**: Con una ficha elegida, Eliminar pide confirmación. Cancelar deja la ficha y la sesión. Confirmar quita la ficha de la barra (mismo comando ya existente).

**Independent Test**: Elegir una ficha, eliminar y cancelar: la ficha sigue. Confirmar: desaparece y las demás quedan. Sin preguntas de paquetes/Kubernetes/tmux.

### Implementation for User Story 3

- [x] T008 [US3] En `ui/src/App.tsx` asegurar que Eliminar / Confirmar / Cancelar siguen disponibles con una ficha elegida: Cancelar no llama `eliminar_servidor` ni cierra sesión; Confirmar llama `eliminar_servidor`, recarga la lista y la ficha desaparece; no agregar campos de paquetes, Kubernetes ni tmux en esa confirmación

**Checkpoint**: Las tres historias funcionan sobre la misma barra

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Arquitectura documentada y validación manual sin sumar fuera de v1

- [x] T009 [P] Actualizar `docs/arquitectura.md` con una línea: la lista de conexiones se muestra como barra vertical de fichas con casillero de alta (constitución I)
- [x] T010 Revisar que `ui/src/App.tsx` y la barra no incluyen accesos de inicio, ajustes ni un listado distinto de fichas + casillero (`FR-011`)
- [x] T011 Ejecutar la validación de `specs/003-barra-fichas/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: sin dependencias
- **Foundational (Phase 2)**: depende del setup y bloquea todas las historias
- **User Stories (Phase 3+)**: dependen de la fundación; US2 y US3 asumen la barra de US1
- **Polish (Phase 6)**: depende de las historias que se quieran cerrar

### User Story Dependencies

- **User Story 1 (P1)**: empieza después de la fundación. MVP
- **User Story 2 (P1)**: necesita la barra de US1 para colocar el casillero; el componente del casillero (T006) puede ir en paralelo con el cierre de estilos de US1
- **User Story 3 (P2)**: necesita la barra y la selección de US1; el flujo de eliminar ya existe, solo se adapta al contexto de ficha

### Within Each User Story

- Iniciales antes de la ficha
- Componente y CSS de ficha antes (o en paralelo) de cablear `App.tsx`
- Casillero antes de quitar “Nuevo servidor”
- La historia se prueba en su checkpoint antes de pasar a la siguiente prioridad

### Parallel Opportunities

- T003 y T004 pueden ir en paralelo después de T002
- T006 puede ir en paralelo con T004 / el final de T005
- T009 puede ir en paralelo con T010 mientras se prepara T011

---

## Parallel Example: User Story 1

```bash
# Tras T002, en paralelo:
Task: "Crear Ficha en ui/src/servers/Ficha.tsx"
Task: "Estilos del rail en ui/src/App.css"

# Después, cablear:
Task: "Reemplazar la lista por la barra en ui/src/App.tsx"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Completar Phase 1 y Phase 2
2. Completar Phase 3: fichas + selección + nombre completo
3. **STOP**: validar el Independent Test de US1
4. Seguir con casillero (US2) y eliminar (US3)

### Incremental Delivery

1. Setup + iniciales → base lista
2. US1 → barra usable (MVP visual)
3. US2 → alta desde casillero
4. US3 → eliminar con confirmación en el nuevo layout
5. Polish → arquitectura + quickstart

### Parallel Team Strategy

Con una sola persona en UI, el orden es T001 → T002 → (T003 ∥ T004) → T005 → (T006 ∥ prep) → T007 → T008 → (T009 ∥ T010) → T011.

---

## Notes

- [P] = archivos distintos, sin depender de tareas incompletas
- Sin IPC ni crates nuevos
- El punto de la ficha reutiliza `ValorIndicador` de `ui/src/console/Indicator.tsx`; no hay cuarto estado
- Commit después de cada tarea o grupo lógico
- Parar en cualquier checkpoint para validar la historia sola
