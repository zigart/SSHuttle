# Constitución de SSHuttle

## Core Principles

### I. Arquitectura vigente

`docs/arquitectura.md` es la fuente de verdad del producto. Una spec, un plan o una tarea que la contradiga no se implementa. Si una decisión de producto cambia, el mismo cambio actualiza ese documento antes de escribir código.

### II. Spec antes de código

Toda conducta nueva de producto pasa por `/speckit.specify`, `/speckit.plan` y `/speckit.tasks` antes de `/speckit.implement`. El código de una fase solo cubre tareas de su spec. Un arreglo de un comportamiento ya especificado se agrega como tarea de esa spec, sin abrir una fase nueva.

### III. Orden de fases

La implementación sigue este orden y no salta fases:

1. Inventario, llavero y supervisor con reconexión.
2. Sondeo automático y UI según las capacidades halladas.
3. Consola y tmux.
4. Archivos SFTP.
5. Actualizaciones apt y dnf.
6. Pods.
7. Asistente local y por API key.
8. Instaladores Windows, `.deb`, AppImage e iOS.

Queda fuera de v1, y no se especifica ni se codea: agente en el servidor, túnel al API de Kubernetes, aplicar `pacman`, `zypper` o `apk`, salto `ProxyJump`, Android, actualizaciones desatendidas y un modelo embebido.

### IV. Límites de seguridad

El alta de un servidor pide host, puerto, usuario y contraseña. El puerto sale en 22. Las capacidades las descubre el sondeo. Los secretos viven en el llavero del sistema. La base SQLite guarda solo el id del secreto. Los logs no imprimen contraseñas, frases de clave, API keys ni kubeconfig. Borrar un pod, aplicar paquetes o aceptar una acción mutable del asistente exige confirmación de la persona.

### V. Una sola vía de SSH

Solo el supervisor abre sockets SSH. Los crates de capacidades reciben un handle y no conectan por su cuenta. El asistente no habla SSH: llama a esos crates. `kubectl` y el asistente arman argumentos en código, desde una lista blanca. Una cadena de shell que venga de la UI o del modelo no se ejecuta.

## Restricciones de seguridad

- Cero agentes en el servidor. El transporte es SSH con el usuario que ya existe.
- El sondeo corre un conjunto fijo de comandos de solo lectura, con tope de tiempo. No hay scripts definidos por el usuario.
- La UI muestra solo las capacidades halladas. No hay pantallas para activar Kubernetes, elegir el gestor de paquetes ni marcar `tmux`.
- Kubernetes v1 usa `kubectl` en el host SSH si el sondeo lo encuentra. Operaciones permitidas: listar pods, logs, reiniciar (borrar el pod) y exec en la consola.
- Actualizaciones v1: el gestor detectado entre `apt` y `dnf`, con vista previa antes de aplicar. Otros gestores se muestran y no se aplican.
- El asistente permanece apagado hasta que la persona elige un proveedor. No hay modelo embebido. El modelo recibe el resumen del sondeo y un recorte de la pantalla, con tope de tamaño. No recibe contraseñas, claves privadas, kubeconfig ni tráfico SSH.
- Una huella de host distinta corta la conexión y pide confirmación. No se acepta el host nuevo en silencio.
- La reconexión autentica solo con el llavero. Una operación cortada a mitad queda interrumpida y la persona la relanza.

## Flujo de desarrollo

Cada fase es una unidad de trabajo con esta secuencia:

1. Especificar el comportamiento de cara a la persona, citando la fase de `docs/arquitectura.md`.
2. Planear los crates y la UI que esa fase toca. Los crates de capacidades no abren sockets.
3. Partir el plan en tareas verificables.
4. Implementar solo esas tareas.
5. Cerrar con `/speckit.converge`: el código cumple la spec y no incorpora nada de la lista de fuera de v1.

La pila es Tauri 2, núcleo Rust (`russh`, `russh-sftp`) y UI TypeScript con React. Una spec no cambia esa pila.

## Ramas

`develop` es la rama default. `main` es producción. El trabajo no se commitea en `main`.

- `feature/<nombre>` sale de `develop` y se integra solo en `develop`.
- `release/<version>` sale de `develop` cuando una versión está lista. Se integra en `main` y después en `develop`.
- `hotfix/<nombre>` sale de `main` para corregir producción. Se integra en `main` y después en `develop`.

No hay rama larga llamada `release` ni `feature`. No se hace force push ni se borra `main` ni `develop`.

## Governance

Esta constitución manda sobre specs, planes, tareas y código. `docs/arquitectura.md` manda sobre el comportamiento del producto. Si chocan, se detiene el trabajo y se enmienda uno de los dos en el mismo cambio.

Una enmienda sube la versión: MAJOR si se elimina o se redefine un principio, MINOR si se agrega un principio o una sección, PATCH si solo se aclara el texto. La fecha de ratificación no cambia. La fecha de última enmienda es el día del cambio.

Cada spec y cada plan se revisan contra los cinco principios antes de implementar. Una desviación se escribe en la spec y se justifica, o no se implementa.

**Version**: 1.1.0 | **Ratified**: 2026-10-20 | **Last Amended**: 2026-10-20
