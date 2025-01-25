use discord_modloader::config;
use freya::prelude::*;

use crate::constants;

#[component]
pub fn LaunchingPopup(instance: config::Instance) -> Element {
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
                "Launching Discord"
            }
        },
        PopupContent {
            label {
                "Launching instance {instance.name}..."
            }
        }
    })
}
