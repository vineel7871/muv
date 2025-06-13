# guv

A command-line tool for managing global Python virtual environments using uv.

## Features

- Create and manage Python virtual environments
- Install and uninstall packages
- Activate and deactivate environments
- List available environments
- Run commands within environments

## Installation

```bash
cargo install guv
```

## Usage

```bash
# Initialize guv in your shell
guv init

# Create a new environment
guv create myenv --python 3.10

# Activate an environment
guv activate myenv

# Install packages
guv install numpy pandas

# Run a command in an environment
guv run myenv python script.py
```

## License

MIT