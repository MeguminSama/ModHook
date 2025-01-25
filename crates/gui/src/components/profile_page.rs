use freya::prelude::*;
use modloader_core::utils::{find_running_instances, launch_detached_instance};
use modloader_core::{config, paths};

use crate::components::button::Button;
use crate::utils::icons::GetIcon;
use crate::{CONFIG, POPUP_STATE, PopupState, REFRESH_PIDS, THEME};

type ProfileContext = Memo<(String, config::ProfileConfig)>;

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
    let profile_id = use_memo(use_reactive!(|(profile_id,)| profile_id));
    let profile: ProfileContext = use_memo(move || get_profile_by_id(&profile_id()));
    use_context_provider(|| profile);

    let mut active_pids = use_signal(Vec::new);

    // When the PIDs refresh, update the active PIDs list in the current component.
    use_effect(move || {
        REFRESH_PIDS.read();
        let (profile_id_, profile_) = profile();
        spawn(async move {
            if profile().0 == profile_id_ {
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
                *POPUP_STATE.write() = PopupState::ConfirmDeleteProfile(profile_id());
            }
        }

        if active_pids.len() > 0 {
            ActiveInstance {
                active_pids: active_pids,
            }
        }

        ScrollView {
            direction: "vertical",
            spacing: "8",
            scrollbar_theme: theme_with!(ScrollBarTheme {
                background: THEME.read().bg_tertiary.into(),
            }),

            for (instance_id, instance) in profile().1.instances.iter() {
                InstanceCard {
                    instance_id: instance_id.clone(),
                    instance: instance.clone()
                }
            }

            rect {
                width: "fill",
                direction: "horizontal",
                main_align: "end",

                Button {
                    stretch: false,

                    onpress: move |_| {
                        if let Some(profile) = CONFIG.write().profiles.get_mut(&profile_id()) {
                            profile.instances.insert(uuid::Uuid::new_v4().to_string(), config::Instance {
                                mod_id: "Unknown Mod".to_string(),
                                name: "New Instance".to_string(),
                                icon: None,
                                starred: false,
                            });
                        }
                    },

                    svg {
                        width: "24",
                        height: "24",
                        svg_data: static_bytes(crate::assets::PLUS_ICON),
                        fill: THEME.read().text_primary,
                    }

                    label {
                        font_size: "16",
                        "Add New Instance"
                    }
                }
            }
        }
    })
}

#[component]
fn ActiveInstance(active_pids: Signal<Vec<sysinfo::Pid>>) -> Element {
    let ctx: ProfileContext = use_context();
    let profile_id = ctx().0.to_string();

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
            "There are active instances using this profile."
        }

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
    })
}

#[component]
fn InstanceCard(instance_id: String, instance: config::Instance) -> Element {
    let instance_id = use_memo(use_reactive(&instance_id, |i| i));
    let instance = use_memo(use_reactive(&instance, |i| i));

    let ctx: ProfileContext = use_context();
    let profile_id = use_memo(move || ctx().0);
    let profile = use_memo(move || ctx().1);
    let icon = use_memo(move || instance().get_icon().map(dynamic_bytes));

    let mut edit_mode = use_signal(|| false);
    let mut starred = use_signal(|| instance().starred);

    use_effect(move || {
        instance_id.read();
        edit_mode.set(false);
    });

    // use_effect(use_reactive(&instance_id, move |instance_id| {
    //     let profile = ctx.read().1.clone();
    //     let instance = profile.instances.get(&instance_id).cloned();
    //     *edit_mode.write() = false;
    //     if let Some(instance) = instance {
    //         *starred.write() = instance.starred;
    //     }
    // }));

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

            if let Some(icon) = icon() {
                image {
                    width: "48",
                    height: "48",
                    image_data: icon,
                }
            }

            label {
                "{instance().name}"
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
                        move |_| {
                            let args = profile().discord.args;

                            if !find_running_instances(&profile_id(), &profile()).is_empty() {
                                *POPUP_STATE.write() = PopupState::InstanceAlreadyRunning(profile_id(), instance_id(), args);
                            } else {
                                let _ = launch_detached_instance(&profile_id(), &instance_id(), &args, false);
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
                        *edit_mode.write() = !current;
                    },

                    svg {
                        width: "24",
                        height: "24",
                        svg_data: if *edit_mode.read() {
                            static_bytes(crate::assets::CHEVRON_UP_ICON)
                        } else {
                            static_bytes(crate::assets::PENCIL_ICON)
                        },
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

                    // TODO: Really need to standardise writing/updating/loading configs. This is messy.
                    onpress: move |_| {
                        starred.set(!starred());

                        let Some(mut profile) = CONFIG.read().profiles.get(&profile_id()).cloned() else {
                            return;
                        };

                        if let Some(instance) = profile.instances.get_mut(&instance_id()) { instance.starred = starred(); }

                        let config_toml = toml::to_string::<config::ProfileConfig>(&profile);

                        if let Ok(config_toml) = config_toml {
                            let instance_path = paths::config_profile_dir().join(format!("{}.toml", &profile_id));
                            let _ = std::fs::write(instance_path, config_toml);
                        }

                        *CONFIG.write() = config::Config::init();
                    },

                    svg {
                        width: "28",
                        height: "28",
                        svg_data: if starred() {
                            static_bytes(crate::assets::STAR_FILLED_ICON)
                        } else {
                            static_bytes(crate::assets::STAR_HOLLOW_ICON)
                        },
                        fill: if starred() {
                            THEME.read().star_yellow
                        } else {
                            THEME.read().text_primary
                        },
                    }
                }
            }
        }

        if edit_mode() {
            InstanceEdit {
                instance_id: instance_id(),
                instance: instance()
            }
        }
    })
}

