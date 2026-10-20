import { FitAddon } from "@xterm/addon-fit";
import { Terminal } from "@xterm/xterm";
import "@xterm/xterm/css/xterm.css";
import { useEffect, useRef } from "react";
import { listen } from "@tauri-apps/api/event";

import { redimensionarConsola, escribirConsola } from "../api";
import { HostKeyConfirm } from "./HostKeyConfirm";
import { Indicator, type MotivoInactiva, type ValorIndicador } from "./Indicator";

type Props = {
  id: string;
  valor: ValorIndicador;
  motivo?: MotivoInactiva | null;
  interrumpido: boolean;
  onConfirmarHuella: () => void;
  onCerrar: () => void;
};

export function Console({ id, valor, motivo, interrumpido, onConfirmarHuella, onCerrar }: Props) {
  const contenedor = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const nodo = contenedor.current;
    if (!nodo) return;
    const term = new Terminal({
      cursorBlink: true,
      fontFamily: "ui-monospace, SFMono-Regular, Menlo, Consolas, monospace",
      fontSize: 14,
      theme: {
        background: "#1c1915",
        foreground: "#f3ead7",
        cursor: "#e8a87c",
        selectionBackground: "#4a3f32",
      },
    });
    const fit = new FitAddon();
    term.loadAddon(fit);
    term.open(nodo);
    fit.fit();

    const datos = term.onData((texto) => {
      const bytes = Array.from(new TextEncoder().encode(texto));
      void escribirConsola(id, bytes);
    });
    const tamano = term.onResize(({ cols, rows }) => {
      void redimensionarConsola(id, cols, rows);
    });
    void redimensionarConsola(id, term.cols, term.rows);

    const observador = new ResizeObserver(() => fit.fit());
    observador.observe(nodo);

    const salida = listen<{ id: string; datos: number[] }>("salida_consola", (evento) => {
      if (evento.payload.id !== id) return;
      term.write(new Uint8Array(evento.payload.datos));
    });

    return () => {
      datos.dispose();
      tamano.dispose();
      observador.disconnect();
      term.dispose();
      void salida.then((dejar) => dejar());
    };
  }, [id]);

  return (
    <section className="consola">
      <header className="consola-barra">
        <Indicator valor={valor} motivo={motivo} />
        <button type="button" className="secundario" onClick={onCerrar}>
          Cerrar conexión
        </button>
      </header>
      {interrumpido ? (
        <p className="aviso" role="status">
          El trabajo quedó interrumpido. Esta consola es nueva y no continúa el comando anterior.
        </p>
      ) : null}
      {motivo === "huella_distinta" ? <HostKeyConfirm onConfirm={onConfirmarHuella} /> : null}
      <div className="terminal" ref={contenedor} />
    </section>
  );
}
