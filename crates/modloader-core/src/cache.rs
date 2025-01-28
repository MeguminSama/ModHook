use crate::{config, paths, updater};

pub fn create_instance_cache(
    cfg: &crate::config::Config,
    profile_id: &str,
    instance_id: &str,
    force_update: bool,
) -> Result<std::path::PathBuf, String> {
    static ASAR_CUSTOM_PROFILE_JS: &str = include_str!("./asar/custom_profile.js");

    let profile = cfg
        .profiles
        .get(profile_id)
        .unwrap_or_else(|| panic!("Failed to find profile '{}'.", profile_id));

    let instance = profile
        .instances
        .get(instance_id)
        .unwrap_or_else(|| panic!("Failed to find instance '{}'.", instance_id));

    let r#mod = cfg
        .mods
        .get(&instance.mod_id)
        .unwrap_or_else(|| panic!("Failed to find mod '{}'.", instance.mod_id))
        .to_owned();

    // TODO: Make this more robust
    if let Some(ref updater) = r#mod.updater {
        let _ = updater::update(updater, force_update);
    }

    let asar_cache_path = paths::cache_asar_path(profile_id, instance_id, &instance.mod_id);

    paths::ensure_dir(asar_cache_path.parent().unwrap().to_path_buf());

    let profile_dir = if !profile.profile.use_default_profile {
        let profile_dir = paths::ensure_dir(paths::data_profiles_dir().join(profile_id));
        let profile_dir = profile_dir.to_str().unwrap().replace("\\", "\\\\");
        Some(profile_dir)
    } else {
        None
    };

    // If path is not provided, try to use the updater instead.
    let mod_path = if let Some(path) = r#mod.path {
        path
    } else if let Some(updater) = r#mod.updater {
        paths::cache_mod_files_dir()
            .join(&updater.github_org)
            .join(&updater.github_repo)
            .to_string_lossy()
            .to_string()
    } else {
        panic!("Mod '{}' does not have a path or updater.", r#mod.name);
    };

    let mod_entrypoint = std::path::Path::new(&mod_path).join(&r#mod.entrypoint);
    let mod_entrypoint = mod_entrypoint.to_str().unwrap().replace("\\", "\\\\");

    let mut custom_loader = String::from("console.log(\"Launching with Discord Modloader.\");\n");

    // If using a custom profile directory, insert this.
    if profile_dir.is_some() {
        custom_loader.push_str(ASAR_CUSTOM_PROFILE_JS);
    }

    let loader = if let Some(ref loader) = r#mod.loader {
        [
            loader.prefix.clone().unwrap_or_default(),
            loader
                .require
                .clone()
                .unwrap_or(config::ModLoader::default_require().unwrap()),
            loader.suffix.clone().unwrap_or_default(),
        ]
        .join("\n")
    } else {
        config::ModLoader::default_require().unwrap()
    };

    custom_loader.push_str(&loader);

    let mut asar = electron_hook::asar::Asar::new()
        .with_id(&instance.mod_id)
        .with_template(&custom_loader)
        .with_mod_entrypoint(&mod_entrypoint);

    if let Some(profile_dir) = profile_dir {
        asar = asar.with_profile_dir(&profile_dir);
    }

    asar.create()
}
