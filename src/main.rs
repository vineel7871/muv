use std::io;

use anyhow::Result;

mod cli;
mod commands;
mod error;
mod utils;

use clap::{CommandFactory, Parser};
use clap_complete::generate;
use cli::{Cli, Commands};

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
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            generate(shell, &mut cmd, "muv", &mut io::stdout());
            Ok(())
        }
    }
}
