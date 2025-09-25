use std::fs::{create_dir_all, File};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,
}

fn main() {
    // TODO: Use $HOME and realpath
    let args = Args::parse();
    let dir_path = format!("/Users/john/audits/{}", args.name);
    create_dir_all(&dir_path).expect("cannot continue without creating the root dir");

    let files = [ 
        "access-control",
        "core-concepts",
        "findings",
        "flows",
        "notes",
        "resources",
        "questions",
    ];
    for file in files {
        let _ = File::create(format!("{dir_path}/{file}.md"));
    }
}
