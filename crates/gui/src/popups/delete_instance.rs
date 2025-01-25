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
pub fn DeleteInstance(profile_id: String, instance_id: String) -> Element {
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
                "Delete instance"
            }
        },
        PopupContent {
            label {
                "Are you sure you want to delete the instance?"
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
                        let result = delete_instance(ctx, profile_id.clone(), instance_id.clone());
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
