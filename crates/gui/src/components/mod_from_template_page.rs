use freya::prelude::*;
use modloader_core::{config, paths, updater};

use crate::components::button::Button;
use crate::{AppPage, CONFIG, CURRENT_PAGE, THEME, assets};

#[component]
pub fn ModFromTemplatePage() -> Element {
    let templates = assets::ModTemplates::get_all();

    rsx!(rect {
        direction: "vertical",
        padding: "8",

        Header { }

        ScrollView {
            direction: "vertical",
            scrollbar_theme: theme_with!(ScrollBarTheme {
                background: THEME.read().bg_tertiary.into(),
            }),

            for template in templates {
                rect {
                    direction: "horizontal",
                    spacing: "8",
                    content: "fit",

                    if let Some(ref link) = template.support_link {
                        SupportButton { link }
                    }

                    ModTemplateCard { template: template.clone() }
                }
            }
        }
    })
}

#[component]
fn SupportButton(link: String) -> Element {
    let support_icon = static_bytes(assets::CIRCLE_HELP_ICON);

    rsx!(
        Button {
            width: "64",
            height: "64",
            padding: "8",
            margin: "8 0",
            corner_radius: "8",
            base_color: THEME.read().bg_primary,

            onpress: move |_| { let _ = open::that(&link); },

            svg {
                width: "48",
                height: "48",

                fill: THEME.read().text_primary,

                svg_data: support_icon,
            }
        }
    )
}

#[component]
fn ModTemplateCard(template: assets::ModTemplate) -> Element {
    let config_template = template.config.clone();

    let onpress = move || {
        let uuid = uuid::Uuid::new_v4().to_string();
        let mod_path = paths::config_mods_dir().join(format!("{}.toml", uuid));

        let mod_toml = toml::to_string(&config::ModConfig {
            r#mod: config_template.clone(),
        })
        .unwrap();

        let _ = std::fs::write(mod_path, mod_toml);

        *CONFIG.write() = config::Config::init();

        *CURRENT_PAGE.write() = AppPage::Mod(uuid);

        if let Some(updater) = &config_template.updater {
            let _ = updater::update(updater, true);
        }
    };

    // BUG: The button stretches off the screen for some reason.
    // Possibly an issue with layout calculation since the button is rendered in a for loop?
    // It's not harmful, but just a bit weird.
    rsx!(Button {
        height: "64",
        padding: "8",
        margin: "8 0",
        corner_radius: "8",
        base_color: THEME.read().bg_primary,

        onpress: move |_| { onpress(); },

        cross_align: "center",

        spacing: "12",

        if let Some(icon) = template.icon {
            match icon {
                assets::ModTemplateIcon::Svg(svg) => {
                    let image = static_bytes(svg);
                    rsx!(svg {
                        width: "48",
                        height: "48",

                        fill: THEME.read().text_primary,

                        svg_data: image,
                    })
                }
                assets::ModTemplateIcon::Image(image) => {
                    let image = static_bytes(image);
                    rsx!(image {
                        width: "48",
                        height: "48",

                        image_data: image,
                    })
                }
            }
        }

        label {
            font_size: "18",

            "{template.config.name}"
        }
    })
}

#[component]
fn Header() -> Element {
    rsx!(rect {
        width: "100%",
        padding: "12",
        corner_radius: "8",
        border: "0 0 2 0 outer {THEME.read().bg_secondary}",

        background: THEME.read().bg_primary,

        direction: "horizontal",
        cross_align: "center",

        spacing: "12",

        label {
            font_size: "18",

            "Create a new mod from a template"
        }
    })
}
