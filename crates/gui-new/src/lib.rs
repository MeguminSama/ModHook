pub(crate) mod app;
pub(crate) mod assets;
pub(crate) mod components;
pub(crate) mod constants;
pub(crate) mod utils;

use constants::ThemeDef;

use freya::prelude::*;

pub static CURRENT_PAGE: GlobalSignal<AppPage> = Signal::global(|| AppPage::Home);
pub static THEME: GlobalSignal<ThemeDef> = Signal::global(|| constants::DARK_THEME);
pub static CONFIG: GlobalSignal<discord_modloader::config::Config> =
    Signal::global(discord_modloader::config::Config::init);
pub static POPUP_STATE: GlobalSignal<PopupState> = Signal::global(|| PopupState::None);
pub static REFRESH_PIDS: GlobalSignal<()> = Signal::global(|| ());

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppPage {
    Home,
    Mod(String),
    ModFromTemplate,
    Profile(String),
    Settings,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PopupState {
    None,
    ConfirmDeleteMod(String),
    ConfirmDeleteProfile(String),
    ConfirmKillProfile(String),
    InstanceAlreadyRunning(String, String, String),
}

pub enum AppTheme {
    Dark,
    Light,
}

pub fn start_gui() {
    let cfg: LaunchConfig<()> = LaunchConfig::new()
        .with_title("Discord Mod Launcher")
        .with_size(1080., 720.)
        .with_font("Inter", assets::FONT_INTER)
        .with_default_font("Inter")
        .with_decorations(true)
        .with_transparency(true);

    launch_cfg(app, cfg);
}

fn app() -> Element {
    app::app()
}
