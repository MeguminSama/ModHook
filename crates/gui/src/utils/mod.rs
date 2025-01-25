use std::path::PathBuf;

pub mod hoverable;
pub mod images;
pub mod launch;

macro_rules! p2s {
    ($path:expr) => {
        $path.to_str().unwrap().to_string()
    };
}

// from: https://github.com/Vencord/Installer
const DISCORD_NAMES: &[&str] = &[
    "Discord",
    "DiscordPTB",
    "DiscordCanary",
    "DiscordDevelopment",
    "discord",
    "discordptb",
    "discordcanary",
    "discorddevelopment",
    "discord-ptb",
    "discord-canary",
    "discord-development",
    // Flatpak. Untested. May not work.
    "com.discordapp.Discord",
    "com.discordapp.DiscordPTB",
    "com.discordapp.DiscordCanary",
    "com.discordapp.DiscordDevelopment",
];

#[cfg(target_os = "linux")]
fn paths_to_check() -> Vec<String> {
    let home = dirs::home_dir().unwrap();
    let local_share = home.join(".local").join("share");
    let dvm = home.join(".dvm").join("branches");

    vec![
        "/usr/share".to_string(),
        "/usr/lib64".to_string(),
        "/opt".to_string(),
        p2s!(local_share),
        p2s!(home.join(".local").join("share")),
        p2s!(dvm.join("canary")),
        p2s!(dvm.join("ptb")),
        p2s!(dvm.join("stable")),
        p2s!(dvm.join("development")),
        // Flatpak. Untested. May not work.
        "/var/lib/flatpak/app".to_string(),
        p2s!(local_share.join("flatpak").join("app")),
    ]
}

#[cfg(target_os = "linux")]
pub fn find_discord_installations() -> Vec<String> {
    let mut instances = Vec::new();

    for dir in paths_to_check() {
        let children = match std::fs::read_dir(&dir) {
            Ok(children) => children,
            Err(_) => continue,
        };

        for child in children.flatten() {
            let metadata = match child.metadata() {
                Ok(metadata) => metadata,
                Err(_) => continue,
            };

            if !metadata.is_dir() {
                continue;
            }

            let name = match child.file_name().to_str() {
                Some(name) => name,
                None => continue,
            }
            .to_string();

            // check if name contains any of the discord names
            if DISCORD_NAMES.iter().any(|&x| name.ends_with(x)) {
                let path = parse_discord_instance(child.path());
                if let Some(path) = path {
                    instances.push(path);
                }
            }
        }
    }

    instances
}

#[cfg(target_os = "linux")]
fn parse_discord_instance(path: PathBuf) -> Option<String> {
    DISCORD_NAMES.iter().find_map(|n| {
        let path = path.join(n);
        match path.exists() {
            true => Some(p2s!(path)),
            false => None,
        }
    })
}
