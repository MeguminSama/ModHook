use discord_modloader::{config, paths};
use freya::prelude::*;

use crate::{components::custom_button::CustomButton, constants};

fn delete_profile(id: String) -> Result<(), Box<dyn std::error::Error>> {
    let profile_file = paths::config_profile_dir().join(format!("{id}.toml"));

    std::fs::remove_file(profile_file)?;

    Ok(())
}

#[component]
pub fn DeleteProfile(profile_id: String) -> Element {
    let mut ctx = use_context::<Signal<crate::AppState>>();

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
                "Delete profile"
            }
        },
        PopupContent {
            label {
                "Are you sure you want to delete this profile?"
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
                        "Delete"
                    }
                    onclick: move |_| {
                        let result = delete_profile(profile_id.clone());
                        if result.is_ok() {
                            ctx.write().config = config::Config::init();
                            ctx.write().popup_state = super::PopupState::Hidden;
                        }
                    }
                }
            }
        }
    })
}
