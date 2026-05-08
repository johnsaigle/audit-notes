use clap::Parser;
use std::fs::{File, create_dir_all};
use std::io::Write;

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
        "llm",
        "notes",
        "resources",
        "questions",
    ];
    for file in files {
        let _ = File::create(format!("{dir_path}/{file}.md"));
    }

    // Populate llm.md with title
    let llm_content = "# LLM Summary\n\n";
    let mut llm_file = File::create(format!("{dir_path}/llm.md"))
        .expect("cannot create llm.md");
    llm_file
        .write_all(llm_content.as_bytes())
        .expect("cannot write to llm.md");

    // Populate notes.md with project name as h1, date, and TODOs
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let title = args.name.split('-').map(|w| {
        let mut c = w.chars();
        c.next().map_or_else(String::new, |first| first.to_uppercase().collect::<String>() + c.as_str())
    }).collect::<Vec<_>>().join(" ");

    let notes_content = format!(
        "# {title} - {today}\n\n## TODO\n\n- Map primary flows\n- Run tests\n- Read Docs\n\n### SAST\n- Run static analysis tools\n- Review compiler warnings\n- Check for known vulnerability patterns\n\n## Notes\n\n"
    );
    let mut notes_file = File::create(format!("{dir_path}/notes.md"))
        .expect("cannot create notes.md");
    notes_file
        .write_all(notes_content.as_bytes())
        .expect("cannot write to notes.md");

    // Populate invariants.md with template content
    let invariants_content = "# Invariants\n\n## Explicit Invariants\n\n## Implicit Invariants\n\n";
    let mut invariants_file = File::create(format!("{dir_path}/invariants.md"))
        .expect("cannot create invariants.md");
    invariants_file
        .write_all(invariants_content.as_bytes())
        .expect("cannot write to invariants.md");

    println!("{dir_path}");
}
