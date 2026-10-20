use sshuttle_terminal::{SESION, SONDA};

#[test]
fn los_comandos_son_los_de_la_spec() {
    assert_eq!(SONDA, "command -v tmux");
    assert_eq!(SESION, "tmux new-session -A -s sshuttle");
}
