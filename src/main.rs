use clap::{Parser, Subcommand};

mod command;
mod python;
mod util;

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run all seeds, models, snapshots, and tests in DAG order
    Build {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
        _args: Vec<String>,
    },

    /// Delete all folders in the clean-targets list (usually the dbt_packages and target directories.)
    Clean {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
        _args: Vec<String>,
    },

    /// Create clones of selected nodes based on their location in the manifest provided to --state.
    Clone {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
        _args: Vec<String>,
    },

    /// Generates executable SQL from source, model, test and analysis files.
    Compile {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
        _args: Vec<String>,
    },

    /// Show information on the current dbt environment and check dependencies, then
    /// test the database connection. Not to be confused with the --debug option
    /// which increases verbosity.
    Debug {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
        _args: Vec<String>,
    },

    /// Pull the most recent version of the dependencies listed in packages.yml
    Deps {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
        _args: Vec<String>,
    },

    /// Generate or serve the documentation website for your project
    Docs {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
        _args: Vec<String>,
    },

    /// Initialize a new dbt project.
    Init {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
        _args: Vec<String>,
    },

    /// List the resources in your project
    List {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
        _args: Vec<String>,
    },

    /// Parses the project and provides information on performance
    Parse {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
        _args: Vec<String>,
    },

    /// Retry the nodes that failed in the previous run.
    Retry {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
        _args: Vec<String>,
    },

    /// Compile SQL and execute against the current target database.
    Run {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
        _args: Vec<String>,
    },

    /// Run the named macro with any supplied arguments.
    RunOperation {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
        _args: Vec<String>,
    },

    /// Load data from csv files into your data warehouse.
    Seed {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
        _args: Vec<String>,
    },

    /// Generates executable SQL for a named resource or inline query, runs that
    /// SQL, and returns a preview of the results. Does not materialize anything to
    /// the warehouse
    Show {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
        _args: Vec<String>,
    },

    /// Execute snapshots defined in your project
    Snapshot {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
        _args: Vec<String>,
    },

    /// Manage your project's sources
    Source {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
        _args: Vec<String>,
    },

    /// Runs tests on data in deployed models.
    Test {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
        _args: Vec<String>,
    },

    /// Print version information
    Version {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
        _args: Vec<String>,
    },
}

fn main() {
    util::logger::init();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Build { _args: args }) => command::build::run(args),
        Some(Commands::Clean { _args: args }) => command::clean::run(args),
        Some(Commands::Clone { _args: args }) => command::clone::run(args),
        Some(Commands::Compile { _args: args }) => command::compile::run(args),
        Some(Commands::Debug { _args: args }) => command::debug::run(args),
        Some(Commands::Deps { _args: args }) => command::deps::run(args),
        Some(Commands::Docs { _args: args }) => command::docs::run(args),
        Some(Commands::Init { _args: args }) => command::init::run(args),
        Some(Commands::List { _args: args }) => command::list::run(args),
        Some(Commands::Parse { _args: args }) => command::parse::run(args),
        Some(Commands::Retry { _args: args }) => command::retry::run(args),
        Some(Commands::Run { _args: args }) => command::run::run(args),
        Some(Commands::RunOperation { _args: args }) => command::run_operation::run(args),
        Some(Commands::Seed { _args: args }) => command::seed::run(args),
        Some(Commands::Show { _args: args }) => command::show::run(args),
        Some(Commands::Snapshot { _args: args }) => command::snapshot::run(args),
        Some(Commands::Source { _args: args }) => command::source::run(args),
        Some(Commands::Test { _args: args }) => command::test::run(args),
        Some(Commands::Version { _args: args }) => command::version::run(args),

        Some(_) => {}
        None => {}
    };
}
