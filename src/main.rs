use rayon::prelude::*;
use serde::Deserialize;
use std::{
    env,
    fs::{self},
    path::PathBuf,
};
use walkdir::{DirEntry, WalkDir};

struct Args {
    vault: String,
    area: String,
}

struct Note {
    path: PathBuf,
    area: String,
    linked_paths: Vec<String>,
}

#[derive(Deserialize)]
struct Frontmatter {
    post: Option<String>,
}

impl TryFrom<PathBuf> for Note {
    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let doc = fs::read_to_string(&path).map_err(|_| "File not found")?;
        let (frontmatter, body) =
            markdown_frontmatter::parse::<Frontmatter>(&doc).map_err(|_| "Parsing error")?;

        let Some(area) = frontmatter.post else {
            return Err("Not a public note");
        };

        let wikilinks: Vec<_> = body
            .split("[[")
            .filter_map(|segment| segment.split("]]").next())
            .enumerate()
            .filter(|(idx, _)| *idx != 0)
            .map(|(_, link)| link.to_string())
            .collect();

        Ok(Self {
            path,
            area,
            linked_paths: wikilinks,
        })
    }

    type Error = &'static str;
}

fn main() {
    let Some(Args { vault, area }) = parse_args() else {
        show_help();
        return;
    };

    WalkDir::new(&vault)
        .into_iter()
        .par_bridge()
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
        .flat_map(Note::try_from)
        .filter(|note| {
            if area == "*" {
                return true;
            }

            note.area == area
                || note.area == format!("[[{area}]]")
                || note.area == format!("\"[[{area}]]\"")
        })
        .for_each(|note| {
            println!("{}", note.path.display());
            note.linked_paths
                .iter()
                .for_each(|asset| println!("{}", PathBuf::from_iter([&vault, asset]).display()));
        });
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
