use modloader_core::config;

pub const CHEVRON_DOWN_ICON: &[u8] = include_bytes!("../../../assets/icons/chevron-down.svg");
pub const CHEVRON_UP_ICON: &[u8] = include_bytes!("../../../assets/icons/chevron-up.svg");
pub const CIRCLE_HELP_ICON: &[u8] = include_bytes!("../../../assets/icons/circle-help.svg");
pub const DISCORD_ICON: &[u8] = include_bytes!("../../../assets/icons/discord.svg");
pub const FLOPPY_DISK_ICON: &[u8] = include_bytes!("../../../assets/icons/floppy-disk.svg");
pub const FOLDER_ICON: &[u8] = include_bytes!("../../../assets/icons/folder.svg");
pub const GEAR_ICON: &[u8] = include_bytes!("../../../assets/icons/gear.svg");
pub const GITHUB_ICON: &[u8] = include_bytes!("../../../assets/icons/github.svg");
pub const HOME_ICON: &[u8] = include_bytes!("../../../assets/icons/home.svg");
pub const PENCIL_ICON: &[u8] = include_bytes!("../../../assets/icons/pen.svg");
pub const PEN_TO_SQUARE_ICON: &[u8] = include_bytes!("../../../assets/icons/pen-to-square.svg");
pub const PLAY_ICON: &[u8] = include_bytes!("../../../assets/icons/play.svg");
pub const PLUS_ICON: &[u8] = include_bytes!("../../../assets/icons/plus.svg");
pub const REFRESH_ARROWS_ICON: &[u8] = include_bytes!("../../../assets/icons/refresh-arrows.svg");
pub const STAR_FILLED_ICON: &[u8] = include_bytes!("../../../assets/icons/star-filled.svg");
pub const STAR_HOLLOW_ICON: &[u8] = include_bytes!("../../../assets/icons/star-hollow.svg");
pub const STOP_ICON: &[u8] = include_bytes!("../../../assets/icons/stop.svg");
pub const TRASH_ICON: &[u8] = include_bytes!("../../../assets/icons/trash.svg");

pub const FONT_INTER: &[u8] = include_bytes!("../../../assets/fonts/Inter-Variable.ttf");

pub const DISCORD_INVITE_LINK: &str = "https://discord.gg/r5bmSXBEPC";
pub const GITHUB_REPO_LINK: &str = "https://github.com/meguminsama/discord-modloader";

pub const MOONLIGHT_LOGO: &[u8] = include_bytes!("../../../configs/icons/moonlight.png");
pub const VENCORD_LOGO: &[u8] = include_bytes!("../../../configs/icons/vencord.png");

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModTemplateIcon {
    Svg(&'static [u8]),
    Image(&'static [u8]),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModTemplate {
    pub config: config::Mod,
    pub icon: Option<ModTemplateIcon>,
    pub support_link: Option<String>,
}

pub struct ModTemplates;

impl ModTemplates {
    pub fn get_all() -> Vec<ModTemplate> {
        vec![Self::blank(), Self::moonlight(), Self::vencord()]
    }

    pub fn blank() -> ModTemplate {
        let config = config::Mod {
            name: "New Blank Template".to_string(),
            entrypoint: "injector.js".to_string(),

            updater: None,

            icon: None,
            loader: None,
            path: None,
        };

        ModTemplate {
            config,
            icon: Some(ModTemplateIcon::Svg(PLUS_ICON)),
            support_link: None,
        }
    }

    pub fn moonlight() -> ModTemplate {
        let config = config::Mod {
            name: "New Moonlight Template".to_string(),
            entrypoint: "injector.js".to_string(),

            updater: Some(config::ModUpdater {
                github_org: "moonlight-mod".to_string(),
                github_repo: "moonlight".to_string(),
                dist_file_names: vec!["dist.tar.gz".to_string()],
                dist_file_type: config::DistFileType::TarGz,
                icon_url: Some(
                    "https://raw.githubusercontent.com/moonlight-mod/moonlight-mod.github.io/main/src/img/logo.png".to_string(),
                ),
                ask_before_update: true,
                auto_update: true,
            }),

            icon: None,
            loader: None,
            path: None,
        };

        ModTemplate {
            config,
            icon: Some(ModTemplateIcon::Image(MOONLIGHT_LOGO)),
            support_link: Some("https://moonlight-mod.github.io".to_string()),
        }
    }

    pub fn vencord() -> ModTemplate {
        let config = config::Mod {
            name: "New Vencord Template".to_string(),
            entrypoint: "patcher.js".to_string(),

            updater: Some(config::ModUpdater {
                github_org: "vendicated".to_string(),
                github_repo: "vencord".to_string(),
                // TODO: Force vencord to have a single dist file because RAAAAAAAAAA
                dist_file_names: vec!["patcher.js", "preload.js", "renderer.js", "renderer.css"]
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect(),
                dist_file_type: config::DistFileType::Raw,
                icon_url: Some(
                    "https://raw.githubusercontent.com/Vencord/Vesktop/main/static/icon.png"
                        .to_string(),
                ),
                ask_before_update: true,
                // TODO: Check this works properly, otherwise, let vencord use it's own updater.
                auto_update: true,
            }),

            icon: None,
            loader: None,
            path: None,
        };

        ModTemplate {
            config,
            icon: Some(ModTemplateIcon::Image(VENCORD_LOGO)),
            support_link: Some("https://vencord.dev".to_string()),
        }
    }
}
