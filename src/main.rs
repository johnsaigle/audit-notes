use std::fs::File;
use std::io::Error;
use clap::Parser;


fn make_files(name: String) -> Result<(), Error> {
    let files = [ 
        "access-control",
        "core-concepts",
        "findings",
        "flows",
        "notes",
        "resources",
    ];
    // TODO: switch to $HOME if possible
    let dir_name = format!("/Users/john/audits/{name}");
    std::fs::create_dir_all(&dir_name).unwrap();
    for file in files {
        let _ = File::create(format!("{dir_name}/{file}.md"));
    }
    Ok(())
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,
}

fn main() {
    let args = Args::parse();
    make_files(args.name).unwrap();
}
