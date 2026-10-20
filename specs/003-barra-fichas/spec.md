# Feature Specification: Barra de fichas

**Feature Branch**: `feature/barra-fichas`

**Created**: 2026-10-20

**Status**: Draft

**Input**: User description: "La lista de conexiones pasa a ser una barra vertical de fichas, como un rail de íconos. Cada conexión guardada es una ficha con las iniciales del nombre y un punto de estado. La conexión elegida se destaca. Agregar una conexión usa un casillero vacío de esa misma barra y después pide solo host, puerto, usuario y contraseña. No se pregunta por paquetes, Kubernetes ni tmux. Eliminar sigue pidiendo confirmación."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Ver las conexiones como fichas (Priority: P1)

La lista de servidores deja de ser una columna de nombres y pasa a ser una barra vertical angosta. Cada conexión guardada es una ficha redondeada con las iniciales de su nombre visible y un punto que muestra si está activa, inactiva o reintentando. La ficha elegida se distingue de las demás. El operador sigue pudiendo abrir esa conexión y ver su consola.

**Why this priority**: Es el cambio que se ve al abrir la aplicación. Sin las fichas, el alta y el borrado no tienen dónde vivir en la barra.

**Independent Test**: Con tres servidores guardados, la barra muestra tres fichas con iniciales y un punto de estado. Al elegir una, solo esa se destaca y su consola queda a la vista. El punto cambia cuando esa conexión pasa de inactiva a activa o a reintentando.

**Acceptance Scenarios**:

1. **Given** varios servidores guardados, **When** el operador abre la aplicación, **Then** ve una barra vertical de fichas, una por servidor, y no una lista de nombres completos.
2. **Given** un servidor cuyo nombre visible tiene dos palabras, **When** aparece su ficha, **Then** las iniciales son la primera letra de cada una de esas dos palabras.
3. **Given** un servidor cuyo nombre visible es una sola palabra, **When** aparece su ficha, **Then** las iniciales son las dos primeras letras de esa palabra.
4. **Given** una conexión activa, inactiva o reintentando, **When** el operador mira su ficha, **Then** el punto de estado corresponde a ese valor y no hay un cuarto estado.
5. **Given** varias fichas, **When** el operador elige una, **Then** solo esa queda destacada y las otras no.
6. **Given** la ficha elegida, **When** el operador la mira junto con el resto de la pantalla, **Then** puede reconocer el nombre visible completo de esa conexión, no solo las iniciales.

---

### User Story 2 - Agregar desde un casillero vacío (Priority: P1)

Al final de la barra hay un casillero vacío. Elegirlo pide host, puerto, usuario y contraseña, igual que el alta actual. No pregunta por paquetes, Kubernetes ni tmux. El casillero no es una conexión guardada.

**Why this priority**: Sin ese casillero la barra no permite sumar un servidor. Es el reemplazo del botón de servidor nuevo.

**Independent Test**: Con la barra visible, elegir el casillero vacío y completar host, usuario y contraseña dejando el puerto vacío. El puerto queda en 22, no aparece ninguna pregunta de paquetes, Kubernetes o tmux, y al guardar aparece una ficha nueva.

**Acceptance Scenarios**:

1. **Given** la barra, con o sin conexiones, **When** el operador la recorre, **Then** hay un solo casillero vacío para agregar, distinto de las fichas de conexiones guardadas.
2. **Given** el casillero vacío, **When** el operador lo elige, **Then** puede ingresar host, puerto, usuario y contraseña.
3. **Given** el formulario de alta abierto desde el casillero, **When** el operador lo completa, **Then** no se le pide gestor de paquetes, Kubernetes ni tmux.
4. **Given** el alta con el puerto vacío, **When** el servidor queda guardado, **Then** el puerto es 22 y la barra muestra una ficha nueva con sus iniciales.
5. **Given** el casillero vacío, **When** el operador lo elige, **Then** ninguna ficha de conexión queda marcada como si ese casillero fuera un servidor.

---

### User Story 3 - Eliminar sigue pidiendo confirmación (Priority: P2)

Elegir una ficha sigue permitiendo eliminar esa conexión. La aplicación pide confirmación antes de borrar. Cancelar deja la ficha. Confirmar la quita de la barra, cierra la sesión si estaba abierta y olvida la contraseña, como ya está definido.

**Why this priority**: El cambio de forma no puede saltarse la confirmación. No es el motivo de esta spec, pero tiene que seguir estando.

