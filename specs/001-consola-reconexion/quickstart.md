# Quickstart: Consola con reconexión

Validación manual del hito 1. Hace falta un servidor SSH de prueba que acepte usuario y contraseña. No hace falta Kubernetes ni un gestor de paquetes.

## Preparar

1. Estar en la rama `feature/consola-reconexion`, salida de `develop`.
2. Arrancar la app de escritorio.
3. Tener a mano un servidor alcanzable y, para el caso de tmux, otro o el mismo con `tmux` instalado.

## Alta y consola

1. Crear un servidor con host, usuario y contraseña. Dejar el puerto vacío.
2. Confirmar que el puerto quedó en 22 y el nombre visible es el host.
3. Confirmar que no se preguntó por paquetes, Kubernetes ni tmux.
4. Cuando el servidor acepta, escribir en la consola y ver la respuesta.
5. Confirmar que el indicador dice `activa`.
6. Cerrar y abrir la app. El servidor sigue en la lista.

## Secreto

1. Cerrar la conexión y volver a abrir el mismo servidor.
2. Confirmar que no pide usuario ni contraseña y el indicador vuelve a `activa`.
3. Repetir el alta contra un servidor con contraseña incorrecta. El indicador queda `inactiva`, se ve el error y un reintento no ocurre solo.
4. Quitar el secreto del llavero del sistema y conectar. El indicador queda `inactiva` e informa que falta el secreto.

## Corte de red

1. Con el indicador `activa`, cortar la red del servidor.
2. Confirmar que el indicador pasa a `reintentando` y no aparece el formulario de contraseña.
3. Restaurar la red. El indicador vuelve a `activa`.
4. Cortar otra vez y cerrar la conexión durante los reintentos. El indicador queda `inactiva` y no sigue intentando.

## Identidad

1. Conectar una vez y volver a conectar: entra sin preguntar.
2. Cambiar la clave de host del servidor de prueba.
3. Confirmar que el indicador queda `inactiva` y no hay consola hasta confirmar la identidad nueva.

## tmux

1. En un servidor con tmux, conectar, escribir texto y cortar la red.
2. Al volver, el texto de la sesión `sshuttle` sigue visible y no aparece el aviso de trabajo interrumpido.
3. En un servidor sin tmux, dejar un comando en curso, cortar y volver.
4. Hay una consola nueva, el comando no sigue solo y se ve que el trabajo quedó interrumpido.

## Queda fuera de esta validación

Archivos, actualizaciones, pods, asistente e instaladores.
