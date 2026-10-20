# Data model: Barra de fichas

No hay tablas ni secretos nuevos. Las entidades son de presentación sobre el servidor guardado y el indicador ya existentes.

## Servidor guardado

Sin cambio de esquema. Campos que la barra usa:

| Campo | Uso en la barra |
| --- | --- |
| `id` | Identidad de la ficha y de la selección |
| `nombre_visible` | Fuente de las iniciales y del nombre completo al elegir |
| `host`, `puerto`, `usuario` | Formulario al editar / reconectar (igual que hoy) |
| `tiene_secreto` | Si el formulario pide contraseña (igual que hoy) |

## Ficha

Representación de un servidor en la barra.

| Campo derivado | Regla |
| --- | --- |
| `iniciales` | Ver [contracts/ui.md](contracts/ui.md): dos letras en mayúscula desde `nombre_visible` |
| `estado` | `activa` \| `inactiva` \| `reintentando`; por defecto `inactiva` si aún no llegó evento |
| `elegida` | `true` solo si `id === seleccion` |

Relación: una ficha ↔ un servidor. Dos servidores con las mismas iniciales son dos fichas distintas.

## Casillero vacío

No es un servidor. No tiene `id`, iniciales ni punto de estado.

- Hay exactamente uno en la barra, al final.
- Elegirlo: `seleccion = null`, formulario de alta, ninguna ficha marcada.

## Punto de estado

Valor visible en la ficha. Transiciones: las del evento `indicador` del supervisor. Sin estados extra.

| Valor | Significado (igual que la consola) |
| --- | --- |
| `activa` | Sesión autenticada |
| `inactiva` | Sin sesión / cerrada / falta secreto / etc. |
| `reintentando` | Reconexión en curso |

## Selección

| Estado de UI | Barra | Panel |
| --- | --- | --- |
| Ficha elegida | Esa ficha destacada | Nombre completo + formulario + eliminar + consola |
| Casillero / alta | Ninguna ficha destacada | Formulario de alta (pide contraseña) |
| Tras eliminar confirmado | Selección limpia; solo casillero si no quedan fichas | Alta o vacío según queden servidores |
