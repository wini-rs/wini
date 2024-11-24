use std::{
    env,
    path::{Path, PathBuf},
};

pub fn search() -> Option<PathBuf> {
    let mut current_dir = env::current_dir().expect("Failed to get current directory");

    loop {
        if check_for_justfile_in_dir(&current_dir) {
            current_dir.push("justfile");
            return Some(current_dir);
        }

        // Move to the parent directory
        if let Some(parent) = current_dir.parent() {
            current_dir = parent.to_path_buf();
        } else {
            return None;
        }
    }
}

fn check_for_justfile_in_dir(dir: &Path) -> bool {
    let path = dir.join("justfile");
    path.exists() && path.is_file()
}
