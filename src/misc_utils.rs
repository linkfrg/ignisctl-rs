use std::{fs, path::Path};

pub fn resolve_path(raw_string: &String) -> String {
    let raw_path = Path::new(raw_string);

    match fs::canonicalize(raw_path) {
        Ok(absolute_path) => return String::from(absolute_path.to_str().unwrap()),
        Err(e) => {
            eprintln!("Failed to resolve path: {e}");
            std::process::exit(1);
        }
    }
}
