use std::process::Stdio;

#[cfg(target_os = "linux")]
pub fn launch_instance(
    profile_id: &str,
    instance_id: &str,
    args: &str,
) -> Result<u32, Box<dyn std::error::Error>> {
    #[allow(clippy::zombie_processes, reason = "Process detaches from parent.")]
    let target = std::process::Command::new(std::env::current_exe().unwrap())
        .arg("--profile")
        .arg(profile_id)
        .arg("--instance")
        .arg(instance_id)
        .arg("--")
        .arg(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .spawn()
        .expect("Failed to launch instance.");

    let pid = target.id();

    Ok(pid)
}

#[cfg(target_os = "windows")]
pub fn launch_instance(
    profile_id: &str,
    instance_id: &str,
    args: &str,
) -> Result<u32, Box<dyn std::error::Error>> {
    todo!();
}
