# Contrato de interfaz: barra de fichas

No se agregan comandos IPC. La UI sigue usando `listar_servidores`, `guardar_servidor`, `conectar`, `eliminar_servidor` y el evento `indicador` como en la consola y en eliminar-conexión.

## Barra

- Contenedor vertical a la izquierda (o rail equivalente).
- Una ficha por servidor de `listar_servidores`, en el orden de la lista.
- Un solo casillero vacío al final, siempre visible.
- Sin controles de inicio, ajustes ni un listado distinto de fichas + casillero.

## Ficha

Entrada: `id`, `nombre_visible`, valor de indicador para ese `id` (o `inactiva`), y si está elegida.

Salida visual:

- Iniciales según la función de abajo.
- Punto de estado con exactamente `activa` | `inactiva` | `reintentando`.
- Estilo distinto solo si está elegida.
- Al activarla: fija `seleccion` a ese `id` y muestra el `nombre_visible` completo fuera de la ficha (panel).

## Casillero vacío

- No llama a inventario al elegirlo.
- Pone la UI en alta nueva: ninguna ficha elegida, formulario con host, puerto, usuario y contraseña.
- Tras un alta exitosa, la barra incluye la ficha nueva (vía `listar_servidores`) y esa conexión queda elegida.

## Eliminar

Sin cambio de contrato: confirmación en UI; Cancelar no invoca `eliminar_servidor`; Confirmar sí. La ficha desaparece al volver a listar.

## Iniciales

Función pura `iniciales(nombre_visible: string) -> string`:

1. `trim` y partir por espacios (`/\s+/`), descartar segmentos vacíos.
2. Si `palabras.length >= 2`: primer carácter de `palabras[0]` y de `palabras[1]`.
3. Si `palabras.length === 1`: hasta dos primeros caracteres de esa palabra.
4. Si no hay palabras: cadena vacía (no debería pintarse una ficha sin nombre).
5. Resultado en mayúsculas (`toLocaleUpperCase` o equivalente del locale de la UI).

Ejemplos:

| `nombre_visible` | Iniciales |
| --- | --- |
| `bastion prod` | `BP` |
| `bastion` | `BA` |
| `a` | `A` |
| `  web  api  ` | `WA` |

## Eventos

El punto se actualiza con el evento `indicador` existente (`id`, `valor`). No hay evento nuevo para la barra.
