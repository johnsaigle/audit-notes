use clap::Parser;
use std::fs::{File, create_dir_all};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,
}

fn main() {
    let args = Args::parse();

    // Store notes under `$HOME/audits/project-name`
    let dir_path = format!(
        "{}/audits/{}",
        std::env::home_dir().expect("must access home dir").display(),
        args.name
    );
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
    // TODO: Populate files with some markdown template

    println!("{dir_path}");
}
