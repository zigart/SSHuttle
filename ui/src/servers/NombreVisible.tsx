import { FormEvent, useEffect, useState } from "react";

type Props = {
  valor: string;
  enviando: boolean;
  onGuardar: (nombre: string) => void;
};

/** Campo editable del nombre visible de la conexión elegida (FR-003). */
export function NombreVisible({ valor, enviando, onGuardar }: Props) {
  const [texto, setTexto] = useState(valor);

  useEffect(() => {
    setTexto(valor);
  }, [valor]);

  function enviar(evento: FormEvent) {
    evento.preventDefault();
    const limpio = texto.trim();
    if (limpio.length === 0 || limpio === valor) return;
    onGuardar(limpio);
  }

  return (
    <form className="nombre-elegido-form" onSubmit={enviar}>
      <label>
        Nombre
        <input
          value={texto}
          onChange={(evento) => setTexto(evento.target.value)}
          onBlur={() => {
            const limpio = texto.trim();
            if (limpio.length === 0) {
              setTexto(valor);
              return;
            }
            if (limpio !== valor) onGuardar(limpio);
          }}
          disabled={enviando}
          autoComplete="off"
          aria-label="Nombre visible"
        />
      </label>
    </form>
  );
}
