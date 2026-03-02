use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use tauri::api::path::document_dir;

#[derive(Serialize)]
pub struct Node {
    pub name: String,
    pub path: String,
    pub node_type: String, // "folder" | "file"
    pub children: Option<Vec<Node>>,
}


pub fn notes_root() -> PathBuf {
    let mut path = document_dir().expect("Cannot access Documents directory");
    path.push("orion");
    path.push("notes");

    if !path.exists() {
        fs::create_dir_all(&path).expect("Failed to create notes directory");
    }

    path
}


// function to walk a directory and convert filesystem into tree structure
pub fn read_tree(dir: &Path, base: &Path) -> Vec<Node> {
    let mut nodes = Vec::new();

    let entries = fs::read_dir(dir).expect("Failed to read directory");

    for entry in entries {
        let entry = entry.expect("Invalid directory entry");
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        let relative_path = path
            .strip_prefix(base)
            .unwrap()
            .to_string_lossy()
            .to_string();

        if path.is_dir() {
            nodes.push(Node {
                name,
                path: relative_path,
                node_type: "folder".into(),
                children: Some(read_tree(&path, base)),
            });
        } else {
            nodes.push(Node {
                name,
                path: relative_path,
                node_type: "file".into(),
                children: None,
            });
        }
    }

    nodes
}
