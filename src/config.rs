use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_volume")]
    pub master_volume: f32,
    #[serde(default = "default_volume")]
    pub music_volume: f32,
    #[serde(default)]
    pub muted: bool,
    #[serde(default = "default_true")]
    pub fullscreen: bool,
    #[serde(default = "default_true")]
    pub mute_on_focus_loss: bool,
    #[serde(default = "default_true")]
    pub ui_sounds: bool,
    #[serde(default)]
    pub reduced_motion: bool,
    #[serde(default)]
    pub settings_tab: usize,
    #[serde(default = "default_game_scroll_speed")]
    pub game_scroll_speed: f32,
    #[serde(default = "default_true")]
    pub window_maximized: bool,
}

fn default_volume() -> f32 { 0.8 }
fn default_true() -> bool { true }
fn default_game_scroll_speed() -> f32 { 1.0 }

impl Default for Config {
    fn default() -> Self {
        Self {
            master_volume: 0.8,
            music_volume: 0.8,
            muted: false,
            fullscreen: true,
            mute_on_focus_loss: true,
            ui_sounds: true,
            reduced_motion: false,
            settings_tab: 0,
            game_scroll_speed: 1.0,
            window_maximized: true,
        }
    }
}

impl Config {
    pub fn load(paths: &Paths) -> Self {
        let path = paths.config_file();
        let Ok(text) = std::fs::read_to_string(&path) else {
            return Self::default();
        };
        toml::from_str::<Self>(&text).unwrap_or_default()
    }

    pub fn save(&self, paths: &Paths) {
        let path = paths.config_file();
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(text) = toml::to_string_pretty(self) {
            let _ = std::fs::write(&path, text);
        }
    }
}

#[derive(Clone)]
pub struct Paths {
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub state_dir: PathBuf,
    pub project_dir: PathBuf,
}

fn runtime_project_dir() -> PathBuf {
    let ps4_app0 = PathBuf::from("/app0");
    if ps4_app0.join("assets").is_dir() {
        return ps4_app0;
    }
    if let Ok(current_dir) = std::env::current_dir()
        && current_dir.join("assets").is_dir()
    {
        return current_dir;
    }
    PathBuf::from(".")
}

impl Paths {
    pub fn new() -> Self {
        let project_dir = runtime_project_dir();
        let base = PathBuf::from("/data/roblox-2016-xbox");
        Self {
            config_dir: base.join("config"),
            data_dir: base.join("data"),
            cache_dir: base.join("cache"),
            state_dir: base.join("state"),
            project_dir,
        }
    }

    pub fn config_file(&self) -> PathBuf {
        self.config_dir.join("config.toml")
    }

    pub fn project_assets_dir(&self) -> PathBuf {
        self.project_dir.join("assets")
    }

    pub fn music_dir(&self) -> PathBuf {
        self.project_assets_dir().join("music")
    }

    fn fallback_music_dir(&self) -> PathBuf {
        self.data_dir.join("music")
    }

    pub fn music_file(&self) -> Option<PathBuf> {
        for dir in [self.music_dir(), self.fallback_music_dir()] {
            if let Some(path) = ["menu.mp3", "menu.wav", "menu.flac"]
                .iter()
                .map(|name| dir.join(name))
                .find(|path| path.is_file())
            {
                return Some(path);
            }
        }
        None
    }

    pub fn sounds_dir(&self) -> PathBuf {
        self.project_assets_dir().join("sounds")
    }

    fn fallback_sounds_dir(&self) -> PathBuf {
        self.data_dir.join("sounds")
    }

    pub fn sfx_file(&self, name: &str) -> Option<PathBuf> {
        for dir in [self.sounds_dir(), self.fallback_sounds_dir()] {
            if let Some(path) = ["mp3", "wav", "flac"]
                .iter()
                .map(|extension| dir.join(format!("{name}.{extension}")))
                .find(|path| path.is_file())
            {
                return Some(path);
            }
        }
        None
    }

    pub fn background_dir(&self) -> PathBuf {
        self.project_assets_dir().join("background")
    }

    fn fallback_background_dir(&self) -> PathBuf {
        self.data_dir.join("background")
    }

    pub fn background_video_file(&self) -> Option<PathBuf> {
        for dir in [self.background_dir(), self.fallback_background_dir()] {
            if let Some(path) = ["menu.mp4", "background.mp4"]
                .iter()
                .map(|name| dir.join(name))
                .find(|path| path.is_file())
            {
                return Some(path);
            }
        }
        None
    }

    pub fn thumbnail_dir(&self) -> PathBuf {
        self.cache_dir.join("thumbnails")
    }

    pub fn outfits_file(&self, user_id: u64) -> PathBuf {
        self.data_dir.join("outfits").join(format!("{user_id}.json"))
    }

    pub fn screenshot_dir(&self) -> PathBuf {
        self.state_dir.join("screenshots")
    }

    pub fn sober_cookie_file(&self) -> PathBuf {
        self.data_dir.join("session_cookie.txt")
    }

    pub fn sober_log_dir(&self) -> PathBuf {
        self.state_dir.join("logs")
    }

    pub fn ensure_directories(&self) {
        for path in [
            &self.config_dir,
            &self.data_dir,
            &self.fallback_music_dir(),
            &self.fallback_sounds_dir(),
            &self.fallback_background_dir(),
            &self.cache_dir,
            &self.thumbnail_dir(),
            &self.state_dir,
            &self.screenshot_dir(),
            &self.sober_log_dir(),
        ] {
            let _ = std::fs::create_dir_all(path);
        }
    }
}

pub fn open_path(path: &Path) -> Result<(), String> {
    Err(format!(
        "Direct folder browsing is unavailable on PS4 ({})",
        path.display()
    ))
}
