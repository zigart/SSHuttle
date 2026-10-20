# Quickstart: Eliminar una conexión

Validación manual sobre la consola ya integrada. Hace falta un servidor SSH de prueba solo para el caso con sesión abierta. Eliminar no lo modifica.

## Preparar

1. Estar en la rama `feature/eliminar-conexion`, actualizada con `develop`.
2. Arrancar la app de escritorio.
3. Tener dos servidores en la lista. Uno puede no haberse conectado nunca.

## Quitar uno y cancelar el otro

1. Pedir eliminar el primer servidor y cancelar.
2. Confirmar que sigue en la lista, con su host, puerto y usuario.
3. Pedir eliminar el segundo y confirmar.
4. Confirmar que ese desaparece y el primero sigue igual.
5. Confirmar que no se preguntó por paquetes, Kubernetes ni tmux.
6. Cerrar y abrir la app. Solo el primero está en la lista.

## Sesión y contraseña

1. Conectar el servidor que queda hasta que el indicador diga `activa`.
2. Eliminarlo y confirmar.
3. Confirmar que la consola se cierra y no pasa a `reintentando`.
4. Dar de alta el mismo host y usuario. La app pide la contraseña otra vez.
5. Repetir el alta, conectar, cortar la red hasta ver `reintentando` y eliminar confirmando. El servidor desaparece y no sigue intentando.
6. Guardar un servidor sin llegar a conectar y eliminarlo. Desaparece aunque no tenga contraseña.
