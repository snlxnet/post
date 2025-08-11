use std::{collections::HashMap, env, path::PathBuf};
use walkdir::{DirEntry, WalkDir};

struct Args {
    vault: String,
    area: String,
}

struct Note {
    path: PathBuf,
    frontmatter: HashMap<String, String>,
}

impl From<PathBuf> for Note {
    fn from(value: PathBuf) -> Self {
        todo!()
    }
}

fn main() {
    let Some(Args { vault, area }) = parse_args() else {
        show_help();
        return;
    };

    WalkDir::new(vault)
        .into_iter()
        .flatten()
        .map(DirEntry::into_path)
        .map(Note::from)
        .filter(|note| note.frontmatter.contains_key(&area))
        .for_each(|note| println!("{}", note.path.display()));
}

fn show_help() {
    todo!();
}

fn parse_args() -> Option<Args> {
    let mut args = env::args();

    let vault = args.next()?;
    let area = args.next()?;

    Some(Args { vault, area })
}
