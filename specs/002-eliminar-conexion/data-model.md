# Data model: Eliminar una conexión

No hay entidades nuevas. Se quitan un perfil y, si existe, su secreto.

## Servidor guardado

La fila es la de la consola con reconexión: `id`, `nombre_visible`, `host`, `puerto`, `usuario`, `secreto_id`, `huella_host`. Sin columna de contraseña.

Al confirmar la eliminación:

- Esa fila desaparece.
- Las demás filas no cambian.
- Al reabrir la aplicación la fila sigue ausente.
- Volver a dar de alta el mismo host crea un perfil nuevo, con `secreto_id` vacío, y la UI pide la contraseña.

Si el llavero no puede borrar el secreto, la fila no se toca.

## Secreto

Sigue en el llavero, servicio `sshuttle`, cuenta `secreto_id`.

- Con `secreto_id` vacío no hay entrada que borrar.
- Con entrada presente, se borra sin leerla.
- Si la entrada ya no está, se considera borrada.
- Si el llavero rechaza el borrado, la entrada y la fila quedan.

## Sesión

Una sesión por `id`, ya definida por el supervisor.

- Confirmar manda cerrar. El lazo termina: no queda `activa` ni `reintentando` para ese id, aunque en ese instante se caiga la red o haya una pregunta de huella pendiente.
- El comando espera ese fin antes de borrar el secreto.
- Cancelar no envía cierre: la sesión sigue como estaba.
- Si el llavero falla después del cierre, la sesión ya está cerrada y el perfil sigue. El operador puede volver a conectar con la contraseña que todavía está guardada.

## Lista vacía

Borrar el único servidor deja la lista vacía. El alta de uno nuevo sigue disponible.
