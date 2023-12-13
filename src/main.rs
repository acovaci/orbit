use std::process::Stdio;

use clap::{Parser, Subcommand};
use tokio::process::{Child, Command};

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run all seeds, models, snapshots, and tests in DAG order
    Build {},

    /// Cancel the most recent invocation
    Cancel {},

    /// Delete all folders in the clean-targets list (usually the dbt_packages and target directories.)
    Clean {},

    /// Create clones of selected nodes based on their location in the manifest provided to --state.
    Clone {},

    /// Generates executable SQL from source, model, test and analysis files.
    Compile {},

    /// Pull the most recent version of the dependencies listed in packages.yml
    Deps {},

    /// Generate or serve the documentation website for your project
    Docs {},

    /// List the resources in your project
    List {},

    /// Parses the project and provides information on performance
    Parse {},

    /// Interact with dbt projects
    Project {},

    /// Reattach to the most recent invocation to retrieve logs and artifacts
    Reattach {},

    /// Retry the nodes that failed in the previous run.
    Retry {},

    /// Compile SQL and execute against the current target database.
    Run {},

    /// Run the named macro with any supplied arguments.
    RunOperation {},

    /// Load data from csv files into your data warehouse.
    Seed {},

    /// Generates executable SQL for a named resource or inline query, runs that
    /// SQL, and returns a preview of the results. Does not materialize anything to
    /// the warehouse
    Show {},

    /// Query metrics or metadata against your semantic layer.
    Sl {},

    /// Execute snapshots defined in your project
    Snapshot {},

    /// Manage your project's sources
    Source {},

    /// Runs tests on data in deployed models.
    Test {},

    /// Print version information
    Version {},
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Version {}) => version().await,
        Some(_) => {}
        None => {}
    };
}

// async fn commands() {
//     let mut process = spawn_venv_command("dbt", &[]);

//     let stdout = process.stdout.take().expect("failed to get stdout");
//     let reader = BufReader::new(stdout).lines();

//     tokio::pin!(reader);
//     while let Some(line) = reader.next_line().await.expect("failed to read line") {
//         println!("{}", line);
//     }
// }

async fn version() {
    let crate_version = clap::crate_version!();

    let process = spawn_venv_command("dbt", &["version"]);

    let stdout = process
        .wait_with_output()
        .await
        .expect("failed to wait on child")
        .stdout;

    let output = String::from_utf8(stdout).expect("failed to parse output");

    let re =
        regex::Regex::new(r"dbt (?P<variant>.*) - (?P<version>\d+\.\d+\.\d+) \((?P<commit>[a-zA-Z\d]{40}) (?P<timestamp>\d+-\d+-\d+T\d+:\d+:\d+Z)\)")
            .unwrap();

    let captures = re.captures(&output).unwrap();

    let variant = captures.name("variant").unwrap().as_str();
    let version = captures.name("version").unwrap().as_str();
    let commit = captures.name("commit").unwrap().as_str();
    let timestamp = captures.name("timestamp").unwrap().as_str();

    println!("Orbit version: {}", crate_version);
    println!(
        "DBT version:   {} ({} {} {})",
        version, variant, commit, timestamp
    );
}

fn spawn_venv_command(program: &str, args: &[&str]) -> Child {
    Command::new(format!(".venv/bin/{}", program))
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn process")
}
