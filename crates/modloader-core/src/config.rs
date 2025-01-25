use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::paths::{self, ensure_dir};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfigFile {
    Instance(Instance),
    Mod(Mod),
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    pub profiles: BTreeMap<String, ProfileConfig>,
    pub mods: BTreeMap<String, Mod>,
    pub settings: Settings,
}

impl Config {
    pub fn add_profile(
        &mut self,
        profile_id: &str,
        profile: ProfileConfig,
    ) -> Option<ProfileConfig> {
        self.profiles.insert(profile_id.to_string(), profile)
    }

    pub fn remove_profile(&mut self, profile_id: &str) -> Option<ProfileConfig> {
        self.profiles.remove(profile_id)
    }

    pub fn add_mod(&mut self, mod_id: &str, r#mod: Mod) -> Option<Mod> {
        self.mods.insert(mod_id.to_string(), r#mod)
    }

    pub fn remove_mod(&mut self, mod_id: &str) -> Option<Mod> {
        self.mods.remove(mod_id)
    }

    pub fn sync(&mut self) {
        self.sync_profiles();
        self.sync_mods();
        self.sync_settings();
    }

    pub fn sync_profiles(&mut self) {
        let profiles_config_dir = ensure_dir(paths::config_profile_dir());

        let profile_files = paths::read_dir(&profiles_config_dir);
        for profile in profile_files
            .iter()
            .filter(|p| p.path().to_string_lossy().ends_with(".toml"))
        {
            let id = profile.file_name().to_string_lossy().replace(".toml", "");

            match std::fs::read_to_string(profile.path()) {
                Ok(profile_content) => match toml::from_str::<ProfileConfig>(&profile_content) {
                    Ok(profile) => {
                        self.profiles.insert(id, profile);
                    }
                    Err(_) => eprintln!("Failed to parse profile config: {:?}", profile),
                },
                Err(_) => eprintln!("Failed to read profile config: {:?}", profile),
            }
        }

        self.profiles.retain(|prof_id, _| {
            profile_files.iter().any(|file_name| {
                file_name.file_name().to_string_lossy().replace(".toml", "") == *prof_id
            })
        });
    }

    pub fn sync_mods(&mut self) {
        let mods_config_dir = ensure_dir(paths::config_mods_dir());

        let mod_files = paths::read_dir(&mods_config_dir);
        for mod_ in mod_files
            .iter()
            .filter(|m| m.path().to_string_lossy().ends_with(".toml"))
        {
            let id = mod_.file_name().to_string_lossy().replace(".toml", "");

            match std::fs::read_to_string(mod_.path()) {
                Ok(mod_content) => match toml::from_str::<ModConfig>(&mod_content) {
                    Ok(mod_) => {
                        self.mods.insert(id, mod_.r#mod);
                    }
                    Err(_) => eprintln!("Failed to parse mod config: {:?}", mod_),
                },
                Err(_) => eprintln!("Failed to read mod config: {:?}", mod_),
            }
        }

        self.mods.retain(|mod_id, _| {
            mod_files.iter().any(|mod_file| {
                mod_file.file_name().to_string_lossy().replace(".toml", "") == *mod_id
            })
        });
    }

    pub fn sync_settings(&mut self) {
        let config_dir = ensure_dir(paths::configs_dir());
        let settings_file = config_dir.join("settings.toml");

        self.settings = if let Ok(settings) = std::fs::read_to_string(&settings_file) {
            toml::from_str::<Settings>(&settings).unwrap_or_default()
        } else {
            Settings::default()
        };
    }

    pub fn init() -> Config {
        let mut config = Config::default();

        let config_dir = ensure_dir(paths::configs_dir());
        let settings_file = config_dir.join("settings.toml");

        config.settings = if let Ok(settings) = std::fs::read_to_string(&settings_file) {
            toml::from_str::<Settings>(&settings).unwrap_or_default()
        } else {
            Settings::default()
        };

        config.sync_mods();
        config.sync_profiles();

        config
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

    /// Whether the profile has been starred/favourited.
    ///
    /// Starred profiles will be shown on the homepage of the GUI.
    #[serde(default)]
    pub starred: bool,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ModConfig {
    pub r#mod: Mod,
}

impl ModConfig {
    pub fn save(&self, mod_id: &str) {
        let mods_config_dir = ensure_dir(paths::config_mods_dir());
        let mod_file = mods_config_dir.join(format!("{mod_id}.toml"));

        if let Ok(mod_content) = toml::to_string_pretty(self) {
            if let Err(e) = std::fs::write(mod_file, mod_content) {
                eprintln!("Failed to write mod config: {:?}", e);
            }
        }
    }
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

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ProfileConfig {
    pub profile: Profile,

    #[serde(rename = "instance", default)]
    pub instances: BTreeMap<String, Instance>,

    pub discord: Discord,
}

impl ProfileConfig {
    pub fn save(&self, profile_id: &str) {
        let profiles_config_dir = ensure_dir(paths::config_profile_dir());
        let profile_file = profiles_config_dir.join(format!("{profile_id}.toml"));

        if let Ok(profile_content) = toml::to_string_pretty(self) {
            if let Err(e) = std::fs::write(profile_file, profile_content) {
                eprintln!("Failed to write profile config: {:?}", e);
            }
        }
    }
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
