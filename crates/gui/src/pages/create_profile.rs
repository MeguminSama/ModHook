use std::collections::BTreeMap;

use dialog::DialogBox as _;
use discord_modloader::{config, paths};
use freya::prelude::*;

use crate::{components::custom_button::CustomButton, constants};

#[component]
pub fn CreateNewProfile(profile_id: Option<String>) -> Element {
    let mut ctx: Signal<crate::AppState> = use_context();

    let is_new_profile = use_signal(|| profile_id.is_none());

    let uuid = use_signal(|| profile_id.unwrap_or(uuid::Uuid::new_v4().to_string()));

    let mut profile = use_signal(|| match ctx().config.profiles.get(&uuid()) {
        Some(profile) => profile.clone(),
        _ => config::ProfileConfig {
            profile: config::Profile {
                name: "My Cool Profile".to_string(),
                use_default_profile: false,
            },
            discord: config::Discord {
                executable: "".to_string(),
                args: "".to_string(),
            },
            instances: BTreeMap::new(),
        },
    });

    let mut selected_executable = use_signal(|| match profile().discord.executable.as_str() {
        "" => InstallKind::NoneSelected,
        install => InstallKind::Custom(install.to_string()),
    });

    use_memo(move || match selected_executable() {
        InstallKind::Selected(install) => profile.write().discord.executable = install,
        InstallKind::Custom(path) => profile.write().discord.executable = path,
        _ => {}
    });

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
                if is_new_profile() { "Create new Profile" } else { "Edit Profile" }
            },
        },

        rect {
            width: "fill",
            padding: "8 0 0 0",
            width: "100%",

            label {
                "Profile Name"
            }

            Input {
                theme: theme_with!(InputTheme {
                    width: "100%".into(),
                    background: constants::BG_SECONDARY.into(),
                    hover_background: constants::BG_SECONDARY.into(),
                    font_theme: theme_with!(FontTheme {
                        color: constants::TEXT_PRIMARY.into(),
                    })
                }),
                placeholder: "Profile Name",
                value: profile().profile.name,
                onchange: move |value| {
                    profile.write().profile.name = value;
                }
            }
        },

        rect {
            width: "fill",
            padding: "8 0 0 0",
            width: "100%",

            label {
                "Discord Executable"
            }

            InstallDropdown { selected_executable }

            if let InstallKind::Custom(_) = selected_executable() {
                rect {
                    direction: "horizontal",
                    cross_align: "center",

                    CustomButton {
                        bg_anim: (constants::BG_PRIMARY.into(), constants::BLURPLE.into()),
                        height: "48",
                        padding: "8 0 0 0",
                        onclick: move |_| {
                            let file = dialog::FileSelection::new("Select Discord executable")
                                .title("Select Discord executable")
                                .show()
                                .expect("Failed to open file selection dialog");

                            if let Some(file) = file {
                                selected_executable.set(InstallKind::Custom(file));
                            }
                        },

                        label {
                            "Browse"
                        }
                    }


                    Input {
                        theme: theme_with!(InputTheme {
                            width: "fill".into(),
                            background: constants::BG_SECONDARY.into(),
                            hover_background: constants::BG_SECONDARY.into(),
                            font_theme: theme_with!(FontTheme {
                                color: constants::TEXT_PRIMARY.into(),
                            })
                        }),
                        placeholder: "Discord Executable",
                        value: profile().discord.executable,
                        onchange: move |value| {
                            selected_executable.set(InstallKind::Custom(value));
                        }
                    }
                }
            }
        },

        rect {
            width: "fill",
            padding: "8 0 0 0",
            width: "100%",

            label {
                "Launch Arguments"
            }

            Input {
                theme: theme_with!(InputTheme {
                    width: "100%".into(),
                    background: constants::BG_SECONDARY.into(),
                    hover_background: constants::BG_SECONDARY.into(),
                    font_theme: theme_with!(FontTheme {
                        color: constants::TEXT_PRIMARY.into(),
                    })
                }),
                placeholder: "--some=thing --hello=world",
                value: profile().discord.args,
                onchange: move |value| {
                    profile.write().discord.args = value;
                }
            }
        },

        rect {
            CustomButton {
                bg_anim: (constants::BG_PRIMARY.into(), constants::BLURPLE.into()),
                height: "48",
                padding: "8 0 0 0",
                onclick: move |_| {
                    if let Ok(id) = create_profile(uuid(), profile()) {
                        ctx.write().config = config::Config::init();
                        ctx.write().current_page = crate::CurrentPage::SelectedProfile(id);
                    };
                },

                label {
                    "Save Profile"
                }
            }
        }
    })
}

fn create_profile(
    id: String,
    profile: config::ProfileConfig,
) -> Result<String, Box<dyn std::error::Error>> {
    let profile_file = paths::config_profile_dir().join(format!("{id}.toml"));

    let profile: String = toml::to_string(&profile)?;

    std::fs::write(profile_file, profile)?;

    Ok(id.to_string())
}

#[derive(Debug, Clone, PartialEq)]
enum InstallKind {
    NoneSelected,
    Selected(String),
    Custom(String),
}

#[component]
fn InstallDropdown(selected_executable: Signal<InstallKind>) -> Element {
    let found_installs: Signal<Vec<String>> = use_signal(|| {
        let mut installs = crate::utils::find_discord_installations();
        // Sort by length. Shorter first, longer last.
        installs.sort_by(|a, b| match b.len() > a.len() {
            true => std::cmp::Ordering::Less,
            false => std::cmp::Ordering::Greater,
        });
        installs
    });

    let value = match selected_executable() {
        InstallKind::NoneSelected => "Select a Discord instance".to_string(),
        InstallKind::Selected(install) => install,
        InstallKind::Custom(_) => "Custom Instance".to_string(),
    };

    rsx!(Dropdown {
        theme: theme_with!(DropdownTheme {
                width: "100%".into(),
                background_button: constants::BG_SECONDARY.into(),
                dropdown_background: constants::BG_SECONDARY.into(),
                hover_background: constants::BG_PRIMARY.into(),
                font_theme: theme_with!(FontTheme {
                    color: constants::TEXT_PRIMARY.into(),
                })
            }),
        value: value,
        for install in found_installs() {
            DropdownItem {
                theme: theme_with!(DropdownItemTheme {
                    background: constants::BG_SECONDARY.into(),
                    hover_background: constants::BG_PRIMARY.into(),
                    select_background: constants::BG_PRIMARY.into(),
                    font_theme: theme_with!(FontTheme {
                        color: constants::TEXT_PRIMARY.into(),
                    })
                }),
                value: install.clone(),
                onpress: {
                    to_owned![install];
                    move |_| {
                        selected_executable.set(InstallKind::Selected(install.clone()));
                    }
                },

                label { "{install}" },
            }
        }

        DropdownItem {
            theme: theme_with!(DropdownItemTheme {
                background: constants::BG_SECONDARY.into(),
                hover_background: constants::BG_PRIMARY.into(),
                select_background: constants::BG_PRIMARY.into(),
                font_theme: theme_with!(FontTheme {
                    color: constants::TEXT_PRIMARY.into(),
                })
            }),
            value: "Custom Path".to_string(),
            onpress: move |_| {
                selected_executable.set(InstallKind::Custom("".to_string()));
            },

            label { "Custom Path" },
        }
    })
}
