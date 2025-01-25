use freya::prelude::*;
use strum::IntoEnumIterator as _;

use crate::components::button::Button;
use crate::utils::icons::GetIcon;
use crate::{CONFIG, POPUP_STATE, PopupState, THEME};
use modloader_core::{config, paths};

type ModContext = Signal<(String, config::Mod)>;

fn get_mod_by_id(mod_id: &str) -> (String, config::Mod) {
    CONFIG
        .read()
        .mods
        .get(mod_id)
        .map(|mod_| (mod_id.to_string(), mod_.clone()))
        .unwrap_or_else(|| {
            let mod_ = config::Mod {
                name: "New Mod".into(),
                ..Default::default()
            };
            (mod_id.to_string(), mod_)
        })
}

#[component]
pub fn ModPage(mod_id: String) -> Element {
    let mut mod_: ModContext = use_signal(|| get_mod_by_id(&mod_id));

    use_context_provider(|| mod_);

    use_effect(use_reactive(&mod_id, move |mod_id| {
        *mod_.write() = get_mod_by_id(&mod_id);
    }));

    rsx!(rect {
        direction: "vertical",

        padding: "8",

        Header {
            on_delete_pressed: move |_| {
                *POPUP_STATE.write() = PopupState::ConfirmDeleteMod(mod_id.clone());
            }
        }

        Form { }
    })
}

