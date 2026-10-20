use serde::Serialize;

/// Únicos valores visibles del indicador. No hay un cuarto.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Indicador {
    Activa,
    Inactiva,
    Reintentando,
}

/// Acompaña a `inactiva`. No cambia el valor visible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MotivoInactiva {
    FaltaSecreto,
    ClaveRechazada,
    HuellaDistinta,
    Cerrada,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EstadoIndicador {
    pub valor: Indicador,
    pub motivo: Option<MotivoInactiva>,
}

impl EstadoIndicador {
    pub fn activa() -> Self {
        Self { valor: Indicador::Activa, motivo: None }
    }

    pub fn reintentando() -> Self {
        Self { valor: Indicador::Reintentando, motivo: None }
    }

    pub fn inactiva(motivo: MotivoInactiva) -> Self {
        Self { valor: Indicador::Inactiva, motivo: Some(motivo) }
    }
}

pub fn tras_corte_de_red() -> EstadoIndicador {
    EstadoIndicador::reintentando()
}

pub fn tras_autenticacion() -> EstadoIndicador {
    EstadoIndicador::activa()
}
