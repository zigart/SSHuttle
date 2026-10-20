# Research: Eliminar una conexión

## Orden del borrado

- **Decision**: Tras la confirmación, el supervisor cierra la sesión de ese id y el comando espera a que ese lazo termine. Después se borra la entrada del llavero. Solo si eso sale bien se borra la fila de SQLite. Si el llavero falla, la fila queda y la UI muestra que no se pudo eliminar.
- **Rationale**: El lazo de reconexión vuelve a leer el llavero en cada intento. Borrar el secreto con el lazo vivo puede dejar un reintento a medias. Borrar la fila antes que el secreto deja un secreto sin perfil, o al revés un perfil cuyo secreto ya no está. La spec pide que un fallo del llavero conserve el servidor.
- **Alternatives considered**: Borrar la fila y el secreto en paralelo (carrera con el reintento). Borrar el secreto primero y cerrar después (el lazo puede leer el secreto mientras se borra). Dejar la sesión abierta si el llavero falla (el lazo seguiría vivo y podría volver a leerlo).

## Secreto ausente

- **Decision**: `Entry::delete_credential` del llavero. `NoEntry` cuenta como éxito: el secreto ya no está. Un `secreto_id` vacío no llama al llavero. Cualquier otro error del llavero aborta el borrado del perfil.
- **Rationale**: Un servidor que nunca autenticó no tiene contraseña y la spec igual permite eliminarlo. Un segundo intento sobre un secreto ya borrado no debe fallar.
- **Alternatives considered**: Tratar `NoEntry` como fallo (impediría borrar un perfil cuyo secreto ya no está). Ignorar todo error del llavero (violaría la spec cuando el llavero no responde).

## Confirmación

- **Decision**: Confirmar y cancelar ocurren en la UI. Cancelar no invoca comandos. Confirmar llama a `eliminar_servidor` con el id. El núcleo no pide una segunda confirmación.
- **Rationale**: La spec exige un paso explícito antes de cambiar lista, sesión o contraseña. El núcleo no tiene forma de saber si la persona canceló si la UI no llama.
- **Alternatives considered**: Un comando `confirmar_eliminacion` aparte (dos viajes para un gesto local). `window.confirm` del navegador (fuera del estilo de la app y fácil de saltear en el escritorio).

## Servidor que ya no está

- **Decision**: Si el id no existe, el comando termina bien y no toca el llavero. La UI vuelve a listar.
- **Rationale**: La spec dice que pedirlo dos veces deja la lista sin ese servidor. No es un error para el operador.
- **Alternatives considered**: Devolver error de "no encontrado" (la lista ya cumple el resultado y el mensaje estorba).

## Registro

- **Decision**: El log anota la cuenta `secreto_id`, nunca la contraseña ni el contenido leído del llavero. El mismo criterio que al guardar.
- **Rationale**: La constitución prohíbe imprimir secretos. Borrar no necesita leer el valor.
- **Alternatives considered**: Registrar el host junto con el resultado (no hace falta para diagnosticar el llavero y mezcla datos del perfil en el log del secreto).
