# Research: Consola con reconexión

## SSH y canal de consola

- **Decision**: `russh` abre la única sesión SSH y pide un PTY. `russh-sftp` no se usa en este hito.
- **Rationale**: La arquitectura fija Rust puro, compilable a iOS, y prohíbe `libssh2`. El PTY es el canal de la consola.
- **Alternatives considered**: `libssh2` (descartada por la arquitectura); ejecutar el binario `ssh` del sistema (no hay un solo supervisor ni indicador fiable).

## Keepalive y reintentos

- **Decision**: Keepalive cada 15 segundos. Tras un corte de red o un servidor inalcanzable, la espera es 1, 2, 4, 8, 16 y después 30 segundos, y se queda en 30. El indicador pasa a `reintentando` en el primer intento fallido de red.
- **Rationale**: La spec exige espera creciente con tope de 30 segundos y estado visible mientras reintenta. 15 segundos evita cortes por inactividad breve sin generar tráfico constante.
- **Alternatives considered**: Reintento fijo de 30 segundos (no crece). Reintento inmediato en bucle (satura el servidor y no cumple el tope como espera entre intentos).

## Cuándo no se reintenta

- **Decision**: No hay reintento automático si falta el secreto, si el servidor rechaza la contraseña o si la identidad del host cambió. Esos casos dejan el indicador en `inactiva` y muestran el motivo. El secreto se escribe en el llavero solo después de una autenticación aceptada.
- **Rationale**: La spec prohíbe reintentos silenciosos y guardar una contraseña rechazada.
- **Alternatives considered**: Reintentar también los fallos de autenticación (volvería a pedir o a martillar una clave mala).

## Indicador

- **Decision**: Un solo indicador por servidor, con los textos `activa`, `inactiva` y `reintentando`. `activa` solo con consola usable. `reintentando` solo con reintentos en curso. Todo lo demás es `inactiva`, con un mensaje aparte (falta secreto, clave rechazada, identidad cambiada, el operador cerró).
- **Rationale**: La spec exige exactamente esos tres valores visibles.
- **Alternatives considered**: Mostrar desconectado, conectando, listo y falta secreto como cuatro indicadores (lo pide la arquitectura de estados internos, pero la spec del hito lo reemplazó).

## Secreto y perfil

- **Decision**: SQLite guarda el perfil y un id de secreto. La contraseña vive en el llavero del sistema (`keyring`: Credential Manager, Secret Service, Keychain). Servicio `sshuttle`, cuenta igual al id. Los logs no imprimen la contraseña ni el contenido del llavero.
- **Rationale**: La constitución y la spec separan el perfil del secreto.
- **Alternatives considered**: Contraseña cifrada dentro de SQLite (un solo almacén, pero contradice el llavero del sistema).

## Identidad del host

- **Decision**: En la primera autenticación exitosa se guarda la huella SHA256 de la clave pública del servidor. Si una conexión posterior presenta otra huella, se corta, el indicador queda `inactiva` y no se sigue hasta que el operador confirme. Confirmar reemplaza la huella guardada y permite un intento nuevo.
- **Rationale**: Trust on first use, sin aceptar un host nuevo en silencio.
- **Alternatives considered**: Preguntar siempre, incluso la primera vez (agrega un paso que la spec no pide). Comparar la clave completa en vez de la huella (equivalente, peor de mostrar).

## tmux

- **Decision**: Después de autenticar, se consulta `command -v tmux`. Si existe, el PTY ejecuta `tmux new-session -A -s sshuttle`. Si no existe, el PTY abre el shell del usuario. La app no reenvía lo que el operador ya había escrito. Si no hay tmux, al volver se muestra que el trabajo quedó interrumpido. Si tmux reengancha `sshuttle`, se muestra esa sesión y no el aviso de interrupción.
- **Rationale**: La sesión recuperable es la de tmux. Sin tmux, una shell nueva no puede continuar un comando muerto con el canal. La app no relanza comandos por su cuenta.
- **Alternatives considered**: Sondeo completo de capacidades (paquetes, Kubernetes, SFTP). Queda fuera de este hito por la spec.

## Interfaz

- **Decision**: Tauri 2 expone comandos y eventos. La UI es React con xterm.js. El tamaño de la ventana se traduce a columnas y filas del PTY. Cada perfil tiene su propia sesión y su propio indicador.
- **Rationale**: Es la pila fijada en la arquitectura. xterm.js es el visor de la consola; no abre SSH.
- **Alternatives considered**: Terminal nativo por plataforma (duplica Windows, Linux e iOS).

## Teléfono

- **Decision**: Este hito no crea un proceso en segundo plano. En iOS la reconexión corre con la app visible o al volver a primer plano. Los instaladores quedan fuera.
- **Rationale**: La spec lo acota. El empaquetado es la fase 8.
- **Alternatives considered**: Un demonio que reintente con la app suspendida (iOS no lo permite y la spec lo excluye).
