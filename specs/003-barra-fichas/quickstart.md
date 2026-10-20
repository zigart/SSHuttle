# Quickstart: Barra de fichas

Validación manual sobre la consola ya integrada. No hace falta un servidor SSH para ver fichas e iniciales; sí para comprobar el punto `activa` / `reintentando`.

## Preparar

1. Estar en la rama `feature/barra-fichas`, actualizada con `develop`.
2. Arrancar la app de escritorio.
3. Tener al menos tres servidores guardados (pueden no haberse conectado). Uno con nombre de una palabra y otro con dos (p. ej. host `bastion` y otro renombrado o con host de dos tokens si aplica vía el nombre visible).

## Fichas y selección

1. Abrir la app: la izquierda es una barra de fichas, no una lista de nombres completos.
2. Cada ficha muestra dos iniciales en mayúscula y un punto (por defecto inactiva).
3. Elegir una ficha: solo esa queda destacada; el panel muestra el nombre visible completo.
4. Confirmar que no hay íconos de inicio, ajustes ni un listado aparte en esa barra.

## Casillero de alta

1. Elegir el casillero vacío al final: ninguna ficha queda marcada.
2. Completar host, usuario y contraseña; dejar el puerto vacío.
3. Confirmar que no se pide paquetes, Kubernetes ni tmux.
4. Tras guardar: aparece una ficha nueva con sus iniciales y el puerto efectivo es 22.

## Estado y eliminación

1. Conectar una ficha hasta `activa`: el punto de esa ficha pasa a activa sin reiniciar la app.
2. Elegir otra ficha, pedir eliminar y cancelar: la cantidad de fichas no cambia.
3. Pedir eliminar y confirmar: esa ficha desaparece; las demás siguen.
4. Con lista vacía: solo se ve el casillero vacío.
