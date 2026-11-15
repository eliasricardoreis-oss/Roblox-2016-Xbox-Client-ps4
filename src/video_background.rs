use std::path::{Path, PathBuf};
use egui::{Context, TextureHandle};

pub struct VideoBackground {
    path: PathBuf,
}

impl VideoBackground {
    pub fn start(path: &Path) -> Result<Self, String> {
        Err(format!(
            "External ffmpeg decoder disabled on PS4 ({})",
            path.display()
        ))
    }

    pub fn poll(&mut self, _ctx: &Context) -> Option<TextureHandle> {
        None
    }

    pub fn has_frame(&self) -> bool {
        false
    }

    pub fn ended(&self) -> bool {
        true
    }

    pub fn luma(&self) -> f32 {
        0.0
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}
