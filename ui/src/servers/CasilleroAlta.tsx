type Props = {
  elegido: boolean;
  onElegir: () => void;
};

/** Casillero vacío al final de la barra: abre el alta, no es un servidor. */
export function CasilleroAlta({ elegido, onElegir }: Props) {
  return (
    <button
      type="button"
      className={elegido ? "casillero-alta elegido" : "casillero-alta"}
      onClick={onElegir}
      aria-pressed={elegido}
      aria-label="Agregar conexión"
      title="Agregar conexión"
    >
      <span aria-hidden="true">+</span>
    </button>
  );
}
