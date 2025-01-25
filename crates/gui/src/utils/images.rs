const LIB_PATH: &str = "/usr/lib/discord-modloader";

fn get_icons_from_dir(path: std::path::PathBuf) -> Vec<(String, Vec<u8>)> {
    let mut icons = Vec::new();

    if path.exists() && path.is_dir() {
        if let Ok(files) = std::fs::read_dir(path) {
            for entry in files {
                let entry = entry.unwrap();
                let name = entry.file_name().into_string().unwrap();
                let icon = std::fs::read(entry.path()).unwrap();
                icons.push((name, icon));
            }
        }
    }

    icons
}

pub fn get_all_icons() -> Vec<(String, Vec<u8>)> {
    let mut icons = Vec::new();

    let config_path = dirs::config_local_dir().unwrap().join("discord-modloader");
    let icon_path = config_path.join("icons");
    icons.extend(get_icons_from_dir(icon_path));

    let lib_path = std::path::PathBuf::from(LIB_PATH);
    let icon_path = lib_path.join("icons");
    icons.extend(get_icons_from_dir(icon_path));

    let current_exe = std::env::current_exe().unwrap();
    let icon_path = current_exe.with_file_name("icons");
    icons.extend(get_icons_from_dir(icon_path));

    let cwd = std::env::current_dir().unwrap();
    let icon_path = cwd.join("icons");
    icons.extend(get_icons_from_dir(icon_path));

    let icon_path = cwd.join("configs").join("icons");
    icons.extend(get_icons_from_dir(icon_path));

    // deduplicate

    let mut seen = std::collections::HashSet::new();
    icons.retain(|(name, _)| seen.insert(name.clone()));

    icons
}

pub fn get_icon(name: &str) -> Option<Vec<u8>> {
    let config_path = dirs::config_local_dir().unwrap().join("discord-modloader");
    let icon_path = config_path.join("icons").join(name);
    if icon_path.exists() {
        return std::fs::read(icon_path).ok();
    }

    let lib_path = std::path::PathBuf::from(LIB_PATH);

    let icon_path = lib_path.join("icons").join(name);
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
