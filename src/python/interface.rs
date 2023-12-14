use tokio::io::AsyncBufReadExt as _;

pub fn dbt(args: Vec<String>) -> tokio::process::Child {
    log::warn!("Extra args: {:?}", args.clone());

    spawn("dbt", args)
}

pub fn dbt_run(command: &str, args: Vec<String>) {
    let args = vec![vec![command.to_string()], args].concat();
    let runtime = get_runtime();

    runtime.block_on(async move { crate::python::interface::wrap_around(args).await });
}

fn get_runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Runtime::new().expect("failed to create runtime")
}

async fn wrap_around(args: Vec<String>) {
    let mut process = dbt(args);

    let stdout = process.stdout.take().expect("failed to get stdout");
    let reader = tokio::io::BufReader::new(stdout).lines();

    tokio::pin!(reader);
    while let Some(line) = reader.next_line().await.expect("failed to read line") {
        log::info!("{}", line);
    }
}

fn spawn(program: &str, args: Vec<String>) -> tokio::process::Child {
    tokio::process::Command::new(format!(".venv/bin/{}", program))
        .args(args)
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn process")
}
