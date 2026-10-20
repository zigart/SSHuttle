import { FormEvent, useState } from "react";

type Props = {
  hostInicial?: string;
  puertoInicial?: number;
  usuarioInicial?: string;
  pedirContrasena: boolean;
  enviando: boolean;
  onSubmit: (datos: { host: string; puerto?: number; usuario: string; contrasena?: string }) => void;
};

export function ServerForm({
  hostInicial = "",
  puertoInicial,
  usuarioInicial = "",
  pedirContrasena,
  enviando,
  onSubmit,
}: Props) {
  const [host, setHost] = useState(hostInicial);
  const [puerto, setPuerto] = useState(puertoInicial ? String(puertoInicial) : "");
  const [usuario, setUsuario] = useState(usuarioInicial);
  const [contrasena, setContrasena] = useState("");

  function enviar(evento: FormEvent) {
    evento.preventDefault();
    const puertoNumero = puerto.trim() === "" ? undefined : Number(puerto);
    const clave = contrasena;
    setContrasena("");
    onSubmit({
      host: host.trim(),
      puerto: puertoNumero,
      usuario: usuario.trim(),
      contrasena: pedirContrasena ? clave : undefined,
    });
  }

  return (
    <form className="alta" onSubmit={enviar}>
      <label>
        Host
        <input value={host} onChange={(evento) => setHost(evento.target.value)} autoComplete="off" required />
      </label>
      <label>
        Puerto
        <input
          value={puerto}
          onChange={(evento) => setPuerto(evento.target.value)}
          inputMode="numeric"
          placeholder="22"
          aria-describedby="ayuda-puerto"
        />
      </label>
      <p id="ayuda-puerto" className="ayuda">
        Si lo dejas vacío se guarda el 22.
      </p>
      <label>
        Usuario
        <input value={usuario} onChange={(evento) => setUsuario(evento.target.value)} autoComplete="username" required />
      </label>
      {pedirContrasena ? (
        <label>
          Contraseña
          <input
            type="password"
            value={contrasena}
            onChange={(evento) => setContrasena(evento.target.value)}
            autoComplete="new-password"
            required
          />
        </label>
      ) : null}
      <button type="submit" disabled={enviando}>
        {pedirContrasena ? "Abrir consola" : "Conectar"}
      </button>
    </form>
  );
}
