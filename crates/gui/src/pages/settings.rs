use discord_modloader::{config, paths};
use freya::prelude::*;

use crate::{components::custom_button::CustomButton, constants, utils};

#[component]
pub fn Settings() -> Element {
    let mut icons = use_signal(utils::images::get_all_icons);

    let mut ctx = use_context::<Signal<crate::AppState>>();

    use_memo(move || {
        // write settings to file
        let settings = toml::to_string(&ctx().config.settings);
        if let Ok(settings) = settings {
            let settings_file = paths::configs_dir().join("settings.toml");
            let _ = std::fs::write(settings_file, settings);
        };
    });

    rsx!(rect {
        width: "100%",
        height: "100%",
        direction: "vertical",
        padding: "8",
        spacing: "8",
        background: constants::BG_SECONDARY,
        color: "white",

        ScrollView {
            rect {
                direction: "horizontal",
                cross_align: "center",
                height: "48",

                label {
                    font_size: "18",
                    font_weight: "bold",
                    { "Settings" }
                },

                rect {
                    width: "fill",
                    direction: "horizontal",
                    main_align: "end",
                    spacing: "4",

                    CustomButton {
                        height: "48",
                        bg_anim: (constants::BG_PRIMARY.into(), constants::BLURPLE.into()),
                        onclick: move |_| {
                            ctx.write().config = config::Config::init();
                            icons.set(utils::images::get_all_icons());
                        },

                        label {
                            "Refresh settings"
                        }
                    }

                    CustomButton {
                        height: "48",
                        bg_anim: (constants::BG_PRIMARY.into(), constants::BLURPLE.into()),
                        onclick: move |_| {
                            let _ = open::that(paths::configs_dir());
                        },

                        label {
                            "Open settings folder"
                        }
                    }
                }
            },

            rect {
                width: "fill",
                padding: "8 0 0 0",
                width: "100%",
                direction: "horizontal",
                cross_align: "center",
                spacing: "8",

                label {
                    "Hide window when launching Discord"
                }

                Switch {
                    enabled: ctx().config.settings.hide_window_on_launch,
                    ontoggled: move |_| {
                        ctx.write().config.settings.hide_window_on_launch ^= true;
                    }
                }
            },

            rect {
                width: "fill",
                padding: "8 0 0 0",
                width: "100%",

                label {
                    "Icons"
                }

                rect {
                    max_height: "256",
                    ScrollView {
                        for (name, icon) in icons() {
                            IconEntry {
                                name: name,
                                icon: icon,
                            }
                        }
                    }
                }
            }
        }
    })
}

#[component]
fn IconEntry(name: String, icon: Vec<u8>) -> Element {
    let hoverable = crate::utils::hoverable::hoverable!(move |_conf| {
        AnimColor::new(constants::BG_PRIMARY, constants::BG_TERTIARY)
            .ease(Ease::InOut)
            .time(10)
    });

    let hoverable_anim = hoverable.animation.get();

    let background = hoverable_anim.read().read();

    rsx!(rect {
        background: "{background}",
        onmouseenter: hoverable.onmouseenter,
        onmouseleave: hoverable.onmouseleave,

        corner_radius: "4",
        padding: "4",
        spacing: "4",
        height: "64",
        width: "100%",
        direction: "horizontal",
        cross_align: "center",

        image {
            width: "40",
            height: "40",
            image_data: dynamic_bytes(icon),
        }

        label {
            font_size: "12",
            font_weight: "bold",
            "{name}"
        }
    })
}
