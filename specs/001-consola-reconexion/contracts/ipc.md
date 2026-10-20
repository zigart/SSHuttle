# Contrato de interfaz: consola

La UI no abre SSH. Llama a estos comandos y escucha estos eventos. Ningún argumento ni evento lleva la contraseña de vuelta hacia la UI después de enviarla.

## Comandos

### listar_servidores

Devuelve los perfiles guardados: id, nombre visible, host, puerto, usuario, si hay secreto y la huella si existe. No devuelve la contraseña.

### guardar_servidor

Crea o edita nombre visible, host, puerto y usuario. El puerto omitido se guarda como 22. No pide ni guarda capacidades del servidor.

### conectar

Argumentos: id del servidor y, solo en el primer éxito, la contraseña. Si ya hay secreto, la contraseña se omite. Empieza el intento y publica el indicador.

### cerrar

Argumentos: id del servidor. Pasa el indicador a `inactiva` y cancela los reintentos.

### confirmar_huella

Argumentos: id del servidor. Reemplaza la huella guardada por la presentada y permite un intento nuevo. Sin esta confirmación, una huella distinta no conecta.

### escribir_consola

Argumentos: id del servidor y bytes que el operador escribió. Solo tiene efecto con indicador `activa`.

### redimensionar_consola

Argumentos: id del servidor, columnas y filas. Se aplican al PTY activo.

## Eventos

### indicador

`id`, valor `activa` | `inactiva` | `reintentando`, y un motivo opcional cuando es `inactiva` (`falta_secreto`, `clave_rechazada`, `huella_distinta`, `cerrada`).

### salida_consola

`id` y bytes de salida del PTY. No se usa para mostrar la contraseña guardada.

### trabajo_interrumpido

`id`. Se emite una vez al volver sin tmux, después de un corte. No se emite si se reenganchó la sesión `sshuttle`.
