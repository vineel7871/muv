# guv
**guv** is a global environment management tool for macOS, inspired by [UV](https://github.com/astral-sh/uv). It helps you manage and switch between different development environments seamlessly.

## Features

- Manage multiple global environments for various languages and tools.
- Simple CLI for creating, activating, and removing environments.
- Fast and lightweight, designed for macOS.

## Installation

```sh
git clone https://github.com/yourusername/guv.git
cd guv
cargo install --path .
```

## Usage

### Initial Setup

Before using guv, run the following command to set up the environment:

```sh
guv init
```

### Create a new environment

```sh
guv create <env-name>
```

### List environments

```sh
guv list
```

### Activate an environment

```sh
guv activate <env-name>
```

### Remove an environment

```sh
guv remove <env-name>
```

## Contributing

Contributions are welcome! Please open issues or pull requests.

## License

This project is licensed under the Apache License.