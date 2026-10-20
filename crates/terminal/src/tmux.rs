pub const SONDA: &str = "command -v tmux";
pub const SESION: &str = "tmux new-session -A -s sshuttle";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn los_comandos_son_los_de_la_spec() {
        assert_eq!(SONDA, "command -v tmux");
        assert_eq!(SESION, "tmux new-session -A -s sshuttle");
    }
}
