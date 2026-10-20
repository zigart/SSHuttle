# Feature Specification: Eliminar una conexión

**Feature Branch**: `feature/eliminar-conexion`

**Created**: 2026-10-20

**Status**: Draft

**Input**: User description: "El operador puede eliminar una conexión guardada. Elige un servidor de la lista, confirma, y la app cierra la sesión si estaba abierta, borra el perfil de SQLite y elimina la contraseña del llavero. No pide paquetes, Kubernetes ni tmux."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Quitar un servidor de la lista (Priority: P1)

El operador elige un servidor guardado y lo elimina. Antes de borrar nada, la aplicación pide confirmación. Si confirma, ese servidor desaparece de la lista y no vuelve al reabrir la aplicación. Si cancela, el servidor sigue igual. No se pregunta por paquetes, Kubernetes ni tmux, y no se borra nada en la máquina remota.

Corresponde a la fase de inventario de [docs/arquitectura.md](../../docs/arquitectura.md).

**Why this priority**: Sin esta acción la lista solo crece. Un servidor que ya no se usa sigue ocupando la lista y guardando su contraseña.

**Independent Test**: Con dos servidores guardados y sin sesión abierta, eliminar uno tras confirmar. Ese desaparece y el otro sigue. Cancelar en otro servidor no cambia la lista. Cerrar y abrir la aplicación deja el mismo resultado.

**Acceptance Scenarios**:

1. **Given** un servidor guardado, **When** el operador pide eliminarlo y confirma, **Then** ese servidor deja de aparecer en la lista.
2. **Given** el pedido de confirmación visible, **When** el operador cancela, **Then** el servidor sigue en la lista y no se elimina su contraseña.
3. **Given** un servidor recién eliminado, **When** el operador cierra y vuelve a abrir la aplicación, **Then** ese servidor no está en la lista.
4. **Given** dos servidores guardados, **When** el operador elimina uno, **Then** el otro permanece con su nombre, host, puerto y usuario.
5. **Given** la confirmación de eliminación, **When** el operador la ve, **Then** no se le pide gestor de paquetes, Kubernetes ni tmux.
6. **Given** un servidor guardado, **When** el operador lo elimina, **Then** la máquina remota no se modifica: solo desaparece la conexión guardada en esta aplicación.

---

### User Story 2 - Cerrar la sesión y olvidar la contraseña (Priority: P1)

Si el servidor elegido tiene una sesión abierta o está reintentando, al confirmar se cierra esa sesión y no vuelve a conectarse sola. La contraseña guardada para ese servidor se elimina. La próxima vez que el operador agregue el mismo host, la aplicación vuelve a pedir la contraseña. Un servidor que nunca llegó a guardar contraseña también se puede eliminar.

**Why this priority**: Borrar solo la fila de la lista dejaría una sesión viva y una contraseña en el llavero. Eliminar tiene que cerrar las dos.

**Independent Test**: Conectar un servidor, eliminarlo confirmando, y comprobar que la consola se cierra y no pasa a "reintentando". Agregar de nuevo el mismo host: se pide la contraseña. Repetir con un servidor que está en "reintentando" y con uno que no tiene contraseña guardada.

**Acceptance Scenarios**:

1. **Given** un servidor con indicador "activa", **When** el operador lo elimina y confirma, **Then** la consola se cierra y la aplicación no vuelve a abrir esa sesión.
2. **Given** un servidor con indicador "reintentando", **When** el operador lo elimina y confirma, **Then** los reintentos se detienen y el servidor desaparece de la lista.
3. **Given** un servidor cuya contraseña está guardada, **When** la eliminación se confirma, **Then** esa contraseña deja de estar guardada y no aparece en pantalla ni en los registros.
4. **Given** un servidor eliminado, **When** el operador lo agrega otra vez con el mismo host y usuario, **Then** la aplicación pide la contraseña de nuevo.
5. **Given** un servidor guardado que todavía no tiene contraseña, **When** el operador lo elimina y confirma, **Then** desaparece de la lista.
6. **Given** una contraseña que no se puede quitar del llavero, **When** el operador confirma la eliminación, **Then** el servidor sigue en la lista y se le informa que no se pudo eliminar.

