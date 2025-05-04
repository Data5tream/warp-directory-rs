use clap::ArgMatches;

use crate::construct_command;

/// Generate the shell function that wraps the rust binary and allows us to change the calling
/// shells working directory
///
/// This functions uses the generated clap command to get all direct subcommands, aliases and flags
/// to create a whitelist of first arguments that should **not** be wrapped in the cd function.
/// Everything else is assumed to be a warp point and the stdout output of the command invocation
/// will be passed to `cd`.
fn print_shell_wrapper(shell: Option<&str>) {
    let command = construct_command();
    let mut ignored_args: Vec<String> = vec![
        "help".into(),
        "--help".into(),
        "-h".into(),
        "--version".into(),
        "-V".into(),
    ];

    for sub in command.get_subcommands() {
        ignored_args.push(String::from(sub.get_name()));
        for alias in sub.get_aliases() {
            ignored_args.push(String::from(alias));
        }
        if let Some(short_flag) = sub.get_short_flag() {
            ignored_args.push(format!("-{short_flag}"));
        }
        if let Some(long_flag) = sub.get_long_flag() {
            ignored_args.push(format!("--{long_flag}"));
        }
    }
    let ignored_arg_string = ignored_args.join("|");

    match shell {
        Some("zsh") | None => {
            println!(
                r#"
function warp() {{
    if [[ -z "$1" ]]; then
        warp-directory list
        return 0
    fi
    local first="$1"
    shift

    case "$first" in
        {ignored_arg_string})
            warp-directory "$first" "$@"
            return $?
            ;;
    esac

    local dir
    dir=$(warp-directory "$first" "$@") || return $?
    [[ -d "$dir" ]] && cd "$dir" || {{ [[ -n "$dir" ]] && print -r -- "$dir"; return 1; }}
}}
"#
            );
        }
        Some("bash") => {
            println!(
                r#"
function warp() {{
    if [[ -z "$1" ]]; then
        warp-directory list
        return 0
    fi
    local first="$1"
    shift

    case "$first" in
        {ignored_arg_string})
            warp-directory "$first" "$@"
            return $?
            ;;
    esac

    local dir
    dir=$(warp-directory "$first" "$@") || return $?
    if [[ -d "$dir" ]]; then
        cd "$dir"
    elif [[ -n "$dir" ]]; then
        echo "$dir"
        return 1
    fi
}}
"#
            );
        }
        Some(shell) => {
            eprintln!("Unsupported shell: {shell}");
            std::process::exit(1);
        }
    }
}

/// Prints the shell wrapper function to stdout
pub fn print_init(submatches: &ArgMatches) {
    let shell = submatches.get_one::<String>("shell");
    print_shell_wrapper(shell.map(String::as_str));
}
