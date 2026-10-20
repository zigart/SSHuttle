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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solo_hay_tres_valores_visibles() {
        let valores = [Indicador::Activa, Indicador::Inactiva, Indicador::Reintentando]
            .map(|valor| serde_json::to_string(&valor).unwrap());
        assert_eq!(valores, [r#""activa""#, r#""inactiva""#, r#""reintentando""#]);
    }

    #[test]
    fn el_corte_reintenta_y_la_autenticacion_activa() {
        assert_eq!(tras_corte_de_red().valor, Indicador::Reintentando);
        assert!(tras_corte_de_red().motivo.is_none());
        assert_eq!(tras_autenticacion(), EstadoIndicador::activa());
        assert_eq!(
            EstadoIndicador::inactiva(MotivoInactiva::FaltaSecreto).valor,
            Indicador::Inactiva
        );
    }
}
