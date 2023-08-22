use std::{
    env,
    fs::{self, DirEntry},
};

fn main() {
    let dir_names = vec!["node_modules", "target"];
    let current_dir = env::current_dir().unwrap();

    for entry in fs::read_dir(current_dir).unwrap() {
        let entry = entry.unwrap();
        handle_dir(entry, &dir_names);
    }
}

fn handle_dir(entry: DirEntry, dir_names: &[&str]) {
    if entry.file_type().map(|t| !t.is_dir()).unwrap_or(true) {
        return;
    }

    let name = entry.file_name();
    let name = name.to_str().unwrap();

    if dir_names.contains(&name) {
        fs::remove_dir_all(entry.path()).unwrap();
    }
}
