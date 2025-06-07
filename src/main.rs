// src/main.rs
use clap::{Parser, Subcommand, Args};
// use std::path::PathBuf; // Removed unused import
use anyhow::Result; // Using anyhow for main error handling for simplicity

mod commands;
mod utils;
mod error; // If using custom error

#[derive(Parser, Debug)]
#[clap(author, version, about = "Global environment management tool using uv", long_about = None)]
#[clap(propagate_version = true)]
struct Cli { // This struct itself doesn't need to be pub
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
// pub enum Commands { // This could be pub, or not, depending on direct usage outside.
// For now, clap handles its use internally via Cli.
pub enum Commands { // Making it pub for clarity, though not strictly required by errors shown
    Init(InitArgs),
    Create(CreateArgs),
    List,
    Activate(EnvNameArg),
    Deactivate,
    Delete(DeleteArgs),
    // Add(PackageManagementArgs),
    // Remove(PackageManagementArgs),
    Install(PackageManagementArgs),
    Uninstall(PackageManagementArgs),
    Freeze(EnvNameArg),
    Path(EnvNameArg),
    Home,
    Run(RunArgs),
}

#[derive(Args, Debug)]
pub struct CreateArgs { // MUST be pub
    /// Name of the environment to create
    pub name: String,
    /// Python version to use (e.g., 3.10, python3.11, /usr/bin/python3)
    #[clap(short, long)]
    pub python: Option<String>,
}

#[derive(Args, Debug)]
pub struct InitArgs {
    /// Force re-initialization even if already set up.
    #[clap(long)]
    force: bool,
    // You could add a --shell option to manually specify if needed
}

#[derive(Args, Debug)]
pub struct EnvNameArg { // MUST be pub
    /// Name of the environment
    pub name: String,
}

#[derive(Args, Debug)]
pub struct DeleteArgs { // MUST be pub
    /// Name of the environment to delete
    pub name: String,
    /// Skip confirmation prompt
    #[clap(short, long)]
    pub yes: bool,
}


#[derive(Args, Debug)]
pub struct PackageManagementArgs { // MUST be pub
    /// Name of the environment
    pub env_name: String,
    /// Packages to manage (e.g., requests, numpy, "flask>=2.0")
    #[clap(required = true, num_args = 1..)]
    pub packages: Vec<String>,
}

#[derive(Args, Debug)]
pub struct RunArgs { // MUST be pub
    /// Name of the environment
    pub env_name: String,
    /// The command and its arguments to run (e.g., python script.py --arg value)
    #[clap(last = true, required = true, num_args = 1..)]
    pub command_and_args: Vec<String>,
}


fn main() -> Result<()> {
    if let Err(e) = utils::check_uv_exists() {
        // `e` here is GuvError. `thiserror` ensures it implements `Display`.
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