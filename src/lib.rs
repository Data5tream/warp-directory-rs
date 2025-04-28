use clap::builder::styling;
use clap::{arg, Command};

use crate::commands::{add_warp_point, delete_warp_point, list_warp_points, warp_to_point};

mod commands;
mod storage;

fn build_clap_styles() -> styling::Styles {
    styling::Styles::styled()
        .header(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .usage(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .literal(styling::AnsiColor::Blue.on_default() | styling::Effects::BOLD)
        .placeholder(styling::AnsiColor::Cyan.on_default())
}

fn construct_command() -> Command {
    Command::new("warp-directory")
        .version("0.1.0")
        .author("Simon Barth <data5tream@proton.me>")
        .about("Set and warp to directories")
        .arg(arg!([warp_point] "The warp point to warp to"))
        .subcommand(
            Command::new("add")
                .short_flag('a')
                .about("Add a directory warp point")
                .arg(arg!([name] "Name of the warp point").required(true))
                .arg(arg!([path] "Target path of the warp point").required(false))
                .arg(arg!(-f --force "Force overwrite an existing warp point").required(false))
                .arg(
                    arg!(--description <description> "Description of the warp point")
                        .required(false),
                ),
        )
        .subcommand(
            Command::new("add-directory")
                .short_flag('A')
                .alias("add-dir")
                .about("Add a directory warp point")
                .long_about("Add a directory warp point. All direct subdirectories will be added as warp points.")
                .arg(arg!([name] "Name of the warp point").required(true))
                .arg(arg!([path] "Target path of the warp point").required(true))
                .arg(arg!(-f --force "Force overwrite an existing warp point").required(false))
                .arg(
                    arg!(--description <description> "Description of the warp point")
                        .required(false),
                )
                .arg(
                    arg!(--"strip-prefix" <prefix> "Prefix to strip from the directory names")
                        .required(false),
                ),
        )
        .subcommand(
            Command::new("list")
                .short_flag('l')
                .about("List all directory warp points"),
        )
        .subcommand(
            Command::new("delete")
                .short_flag('d')
                .about("Delete a directory warp point")
                .arg(arg!([name] "Name of the warp point").required(true)),
        )
        .subcommand(
            Command::new("init")
                .short_flag('i')
                .hide(true)
                .about("Initialize the warp directory")
                .arg(arg!([shell] "Name of shell").required(true)),
        )
        .arg_required_else_help(false)
        .styles(build_clap_styles())
}

fn print_init(shell: Option<&str>) {
    match shell {
        Some("zsh") | None => {
            println!(
                r#"
function warp() {{
    local dir
    dir=$(warp-directory "$@") || return $?
    [[ -d "$dir" ]] && cd "$dir" || {{ [[ -n "$dir" ]] && print -r -- "$dir"; return 1; }}
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

pub fn app() {
    let matches = construct_command().get_matches();

    match matches.subcommand() {
        None => {
            if matches.args_present() {
                if let Some(warp_point) = matches.get_one::<String>("warp_point") {
                    warp_to_point(warp_point);
                }
            } else {
                list_warp_points();
            }
        }
        Some(("add", submatches)) => add_warp_point(submatches),
        Some(("list", _)) => list_warp_points(),
        Some(("delete", submatches)) => delete_warp_point(submatches),
        Some(("init", submatches)) => {
            let shell = submatches.get_one::<String>("shell");
            print_init(shell.map(|s| s.as_str()));
        }
        _ => {}
    }
}