---

### Edge Cases

- El operador confirma y, en ese momento, la sesión se corta por la red: la eliminación igual cierra lo que quede y quita el servidor. No queda un reintento huérfano.
- La lista tiene un solo servidor: al eliminarlo, la lista queda vacía y se puede agregar uno nuevo.
- El operador pide eliminar dos veces el mismo servidor: la segunda vez ya no está en la lista.
- Hay una confirmación de identidad del host pendiente: al eliminar, esa pregunta desaparece junto con el servidor.
- Cancelar no cierra una sesión que ya estaba abierta.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: El operador MUST poder elegir un servidor de la lista y pedir eliminarlo.
- **FR-002**: La aplicación MUST pedir confirmación antes de eliminar. Sin esa confirmación no cambia la lista, la sesión ni la contraseña.
- **FR-003**: Si el operador cancela, el servidor MUST permanecer como estaba, incluida una sesión abierta o un reintento en curso.
- **FR-004**: Si el operador confirma, ese servidor MUST desaparecer de la lista y MUST seguir ausente al reabrir la aplicación.
- **FR-005**: Si ese servidor tiene una sesión abierta o está reintentando, la aplicación MUST cerrarla y MUST NOT volver a conectarla por su cuenta.
- **FR-006**: La contraseña guardada de ese servidor MUST eliminarse del llavero y MUST NOT quedar visible en pantalla ni en los registros.
- **FR-007**: Si la contraseña no se puede quitar del llavero, el servidor MUST permanecer en la lista y el operador MUST ver que no se pudo eliminar.
- **FR-008**: Los demás servidores guardados MUST permanecer sin cambios.
- **FR-009**: Un servidor sin contraseña guardada MUST poder eliminarse igual.
- **FR-010**: La eliminación MUST NOT pedir gestor de paquetes, Kubernetes ni tmux.
- **FR-011**: La eliminación MUST NOT modificar la máquina remota. Solo quita la conexión guardada en esta aplicación.

### Key Entities

- **Servidor guardado**: Una conexión de la lista, con nombre visible, host, puerto y usuario. Es lo que el operador elige para eliminar.
- **Contraseña guardada**: El secreto de ese servidor en el llavero. Se elimina junto con el servidor cuando la eliminación se confirma y el llavero lo permite.
- **Sesión**: La consola abierta o el reintento en curso de ese servidor. Se cierra al confirmar y no continúa.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: El operador elimina un servidor de la lista con una sola confirmación, en menos de un minuto.
- **SC-002**: Después de confirmar, ese servidor no aparece al reabrir la aplicación.
- **SC-003**: Al agregar de nuevo el mismo host y usuario, la aplicación pide la contraseña otra vez.
- **SC-004**: Una sesión "activa" o "reintentando" de ese servidor se detiene al confirmar y no vuelve sola.
- **SC-005**: Cancelar deja el servidor, la sesión y la contraseña como estaban.
- **SC-006**: En una lista de varios servidores, eliminar uno deja intactos todos los demás.

## Assumptions

- La lista de servidores y el guardado de la contraseña en el llavero ya existen por la consola con reconexión. Esta spec solo agrega quitarlos.
- La confirmación es un paso explícito del operador, del mismo tipo que ya exige la constitución antes de borrar. No hay eliminación masiva ni deshacer.
- Si el llavero rechaza el borrado de la contraseña, no se quita el servidor: así no queda una fila sin secreto ni un secreto sin fila.
- Queda fuera de esta spec editar el servidor, borrar varios a la vez, archivos, actualizaciones, pods y el asistente.
