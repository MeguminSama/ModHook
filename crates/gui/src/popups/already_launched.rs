use discord_modloader::paths;
use freya::prelude::*;

use crate::{components::custom_button::CustomButton, constants, utils};

#[component]
pub fn AlreadyLaunchedPopup(profile_id: String, instance_id: String) -> Element {
    let mut ctx = use_context::<Signal<crate::AppState>>();

    let mut relaunch_instance = move || {
        ctx.write().popup_state = super::PopupState::Hidden;

        let system = sysinfo::System::new_with_specifics(
            sysinfo::RefreshKind::nothing()
                .with_processes(sysinfo::ProcessRefreshKind::everything()),
        );

        let profile = ctx().config.profiles.get(&profile_id).unwrap().clone();

        system.processes().iter().for_each(|(_pid, proc)| {
            let cmd = proc.cmd().join(std::ffi::OsStr::new(" "));
            let cmd = cmd.to_str().unwrap().to_string();

            let should_kill = match !profile.profile.use_default_profile {
                true => {
                    let profile_path = paths::data_profiles_dir().join(profile_id.clone());
                    let profile_path = profile_path.to_str().unwrap();
                    cmd.contains(&format!("--user-data-dir={}", profile_path))
                }
                false => {
                    // If using the default discord profile (doesn't use our discord-modloader profiles dir)
                    // and the command contains the discord executable path, it's a duplicate profile.
                    !cmd.contains(paths::data_profiles_dir().to_str().unwrap())
                        && cmd.contains(&profile.discord.executable)
                }
            };

            if should_kill {
                proc.kill_with(sysinfo::Signal::Kill);
            }
        });

        if let Some(profile) = ctx().config.profiles.get(&profile_id) {
            let _ =
                utils::launch::launch_instance(&profile_id, &instance_id, &profile.discord.args);
        };
    };

    rsx!(Popup {
        theme: theme_with!(PopupTheme {
            background: constants::BG_PRIMARY.into(),
            color: constants::TEXT_PRIMARY.into(),
        }),
        close_on_escape_key: true,

        oncloserequest: move |_| {
            ctx.write().popup_state = super::PopupState::Hidden;
        },
        PopupTitle {
            label {
                "Instance already running"
            }
        },
        PopupContent {
            label {
                "An instance of this profile is already running."
            }

            label {
                "Would you like to kill the existing process, and launch a new one?"
            }

            rect {
                height: "fill",
                width: "100%",
                main_align: "end",
                cross_align: "end",
                CustomButton {
                    bg_anim: (constants::RED.into(), constants::BLURPLE.into()),
                    height: "32",
                    padding: "0 0 8 0",
                    label {
                        "Kill & Relaunch"
                    }
                    onclick: move |_| {
                        relaunch_instance();
                    }
                }
            }

        }
    })
}
