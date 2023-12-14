use std::process::{Command, Stdio};

fn main() {
    Command::new("python3")
        .args(&["-m", "venv", ".venv"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn process")
        .wait()
        .expect("failed to wait on child");

    get_venv_command("python")
        .args(&["-m", "pip", "install", "dbt-core"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn process")
        .wait()
        .expect("failed to wait on child");
}

fn get_venv_command(program: &str) -> Command {
    Command::new(format!(".venv/bin/{}", program))
}
