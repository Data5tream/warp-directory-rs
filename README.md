# warp-directory-rs

Save a list of directories and easily switch to them.

## Installation

Clone the repository and install the `warp-directory` binary using `cargo` (prebuilds comming soon):

```bash
cargo install --path .
```

Add the following to your zsh configuration file (e.g., `~/.zshrc`) (other shells coming soon):

```bash
eval "$(warp-directory init zsh)"
```

Source your config or open a new terminal. Now you can use the `warp` command (checkout
`warp --help`).

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
