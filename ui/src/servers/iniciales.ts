/** Iniciales del nombre visible para la barra de fichas. Ver contracts/ui.md. */
export function iniciales(nombreVisible: string): string {
  const palabras = nombreVisible.trim().split(/\s+/).filter((p) => p.length > 0);
  if (palabras.length === 0) {
    return "";
  }
  let crudo: string;
  if (palabras.length >= 2) {
    crudo = `${palabras[0]![0] ?? ""}${palabras[1]![0] ?? ""}`;
  } else {
    crudo = palabras[0]!.slice(0, 2);
  }
  return crudo.toLocaleUpperCase();
}
