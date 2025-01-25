use std::{error::Error, path::PathBuf};

/// Returns the path to the Discord executable based on the Discord folder.
///
/// e.g. "C:\Users\Megu\AppData\Local\discordptb" -> "C:\Users\Megu\AppData\Local\discordptb\app-1.0.9023\Discord.exe"
pub fn get_discord_exe(path: &str) -> Result<PathBuf, Box<dyn Error>> {
    let mut new_path = PathBuf::from(path);

    let new_path_executable = new_path.clone();
    let executable_name = new_path_executable.file_name().unwrap().to_str().unwrap();

    let versions: Vec<_> = new_path
        .read_dir()?
        .filter_map(|p| {
            let path = p.unwrap();
            let file_name = path.file_name();
            let file_name_str = file_name.to_str().unwrap();
            if file_name_str.starts_with("app-") {
                Some(file_name_str.to_string())
            } else {
                None
            }
        })
        .collect();

    if versions.is_empty() {
        return Err("No discord versions found".into());
    }

    // app-1.0.9023

    let mut sorted: Vec<(&String, u32)> = versions
        .iter()
        .filter_map(|v| {
            let mut split = v.split('-');
            let version = split.nth(1).unwrap();
            let version = version.replace('.', "");
            let version = version.parse::<u32>();
            if let Ok(version) = version {
                Some((v, version))
            } else {
                None
            }
        })
        .collect();

    sorted.sort_by(|(_, a), (_, b)| a.cmp(b));

    let latest_version = sorted.last().unwrap().0;

    new_path.push(latest_version);
    new_path.push(format!("{}.exe", executable_name));

    if !new_path.exists() {
        return Err("Discord.exe not found".into());
    }

    Ok(new_path)
}
