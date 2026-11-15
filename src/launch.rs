use std::path::{Path, PathBuf};
use std::process::Child;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LaunchRuntime {
    Sober,
    Cordial,
    None,
}

pub fn uri_for_place(place_id: u64) -> String {
    format!("roblox://experiences/start?placeId={place_id}")
}

pub fn spawn_launch(place_id: u64, _runtime: LaunchRuntime) -> Result<Child, String> {
    let uri = uri_for_place(place_id);
    let _ = std::fs::create_dir_all("/data/roblox-2016-xbox");
    std::fs::write("/data/roblox-2016-xbox/pending_launch.txt", &uri)
        .map_err(|e| format!("Failed to write PS4 launch payload: {e}"))?;

    Err("PS4 launch URI queued in /data/roblox-2016-xbox/pending_launch.txt".to_owned())
}

pub fn open_sober() -> Result<Child, String> {
    Err("External runtime launcher is handled by PS4 System Service.".to_owned())
}

pub struct LaunchLog {
    path: PathBuf,
}

impl LaunchLog {
    pub fn snapshot(dir: &Path) -> Self {
        Self {
            path: dir.join("latest.log"),
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn poll(&mut self, _place_id: u64) -> bool {
        false
    }
}
