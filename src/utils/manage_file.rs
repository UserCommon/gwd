use super::state_path;
use std::{env, fs, path};

/// Return state from .gwd_state | it's a path of last cd
pub fn read_state() -> std::io::Result<String> {
    create_if_not_exist().unwrap(); // misdesign
    fs::read_to_string(state_path())
}

/// Write state to .gwd_state
pub fn write_state(state: impl Into<String>) -> std::io::Result<()> {
    fs::File::create(state_path()).unwrap();
    fs::write(state_path(), state.into())
}

fn create_if_not_exist() -> std::io::Result<()> {
    if !path::Path::new(&state_path()).exists() {
        fs::File::create(".gwd_state")?;
    }
    Ok(())
}
