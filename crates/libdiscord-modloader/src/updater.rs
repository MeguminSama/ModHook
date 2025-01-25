pub fn update(config: &crate::config::ModUpdater, force: bool) -> Result<(), String> {
    use octocrab::models::repos::Release;

    println!("Checking for mod updates...");

    let metadata_dir = crate::paths::cache_mod_metadata_dir()
        .join(&config.github_org)
        .join(&config.github_repo);

    if !metadata_dir.exists() {
        std::fs::create_dir_all(&metadata_dir)
            .map_err(|e| format!("Failed to create metadata directory: {}", e))?;
    }

    let output_dir = crate::paths::cache_mod_files_dir()
        .join(&config.github_org)
        .join(&config.github_repo);

    if !output_dir.exists() {
        std::fs::create_dir_all(&output_dir)
            .map_err(|e| format!("Failed to create mod cache directory: {}", e))?;
    }

    let release_url = format!(
        "https://api.github.com/repos/{}/{}/releases/latest",
        &config.github_org, &config.github_repo
    );

    let release_info: Release = ureq::get(&release_url)
        .call()
        .map_err(|e| format!("Failed to get release: {}", e))?
        .into_json()
        .map_err(|e| format!("Failed to parse release: {}", e))?;

    // If the version IDs are the same, and force is not enabled, then return early.
    let release_version_file = metadata_dir.join("release.json");
    if !force && release_version_file.exists() {
        if let Ok(file) = std::fs::File::open(&release_version_file) {
            if let Ok(release) = serde_json::from_reader::<_, Release>(file) {
                // Vencord only changes the name, and doesn't create new releases...
                if release.tag_name == release_info.tag_name && release.name == release_info.name {
                    println!(
                        "Mod is already the latest version... ({})",
                        release.tag_name
                    );
                    return Ok(());
                }
            }
        }
    }

    for dist_file in &config.dist_file_names {
        // TODO: Should the download URL be pulled from the release assets?
        // Will be a bit annoying if there are multiple assets though.
        let download_url = format!(
            "https://github.com/{}/{}/releases/download/{}/{}",
            config.github_org, config.github_repo, release_info.tag_name, &dist_file
        );

        let mut reader = ureq::get(&download_url)
            .call()
            .map_err(|e| format!("Failed to download release tarball: {}", e))?
            .into_reader();

        match config.dist_file_type {
            crate::config::DistFileType::TarGz => {
                use flate2::read::GzDecoder;
                use tar::Archive;

                let tar = GzDecoder::new(reader);
                let mut archive = Archive::new(tar);

                // TODO: If files are already present, should they be deleted?

                archive
                    .unpack(&output_dir)
                    .map_err(|e| format!("Failed to unpack tarball: {}", e))?;
            }
            crate::config::DistFileType::Zip => {
                use std::io::Cursor;
                use zip::read::ZipArchive;

                let mut bytes = Vec::new();
                reader
                    .read_to_end(&mut bytes)
                    .map_err(|e| format!("Failed to read zip file: {}", e))?;

                let reader = Cursor::new(bytes.as_slice());

                let mut zip = ZipArchive::new(reader)
                    .map_err(|e| format!("Failed to read zip file: {}", e))?;

                zip.extract(&output_dir)
                    .map_err(|e| format!("Failed to extract zip file: {}", e))?;
            }
            crate::config::DistFileType::Raw => {
                let mut file = std::fs::File::create(output_dir.join(dist_file))
                    .map_err(|e| format!("Failed to create raw file: {}", e))?;

                std::io::copy(&mut reader, &mut file)
                    .map_err(|e| format!("Failed to write raw file: {}", e))?;
            }
        }
    }

    let version_content = serde_json::to_string(&release_info)
        .map_err(|e| format!("Failed to serialize mod release version: {}", e))?;

    std::fs::write(&release_version_file, version_content)
        .map_err(|e| format!("Failed to write version to file: {}", e))?;

    if let Some(ref icon_url) = config.icon_url {
        let mut icon_file_bytes = Vec::new();
        ureq::get(icon_url)
            .call()
            .map_err(|e| format!("Failed to download mod icon: {}", e))?
            .into_reader()
            .read_to_end(&mut icon_file_bytes)
            .map_err(|e| format!("Failed to read mod icon: {}", e))?;

        let icon_file_path = metadata_dir.join("icon.png");

        std::fs::write(&icon_file_path, &mut icon_file_bytes)
            .map_err(|e| format!("Failed to write mod icon: {}", e))?;
    }

    println!("Finished updating mod!");
    Ok(())
}
