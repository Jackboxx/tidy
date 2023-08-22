use std::{
    env,
    fs::{self, DirEntry},
    io,
};

const DIRS: &[&str] = &["node_modules", "target"];

fn main() {
    let Ok(current_dir) = env::current_dir() else {
        eprintln!("failed to access current working directory");
        return;
    };

    let Ok( entries ) = fs::read_dir(current_dir) else {
        eprintln!("failed to read content of current working directory");
        return;
    };

    entries.flatten().for_each(|e| {
        handle_dir(e);
    });
}

fn handle_dir(entry: DirEntry) {
    if entry.file_type().map(|t| !t.is_dir()).unwrap_or(true) {
        return;
    }

    let name = entry.file_name();
    let name = name.to_str().unwrap_or("");

    if DIRS.contains(&name) {
        let path = entry.path();
        let str_path = path.to_string_lossy();

        match fs::remove_dir_all(entry.path()) {
            Ok(()) => println!("{str_path}"),
            Err(err) => match err.kind() {
                io::ErrorKind::PermissionDenied => {
                    println!("missing permissions to delete {str_path}")
                }
                _ => println!("failed to delete {str_path}\n{err}"),
            },
        }
    } else {
        handle_dir(entry);
    }
}
