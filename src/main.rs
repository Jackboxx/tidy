use std::{
    env,
    fs::{self, DirEntry},
    io,
};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    #[arg(short, long, value_delimiter = ',', num_args = 1..)]
    ignore_dirs: Option<Vec<String>>,
    #[arg(short, long, value_delimiter = ',', num_args = 1..)]
    target_dirs: Option<Vec<String>>,
    #[arg(short = 'd', long, default_value_t = 10)]
    max_depth: usize,
}

fn main() {
    let args = CliArgs::parse();

    let target_dir: Vec<_> = args.target_dirs.unwrap_or(
        vec!["node_modules", "target"]
            .into_iter()
            .map(|e| e.to_owned())
            .collect(),
    );

    let ignore_dirs: Vec<_> = args.ignore_dirs.unwrap_or(
        vec![".cache", ".local", ".config"]
            .into_iter()
            .map(|e| e.to_owned())
            .collect(),
    );

    let Ok(current_dir) = env::current_dir() else {
        eprintln!("failed to access current working directory");
        return;
    };

    let Ok( entries ) = fs::read_dir(current_dir) else {
        eprintln!("failed to read content of current working directory");
        return;
    };

    entries.flatten().for_each(|e| {
        handle_dir(
            e,
            0,
            args.max_depth,
            &target_dir.iter().map(|e| e.as_str()).collect::<Vec<_>>(),
            &ignore_dirs.iter().map(|e| e.as_str()).collect::<Vec<_>>(),
        );
    });
}

fn handle_dir(
    entry: DirEntry,
    depth: usize,
    max_depth: usize,
    target_dirs: &[&str],
    ignore_dirs: &[&str],
) {
    if depth >= max_depth || entry.file_type().map(|t| !t.is_dir()).unwrap_or(true) {
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
                handle_dir(e, depth + 1, max_depth, target_dirs, ignore_dirs);
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
