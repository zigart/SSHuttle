type Props = {
  onConfirm: () => void;
};

export function HostKeyConfirm({ onConfirm }: Props) {
  return (
    <div className="huella" role="alertdialog" aria-labelledby="huella-titulo">
      <h2 id="huella-titulo">Confirmar identidad</h2>
      <p>La huella del servidor no coincide con la guardada. La consola sigue cerrada hasta que la confirmes.</p>
      <button type="button" onClick={onConfirm}>
        Aceptar esta identidad
      </button>
    </div>
  );
}
