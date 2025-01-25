use discord_modloader::{config, paths};
use freya::prelude::*;

use crate::{
    components::custom_button::CustomButton,
    constants,
    utils::{self, images},
};

#[component]
pub fn SelectedProfile(id: String) -> Element {
    let mut ctx = use_context::<Signal<crate::AppState>>();

    let profile_id = id.clone();
    let profile = use_memo(move || ctx().config.profiles.get(&profile_id).cloned());

    match profile() {
        None => {
            rsx!(rect {
                width: "100%",
                height: "100%",
                direction: "vertical",
                padding: "8",
                spacing: "8",
                background: constants::BG_SECONDARY,
                color: "white",

                label {
                    "Profile not found"
                }
            })
        }
        Some(profile) => {
            rsx!(rect {
                width: "100%",
                height: "100%",
                direction: "vertical",
                padding: "8",
                spacing: "8",
                background: constants::BG_SECONDARY,
                color: "white",

                rect {
                    direction: "horizontal",
                    cross_align: "center",
                    height: "48",

                    label {
                        font_size: "18",
                        font_weight: "bold",
                        { profile.profile.name }
                    },

                    rect {
                        width: "fill",
                        direction: "horizontal",
                        main_align: "end",
                        CustomButton {
                            height: "48",
                            bg_anim: (constants::BG_PRIMARY.into(), constants::RED.into()),
                            onclick: {
                                to_owned![id];
                                move |_| {
                                    ctx.write().popup_state = crate::PopupState::DeleteProfile(id.clone());
                                }
                            },

                            label {
                                "Delete Profile"
                            }
                        },
                        CustomButton {
                            height: "48",
                            bg_anim: (constants::BG_PRIMARY.into(), constants::BLURPLE.into()),
                            onclick: {
                                to_owned![id];
                                move |_| {
                                    ctx.write().current_page = crate::CurrentPage::CreateNewProfile(Some(id.clone()));
                                }
                            },

                            label {
                                "Edit Profile"
                            }
                        }
                    }
                },

                rect {
                    direction: "horizontal",
                    cross_align: "center",

                    label {
                        "Instances:"
                    },

                    rect {
                        width: "fill",
                        direction: "horizontal",
                        main_align: "end",
                        CustomButton {
                            bg_anim: (constants::BG_PRIMARY.into(), constants::BLURPLE.into()),
                            height: "48",
                            label {
                                "Add New Instance"
                            },
                            onclick: {
                                to_owned![id];
                                move |_| {
                                    ctx.write().popup_state = crate::PopupState::CreateNewInstance(id.clone());
                                }
                            }
                        }
                    },
                },

                ScrollView {
                    spacing: "4",

                    {profile.instances.iter().map(|(instance_id, instance)| {
                        to_owned![id];
                        rsx!(InstanceEntry { profile_id: id, instance_id: instance_id, instance: instance.clone() })
                    })}
                }
            })
        }
    }
}

#[component]
pub fn InstanceEntry(
    instance_id: String,
    instance: config::Instance,
    profile_id: String,
) -> Element {
    let mut ctx = use_context::<Signal<crate::AppState>>();

    let image = match instance.icon {
        Some(ref icon) => {
            let image = images::get_icon(icon);
            image.map(dynamic_bytes)
        }
        None => None,
    };

    rsx!(rect {
        width: "100%",
        height: "{ 64 + 8 + 8 }",
        padding: "8",
        color: "white",
        corner_radius: "4",
        background: constants::BG_TERTIARY,
        direction: "horizontal",

        image {
            width: "64",
            height: "64",
            image_data: image,
        }

        rect {
            height: "100%",
            main_align: "center",
            padding: "0 0 0 8",

            label {
                font_size: "18",
                font_weight: "bold",
                { instance.name.clone() }
            }

            label {
                { instance.mod_id.clone() }
            }
        }

        rect {
            height: "64",
            width: "fill",
            direction: "horizontal",
            main_align: "end",

            CustomButton {
                bg_anim: (constants::BG_PRIMARY.into(), constants::GREEN.into()),
                onclick: {
                    to_owned![profile_id, instance_id, instance];
                    move |_| {
                        let system = sysinfo::System::new_with_specifics(
                            sysinfo::RefreshKind::nothing()
                                .with_processes(sysinfo::ProcessRefreshKind::everything()),
                        );

                        let profile = ctx().config.profiles.get(&profile_id).unwrap().clone();

                        if system.processes().iter().any(|(_, proc)| {
                            let cmd = proc.cmd().join(std::ffi::OsStr::new(" "));
                            let cmd = cmd.to_str().unwrap().to_string();

                            match !profile.profile.use_default_profile {
                                true => {
                                    let profile_path = paths::data_profiles_dir().join(profile_id.clone());
                                    let profile_path = profile_path.to_str().unwrap();
                                    cmd.contains(&format!("--user-data-dir={}", profile_path))
                                }
                                false => {
                                    // If using the default discord profile (doesn't use our discord-modloader profiles dir)
                                    // and the command contains the discord executable path, it's a duplicate profile.
                                    !cmd.contains(paths::data_profiles_dir().to_str().unwrap()) && cmd.contains(&profile.discord.executable)
                                }
                            }
                        }) {
                            ctx.write().popup_state = crate::PopupState::AlreadyLaunched(profile_id.clone(), instance_id.clone());
                            return;
                        }

                        ctx.write().popup_state = crate::PopupState::Launching(instance.clone());

                        if let Some(profile) = ctx().config.profiles.get(&profile_id) {
                            let result = utils::launch::launch_instance(&profile_id, &instance_id, &profile.discord.args);

                            if result.is_ok() {
                                ctx.write().popup_state = crate::PopupState::Hidden;

                                if ctx().config.settings.hide_window_on_launch {
                                    use_platform().set_minimize_window(true);
                                }
                            }
                        };
                    }
                },

                label {
                    "Launch"
                }
            }

            CustomButton {
                bg_anim: (constants::BG_PRIMARY.into(), constants::BLURPLE.into()),
                onclick: move |_| {
                    // TODO: Implement edit instance
                },

                label {
                    "Edit"
                }
            }

            CustomButton {
                bg_anim: (constants::BG_PRIMARY.into(), constants::RED.into()),
                onclick: move |_| {
                    ctx.write().popup_state = crate::PopupState::DeleteInstance(profile_id.clone(), instance_id.clone());
                },

                label {
                    "Delete"
                }
            }
        }
    })
}
