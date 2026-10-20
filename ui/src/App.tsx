import { useCallback, useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";

import { cerrar, confirmarHuella, conectar, guardarServidor, listarServidores, type Servidor } from "./api";
import { Console } from "./console/Console";
import { Indicator, esMotivo, esValorIndicador, type MotivoInactiva, type ValorIndicador } from "./console/Indicator";
import { ServerForm } from "./servers/ServerForm";

type EstadoConexion = {
  valor: ValorIndicador;
  motivo: MotivoInactiva | null;
};

const INACTIVA: EstadoConexion = { valor: "inactiva", motivo: null };

function enEscritorio(): boolean {
  return "__TAURI_INTERNALS__" in window || "__TAURI__" in window;
}

export function App() {
  const [servidores, setServidores] = useState<Servidor[]>([]);
  const [seleccion, setSeleccion] = useState<string | null>(null);
  const [estados, setEstados] = useState<Record<string, EstadoConexion>>({});
  const [interrumpidos, setInterrumpidos] = useState<Record<string, boolean>>({});
  const [error, setError] = useState<string | null>(null);
  const [enviando, setEnviando] = useState(false);

  const recargar = useCallback(async () => {
    const lista = await listarServidores();
    setServidores(lista);
    return lista;
  }, []);

  useEffect(() => {
    if (!enEscritorio()) return;
    void recargar().catch((fallo: unknown) => {
      setError(fallo instanceof Error ? fallo.message : "No se pudieron leer los servidores.");
    });
  }, [recargar]);

  useEffect(() => {
    if (!enEscritorio()) return;
    const indicador = listen<{ id: string; valor: string; motivo?: string | null }>("indicador", (evento) => {
      const valor = evento.payload.valor;
      if (!esValorIndicador(valor)) return;
      const motivo = evento.payload.motivo && esMotivo(evento.payload.motivo) ? evento.payload.motivo : null;
      setEstados((previo) => ({
        ...previo,
        [evento.payload.id]: { valor, motivo },
      }));
      if (evento.payload.valor === "activa") {
        void recargar().catch(() => undefined);
      }
    });
    const corte = listen<{ id: string }>("trabajo_interrumpido", (evento) => {
      setInterrumpidos((previo) => ({ ...previo, [evento.payload.id]: true }));
    });
    return () => {
      void indicador.then((dejar) => dejar());
      void corte.then((dejar) => dejar());
    };
  }, [recargar]);

  const actual = servidores.find((servidor) => servidor.id === seleccion) ?? null;
  const estado = seleccion ? (estados[seleccion] ?? INACTIVA) : INACTIVA;

  async function abrir(datos: { host: string; puerto?: number; usuario: string; contrasena?: string }) {
    setEnviando(true);
    setError(null);
    try {
      const guardado = await guardarServidor({
        id: actual?.id,
        host: datos.host,
        puerto: datos.puerto,
        usuario: datos.usuario,
      });
      setSeleccion(guardado.id);
      setInterrumpidos((previo) => ({ ...previo, [guardado.id]: false }));
      await conectar(guardado.id, datos.contrasena);
      await recargar();
    } catch (fallo) {
      setError(fallo instanceof Error ? fallo.message : "No se pudo abrir la consola.");
    } finally {
      setEnviando(false);
    }
  }

  return (
    <main className="app">
      <aside className="panel">
        <header className="marca">
          <h1>SSHuttle</h1>
          <p>Consola SSH</p>
        </header>
        <ul className="lista">
          {servidores.map((servidor) => {
            const item = estados[servidor.id] ?? INACTIVA;
            return (
              <li key={servidor.id}>
                <button
                  type="button"
                  className={servidor.id === seleccion ? "servidor activo" : "servidor"}
                  onClick={() => setSeleccion(servidor.id)}
                >
                  <span>{servidor.nombre_visible}</span>
                  <Indicator valor={item.valor} />
                </button>
              </li>
            );
          })}
        </ul>
        <button type="button" className="secundario" onClick={() => setSeleccion(null)}>
          Nuevo servidor
        </button>
        <ServerForm
          key={actual?.id ?? "nuevo"}
          hostInicial={actual?.host}
          puertoInicial={actual?.puerto}
          usuarioInicial={actual?.usuario}
          pedirContrasena={!actual?.tiene_secreto}
          enviando={enviando}
          onSubmit={(datos) => void abrir(datos)}
        />
        {error ? <p className="error">{error}</p> : null}
      </aside>
      {actual ? (
        <Console
          id={actual.id}
          valor={estado.valor}
          motivo={estado.motivo}
          interrumpido={Boolean(interrumpidos[actual.id])}
          onConfirmarHuella={() => void confirmarHuella(actual.id)}
          onCerrar={() => {
            setInterrumpidos((previo) => ({ ...previo, [actual.id]: false }));
            void cerrar(actual.id);
          }}
        />
      ) : (
        <section className="vacio">
          <p>Da de alta un servidor con host, puerto, usuario y contraseña.</p>
        </section>
      )}
    </main>
  );
}
