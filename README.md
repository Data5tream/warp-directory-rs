# warp-directory-rs

![Crates.io Version](https://img.shields.io/crates/v/warp-directory)
![GitHub Release](https://img.shields.io/github/v/release/data5tream/warp-directory-rs?label=GitHub%20version)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/data5tream/warp-directory-rs/lint.yml?label=clippy)
![GitHub License](https://img.shields.io/github/license/data5tream/warp-directory-rs)

Save a list of directories and easily switch to them. Currently, zsh and bash are supported.

## Installation

### Via cargo

Install the `warp-directory` binary using `cargo`:

```bash
cargo install warp-directory
```

### Build from source

Clone the repository and install the `warp-directory` binary using `cargo`:

```bash
cargo install --path .
```

## Configuration

Add the following to your zsh configuration file (e.g., `~/.zshrc` or `~/.bashrc`):

If running zsh:

```bash
eval "$(warp-directory init zsh)"
```

Or, if you run bash:

```bash
eval "$(warp-directory init bash)"
```

Source your config or open a new terminal.

## Usage

List warp points:

```bash
warp list
```

Add a warp point:

```bash
warp add <name> <path>
```

If `<path>` is not provided, the current directory will be used. You can add a description that will
be shown when using `warp list` by using the `--description` flag.

Remove a warp point:

```bash
warp delete <name>
```

Show the help message:

```bash
warp --help
```
