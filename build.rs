use std::{
    fs::create_dir_all,
    process::{Command, Stdio},
};

fn main() {
    create_dir_all(".venv").expect("failed to create dir");

    Command::new("python3")
        .args(&["-m", "venv", ".venv"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn process");

    get_venv_command("python")
        .args(&["-m", "pip", "install", "-r", "dbt"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn process");
}

fn get_venv_command(program: &str) -> Command {
    Command::new(format!(".venv/bin/{}", program))
}
