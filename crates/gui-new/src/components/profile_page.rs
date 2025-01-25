use discord_modloader::config;
use discord_modloader::utils::{find_running_instances, launch_detached_instance};
use freya::prelude::*;

use crate::components::button::Button;
use crate::utils::icons::GetIcon;
use crate::{CONFIG, POPUP_STATE, PopupState, REFRESH_PIDS, THEME};

type ProfileContext = Signal<(String, config::ProfileConfig)>;

fn get_profile_by_id(profile_id: &str) -> (String, config::ProfileConfig) {
    CONFIG
        .read()
        .profiles
        .get(profile_id)
        .map(|profile| (profile_id.to_string(), profile.clone()))
        .unwrap_or_else(|| {
            let profile = config::Profile {
                name: "New Profile".to_string(),
                ..Default::default()
            };
            (profile_id.to_string(), config::ProfileConfig {
                profile,
                ..Default::default()
            })
        })
}

#[component]
pub fn ProfilePage(profile_id: String) -> Element {
    let mut profile: ProfileContext = use_signal(|| get_profile_by_id(&profile_id));
    let mut active_pids = use_signal(Vec::new);

    use_context_provider(|| profile);

    use_effect(use_reactive(&profile_id, move |profile_id| {
        let (profile_id, new_profile) = get_profile_by_id(&profile_id);
        *active_pids.write() = find_running_instances(&profile_id, &new_profile);
        *profile.write() = (profile_id, new_profile);
    }));

    use_effect(move || {
        REFRESH_PIDS.read();
        // Run pid refresh a second later, since Discord doesn't launch instantly.
        spawn(async move {
            let (profile_id_, profile_) = profile.read().clone();
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            // If the page changes, we don't want to update the old pids anymore.
            if profile.read().0 == profile_id_ {
                *active_pids.write() = find_running_instances(&profile_id_, &profile_);
            }
        });
    });

    rsx!(rect {
        direction: "vertical",
        padding: "8",
        spacing: "8",
        Header {
            on_delete_pressed: move |_| {
                *POPUP_STATE.write() = PopupState::ConfirmDeleteProfile(profile_id.clone());
            }
        }

        ActiveInstance {
            active_pids: active_pids,
        }

        ScrollView {
            direction: "vertical",
            spacing: "8",

            for (instance_id, instance) in profile.read().1.instances.iter() {
                InstanceCard {
                    instance_id: instance_id.clone(),
                    instance: instance.clone()
                }
            }
        }
    })
}

#[component]
fn ActiveInstance(active_pids: Signal<Vec<sysinfo::Pid>>) -> Element {
    let ctx: ProfileContext = use_context();
    let profile_id = ctx.read().0.to_string();

    rsx!(rect {
        height: "64",
        width: "fill",
        padding: "8",
        corner_radius: "8",

        background: THEME.read().bg_primary,

        direction: "horizontal",
        spacing: "8",
        main_align: "start",
        cross_align: "center",

        label {
            if active_pids.len() > 0 {
                "There are active instances using this profile."
            } else {
                "No instances are currently running using this profile."
            }
        }

        if active_pids.len() > 0 {
            rect {
                width: "fill",
                direction: "horizontal",
                spacing: "8",
                main_align: "end",

                Button {
                    height: "48",
                    stretch: false,
                    corner_radius: "8",
                    direction: "horizontal",
                    main_align: "center",
                    cross_align: "center",

                    target_color: THEME.read().bg_danger,

                    onpress: move |_| {
                        *POPUP_STATE.write() = PopupState::ConfirmKillProfile(profile_id.clone());
                    },

                    svg {
                        width: "24",
                        height: "24",
                        svg_data: static_bytes(crate::assets::STOP_ICON),
                        fill: THEME.read().text_primary,
                    }

                    label {
                        font_size: "16",
                        "Kill Active Instance"
                    }
                }
            }
        }
    })
}

