use discord_modloader::{config, paths};
use freya::prelude::*;

use crate::{components::custom_button::CustomButton, constants};

fn delete_instance(
    mut ctx: Signal<crate::AppState>,
    profile_id: String,
    instance_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    match ctx.write().config.profiles.get_mut(&profile_id) {
        Some(profile) => {
            profile.instances.remove(&instance_id);
            let profile_path = paths::config_profile_dir().join(format!("{profile_id}.toml"));
            std::fs::write(profile_path, toml::to_string_pretty(&profile)?)?;
        }
        _ => {
            return Err("Profile not found".into());
        }
    }

    Ok(())
}

#[component]
pub fn CreateNewInstance(profile_id: String) -> Element {
    let mut ctx = use_context::<Signal<crate::AppState>>();

    let instance_id = use_signal(|| uuid::Uuid::new_v4().to_string());

    let mut instance = use_signal(|| config::Instance {
        name: "".to_string(),
        mod_id: "".to_string(),
        icon: None,
    });

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
                "Create new instance"
            }
        },
        PopupContent {
            rect {
                width: "100%",
                height: "100%",
                direction: "vertical",
                padding: "8",
                spacing: "8",
                background: constants::BG_SECONDARY,
                color: "white",

                rect {
                    width: "fill",
                    padding: "8 0 0 0",
                    width: "100%",

                    label {
                        "Instance Name"
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
                        placeholder: "Instance Name",
                        value: instance().name,
                        onchange: move |value| {
                            instance.write().name = value;
                        }
                    }
                }
            }
        }
    })
}