**Independent Test**: Elegir una ficha, pedir eliminar y cancelar: la ficha sigue. Pedir eliminar y confirmar: la ficha desaparece y las demás quedan.

**Acceptance Scenarios**:

1. **Given** una ficha elegida, **When** el operador pide eliminarla y cancela, **Then** la ficha sigue en la barra y la sesión no se corta por ese pedido.
2. **Given** una ficha elegida, **When** el operador pide eliminarla y confirma, **Then** esa ficha desaparece y las otras fichas siguen igual.
3. **Given** la confirmación de eliminación, **When** el operador la ve, **Then** no se le pide gestor de paquetes, Kubernetes ni tmux.

---

### Edge Cases

- No hay servidores guardados: la barra muestra solo el casillero vacío.
- Hay un solo servidor: una ficha destacada al elegirla, más el casillero vacío.
- Dos servidores producen las mismas iniciales: las dos fichas se muestran; la destacada indica cuál está elegida, y el nombre visible completo de la elegida sigue disponible.
- El punto de estado cambia mientras la barra está a la vista: la ficha pasa a mostrar activa, inactiva o reintentando sin recargar la aplicación a mano.
- El nombre visible usa mayúsculas o minúsculas: las iniciales de la ficha se muestran en mayúscula.
- Los íconos de inicio, ajustes y listado que aparecen en la referencia visual no forman parte de esta barra.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Las conexiones guardadas MUST mostrarse en una barra vertical de fichas, una ficha por conexión.
- **FR-002**: Cada ficha MUST mostrar las iniciales del nombre visible: la primera letra de las dos primeras palabras, o las dos primeras letras si el nombre es una sola palabra.
- **FR-003**: Cada ficha MUST mostrar un punto de estado con exactamente uno de estos valores: activa, inactiva o reintentando.
- **FR-004**: La ficha elegida MUST distinguirse de las no elegidas. Solo una queda destacada a la vez.
- **FR-005**: Al elegir una ficha, el operador MUST poder ver el nombre visible completo de esa conexión.
- **FR-006**: La barra MUST ofrecer un solo casillero vacío para agregar una conexión. Ese casillero no es un servidor guardado.
- **FR-007**: Elegir el casillero vacío MUST pedir solo host, puerto, usuario y contraseña. El puerto vacío queda en 22.
- **FR-008**: El alta desde el casillero MUST NOT pedir gestor de paquetes, Kubernetes ni tmux.
- **FR-009**: Después de guardar, la barra MUST mostrar la ficha nueva.
- **FR-010**: Eliminar la conexión elegida MUST seguir pidiendo confirmación. Cancelar no quita la ficha ni cierra la sesión. Confirmar la quita de la barra.
- **FR-011**: Esta barra MUST NOT agregar accesos de inicio, ajustes ni un listado distinto de las fichas y el casillero de alta.

### Key Entities

- **Ficha**: La representación de una conexión guardada en la barra. Muestra iniciales y punto de estado. Corresponde a un servidor ya guardado, con su nombre visible, host, puerto y usuario.
- **Casillero vacío**: El único control de la barra para empezar un alta. No tiene iniciales ni punto de estado.
- **Punto de estado**: El indicador visible de esa conexión. Solo puede ser activa, inactiva o reintentando.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Con hasta diez conexiones, el operador identifica la elegida en menos de cinco segundos mirando solo la barra.
- **SC-002**: El operador abre el alta desde el casillero vacío y completa host, usuario y contraseña sin ver preguntas de paquetes, Kubernetes o tmux.
- **SC-003**: El punto de cada ficha coincide con activa, inactiva o reintentando en el momento en que la conexión cambia.
- **SC-004**: Cancelar una eliminación deja la misma cantidad de fichas. Confirmar la reduce en uno.

## Assumptions

- La lista de servidores, la consola y la eliminación con confirmación ya existen. Esta spec cambia cómo se ven y cómo se empieza un alta. No cambia la contraseña en el llavero, la reconexión ni la regla de borrado.
- Las iniciales salen del nombre visible. Si todavía no hay uno distinto, ese nombre es el host, como en el alta actual.
- Hay un solo casillero de alta. Los cuadrados vacíos repetidos de la referencia visual no se copian: no representan servidores.
- El indicador escrito de la consola sigue existiendo. El punto de la ficha muestra el mismo valor, no uno nuevo.
- Quedan fuera los íconos de inicio, ajustes y listado de la referencia, y también archivos, actualizaciones, pods y el asistente.