#[component]
fn InstanceCard(instance_id: String, instance: config::Instance) -> Element {
    let ctx: ProfileContext = use_context();
    let profile_id = ctx.read().0.to_string();
    let profile = ctx.read().1.clone();
    let icon = instance.get_icon().map(dynamic_bytes);
    let mut edit_mode = use_signal(|| false);

    rsx!(rect {
        direction: "vertical",
        spacing: "8",
        corner_radius: "8",

        background: THEME.read().bg_primary,
        padding: "8",

        rect {
            height: "64",
            width: "fill",

            direction: "horizontal",
            spacing: "8",
            main_align: "start",
            cross_align: "center",

            if let Some(icon) = icon {
                image {
                    width: "48",
                    height: "48",
                    image_data: icon,
                }
            }

            label {
                "{instance.name}"
            }

            rect {
                width: "fill",
                direction: "horizontal",
                spacing: "8",
                main_align: "end",

                Button {
                    width: "48",
                    height: "48",
                    stretch: false,
                    padding: "{48/4}",
                    corner_radius: "{48/2}",
                    main_align: "center",
                    cross_align: "center",

                    target_color: THEME.read().bg_success,

                    onpress: {
                        to_owned![profile_id, instance_id];
                        move |_| {
                            let profile_id = profile_id.clone();
                            let instance_id = instance_id.clone();
                            let args = profile.discord.args.clone();

                            if !find_running_instances(&profile_id, &profile).is_empty() {
                                *POPUP_STATE.write() = PopupState::InstanceAlreadyRunning(profile_id, instance_id, args);
                            } else {
                                let _ = launch_detached_instance(&profile_id, &instance_id, &args, false);
                                *REFRESH_PIDS.write() = ();
                            }
                        }
                    },

                    svg {
                        width: "24",
                        height: "24",
                        svg_data: static_bytes(crate::assets::PLAY_ICON),
                        fill: THEME.read().text_primary,
                    }
                }

                Button {
                    width: "48",
                    height: "48",
                    stretch: false,
                    padding: "{48/4}",
                    corner_radius: "{48/2}",
                    main_align: "center",
                    cross_align: "center",

                    target_color: THEME.read().blurple,

                    onpress: move |_| {
                        let current = *edit_mode.read();
                        println!("Edit mode is {current}");
                        *edit_mode.write() = !current;
                    },

                    svg {
                        width: "24",
                        height: "24",
                        svg_data: static_bytes(crate::assets::PEN_ICON),
                        fill: THEME.read().text_primary,
                    }
                }

                Button {
                    width: "48",
                    height: "48",
                    stretch: false,
                    padding: "{48/4}",
                    corner_radius: "{48/2}",
                    main_align: "center",
                    cross_align: "center",

                    target_color: THEME.read().bg_danger,

                    onpress: move |_| {},

                    svg {
                        width: "24",
                        height: "24",
                        svg_data: static_bytes(crate::assets::TRASH_ICON),
                        fill: THEME.read().text_primary,
                    }
                }
            }
        }

        if edit_mode() {
            InstanceEdit {
                instance_id: instance_id.clone(),
                instance: instance.clone()
            }
        }
    })
}

#[component]
fn InstanceEdit(instance_id: String, instance: config::Instance) -> Element {
    let mods = use_signal(|| CONFIG.read().mods.clone());

    // Dropdown doesn't let you use a custom string - only showing the value (key).
    // So we made a struct that implements Display to render the name rather than the key.
    #[derive(Clone, PartialEq)]
    struct SelectedId(String, String);

    impl std::fmt::Display for SelectedId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.1)
        }
    }

    let mut selected_mod = use_signal(|| {
        mods.read()
            .get(&instance.mod_id)
            .map(|m| SelectedId(instance.mod_id, m.name.to_string()))
            .unwrap_or(SelectedId(
                "Unknown Mod".to_string(),
                "Unknown Mod".to_string(),
            ))
    });

    rsx!(rect {
        direction: "vertical",
        spacing: "8",
        label {
            font_size: "16",

            "Test"
        }

        Dropdown {
            value: selected_mod(),
            for (key, value) in mods() {{
                let id = SelectedId(key.clone(), value.name.clone());
                rsx!(DropdownItem {
                    value: id.clone(),
                    onpress: {
                        move |_| {
                            *selected_mod.write() = id.clone();
                        }
                    },

                    label {
                        "{value.name}"
                    }
                })
            }}
        }
    })
}

#[component]
fn Header(on_delete_pressed: EventHandler<()>) -> Element {
    let ctx = use_context::<ProfileContext>();
    let (profile_id, profile) = ctx.read().clone();

    rsx!(rect {
        width: "100%",
        padding: "12 8 12 12",
        corner_radius: "8",
        border: "0 0 2 0 outer {THEME.read().bg_secondary}",

        background: THEME.read().bg_primary,

        direction: "horizontal",
        cross_align: "center",

        spacing: "12",

        label {
            font_size: "18",

            "{profile.profile.name}"
        }

        rect {
            width: "fill",
            direction: "horizontal",
            main_align: "end",
            spacing: "8",

            Button {
                width: "48",
                height: "48",
                stretch: false,
                padding: "{48/4}",
                corner_radius: "{48/2}",
                main_align: "center",
                cross_align: "center",

                onpress: move |_| {
                    let profile_path = discord_modloader::paths::config_profile_dir().join(format!("{}.toml", &profile_id));
                    if profile_path.exists() {
                        if let Err(e) = open::that(profile_path) {
                            eprintln!("Failed to open directory: {}", e);
                        }
                    } else if let Some(parent) = profile_path.parent() {
                        if !parent.exists() {
                            if let Err(e) = std::fs::create_dir_all(parent) {
                                eprintln!("Failed to create directory: {}", e);
                            }
                        }
                        if let Err(e) = open::that(parent) {
                            eprintln!("Failed to open directory: {}", e);
                        }
                    }
                },

                svg {
                    width: "24",
                    height: "24",
                    svg_data: static_bytes(crate::assets::FOLDER_ICON),
                    fill: THEME.read().text_primary,
                }
            }

            Button {
                width: "48",
                height: "48",
                stretch: false,
                padding: "{48/4}",
                corner_radius: "{48/2}",
                main_align: "center",
                cross_align: "center",

                target_color: THEME.read().bg_danger,

                onpress: move |_| on_delete_pressed.call(()),

                svg {
                    width: "24",
                    height: "24",
                    svg_data: static_bytes(crate::assets::TRASH_ICON),
                    fill: THEME.read().text_primary,
                }
            }
        }
    })
}
