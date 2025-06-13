# muv

A command-line tool for managing global Python virtual environments using uv.

## Features

- Create and manage Python virtual environments
- Install and uninstall packages
- Activate and deactivate environments
- List available environments
- Run commands within environments

## Installation

```bash
cargo install muv
```

## Usage

```bash
# Initialize muv in your shell
muv init

# Create a new environment
muv create myenv --python 3.10

# Activate an environment
muv activate myenv

# Install packages
muv install numpy pandas

# Run a command in an environment
muv run myenv python script.py
```

## License

MIT