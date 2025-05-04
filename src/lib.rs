use clap::{Command, arg, builder::styling};

use crate::commands::{
    add_warp_point, delete_warp_point, edit_warp_point, list_warp_points, warp_to_point,
};
use crate::shell_wrapper::print_init;

mod commands;
mod shell_wrapper;
mod storage;
mod util;

/// Set up the command line interface clap styles
fn build_clap_styles() -> styling::Styles {
    styling::Styles::styled()
        .header(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .usage(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .literal(styling::AnsiColor::Blue.on_default() | styling::Effects::BOLD)
        .placeholder(styling::AnsiColor::Cyan.on_default())
}

/// Construct the CLI command using clap
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
                .arg(arg!([path] "Target path of the warp point").required(false))
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
            Command::new("edit")
                .short_flag('e')
                .about("Edit a warp point")
                .arg(arg!([name] "Name of the warp point").required(true))
                .arg(arg!(-p --path [path] "Updated target path").required(false))
                .arg(arg!(-d --description [description] "Updated description").required(false))
        )
        .subcommand(
            Command::new("init")
                .short_flag('i')
                .hide(true)
                .about("Initialize the warp directory")
                .arg(arg!([shell] "Name of shell").required(true)),
        )
        .subcommand(
            Command::new("warp-point-file")
                .about("Print the warp point file path")
                .hide(true)
        )
        .arg_required_else_help(false)
        .styles(build_clap_styles())
}

/// Main function to run the CLI application
///
/// Currently, supports the following subcommands:
/// - `add`: Add a new warp point
/// - `add-directory`: Add a directory warp point
/// - `list`: List all warp points
/// - `delete`: Delete a warp point
/// - `init`: Prints the shell function to get the directory changing to work (hidden)
///
/// Subcommands also have short flags for easier access, they can be consulted using the help flag.
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
        Some(("add-directory", submatches)) => {
            commands::add_warp_directory(submatches);
        }
        Some(("list", _)) => list_warp_points(),
        Some(("delete", submatches)) => delete_warp_point(submatches),
        Some(("edit", submatches)) => edit_warp_point(submatches),
        Some(("init", submatches)) => print_init(submatches),
        Some(("warp-point-file", _)) => {
            let path = storage::get_storage_file();
            println!("{}", path.display());
        }
        _ => {}
    }
}
