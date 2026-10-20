# Contrato de interfaz: eliminar una conexión

Se suma al contrato de la consola. La UI no abre SSH. Este comando no recibe ni devuelve la contraseña.

## Comandos

### eliminar_servidor

Argumentos: `id` del servidor.

La UI lo llama solo después de que el operador confirma. Cancelar no lo llama.

Efecto, en este orden:

1. Cierra la sesión de ese id, si hay una, y espera a que terminen los reintentos.
2. Borra la contraseña del llavero. Si no había contraseña, sigue. Si el llavero no responde, no borra el perfil y devuelve el error para mostrarlo.
3. Borra el perfil. Los otros perfiles quedan.

Si el id ya no existe, termina bien. La UI vuelve a pedir `listar_servidores`.

No pide paquetes, Kubernetes ni tmux. No envía nada al host remoto.

## Eventos

No hay un evento nuevo. El cierre de sesión puede publicar `indicador` `inactiva` con motivo `cerrada` para ese id. La lista visible sale de `listar_servidores`, no de ese evento.
