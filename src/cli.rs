use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about = "GUV: Global UV Environment Manager", long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a new global Python environment using uv
    Create {
        /// Name of the environment to create
        #[arg(short, long)]
        name: String,
        /// Specify Python version (e.g., 3.10, pypy3.9). Passed to `uv venv -p`.
        #[arg(short, long)]
        python: Option<String>,
    },
    /// List all managed global environments
    List,
    /// Remove a global environment
    Delete {
        /// Name of the environment to remove
        #[arg(short, long)]
        name: String,
    },
    /// Show activation commands for a global environment
    Activate {
        /// Name of the environment to activate
        name: String,
        /// Optionally specify the shell (e.g., bash, zsh, fish, powershell, cmd)
        #[arg(short, long)]
        shell: Option<String>,
    },
    Deactivate,

    /// Install packages into a global environment using `uv pip install`
    Install {
        /// Name of the environment
        name: String,
        /// Packages to install (e.g., requests, "fastapi[all]")
        #[arg(required = true, num_args = 1..)]
        packages: Vec<String>,
    },
    /// Uninstall packages from a global environment using `uv pip uninstall`
    Uninstall {
        /// Name of the environment
        name: String,
        /// Packages to uninstall
        #[arg(required = true, num_args = 1..)]
        packages: Vec<String>,
    },
    /// Show installed packages in a global environment using `uv pip freeze`
    Freeze {
        /// Name of the environment
        name: String,
    },
    /// Show the path to a specific global environment
    Path {
        /// Name of the environment
        name: String,
    },
    /// Show the path to the guv environments directory
    Home,

    /// Run a command within a specific global environment
    Run {
        /// Name of the environment to use
        name: String,
        /// The command and its arguments to run (e.g., python --version)
        #[arg(required = true, trailing_var_arg = true)]
        command_with_args: Vec<String>,
    },
}