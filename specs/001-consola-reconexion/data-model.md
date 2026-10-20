# Data model: Consola con reconexión

## Servidor guardado

Persistido en SQLite. No incluye la contraseña.

| Campo | Regla |
| --- | --- |
| id | Identificador estable del perfil |
| nombre_visible | Texto editable. Al crear, se copia el host |
| host | Obligatorio, sin espacios vacíos |
| puerto | Entero 1–65535. Si el operador no lo cambia, 22 |
| usuario | Obligatorio |
| secreto_id | Id en el llavero. Vacío hasta que el servidor acepta la contraseña |
| huella_host | SHA256 de la clave pública del servidor. Vacía hasta la primera autenticación exitosa |

Un perfil sobrevive al cierre de la aplicación. Dos perfiles no comparten conexión ni consola.

## Secreto

No se persiste en SQLite. El llavero del sistema guarda la contraseña con servicio `sshuttle` y cuenta `secreto_id`.

Se escribe solo después de que el servidor acepta la autenticación. Una contraseña rechazada no se guarda. Borrar la entrada del llavero deja `secreto_id` apuntando a un secreto ausente: el indicador pasa a `inactiva` y se informa que falta el secreto.

## Indicador

Un valor por servidor. No hay un cuarto valor visible.

| Indicador | Cuándo |
| --- | --- |
| inactiva | Aún no se conectó, el operador cerró, falta el secreto, la contraseña fue rechazada o la huella no coincide y espera confirmación |
| reintentando | El servidor no se alcanza o la red se cortó, y el operador no cerró. Hay un intento en curso o una espera entre intentos |
| activa | La autenticación fue aceptada y la consola acepta escritura |

Transiciones:

- inactiva → reintentando: el operador pide conectar y hay secreto, o el primer intento no alcanza al servidor.
- inactiva → activa: el servidor acepta usuario y contraseña y la huella es nueva o coincide.
- activa → reintentando: se corta la red o el socket.
- reintentando → activa: un intento autentica.
- reintentando → inactiva: el operador cierra, falta el secreto, la contraseña es rechazada o la huella cambió.
- activa → inactiva: el operador cierra.

Cerrar detiene el temporizador de reintento. La espera siguiente es 1, 2, 4, 8, 16 y luego 30 segundos.

## Consola

Existe una consola por servidor mientras la sesión está `activa` o acaba de volver.

- Columnas y filas siguen a la ventana.
- Si `tmux` está en el servidor, el proceso del PTY es `tmux new-session -A -s sshuttle`.
- Si no está, el proceso es el shell del usuario.
- La app no vuelve a enviar el texto que el operador escribió antes del corte.
- Sin tmux, al recuperar la red se muestra que el trabajo quedó interrumpido.
- Con tmux, se muestra la sesión `sshuttle` y no ese aviso.

## Mensajes que acompañan a `inactiva`

No cambian el indicador. Dicen por qué no hay conexión: falta el secreto, la contraseña fue rechazada, hay que confirmar la identidad nueva, o el operador cerró.
