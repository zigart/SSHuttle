# Feature Specification: Consola con reconexión

**Feature Branch**: `feature/consola-reconexion`

**Created**: 2026-10-20

**Status**: Draft

**Input**: User description: "Consola SSH con reconexión automática, hito 1. Cubre las historias #1 a #6: alta con host, puerto, usuario y contraseña; secreto en el llavero; consola al autenticar; reconexión con espera hasta 30 segundos sin volver a pedir credenciales; sesión tmux sshuttle si existe; huella del host con confirmación si cambia. La constitución y docs/arquitectura.md mandan. No incluye archivos, pods, actualizaciones ni asistente."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Dar de alta un servidor y usar la consola (Priority: P1)

El operador agrega un servidor indicando host, puerto, usuario y contraseña, y en cuanto el servidor lo acepta puede escribir en una consola. El puerto viene en 22. El nombre visible se completa con el host y se puede cambiar. No hay preguntas sobre paquetes, Kubernetes ni tmux.

Corresponde a las historias [#1](https://github.com/zigart/SSHuttle/issues/1) y [#3](https://github.com/zigart/SSHuttle/issues/3).

**Why this priority**: Sin el alta y la consola no hay forma de operar el servidor. Es el recorrido mínimo que ya entrega valor.

**Independent Test**: Con un servidor que acepta usuario y contraseña, el operador completa el alta dejando el puerto vacío y escribe un comando en la consola. No aparece ninguna pantalla de configuración extra.

**Acceptance Scenarios**:

1. **Given** que no hay servidores guardados, **When** el operador ingresa host, usuario y contraseña y deja el puerto vacío, **Then** el puerto queda en 22, el nombre visible es el host y, al aceptar el servidor, la consola permite escribir y ver la respuesta.
2. **Given** el formulario de alta, **When** el operador lo completa, **Then** no se le pide gestor de paquetes, Kubernetes ni tmux.
3. **Given** un servidor guardado, **When** el operador cierra y vuelve a abrir la aplicación, **Then** el servidor sigue en la lista.
4. **Given** dos servidores guardados, **When** el operador abre cada uno, **Then** cada uno tiene su propia conexión y su propia consola.
5. **Given** la consola abierta, **When** el operador cambia el tamaño de la ventana, **Then** la consola se ajusta a ese tamaño.
6. **Given** un servidor aceptado, **When** la consola ya se puede usar, **Then** el indicador de conexión muestra "activa".

---

### User Story 2 - Guardar la contraseña una sola vez (Priority: P1)

La contraseña se pide en la primera conexión. Si el servidor la acepta, queda en el llavero del sistema. El perfil guardado no contiene la contraseña. Las veces siguientes se usa ese secreto sin preguntar. Si no hay secreto, el indicador queda "inactiva", se explica que falta el secreto y no hay reintentos silenciosos.

Corresponde a la historia [#2](https://github.com/zigart/SSHuttle/issues/2).

**Why this priority**: La reconexión automática no puede cumplirse si cada corte vuelve a pedir la contraseña.

**Independent Test**: Conectar una vez, cerrar la conexión y volver a abrirla. La segunda vez no aparece el pedido de contraseña. Borrar el secreto del llavero deja el indicador en "inactiva" y no reintenta solo.

**Acceptance Scenarios**:

1. **Given** un servidor nuevo, **When** el servidor acepta la contraseña, **Then** esa contraseña queda guardada en el llavero y no se muestra de nuevo en pantalla ni en los registros.
2. **Given** un secreto ya guardado, **When** el operador abre ese servidor, **Then** la autenticación usa el llavero y no pide usuario ni contraseña.
3. **Given** un perfil sin secreto en el llavero, **When** el operador intenta conectar, **Then** el indicador muestra "inactiva", se informa que falta el secreto y la aplicación no reintenta sola.
4. **Given** una contraseña que el servidor rechaza, **When** falla el primer intento, **Then** el indicador queda "inactiva", el operador ve el error y esa contraseña no queda guardada para reintentos automáticos.

---

### User Story 3 - Reconectar sola si se corta la red (Priority: P1)

Cada servidor muestra un indicador con solo tres valores: activa, inactiva o reintentando. Con la sesión usable y el secreto guardado, un corte de red pasa el indicador a "reintentando" y la aplicación reintenta hasta volver a "activa", sin pedir credenciales. La espera entre intentos crece y no pasa de 30 segundos. Si el operador cierra, el indicador queda "inactiva" y los reintentos se detienen. Mientras se usa, la conexión se mantiene activa sin que el operador tenga que hacer nada para evitar una caída por inactividad breve.

Corresponde a la historia [#4](https://github.com/zigart/SSHuttle/issues/4).

**Why this priority**: Es el comportamiento que define el hito. La consola deja de servir si cada corte exige volver a identificarse.

**Independent Test**: Con una consola ya autenticada, interrumpir la red. El indicador pasa a "reintentando", no aparece el formulario de contraseña y, al volver la red, el indicador vuelve a "activa". Cerrar durante los reintentos lo deja en "inactiva".

**Acceptance Scenarios**:

1. **Given** una consola con indicador "activa" y un secreto guardado, **When** se corta la red, **Then** el indicador pasa a "reintentando" y se reintenta sin pedir usuario ni contraseña.
2. **Given** reintentos en curso, **When** pasa el tiempo entre un intento y el siguiente, **Then** esa espera es mayor que la anterior y nunca supera los 30 segundos.
3. **Given** reintentos en curso, **When** la red vuelve, **Then** el indicador pasa a "activa".
4. **Given** reintentos en curso, **When** el operador cierra la conexión, **Then** el indicador queda "inactiva" y no sigue intentando.
5. **Given** el servidor no se puede alcanzar en el primer intento, **When** el operador no cierra, **Then** el indicador permanece "reintentando" y siguen los mismos reintentos.
6. **Given** cualquier servidor de la lista, **When** el operador lo mira, **Then** ve un indicador que dice exactamente "activa", "inactiva" o "reintentando".

---

### User Story 4 - Confirmar la identidad del servidor (Priority: P2)

La primera conexión recuerda la identidad del servidor. Si esa identidad cambia, la conexión se corta y no continúa hasta que el operador lo confirme.

Corresponde a la historia [#6](https://github.com/zigart/SSHuttle/issues/6).

**Why this priority**: Evita entrar en silencio a un servidor que ya no es el que el operador guardó. No bloquea el primer uso, pero es obligatorio antes de dar el hito por cerrado.

**Independent Test**: Conectar una vez y comprobar que una segunda conexión con la misma identidad sigue. Cambiar la identidad del servidor deja la consola desconectada hasta una confirmación explícita.

**Acceptance Scenarios**:

1. **Given** un servidor al que nunca se entró, **When** la primera conexión tiene éxito, **Then** la identidad del servidor queda recordada.
2. **Given** una identidad recordada que coincide, **When** el operador conecta de nuevo, **Then** la conexión continúa.
3. **Given** una identidad distinta a la recordada, **When** el operador intenta conectar, **Then** el indicador queda "inactiva", la conexión se corta y no se acepta el servidor nuevo hasta que el operador lo confirme.

---

### User Story 5 - Recuperar la consola si el servidor tiene tmux (Priority: P2)

Si el servidor tiene tmux, al conectar se crea o se retoma una sesión llamada `sshuttle`, de modo que el contenido de la consola sigue ahí después de un corte. Si no hay tmux, al reconectar se abre una consola nueva. Un trabajo que quedó a medias por el corte no continúa solo: queda interrumpido y el operador lo vuelve a lanzar.

Corresponde a la historia [#5](https://github.com/zigart/SSHuttle/issues/5).

**Why this priority**: La reconexión ya devuelve la red. Esta historia devuelve el trabajo en pantalla cuando el servidor puede conservarlo. Sin tmux, una consola nueva sigue siendo usable.

**Independent Test**: En un servidor con tmux, escribir en la consola, cortar la red y volver. El texto de la sesión `sshuttle` sigue visible. Repetir en un servidor sin tmux muestra una consola nueva. Un comando que estaba corriendo al cortarse no sigue solo.

**Acceptance Scenarios**:

1. **Given** un servidor con tmux y sin sesión previa, **When** la consola conecta, **Then** queda abierta una sesión llamada `sshuttle`.
2. **Given** una sesión `sshuttle` ya existente, **When** la conexión vuelve después de un corte, **Then** el operador ve el contenido de esa sesión.
3. **Given** un servidor sin tmux, **When** la conexión vuelve después de un corte, **Then** se abre una consola nueva.
4. **Given** un comando en curso, **When** la conexión se corta a mitad, **Then** al volver no se reanuda solo y el operador puede ver que quedó interrumpido.

---

### Edge Cases

- La contraseña es rechazada: el indicador queda "inactiva", se informa el error y no se guarda para reintentos automáticos.
- El servidor no responde: el indicador queda "reintentando" y los reintentos siguen hasta que el operador cierra.
- El operador cierra durante un reintento: el indicador pasa a "inactiva" y no hay más intentos.
- El secreto se borra del llavero después de haber conectado: el siguiente intento deja el indicador en "inactiva" e informa que falta el secreto.
- La identidad del servidor cambia: el indicador queda "inactiva" y no hay conexión hasta una confirmación explícita.
- El servidor no tiene tmux: hay consola nueva, no un error que impida trabajar.
- En el teléfono, la aplicación no mantiene un proceso permanente en segundo plano. La reconexión corre con la aplicación en primer plano o al volver a ella.
- Hay varios servidores: el corte de uno no corta la consola de otro.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: El operador MUST poder dar de alta un servidor solo con host, puerto, usuario y contraseña.
- **FR-002**: El puerto MUST valer 22 cuando el operador no lo cambia.
- **FR-003**: El nombre visible MUST precargarse con el host y MUST poder editarse.
- **FR-004**: El alta MUST NOT pedir gestor de paquetes, Kubernetes ni tmux.
- **FR-005**: Un servidor guardado MUST seguir disponible después de cerrar y abrir la aplicación.
- **FR-006**: La contraseña MUST guardarse en el llavero del sistema solo después de que el servidor la acepte.
- **FR-007**: El perfil guardado MUST NOT incluir la contraseña.
- **FR-008**: La contraseña guardada MUST NOT volver a mostrarse en pantalla ni en los registros.
- **FR-009**: Si hay secreto guardado, las conexiones siguientes MUST autenticar sin pedir usuario ni contraseña.
- **FR-010**: Si no hay secreto, el indicador MUST mostrar "inactiva", MUST informar que falta el secreto y la aplicación MUST NOT reintentar sola.
- **FR-011**: Una contraseña rechazada MUST dejar el indicador en "inactiva", MUST informar el error y MUST NOT quedar guardada para reintentos automáticos.
- **FR-012**: La consola MUST poder usarse en cuanto el servidor acepta al operador.
- **FR-013**: La consola MUST ajustar su tamaño al de la ventana.
- **FR-014**: Cada servidor guardado MUST tener su propia conexión y su propia consola.
- **FR-015**: Durante el uso, la conexión MUST mantenerse activa sin una acción del operador ante una inactividad breve.
- **FR-016**: Tras un corte, la aplicación MUST reintentar hasta conectar o hasta que el operador cierre.
- **FR-017**: La espera entre reintentos MUST crecer y MUST NOT superar los 30 segundos.
- **FR-018**: Un reintento con secreto guardado MUST NOT pedir usuario ni contraseña.
- **FR-019**: Cada servidor MUST mostrar un indicador visible con exactamente uno de estos valores: "activa", "inactiva" o "reintentando".
- **FR-020**: Cerrar la conexión MUST dejar el indicador en "inactiva" y MUST detener los reintentos.
- **FR-021**: Si el servidor no se alcanza y el operador no cierra, el indicador MUST permanecer en "reintentando" con los mismos reintentos.
- **FR-022**: La primera conexión exitosa MUST recordar la identidad del servidor.
- **FR-023**: Si la identidad cambia, el indicador MUST quedar "inactiva", la conexión MUST cortarse y MUST NOT continuar hasta que el operador lo confirme.
- **FR-028**: El indicador MUST mostrar "activa" solo cuando la consola se puede usar, y "reintentando" solo mientras hay reintentos en curso.
- **FR-024**: Si el servidor tiene tmux, al conectar MUST crearse o retomarse una sesión llamada `sshuttle`.
- **FR-025**: Si el servidor no tiene tmux, al reconectar MUST abrirse una consola nueva.
- **FR-026**: Un trabajo interrumpido por un corte MUST NOT reanudarse solo, y el operador MUST poder ver que quedó interrumpido.
- **FR-027**: Este hito MUST NOT incluir gestión de archivos, actualizaciones de paquetes, pods ni asistente.

### Key Entities

- **Servidor guardado**: Nombre visible, host, puerto, usuario, identidad recordada del servidor y la referencia al secreto. No contiene la contraseña.
- **Secreto**: La contraseña aceptada por el servidor, guardada en el llavero del sistema y asociada a un solo servidor.
- **Consola**: La sesión en la que el operador escribe. Muestra un indicador de conexión (activa, inactiva o reintentando) y, si el servidor tiene tmux, está ligada a la sesión `sshuttle`.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Un operador que conoce host, usuario y contraseña llega a una consola usable en un solo recorrido, sin pantallas de configuración adicionales.
- **SC-002**: Después de guardar la contraseña, el 100% de las reconexiones automáticas ocurren sin pedir usuario ni contraseña.
- **SC-003**: Ninguna espera entre reintentos dura más de 30 segundos.
- **SC-004**: En un servidor con recuperación de sesión, el operador vuelve a ver el contenido de la consola previa después de un corte. En uno sin esa recuperación, ve una consola nueva y puede escribir.
- **SC-005**: Cuando la identidad del servidor cambió, la consola permanece desconectada hasta una confirmación explícita en el 100% de los casos.
- **SC-006**: En el 100% de los servidores de la lista, el operador ve un indicador que dice "activa", "inactiva" o "reintentando", sin consultar registros.

## Assumptions

- Hay un solo operador, en su propia computadora, frente a un servidor que ya acepta entrada por usuario y contraseña.
- La clave SSH y el agente de claves quedan fuera de este hito. El acceso de este recorrido es usuario y contraseña.
- El descubrimiento general del servidor (paquetes, Kubernetes, archivos) queda fuera. Este hito solo distingue si el servidor puede recuperar la sesión de consola.
- En el teléfono no hay un proceso permanente en segundo plano. La reconexión vale con la aplicación visible o al volver a abrirla.
- La rama de trabajo prevista es `feature/consola-reconexion`, salida de `develop`. Esta especificación no crea esa rama.
