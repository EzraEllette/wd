use std::process::exit;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    command: String,
    #[arg(default_value = "")]
    name: String,
}

fn main() {
    let args = Cli::parse();

    match args.name.as_str() {
        "add" | "rm" | "list" => {
            println!("{} is not a valid warp name.", args.name);
            exit(1);
        }
        _ => {}
    }

    match args.command.as_str() {
        "add" => wd::add_warp(args.name),
        "rm" => wd::remove_warp(args.name),
        "list" => wd::list_warps(),
        _ => wd::warp(args.command),
    }
}
