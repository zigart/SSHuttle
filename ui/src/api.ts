import { invoke } from "@tauri-apps/api/core";

export type Servidor = {
  id: string;
  nombre_visible: string;
  host: string;
  puerto: number;
  usuario: string;
  tiene_secreto: boolean;
  huella_host: string | null;
};

export type Alta = {
  id?: string;
  host: string;
  puerto?: number;
  usuario: string;
};

export function listarServidores(): Promise<Servidor[]> {
  return invoke("listar_servidores");
}

export function guardarServidor(alta: Alta): Promise<Servidor> {
  return invoke("guardar_servidor", alta);
}

export function conectar(id: string, contrasena?: string): Promise<void> {
  return invoke("conectar", {
    id,
    contrasena: contrasena && contrasena.length > 0 ? contrasena : null,
  });
}

export function cerrar(id: string): Promise<void> {
  return invoke("cerrar", { id });
}

export function eliminarServidor(id: string): Promise<void> {
  return invoke("eliminar_servidor", { id });
}

export function confirmarHuella(id: string): Promise<void> {
  return invoke("confirmar_huella", { id });
}

export function escribirConsola(id: string, datos: number[]): Promise<void> {
  return invoke("escribir_consola", { id, datos });
}

export function redimensionarConsola(id: string, columnas: number, filas: number): Promise<void> {
  return invoke("redimensionar_consola", { id, columnas, filas });
}
