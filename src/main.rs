use clap::Parser;
use std::process::exit;
use warper::Warper;

mod warper;

#[derive(Parser)]
struct Cli {
    command: String,
    #[arg(default_value = "")]
    name: String,
}

fn main() {
    let args = Cli::parse();
    let mut warper = Warper::new(".wd", "warps.json");

    match args.name.as_str() {
        "add" | "rm" | "list" => {
            println!("{} is not a valid warp name.", args.name);
            exit(1);
        }
        _ => {}
    }

    match args.command.as_str() {
        "add" => warper.add_warp(args.name),
        "rm" => warper.remove_warp(args.name),
        "list" => warper.list_warps(),
        _ => warper.warp(args.command),
    }
}
