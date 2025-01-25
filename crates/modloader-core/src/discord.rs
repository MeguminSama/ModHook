/// Returns the path to the Discord executable based on the Discord folder.
///
/// e.g. "C:\Users\User\AppData\Local\discordptb" -> "C:\Users\User\AppData\Local\discordptb\app-1.0.9023\Discord.exe"
#[cfg(target_os = "windows")]
pub fn get_discord_exe(path: &str) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let mut new_path = std::path::PathBuf::from(path);

    let new_path_executable = new_path.clone();
    let executable_name = new_path_executable
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or("Failed to read executable name")?;

    let versions: Vec<_> = new_path
        .read_dir()?
        .filter_map(|p| {
            let file_name = p.ok()?.file_name();
            let file_name_str = file_name.to_str()?;
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

    // In the format of: app-1.0.9023
    let mut sorted: Vec<(String, u32)> = versions
        .into_iter()
        .filter_map(|v| {
            let mut split = v.split('-');
            let version = split.nth(1)?;
            let version = version.replace('.', "");
            let version = version.parse::<u32>().ok()?;
            Some((v, version))
        })
        .collect();

    sorted.sort_by(|(_, a), (_, b)| a.cmp(b));

    let Some(latest_version) = sorted.last().cloned() else {
        return Err("Failed to get latest version".into());
    };

    new_path.push(latest_version.0);
    new_path.push(format!("{}.exe", executable_name));

    if !new_path.exists() {
        return Err("Discord.exe not found".into());
    }

    Ok(new_path)
}
