use anyhow::Result;
use clap::{Args, Parser, Subcommand};

mod commands;
mod error;
mod utils;

#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about = "Global environment management tool using uv",
    long_about = "guv is a command-line tool for managing global Python virtual environments using uv. \
                    It provides a simple interface for creating, activating, and managing Python environments \
                    with their own isolated packages and dependencies."
)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize guv in your shell configuration
    #[clap(
        long_about = "Initialize guv in your shell configuration to enable environment activation and deactivation"
    )]
    Init(InitArgs),

    /// Create a new virtual environment
    #[clap(
        long_about = "Create a new Python virtual environment with the specified name and Python version"
    )]
    Create(CreateArgs),

    /// List all available environments
    #[clap(long_about = "Display a list of all virtual environments managed by guv")]
    List,

    /// Activate a virtual environment
    #[clap(
        long_about = "Activate a virtual environment to use its Python interpreter and packages"
    )]
    Activate(EnvNameArg),

    /// Deactivate the current virtual environment
    #[clap(long_about = "Deactivate the currently active virtual environment")]
    Deactivate,

    /// Delete a virtual environment
    #[clap(long_about = "Permanently delete a virtual environment and all its installed packages")]
    Delete(DeleteArgs),

    /// Install packages in an environment
    #[clap(long_about = "Install Python packages in the specified or active environment using uv")]
    Install(PackageManagementArgs),

    /// Uninstall packages from an environment
    #[clap(long_about = "Uninstall Python packages from the specified or active environment")]
    Uninstall(PackageManagementArgs),

    /// Output installed packages in requirements format
    #[clap(
        long_about = "Generate a requirements.txt-compatible list of all installed packages in the environment"
    )]
    Freeze(EnvNameArg),

    /// Print the path to an environment
    #[clap(long_about = "Display the full filesystem path to the specified environment")]
    Path(EnvNameArg),

    /// Print the guv home directory
    #[clap(long_about = "Display the path to the guv home directory where environments are stored")]
    Home,

    /// Run a command in an environment
    #[clap(long_about = "Execute a command within the context of the specified environment")]
    Run(RunArgs),
}

#[derive(Args, Debug)]
pub struct CreateArgs {
    /// Name of the environment to create
    #[clap(
        help = "Name of the environment to create",
        long_help = "Specify a name for the new virtual environment. This name will be used to reference the environment in other commands."
    )]
    pub name: String,

    /// Python version to use (e.g., 3.10, python3.11, /usr/bin/python3)
    #[clap(
        short,
        long,
        help = "Python version to use",
        long_help = "Specify which Python version to use for this environment. Can be a version number (e.g., 3.10), an interpreter name (e.g., python3.11), or a path to a Python executable."
    )]
    pub python: Option<String>,

    #[clap(required = true, num_args = 1.., help = "Packages to install", long_help = "List of packages to install or uninstall. You can specify version constraints using standard pip syntax (e.g., 'flask>=2.0', 'requests==2.28.1').")]
    pub packages: Vec<String>,
}

#[derive(Args, Debug)]
pub struct InitArgs {
    /// Force re-initialization even if already set up
    #[clap(
        long,
        help = "Force re-initialization",
        long_help = "Force re-initialization of guv in your shell configuration even if it's already set up. This will replace any existing guv configuration."
    )]
    pub force: bool,
}

#[derive(Args, Debug)]
pub struct EnvNameArg {
    /// Name of the environment (optional if an environment is active)
    #[clap(
        value_name = "ENV_NAME",
        help = "Environment name",
        long_help = "Specify the name of the environment to use. If not provided and an environment is currently active, the active environment will be used."
    )]
    pub name: Option<String>,
}

#[derive(Args, Debug)]
pub struct DeleteArgs {
    /// Name of the environment to delete
    #[clap(
        help = "Environment to delete",
        long_help = "Specify the name of the environment you want to permanently delete."
    )]
    pub name: String,

    /// Skip confirmation prompt
    #[clap(
        short,
        long,
        help = "Skip confirmation",
        long_help = "Skip the confirmation prompt and immediately delete the environment. Use with caution as this action cannot be undone."
    )]
    pub yes: bool,
}

#[derive(Args, Debug)]
pub struct PackageManagementArgs {
    /// Name of the environment (optional if an environment is active)
    #[clap(
        short,
        long,
        value_name = "ENV_NAME",
        help = "Environment name",
        long_help = "Specify the name of the environment to manage packages in. If not provided and an environment is currently active, the active environment will be used."
    )]
    pub env_name: Option<String>,

    /// Packages to manage (e.g., requests, numpy, "flask>=2.0")
    #[clap(required = true, num_args = 1.., help = "Packages to manage", long_help = "List of packages to install or uninstall. You can specify version constraints using standard pip syntax (e.g., 'flask>=2.0', 'requests==2.28.1').")]
    pub packages: Vec<String>,
}

#[derive(Args, Debug)]
pub struct RunArgs {
    /// Name of the environment
    #[clap(
        help = "Environment name",
        long_help = "Specify the name of the environment in which to run the command."
    )]
    pub env_name: String,

    /// The command and its arguments to run (e.g., python script.py --arg value)
    #[clap(last = true, required = true, num_args = 1.., help = "Command to run", long_help = "The command and its arguments to run within the specified environment. For example: 'python script.py --arg value'")]
    pub command_and_args: Vec<String>,
}

fn main() -> Result<()> {
    if let Err(e) = utils::check_uv_exists() {
        eprintln!("Error: {}", e);
        eprintln!("Please ensure 'uv' is installed and in your PATH.");
        std::process::exit(1);
    }

    let cli = Cli::parse();

    match cli.command {
        Commands::Init(args) => commands::init::handle_init(args),
        Commands::Create(args) => commands::create::handle_create(args),
        Commands::List => commands::list::handle_list(),
        Commands::Activate(args) => commands::activate::handle_activate_for_shell_export(args),
        Commands::Deactivate => commands::deactivate::handle_deactivate_for_shell_export(),
        Commands::Delete(args) => commands::delete::handle_delete(args),
        // Commands::Add(args) => commands::add::handle_add(args),
        // Commands::Remove(args) => commands::remove::handle_remove(args),
        Commands::Install(args) => commands::install::handle_install(args),
        Commands::Uninstall(args) => commands::uninstall::handle_uninstall(args),
        Commands::Freeze(args) => commands::freeze::handle_freeze(args),
        Commands::Path(args) => commands::path::handle_path(args),
        Commands::Home => commands::home::handle_home(),
        Commands::Run(args) => commands::run::handle_run(args),
    }
}
