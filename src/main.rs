use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};
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
    fn from(path: PathBuf) -> Self {
        let mut frontmatter = HashMap::new();

        let file = File::open(&path).unwrap();
        let mut reader = BufReader::new(file);
        let mut buf = String::new();
        let mut current_key = String::new();
        let mut current_val = String::new();

        reader.read_line(&mut buf).unwrap();
        if buf.trim() != "---" {
            buf.clear();
            return Self { path, frontmatter };
        }
        buf.clear();

        loop {
            reader.read_line(&mut buf).unwrap();
            match buf.split_once(':') {
                Some((new_key, new_val)) => {
                    if !current_key.is_empty() {
                        frontmatter.insert(current_key, current_val.trim().to_string());
                    }
                    current_key = new_key.to_string();
                    current_val = new_val.to_string();
                }
                None => current_val.push_str(&buf),
            }

            if buf.trim() == "---" {
                break;
            }

            buf.clear();
        }

        Self { path, frontmatter }
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
        .filter(|file| {
            let ext = file
                .path()
                .extension()
                .and_then(|osstr| osstr.to_str())
                .unwrap_or("");
            ext == "md" || ext == "markdown"
        })
        .map(DirEntry::into_path)
        .map(Note::from)
        .filter(|note| {
            let post = note.frontmatter.get("post");

            if area == "*" {
                return post.is_some();
            }

            post == Some(&area)
                || post == Some(&format!("[[{area}]]"))
                || post == Some(&format!("\"[[{area}]]\""))
        })
        .for_each(|note| println!("{}", note.path.display()));
}

fn show_help() {
    todo!();
}

fn parse_args() -> Option<Args> {
    let mut args = env::args();

    let _self = args.next();
    let vault = args.next()?;
    let area = args.next()?;

    Some(Args { vault, area })
}
