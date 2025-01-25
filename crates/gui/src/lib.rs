mod components;
mod constants;
mod pages;
mod popups;
mod utils;

use components::main_content::CurrentPage;
use discord_modloader::config;
use popups::PopupState;

use freya::prelude::*;

#[derive(Clone)]
pub struct AppState {
    config: config::Config,
    popup_state: PopupState,
    current_page: CurrentPage,
}

pub fn start_gui() {
    let cfg: LaunchConfig<()> = LaunchConfig::new()
        .with_title("Discord Modloader")
        .with_size(1080., 720.)
        .with_decorations(true)
        .with_transparency(true);

    launch_cfg(app, cfg);
}

fn app() -> Element {
    use components::main_content::MainContent;
    use components::sidebar::ProfileList;

    let mut ctx = use_signal(|| AppState {
        config: config::Config::init(),
        popup_state: PopupState::Hidden,
        current_page: CurrentPage::Home,
    });
    // Share state signal with global state context
    use_context_provider(|| ctx);

    rsx!(rect {
        direction: "horizontal",
        width: "100%",
        height: "100%",
        background: constants::BG_PRIMARY,

        match ctx().popup_state {
            PopupState::Hidden => None,
            PopupState::Launching(instance) => {
                Some(rsx!(popups::LaunchingPopup { instance }));
                None
            }
            PopupState::AlreadyLaunched(profile_id, instance_id) => {
                Some(rsx!(popups::AlreadyLaunchedPopup { profile_id, instance_id }))
            }
            PopupState::CreateNewInstance(profile_id) => {
                Some(rsx!(popups::CreateNewInstance { profile_id }))
            }
            PopupState::DeleteProfile(profile_id) => {
                Some(rsx!(popups::DeleteProfile { profile_id }))
            }
            PopupState::DeleteInstance(profile_id, instance_id) => {
                Some(rsx!(popups::DeleteInstance { profile_id, instance_id }))
            }
        }

        ProfileList {
            onpagechange: move |page| ctx.write().current_page = page,
        }

        rect {
            width: "fill",
            height: "100%",
            background: constants::BG_SECONDARY,

            MainContent {
                page: ctx().current_page,
            }
        }
    })
}
