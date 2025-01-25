use crate::{config, paths};

use sysinfo::{Pid, ProcessRefreshKind, RefreshKind, System};

pub fn kill_pids(pids: Vec<Pid>) {
    let system = System::new_all();
    for pid in pids {
        if let Some(proc) = system.process(pid) {
            let _ = proc.kill();
        }
    }
}

pub fn find_running_instances(profile_id: &str, profile: &config::ProfileConfig) -> Vec<Pid> {
    let system = System::new_with_specifics(
        RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()),
    );

    println!("Finding instances for {profile_id}");

    system
        .processes()
        .iter()
        .filter_map(|(_pid, proc)| {
            let cmd = proc.cmd().join(std::ffi::OsStr::new(" "));
            let cmd = cmd.to_str().unwrap().to_string();

            let should_kill = match !profile.profile.use_default_profile {
                false => {
                    // If using the default discord profile (doesn't use our discord-modloader profiles dir)
                    // and the command contains the discord executable path, it's a duplicate profile.
                    !cmd.contains(paths::data_profiles_dir().to_str().unwrap())
                        && cmd.contains(&profile.discord.executable)
                }
                true => {
                    let profile_path = paths::data_profiles_dir().join(profile_id);
                    let profile_path = profile_path.to_str().unwrap();
                    cmd.contains(&format!("--user-data-dir={}", profile_path))
                }
            };

            if should_kill {
                Some(proc.pid())
            } else {
                None
            }
        })
        .collect()
}

pub fn launch_detached_instance(
    profile_id: &str,
    instance_id: &str,
    args: &str,
    force_update: bool,
) -> Result<u32, Box<dyn std::error::Error>> {
    let mut target = std::process::Command::new(std::env::current_exe().unwrap());

    target
        .arg("--profile")
        .arg(profile_id)
        .arg("--instance")
        .arg(instance_id);

    if force_update {
        target.arg("--force-update");
    }

    target
        .arg("--")
        .arg(args)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .stdin(std::process::Stdio::null());

    #[allow(clippy::zombie_processes, reason = "Process detaches from parent.")]
    let target = target.spawn().expect("Failed to launch instance.");

    let pid = target.id();

    Ok(pid)
}
