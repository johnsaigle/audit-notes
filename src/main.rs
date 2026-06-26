use clap::{Parser, ValueEnum};
use std::fs::{File, OpenOptions, create_dir_all};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,

    #[arg(long)]
    force: bool,

    #[arg(long, value_enum)]
    preset: Option<Preset>,
}

#[derive(Clone, Debug, ValueEnum)]
enum Preset {
    Evm,
    Solana,
    Rust,
    Go,
    Web2,
}

fn main() {
    let args = Args::parse();
    let home = std::env::home_dir().expect("must access home dir");
    let dir_path = create_audit_notes(&home, &args.name, args.force, args.preset.as_ref());

    println!("{}", dir_path.display());
}

fn create_audit_notes(home: &Path, name: &str, force: bool, preset: Option<&Preset>) -> PathBuf {
    // Store notes under `$HOME/audits/project-name`
    let dir_path = home.join("audits").join(name);
    create_dir_all(&dir_path).expect("cannot continue without creating the root dir");

    write_file(&dir_path, "access-control", "", force);
    write_file(&dir_path, "core-concepts", "", force);
    write_file(&dir_path, "findings", "", force);
    write_file(&dir_path, "flows", "", force);
    write_file(&dir_path, "resources", "", force);
    write_file(&dir_path, "questions", "", force);

    // Populate llm.md with title
    let llm_content = "# LLM Summary\n\n";
    write_file(&dir_path, "llm", llm_content, force);

    // Populate notes.md with project name as h1, date, and TODOs
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let title = name
        .split('-')
        .map(|w| {
            let mut c = w.chars();
            c.next().map_or_else(String::new, |first| {
                first.to_uppercase().collect::<String>() + c.as_str()
            })
        })
        .collect::<Vec<_>>()
        .join(" ");
    let preset_todos = preset_todos(preset);

    let notes_content = format!(
        "# {title} - {today}\n\n## TODO\n\n- Map primary flows\n- Run tests\n- Read Docs\n{preset_todos}\n### SAST\n- Run static analysis tools\n- Review compiler warnings\n- Check for known vulnerability patterns\n\n## Notes\n\n"
    );
    write_file(&dir_path, "notes", &notes_content, force);

    // Populate invariants.md with template content
    let invariants_content = "# Invariants\n\n## Explicit Invariants\n\n## Implicit Invariants\n\n";
    write_file(&dir_path, "invariants", invariants_content, force);

    dir_path
}

fn write_file(dir_path: &Path, name: &str, content: &str, force: bool) {
    let path = dir_path.join(format!("{name}.md"));
    let file = if force {
        File::create(&path)
    } else {
        OpenOptions::new().write(true).create_new(true).open(&path)
    };

    match file {
        Ok(mut file) => {
            file.write_all(content.as_bytes())
                .unwrap_or_else(|err| panic!("cannot write to {}: {err}", path.display()));
        }
        Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => {}
        Err(err) => panic!("cannot create {}: {err}", path.display()),
    }
}

const fn preset_todos(preset: Option<&Preset>) -> &'static str {
    match preset {
        Some(Preset::Evm) => {
            "- Map privileged roles and upgrade paths\n- Review token accounting and external calls\n"
        }
        Some(Preset::Solana) => {
            "- Map account constraints and PDA seeds\n- Review signer, owner, and writable checks\n"
        }
        Some(Preset::Rust) => {
            "- Review panic, unwrap, and arithmetic assumptions\n- Check unsafe and serialization boundaries\n"
        }
        Some(Preset::Go) => {
            "- Review goroutine lifecycles and error handling\n- Check integer conversions and nil paths\n"
        }
        Some(Preset::Web2) => {
            "- Map authn/authz boundaries\n- Review input validation and secret handling\n"
        }
        None => "",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn creates_expected_audit_files() {
        let home = tempfile::tempdir().expect("create temp home");

        let audit_dir = create_audit_notes(home.path(), "client-project", false, None);

        for file in [
            "access-control.md",
            "core-concepts.md",
            "findings.md",
            "flows.md",
            "invariants.md",
            "llm.md",
            "notes.md",
            "questions.md",
            "resources.md",
        ] {
            assert!(audit_dir.join(file).exists(), "missing {file}");
        }

        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let notes = fs::read_to_string(audit_dir.join("notes.md")).expect("read notes.md");
        assert!(notes.contains(&format!("# Client Project - {today}")));
        assert!(notes.contains("## TODO"));
        assert!(notes.contains("- Map primary flows"));
        assert!(notes.contains("### SAST"));

        let invariants =
            fs::read_to_string(audit_dir.join("invariants.md")).expect("read invariants.md");
        assert!(invariants.contains("# Invariants"));
        assert!(invariants.contains("## Explicit Invariants"));
        assert!(invariants.contains("## Implicit Invariants"));
    }

    #[test]
    fn preserves_existing_files_without_force() {
        let home = tempfile::tempdir().expect("create temp home");
        let audit_dir = create_audit_notes(home.path(), "client-project", false, None);
        let notes_path = audit_dir.join("notes.md");
        fs::write(&notes_path, "existing notes\n").expect("write existing notes");

        create_audit_notes(home.path(), "client-project", false, None);

        let notes = fs::read_to_string(notes_path).expect("read notes.md");
        assert_eq!(notes, "existing notes\n");
    }

    #[test]
    fn force_overwrites_existing_files() {
        let home = tempfile::tempdir().expect("create temp home");
        let audit_dir = create_audit_notes(home.path(), "client-project", false, None);
        let notes_path = audit_dir.join("notes.md");
        fs::write(&notes_path, "existing notes\n").expect("write existing notes");

        create_audit_notes(home.path(), "client-project", true, None);

        let notes = fs::read_to_string(notes_path).expect("read notes.md");
        assert!(notes.contains("# Client Project - "));
        assert!(!notes.contains("existing notes"));
    }

    #[test]
    fn optional_preset_adds_targeted_todos() {
        let home = tempfile::tempdir().expect("create temp home");

        let audit_dir =
            create_audit_notes(home.path(), "solana-audit", false, Some(&Preset::Solana));

        let notes = fs::read_to_string(audit_dir.join("notes.md")).expect("read notes.md");
        assert!(notes.contains("- Map account constraints and PDA seeds"));
        assert!(notes.contains("- Review signer, owner, and writable checks"));
    }
}
