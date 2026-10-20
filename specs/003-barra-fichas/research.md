# Research: Barra de fichas

## Alcance: solo UI

- **Decision**: No se agregan comandos Tauri ni columnas en SQLite. La barra consume `listar_servidores`, el mapa de estados del evento `indicador`, y los flujos de alta y eliminación ya existentes.
- **Rationale**: La spec asume lista, consola y borrado con confirmación. Solo cambia cómo se ven y cómo se empieza el alta.
- **Alternatives considered**: Un comando que devuelva iniciales desde Rust (innecesario: el nombre visible ya llega a la UI). Guardar iniciales en la base (se desactualizan si cambia el nombre).

## Iniciales del nombre visible

- **Decision**: Función pura sobre `nombre_visible` recortado:
  1. Partir por espacios en blanco y descartar vacíos.
  2. Si hay dos o más palabras: primera letra de la primera y de la segunda.
  3. Si hay una sola palabra: las dos primeras letras (si tiene una sola letra, esa letra una vez o repetida según longitud disponible: una letra → una inicial; vacío no debería ocurrir porque el alta exige host).
  4. Mostrar siempre en mayúsculas.
- **Rationale**: Cubre los escenarios de aceptación de la spec sin depender del host por separado: el nombre visible ya copia el host al alta.
- **Alternatives considered**: Tres letras (la referencia visual no lo pide). Solo primera letra (demasiadas colisiones). Iniciales en el núcleo (acopla presentación al inventario).

## Punto de estado

- **Decision**: El punto de la ficha usa el mismo `ValorIndicador` (`activa` | `inactiva` | `reintentando`) que ya alimenta la consola. Sin cuarto valor. La ficha muestra solo el punto (compacto); el texto y el motivo siguen en el indicador de la consola.
- **Rationale**: La spec dice que el punto muestra el mismo valor, no uno nuevo. Reutilizar evita desync.
- **Alternatives considered**: Colores distintos sin mapear al evento (divergiría). Reusar el componente `Indicator` completo en cada ficha (ocupa el rail y muestra texto que la ficha no pide).

## Casillero vacío y selección

- **Decision**: Un solo control al final de la barra, distinto de las fichas. Elegirlo pone la selección en “alta nueva” (`seleccion === null`), sin marcar ninguna ficha como elegida, y muestra `ServerForm` pidiendo contraseña. El botón “Nuevo servidor” de la lista larga se retira: el casillero lo reemplaza.
- **Rationale**: FR-006 y FR-007. Evita dos entradas al alta.
- **Alternatives considered**: Varios casilleros vacíos como en la referencia (la spec los excluye). Mantener el botón “Nuevo servidor” además del casillero (duplica el gesto).

## Nombre visible completo

- **Decision**: Con una ficha elegida, el panel lateral (encima o junto al formulario) muestra `nombre_visible` completo. Las fichas no eligen muestran solo iniciales.
- **Rationale**: FR-005. Con iniciales colisionadas, la destacada más el nombre completo desambiguan.
- **Alternatives considered**: Tooltip al pasar el mouse (menos visible en escritorio táctil / sin hover). Solo en la consola (el operador puede no mirar el título del terminal).

## Arquitectura documentada

- **Decision**: En el mismo cambio de implementación, `docs/arquitectura.md` anota que la lista de conexiones se muestra como barra vertical de fichas con casillero de alta.
- **Rationale**: Constitución I: un cambio de presentación de producto se refleja en la arquitectura.
- **Alternatives considered**: Dejar solo la spec (la constitución exige actualizar el documento de arquitectura cuando cambia el producto).
