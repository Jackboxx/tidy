use std::{
    env,
    fs::{self, DirEntry},
    io,
};

fn main() {
    let target_dir = vec!["node_modules", "target"];
    let ignore_dirs = vec![".cache", ".local", ".config"];

    let Ok(current_dir) = env::current_dir() else {
        eprintln!("failed to access current working directory");
        return;
    };

    let Ok( entries ) = fs::read_dir(current_dir) else {
        eprintln!("failed to read content of current working directory");
        return;
    };

    entries.flatten().for_each(|e| {
        handle_dir(e, &target_dir, &ignore_dirs);
    });
}

fn handle_dir(entry: DirEntry, target_dirs: &[&str], ignore_dirs: &[&str]) {
    if entry.file_type().map(|t| !t.is_dir()).unwrap_or(true) {
        return;
    }

    let path = entry.path();
    let str_path = path.to_string_lossy();

    let name = entry.file_name();
    let name = name.to_str().unwrap_or("");

    if target_dirs.contains(&name) {
        match fs::remove_dir_all(entry.path()) {
            Ok(()) => println!("{str_path}"),
            Err(err) => match err.kind() {
                io::ErrorKind::PermissionDenied => {
                    eprintln!("missing permissions to delete {str_path}");
                }
                _ => {
                    eprintln!("failed to delete {str_path}\n{err}");
                }
            },
        }
    } else if !ignore_dirs.contains(&name) {
        match fs::read_dir(entry.path()) {
            Ok(entries) => entries.flatten().for_each(|e| {
                handle_dir(e, target_dirs, ignore_dirs);
            }),
            Err(err) => match err.kind() {
                io::ErrorKind::PermissionDenied => {
                    eprintln!("missing permissions to read content of {str_path}");
                }
                _ => {
                    eprintln!("failed to read content of {str_path}\n{err}")
                }
            },
        }
    }
}
