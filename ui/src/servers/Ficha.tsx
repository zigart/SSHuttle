import type { ValorIndicador } from "../console/Indicator";
import { iniciales } from "./iniciales";

type Props = {
  id: string;
  nombreVisible: string;
  valor: ValorIndicador;
  elegida: boolean;
  onElegir: (id: string) => void;
};

export function Ficha({ id, nombreVisible, valor, elegida, onElegir }: Props) {
  const texto = iniciales(nombreVisible);
  return (
    <button
      type="button"
      className={elegida ? "ficha elegida" : "ficha"}
      onClick={() => onElegir(id)}
      aria-pressed={elegida}
      aria-label={nombreVisible}
      title={nombreVisible}
    >
      <span className="ficha-iniciales">{texto}</span>
      <span className={`ficha-punto ficha-punto-${valor}`} aria-hidden="true" />
    </button>
  );
}
