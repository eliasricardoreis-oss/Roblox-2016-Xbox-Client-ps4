use std::fs;
use std::path::Path;

pub struct InstanceGuard;

impl InstanceGuard {
    pub fn acquire(state_dir: &Path) -> Option<Self> {
        let _ = fs::create_dir_all(state_dir);
        let path = state_dir.join("instance.lock");
        let _ = fs::write(&path, format!("{}", std::process::id()));
        Some(Self)
    }
}