// Dropdown doesn't let you use a custom string - only showing the value (key).
// So we made a struct that implements Display to render the name rather than the key.
#[derive(Clone, PartialEq)]
struct SelectedId(String, String);

impl std::fmt::Display for SelectedId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.1)
    }
}

#[component]
fn InstanceEdit(instance_id: String, instance: config::Instance) -> Element {
    let profile: ProfileContext = use_context();
    let profile_id = use_signal(|| profile().0);

    // make instance_id and instance reactive
    let instance_id = use_memo(use_reactive(&instance_id, |instance_id| instance_id));
    let instance = use_memo(use_reactive(&instance, |instance| instance));

    let mods = use_signal(|| CONFIG.read().mods.clone());

    let mut name = use_signal(|| instance().name);
    let mut icon = use_signal(|| instance().icon);
    let mut selected_mod = use_signal(|| {
        mods.read()
            .get(&instance().mod_id)
            .map(|m| SelectedId(instance().mod_id, m.name.to_string()))
            .unwrap_or(SelectedId(
                "Unknown Mod".to_string(),
                "Unknown Mod".to_string(),
            ))
    });

    let update = {
        to_owned![instance_id];
        move || {
            let mut profile = profile.read().1.clone();
            let mod_id = selected_mod.read().0.clone();
            let icon = icon().clone();
            let name = name().clone();

            profile
                .instances
                .entry(instance_id())
                .and_modify(|instance| {
                    instance.name = name.clone();
                    instance.mod_id = mod_id.clone();
                    instance.icon = icon.clone();
                })
                .or_insert(config::Instance {
                    name,
                    mod_id,
                    icon,
                    starred: false,
                });

            profile.save(&profile_id());

            CONFIG.write().sync();
        }
    };

    rsx!(rect {
        width: "fill",
        direction: "vertical",
        spacing: "8",

        label {
            font_size: "16",
            "Name of this instance"
        }

        Input {
            value: name(),
            onchange: move |e| { name.set(e) },
            placeholder: "New Instance",
            theme: theme_with!(InputTheme {
                width: "100%".into(),
                corner_radius: "8".into(),
                border_fill: THEME.read().bg_secondary.into(),
                font_theme: theme_with!(FontTheme {
                    color: THEME.read().text_primary.into(),
                }),
                background: THEME.read().bg_tertiary.into(),
                hover_background: THEME.read().bg_tertiary.into(),
            })
        }

        label {
            font_size: "16",
            "Mod to use for this instance"
        }

        Dropdown {
            value: selected_mod(),

            theme: theme_with!(DropdownTheme {
                border_fill: THEME.read().bg_secondary.into(),
                focus_border_fill: THEME.read().blurple.into(),
                font_theme: theme_with!(FontTheme {
                    color: THEME.read().text_primary.into(),
                }),
                background_button: THEME.read().bg_tertiary.into(),
                hover_background: THEME.read().bg_tertiary.into(),
                dropdown_background: THEME.read().bg_tertiary.into(),
            }),

            for (key, value) in mods() {{
                let id = SelectedId(key.clone(), value.name.clone());

                rsx!(DropdownItem {
                    value: id.clone(),

                    theme: theme_with!(DropdownItemTheme {
                        font_theme: theme_with!(FontTheme {
                            color: THEME.read().text_primary.into(),
                        }),
                        background: THEME.read().bg_tertiary.into(),
                        select_background: THEME.read().bg_primary.into(),
                        hover_background: THEME.read().bg_primary.into(),
                    }),

                    onpress: move |_| { selected_mod.set(id.clone()) },

                    label {
                        "{value.name}"
                    }
                })
            }}
        }

        // Icon Name
        rect {
            width: "100%",
            label {
                margin: "4",
                "Icon Filename (leave empty to use the mod's icon)"
            }

            label {
                margin: "4",
                color: THEME.read().text_secondary,
                "Place your icon in the root of the icons folder and enter it's name here."
            }

            Input {
                value: icon().unwrap_or_default(),
                onchange: move |e: String| {
                    icon.set(if e.is_empty() { None } else { Some(e) });
                },
                placeholder: "moonlight.png",
                theme: theme_with!(InputTheme {
                    width: "100%".into(),
                    corner_radius: "8".into(),
                    border_fill: THEME.read().bg_secondary.into(),
                    font_theme: theme_with!(FontTheme {
                        color: THEME.read().text_primary.into(),
                    }),
                    background: THEME.read().bg_tertiary.into(),
                    hover_background: THEME.read().bg_tertiary.into(),
                })
            }

            Button {
                stretch: false,
                padding: "8 16",
                margin: "8 0 0 0",
                corner_radius: "8",
                main_align: "center",
                height: "42",

                onpress: move |_| {
                    let _ = open::that(paths::config_icons_dir());
                },

                label {
                    font_size: "16",

                    "Open Icons Folder"
                }
            }
        }

        rect {
            width: "fill",
            direction: "horizontal",
            main_align: "end",
            spacing: "8",

            Button {
                stretch: false,
                height: "48",
                corner_radius: "8",
                target_color: THEME.read().bg_success,

                selected_color: THEME.read().bg_success,
                selected: {
                    let icon_match = icon().as_deref() != instance().icon.as_deref();
                    let mod_match = selected_mod().0.ne(&instance().mod_id);
                    icon_match || mod_match
                },

                onpress: move |_| { update(); },

                svg {
                    width: "20",
                    height: "20",
                    svg_data: static_bytes(crate::assets::FLOPPY_DISK_ICON),
                    fill: THEME.read().text_primary,
                }

                label {
                    font_size: "16",
                    "Save Changes"
                }
            }

            Button {
                stretch: false,
                height: "48",
                corner_radius: "8",
                target_color: THEME.read().bg_danger,

                onpress: {
                    move |_| {
                        *POPUP_STATE.write() = PopupState::ConfirmDeleteInstance(profile_id(), instance_id());
                    }
                },

                svg {
                    width: "20",
                    height: "20",
                    svg_data: static_bytes(crate::assets::TRASH_ICON),
                    fill: THEME.read().text_primary,
                }

                label {
                    font_size: "16",
                    "Delete Instance"
                }
            }
        }
    })
}

#[component]
fn Header(on_delete_pressed: EventHandler<()>) -> Element {
    let ctx = use_context::<ProfileContext>();
    let (profile_id, profile) = ctx.read().clone();

    let mut edit_mode = use_signal(|| false);
    use_effect(use_reactive(&profile_id, move |_profile_id| {
        *edit_mode.write() = false;
    }));

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
                    *CONFIG.write() = config::Config::init();
                },

                svg {
                    width: "24",
                    height: "24",
                    svg_data: static_bytes(crate::assets::REFRESH_ARROWS_ICON),
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

                onpress: move |_| {
                    *edit_mode.write() = !edit_mode();
                },

                svg {
                    width: "24",
                    height: "24",
                    svg_data: if edit_mode() {
                        static_bytes(crate::assets::CHEVRON_UP_ICON)
                    } else {
                        static_bytes(crate::assets::PENCIL_ICON)
                    },
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

                onpress: move |_| {
                    let profile_path = paths::config_profile_dir().join(format!("{}.toml", &profile_id));
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
                    svg_data: static_bytes(crate::assets::PEN_TO_SQUARE_ICON),
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