#[component]
fn Header(on_delete_pressed: EventHandler<()>) -> Element {
    let ctx = use_context::<ModContext>();
    let (_mod_id, mod_) = &*ctx.read();

    let mod_icon = mod_.get_icon().map(dynamic_bytes);

    rsx!(rect {
        width: "100%",
        padding: "12 8 12 12",
        corner_radius: "8",
        border: "0 0 2 0 outer {THEME.read().bg_secondary}",

        background: THEME.read().bg_primary,

        direction: "horizontal",
        cross_align: "center",

        spacing: "12",

        if mod_icon.is_some() {
            image {
                width: "48",
                height: "48",
                image_data: mod_icon,
            }
        }

        label {
            font_size: "18",

            "{mod_.name}"
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
                    let mod_path = paths::config_mods_dir().join(format!("{}.toml", &ctx.read().0));

                    if mod_path.exists() {
                        if let Err(e) = open::that(mod_path) {
                            eprintln!("Failed to open directory: {}", e);
                        }
                    } else if let Some(parent) = mod_path.parent() {
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

#[component]
fn Form() -> Element {
    let ctx = use_context::<ModContext>();

    let mut mod_id = use_signal(|| ctx.read().0.clone());
    let mut mod_form = use_signal(|| ctx.read().1.clone());
    let mut dist_files_form = use_signal(|| {
        ctx.read()
            .1
            .updater
            .clone()
            .map(|u| u.dist_file_names.join(", "))
            .unwrap_or_default()
    });

    use_effect(move || {
        *mod_form.write() = ctx.read().1.clone();
        *mod_id.write() = ctx.read().0.clone();
        *dist_files_form.write() = ctx
            .read()
            .1
            .updater
            .clone()
            .map(|u| u.dist_file_names.join(", "))
            .unwrap_or_default();
    });

    let update = move || {
        let mod_id = ctx.read().0.clone();
        let mod_ = mod_form.read().clone();

        let mod_toml = toml::to_string::<config::ModConfig>(&config::ModConfig { r#mod: mod_ });

        if let Ok(mod_toml) = mod_toml {
            let mod_path = paths::config_mods_dir().join(format!("{}.toml", mod_id));
            let _ = std::fs::write(mod_path, mod_toml);
        }

        *CONFIG.write() = config::Config::init();
    };

    rsx!(rect {
        margin: "8 8 0 0",

        Button {
            corner_radius: "8",
            stretch: false,

            selected_color: THEME.read().bg_success,
            selected: *mod_id.read() == ctx.read().0 && *mod_form.read() != ctx.read().1,

            onpress: move |_| { update(); },

            svg {
                width: "20",
                height: "20",
                svg_data: static_bytes(crate::assets::FLOPPY_DISK_ICON),
                fill: THEME.read().text_primary,
            }

            label {
                "Save"
            }
        }

        ScrollView {
            scrollbar_theme: theme_with!(ScrollBarTheme {
                background: THEME.read().bg_tertiary.into(),
            }),

            label {
                margin: "8",
                font_size: "18",
                "Basic Mod Configuration"
            }

            // Basic Mod Configuration
            rect {
                background: THEME.read().bg_primary,
                corner_radius: "8",
                padding: "8",

                spacing: "8",

                // Mod Name
                rect {
                    width: "100%",
                    label {
                        margin: "4",
                        "Mod Name"
                    }

                    Input {
                        value: "{mod_form().name}",
                        onvalidate: |v: InputValidator| v.set_valid(v.text().len() <= 32),
                        onchange: move |e| mod_form.write().name = e,
                        placeholder: "Mod Name",
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
                }

                // Mod Entrypoint
                rect {
                    width: "100%",
                    label {
                        margin: "4",
                        "Entrypoint File"
                    }

                    label {
                        margin: "4",
                        color: THEME.read().text_secondary,
                        "This is the file that initialises your mod. Usually called something like injector.js or patcher.js"
                    }

                    Input {
                        value: "{mod_form().entrypoint}",
                        onchange: move |e: String| {
                            if e.len() > 64 {
                                return;
                            }
                            mod_form.write().entrypoint = e;
                        },
                        placeholder: "injector.js",
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
                }

                // Mod Path
                rect {
                    width: "100%",
                    label {
                        margin: "4",
                        "Path to the mod's build folder (leave empty to use the auto-updater)"
                    }

                    Input {
                        value: "{mod_form().path.unwrap_or_default()}",
                        onchange: move |e: String| {
                            if e.is_empty() {
                                mod_form.write().path = None;
                            } else {
                                mod_form.write().path = Some(e);
                            }
                        },
                        placeholder: "/path/to/moonlight",
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
                }

                // Icon Name
                rect {
                    width: "100%",
                    label {
                        margin: "4",
                        "Icon Filename (leave empty to use the auto-updater's icon)"
                    }

                    label {
                        margin: "4",
                        color: THEME.read().text_secondary,
                        "Place your icon in the root of the icons folder and enter it's name here."
                    }

                    Input {
                        value: "{mod_form().icon.unwrap_or_default()}",
                        onchange: move |e: String| {
                            if e.is_empty() {
                                mod_form.write().icon = None;
                            } else {
                                mod_form.write().icon = Some(e);
                            }
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


                    rect {
                        width: "fill",
                        direction: "horizontal",
                        main_align: "end",

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
                }
            }

            label {
                margin: "8",
                font_size: "18",
                "Auto Updater (optional)"
            }

            // Auto Updater Configuration
            rect {
                background: THEME.read().bg_primary,
                corner_radius: "8",
                padding: "8",
                spacing: "8",

                // Auto Update
                rect {
                    width: "100%",
                    label {
                        margin: "4",
                        "Enable automatic updater?"
                    }

                    label {
                        margin: "4",
                        color: THEME.read().text_secondary,
                        "If disabled, you will need to use your mod's auto-updater instead."
                    }

                    Tile {
                        onselect: move |_| {
                            let mut new_updater = mod_form().updater.unwrap_or_default();
                            new_updater.auto_update = !new_updater.auto_update;
                            mod_form.write().updater = Some(new_updater);
                        },
                        leading: rsx!(Checkbox {
                            selected: mod_form().updater.map(|u| u.auto_update).unwrap_or_default()
                        }),
                        label {
                            "Enable auto-updater"
                        }
                    }
                }

                if mod_form().updater.unwrap_or_default().auto_update {
                    // Always Update
                    rect {
                        width: "100%",
                        label {
                            margin: "4",
                            "Ask before applying updates?"
                        }

                        Tile {
                            onselect: move |_| {
                                let mut new_updater = mod_form().updater.unwrap_or_default();
                                new_updater.ask_before_update = !new_updater.ask_before_update;
                                mod_form.write().updater = Some(new_updater);
                            },
                            leading: rsx!(Checkbox {
                                selected: mod_form().updater.map(|u| u.ask_before_update).unwrap_or_default()
                            }),
                            label {
                                "Always ask before updating"
                            }
                        }
                    }
                }

                // GitHub Organisation
                rect {
                    width: "100%",
                    label {
                        margin: "4",
                        "GitHub Organisation"
                    }

                    Input {
                        value: "{mod_form().updater.map(|u| u.github_org).unwrap_or_default()}",
                        onchange: move |e: String| {
                            let mut new_updater = mod_form().updater.unwrap_or_default();
                            new_updater.github_org = e.clone();

                            mod_form.write().updater = Some(new_updater);

                        },
                        placeholder: "moonlight-mod",
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
                }

                // GitHub Repository
                rect {
                    width: "100%",
                    label {
                        margin: "4",
                        "GitHub Repository"
                    }

                    Input {
                        value: "{mod_form().updater.map(|u| u.github_repo).unwrap_or_default()}",
                        onchange: move |e: String| {
                            let mut new_updater = mod_form().updater.unwrap_or_default();
                            new_updater.github_repo = e.clone();

                            mod_form.write().updater = Some(new_updater);

                        },
                        placeholder: "moonlight",
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
                }

                // Release File Type
                rect {
                    width: "100%",
                    label {
                        margin: "4",
                        "Release File Type"
                    }

                    Dropdown {
                        value: "{mod_form().updater.map(|u| u.dist_file_type).unwrap_or_default()}",

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

                        for i in config::DistFileType::iter() {
                            DropdownItem {
                                value: i.to_string(),

                                theme: theme_with!(DropdownItemTheme {
                                    font_theme: theme_with!(FontTheme {
                                        color: THEME.read().text_primary.into(),
                                    }),
                                    background: THEME.read().bg_tertiary.into(),
                                    select_background: THEME.read().bg_primary.into(),
                                    hover_background: THEME.read().bg_primary.into(),
                                }),

                                onpress: {
                                    to_owned![i];
                                    move |_| {
                                        let mut new_updater = mod_form().updater.unwrap_or_default();
                                        new_updater.dist_file_type = i.clone();

                                        mod_form.write().updater = Some(new_updater);
                                    }
                                },

                                label {
                                    "{i}"
                                }
                            }
                        }
                    }
                }

                // Release File Name
                rect {
                    width: "100%",
                    label {
                        margin: "4",
                        "Release File Names (The file(s) to download from GitHub Releases) Separate with commas if downloading multiple."
                    }

                    Input {
                        value: "{dist_files_form()}",
                        onchange: move |e: String| {
                            *dist_files_form.write() = e.clone();

                            let split = e.split(", ").map(|s| s.trim().to_string());

                            let mut new_updater = mod_form().updater.unwrap_or_default();
                            new_updater.dist_file_names = split.collect();

                            mod_form.write().updater = Some(new_updater);

                        },
                        placeholder: "dist.tar.gz",
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
                }

                // Icon URL
                rect {
                    width: "100%",
                    label {
                        margin: "4",
                        "Icon URL (Optional, automatically download the latest icon from a URL)"
                    }

                    Input {
                        value: "{mod_form().updater.map(|u| u.icon_url.unwrap_or_default()).unwrap_or_default()}",
                        onchange: move |e: String| {
                            let mut new_updater = mod_form().updater.unwrap_or_default();
                            if e.is_empty() {
                                new_updater.icon_url = None;
                            } else {
                                new_updater.icon_url = Some(e.clone());
                            }
                            mod_form.write().updater = Some(new_updater);

                        },
                        placeholder: "https://raw.githubusercontent.com/moonlight-mod/moonlight-mod.github.io/main/src/img/logo.png",
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
                }
            }
        }
    })
}
