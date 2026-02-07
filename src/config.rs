use crate::logger::LogLevel;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;

const CONFIG_PATH: &str = "config.json";

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct Config {
    pub input: InputConfig,
    pub output: OutputConfig,
    pub logger: LoggerConfig,
}

impl Config {
    pub fn load() -> Self {
        if !fs::exists(CONFIG_PATH).expect("設定ファイルを読み込めませんでした") {
            if fs::File::create(CONFIG_PATH).is_err() {
                log::error!("設定ファイルを作成できませんでした。");
            }
            Config::save(&Config::default());
        }

        if let Ok(config_text) = fs::read_to_string(CONFIG_PATH)
            && let Ok(config) = serde_json::from_str::<Config>(&config_text)
        {
            config
        } else {
            log::warn!("設定ファイルが正しく読み込めませんでした。デフォルト設定で続行します。");
            Self::default()
        }
    }
    pub fn save(&self) {
        if fs::write(CONFIG_PATH, serde_json::to_string_pretty(self).unwrap()).is_err() {
            log::error!("設定ファイルを書き込めませんでした。");
        };
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub enum Codec {
    #[default]
    WebP,
    Png,
    Tiff,
}

impl std::fmt::Display for Codec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Codec::WebP => "webp",
                Codec::Png => "png",
                Codec::Tiff => "tiff",
            }
        )?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct InputConfig {
    pub log_path: String,
    pub picture_path: String,
}

impl Default for InputConfig {
    fn default() -> Self {
        #[cfg(target_os = "linux")]
        let vrc_home_dir = String::from(
            env::home_dir().unwrap().to_string_lossy()
                + "/.local/share/Steam/steamapps/compatdata/438100/pfx/drive_c/users/steamuser",
        );
        #[cfg(target_os = "windows")]
        let vrc_home_dir = String::from(env::home_dir().unwrap().to_string_lossy());
        {
            Self {
                log_path: (vrc_home_dir.clone() + "/AppData/LocalLow/VRChat/VRChat"),
                picture_path: (vrc_home_dir + "/Pictures/VRChat/"),
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct OutputConfig {
    pub codec: Codec,
    pub save_path: String,
    pub name: String,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            codec: Codec::default(),
            save_path: String::from(env::home_dir().unwrap().to_string_lossy() + "/Pictures"),
            name: String::from("VRChat_yyyy-MM-dd_hh-mm-ss.fff_XXXXxYYYY"),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct LoggerConfig {
    pub log_level: LogLevel,
}
