// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


mod notes;

#[tauri::command]
fn list_tree() -> Vec<notes::Node> {
    let root = notes::notes_root();
    notes::read_tree(&root, &root)
}


fn main() {
    orion_lib::run()
}
