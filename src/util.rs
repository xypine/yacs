use std::{path::PathBuf, env};

/// Get the path of the yacs executable
pub fn get_yacs_path() -> PathBuf {
    let exec_path = env::current_exe().unwrap();
    let actual_path = exec_path.parent().unwrap();
    return actual_path.to_path_buf();
    // env::current_dir().unwrap()
}