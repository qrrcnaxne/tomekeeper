use std::env;
use std::fs;
use std::path::PathBuf;
use serde::Serialize;

#[derive(Debug, Serialize)]
enum DirOrFile {
    Dir(Tree),
    File(PathBuf),
}

#[derive(Debug, Serialize)]
pub struct Tree {
    dir: Option<String>,
    children: Vec<DirOrFile>,
}

impl Tree {
    pub fn new(dir: Option<&str>) -> Tree {
        Tree {
            dir: match dir {
                Some(dir) => Some(fs::canonicalize(dir).unwrap().display().to_string()),
                None => None,
            },
            children: Vec::new(),
        }
    }

    pub fn build_tree(&mut self) {
        let entries: fs::ReadDir;
        match &self.dir {
            Some(dir) => {
                let current_dir = env::current_dir().unwrap();
                let new_dir = current_dir.join(dir);
                entries = fs::read_dir(&new_dir).unwrap();
            }
            None => {
                let current_dir = env::current_dir().unwrap();
                entries = fs::read_dir(&current_dir).unwrap();
            }
        }
        for entry in entries {
            let entry = entry.unwrap();
            if entry.path().is_dir() {
                let mut child_tree = Tree::new(Some(entry.path().to_str().unwrap()));
                child_tree.build_tree();
                self.children.push(DirOrFile::Dir(child_tree));
            } else if entry.path().is_file() {
                self.children.push(DirOrFile::File(entry.path()));
            }
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}
