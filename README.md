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

Source your config or open a new terminal. Now you can use the `warp` command (checkout `warp
--help). 