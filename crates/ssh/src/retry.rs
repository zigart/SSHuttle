use std::time::Duration;

/// Espera 1, 2, 4, 8, 16 y después se queda en 30 segundos.
#[derive(Debug, Clone)]
pub struct Espera {
    paso: u32,
}

impl Espera {
    pub fn nueva() -> Self {
        Self { paso: 0 }
    }

    pub fn reiniciar(&mut self) {
        self.paso = 0;
    }

    pub fn siguiente(&mut self) -> Duration {
        let segundos = match self.paso {
            0 => 1,
            1 => 2,
            2 => 4,
            3 => 8,
            4 => 16,
            _ => 30,
        };
        if self.paso < 5 {
            self.paso += 1;
        }
        Duration::from_secs(segundos)
    }
}

impl Default for Espera {
    fn default() -> Self {
        Self::nueva()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crece_hasta_30_y_se_queda() {
        let mut espera = Espera::nueva();
        let segundos: Vec<u64> = (0..7).map(|_| espera.siguiente().as_secs()).collect();
        assert_eq!(segundos, vec![1, 2, 4, 8, 16, 30, 30]);
        espera.reiniciar();
        assert_eq!(espera.siguiente().as_secs(), 1);
    }
}
