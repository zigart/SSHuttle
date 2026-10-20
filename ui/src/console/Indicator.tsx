export const VALORES_INDICADOR = ["activa", "inactiva", "reintentando"] as const;

export type ValorIndicador = (typeof VALORES_INDICADOR)[number];

export type MotivoInactiva = "falta_secreto" | "clave_rechazada" | "huella_distinta" | "cerrada";

const MOTIVOS: Record<MotivoInactiva, string> = {
  falta_secreto: "Falta el secreto en el llavero. No se reintenta.",
  clave_rechazada: "La contraseña fue rechazada y no se guardó.",
  huella_distinta: "La identidad del servidor cambió.",
  cerrada: "Conexión cerrada.",
};

export function esValorIndicador(valor: string): valor is ValorIndicador {
  return (VALORES_INDICADOR as readonly string[]).includes(valor);
}

export function esMotivo(valor: string): valor is MotivoInactiva {
  return valor in MOTIVOS;
}

type Props = {
  valor: ValorIndicador;
  motivo?: MotivoInactiva | null;
};

export function Indicator({ valor, motivo }: Props) {
  return (
    <p className={`indicador indicador-${valor}`}>
      <span className="punto" aria-hidden="true" />
      <span className="valor">{valor}</span>
      {valor === "inactiva" && motivo ? <span className="motivo">{MOTIVOS[motivo]}</span> : null}
    </p>
  );
}
