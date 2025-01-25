use freya::prelude::Readable as _;
use modloader_core::{config, paths};

use crate::CONFIG;

#[cfg(target_os = "linux")]
const LIB_PATH: &str = "/usr/lib/discord-modloader";

pub trait GetIcon<T> {
    fn get_icon(&self) -> Option<Vec<u8>>;
}

// TODO: Should profiles have icons?

impl GetIcon<Self> for config::Instance {
    fn get_icon(&self) -> Option<Vec<u8>> {
        let name = {
            if let Some(ref name) = self.icon {
                Some(name.clone())
            } else if let Some(mod_) = CONFIG.peek().mods.get(&self.mod_id) {
                return mod_.get_icon();
            } else {
                None
            }
        }?;

        scan_icon_by_name(&name)
    }
}

impl GetIcon<Self> for config::Mod {
    fn get_icon(&self) -> Option<Vec<u8>> {
        let name = {
            if let Some(ref name) = self.icon {
                Some(name.to_string())
            } else {
                self.updater.as_ref().map(|updater| {
                    format!("{}/{}/icon.png", updater.github_org, updater.github_repo)
                })
            }
        }?;

        scan_icon_by_name(&name)
    }
}

fn scan_icon_by_name(name: &str) -> Option<Vec<u8>> {
    let config_path = dirs::config_local_dir().unwrap().join("discord-modloader");
    let icon_path = config_path.join("icons").join(name);
    if icon_path.exists() {
        return std::fs::read(icon_path).ok();
    }

    // TODO: Get windows/macos libpath
    #[cfg(target_os = "linux")]
    {
        let lib_path = std::path::PathBuf::from(LIB_PATH);

        let icon_path = lib_path.join("icons").join(name);
        if icon_path.exists() {
            return std::fs::read(icon_path).ok();
        }
    }

    let metadata_dir = paths::cache_mod_metadata_dir();
    let icon_path = metadata_dir.join(name);
    if icon_path.exists() {
        return std::fs::read(icon_path).ok();
    }

    let current_exe = std::env::current_exe().unwrap();
    let icon_path = current_exe.with_file_name("icons").join(name);
    if icon_path.exists() {
        return std::fs::read(icon_path).ok();
    }

    let cwd = std::env::current_dir().unwrap();
    let icon_path = cwd.join("icons").join(name);
    if icon_path.exists() {
        return std::fs::read(icon_path).ok();
    }

    let icon_path = cwd.join("configs").join("icons").join(name);
    if icon_path.exists() {
        return std::fs::read(icon_path).ok();
    }

    None
}
