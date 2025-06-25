# muv

A command-line tool for managing global Python virtual environments using uv.

## Features

- Create and manage Python virtual environments
- Install and uninstall packages
- Activate and deactivate environments
- List available environments
- Run commands within environments

## Installation

### Option 1: Using Homebrew (macOS)

```bash
brew tap vineel7871/muv
brew install muv
```

### Option 2: Using Cargo

```bash
cargo install muv
```

### Option 3: Download pre-built binaries

1. Go to the [Releases page](https://github.com/vineel7871/muv/releases)
2. Download the appropriate binary for your system:
   - `muv-linux-amd64.tar.gz` for Linux (x86_64)
   - `muv-linux-arm64.tar.gz` for Linux (ARM64)
   - `muv-macos-amd64.tar.gz` for macOS (Intel)
   - `muv-macos-arm64.tar.gz` for macOS (Apple Silicon)
3. Extract the binary:
   ```bash
   tar -xzf muv-*.tar.gz
   ```
4. Move the binary to a directory in your PATH:
   ```bash
   sudo mv muv /usr/local/bin/
   # Or for a user-local installation:
   # mkdir -p ~/.local/bin
   # mv muv ~/.local/bin/
   # export PATH="$HOME/.local/bin:$PATH"
   ```
5. Make it executable:
   ```bash
   chmod +x /usr/local/bin/muv
   # Or for user-local:
   # chmod +x ~/.local/bin/muv
   ```

## Getting Started

After installation, you need to initialize muv in your shell:

```bash
# Initialize muv in your shell
muv init

# Source your shell configuration to apply changes
source ~/.bashrc  # or ~/.zshrc if using zsh
```

## Usage

```bash
# Create a new environment

# Create a new environment
muv create myenv --p 3.10

# Activate an environment
muv activate myenv

# Install packages
muv install numpy pandas

# Run a command in an environment
muv run myenv python script.py

# Install from requirements.txt file
muv install -r requirements.txt

# Install from pyproject.toml file
muv install -t pyproject.toml
```

## Add Auto Complete Support
users can generate completion scripts using:

Bash:<br>
`muv completions bash > /usr/local/etc/bash_completion.d/muv`

Zsh:<br>
`muv completions zsh > "${fpath[1]}/_muv"`

Fish:<br>
`muv completions fish > ~/.config/fish/completions/muv.fish"`

And then source or restart their shell.

## Requirements

- `uv` must be installed and available in your PATH. You can install it following the instructions at [https://github.com/astral-sh/uv](https://github.com/astral-sh/uv)

## License

MIT