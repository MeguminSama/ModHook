use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::paths::{self, ensure_dir};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfigFile {
    Instance(Instance),
    Mod(Mod),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub profiles: BTreeMap<String, ProfileConfig>,
    pub mods: BTreeMap<String, Mod>,
    pub settings: Settings,
}

impl Config {
    pub fn init() -> Config {
        let config_dir = ensure_dir(paths::configs_dir());
        let profiles_config_dir = ensure_dir(paths::config_profile_dir());
        let mods_config_dir = ensure_dir(paths::config_mods_dir());

        let settings_file = config_dir.join("settings.toml");
        let settings = if settings_file.exists() {
            if let Ok(settings) = std::fs::read_to_string(&settings_file) {
                toml::from_str::<Settings>(&settings).unwrap_or_default()
            } else {
                eprintln!("Failed to read settings file: {:?}", settings_file);
                Settings::default()
            }
        } else {
            let settings = Settings::default();
            let settings_str = toml::to_string(&settings).unwrap();
            std::fs::write(settings_file, settings_str).unwrap();
            settings
        };

        let mut profile_configs = BTreeMap::new();

        for profile in paths::read_dir(&profiles_config_dir) {
            if !profile.path().to_string_lossy().ends_with(".toml") {
                continue;
            }
            let id = profile.file_name().to_string_lossy().replace(".toml", "");

            let Ok(profile) = std::fs::read_to_string(profile.path()) else {
                eprintln!("Failed to read profile config: {:?}", profile);
                continue;
            };
            let Ok(profile) = toml::from_str::<ProfileConfig>(&profile) else {
                eprintln!("Failed to parse profile config: {:?}", profile);
                continue;
            };

            profile_configs.insert(id, profile);
        }

        let mut mod_configs = BTreeMap::new();

        for mod_ in paths::read_dir(&mods_config_dir) {
            if !mod_.path().to_string_lossy().ends_with(".toml") {
                continue;
            }
            let id = mod_.file_name().to_string_lossy().replace(".toml", "");

            let Ok(mod_) = std::fs::read_to_string(mod_.path()) else {
                eprintln!("Failed to read mod config: {:?}", mod_);
                continue;
            };
            let Ok(mod_) = toml::from_str::<ModConfig>(&mod_) else {
                eprintln!("Failed to read parse config: {:?}", mod_);
                continue;
            };

            mod_configs.insert(id, mod_.r#mod);
        }

        Config {
            profiles: profile_configs,
            mods: mod_configs,
            settings,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Instance {
    // pub id: String,
    /// The display name of the instance. (e.g. "Vencord", "Moonlight", "My Custom Instance")
    ///
    /// Can be duplicate, but not recommended for clarity.
    pub name: String,

    /// A path to the icon to use for the mod.
    pub icon: Option<String>,

    /// The identifier of the mod to use for this instance.
    ///
    /// Internal mods (ones with build-in updaters) will be prefixed with "Internal::"
    pub mod_id: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ModConfig {
    pub r#mod: Mod,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Mod {
    /// The display name of the mod. (e.g. "Vencord", "Moonlight", "BetterDiscord")
    ///
    /// Can be duplicate, but not recommended for clarity.
    pub name: String,

    /// The path to the mod's dist folder. (e.g. "/path/to/moonlight")
    /// If none, there should be an updater.
    pub path: Option<String>,

    /// The entrypoint of the mod. (e.g. "injector.js", "patcher.js")
    pub entrypoint: String,

    /// A path to the icon to use for the mod.
    pub icon: Option<String>,

    /// Provide custom loader JS to be injected into the ASAR index.js.
    pub loader: Option<ModLoader>,

    /// The updater configuration for the mod.
    pub updater: Option<ModUpdater>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct ModUpdater {
    pub github_org: String,
    pub github_repo: String,
    pub dist_file_names: Vec<String>,
    pub dist_file_type: DistFileType,
    pub icon_url: Option<String>,
    #[serde(default = "default_bool_true")]
    pub ask_before_update: bool,
    #[serde(default = "default_bool_true")]
    pub auto_update: bool,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq, strum::EnumIter, Hash)]
pub enum DistFileType {
    #[default]
    Raw,
    TarGz,
    Zip,
}

impl std::fmt::Display for DistFileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DistFileType::Raw => write!(f, "plain file"),
            DistFileType::TarGz => write!(f, "tar.gz"),
            DistFileType::Zip => write!(f, "zip"),
        }
    }
}

/// The loader configuration for the mod.
/// You can use this to specify custom JS to be in your ASAR's index.js.
///
/// The following variables can be used:
///
/// - `$PROFILE`: The directory of the custom profile.
/// - `$ENTRYPOINT`: The entrypoint file of the mod.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct ModLoader {
    pub prefix: Option<String>,
    #[serde(default = "ModLoader::default_require")]
    pub require: Option<String>,
    pub suffix: Option<String>,
}

impl ModLoader {
    pub fn default_require() -> Option<String> {
        Some(include_str!("./asar/require.js").to_string())
    }
}

// [profile]
// name = "test"
// [[instance]]
// ...

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ProfileConfig {
    pub profile: Profile,

    #[serde(rename = "instance", default)]
    pub instances: BTreeMap<String, Instance>,

    pub discord: Discord,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Profile {
    pub name: String,
    #[serde(default)]
    pub use_default_profile: bool,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Discord {
    pub executable: String,
    #[serde(default)]
    pub args: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Settings {
    #[serde(default = "default_bool_true")]
    pub hide_window_on_launch: bool,
}

fn default_bool_true() -> bool {
    true
}
