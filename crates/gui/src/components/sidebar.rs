use freya::prelude::*;
use modloader_core::{config, paths};

use crate::components::button::Button;
use crate::{AppPage, CONFIG, CURRENT_PAGE, THEME, utils::hoverable::hoverable};

const SIDEBAR_WIDTH: &str = "256";

#[component]
pub fn Sidebar() -> Element {
    let mut display_mode = use_signal(|| SidebarDisplayMode::Profiles);

    rsx!(rect {
        width: SIDEBAR_WIDTH,
        height: "100%",
        direction: "vertical",
        margin: "8 0 8 8",
        color: THEME.read().text_primary,

        spacing: "8",

        rect {
            width: "100%",
            height: "{48+12+12}",
            padding: "12",
            direction: "horizontal",
            spacing: "8",
            corner_radius: "8",
            background: THEME.read().bg_primary,


            cross_align: "center",
            main_align: "space-between",

            Button {
                width: "48",
                height: "48",
                stretch: false,
                padding: "{48/4}",
                corner_radius: "{48/2}",
                cross_align: "center",
                main_align: "center",

                onpress: move |_| {
                    *CURRENT_PAGE.write() = AppPage::Home;
                },

                svg {
                    width: "24",
                    height: "24",
                    svg_data: static_bytes(crate::assets::HOME_ICON),
                    fill: THEME.read().text_primary,
                }
            }

            Button {
                width: "48",
                height: "48",
                stretch: false,
                padding: "{48/4}",
                corner_radius: "{48/2}",
                cross_align: "center",
                main_align: "center",

                onpress: move |_| {
                    *CURRENT_PAGE.write() = AppPage::Settings;
                },

                svg {
                    width: "24",
                    height: "24",
                    svg_data: static_bytes(crate::assets::GEAR_ICON),
                    fill: THEME.read().text_primary,
                }
            }

            Button {
                width: "48",
                height: "48",
                stretch: false,
                padding: "{48/4}",
                corner_radius: "{48/2}",
                cross_align: "center",
                main_align: "center",

                onpress: move |_| {
                    let _ = open::that(crate::assets::DISCORD_INVITE_LINK);
                },

                svg {
                    width: "24",
                    height: "24",
                    svg_data: static_bytes(crate::assets::DISCORD_ICON),
                    fill: THEME.read().text_primary,
                }
            }

            Button {
                width: "48",
                height: "48",
                stretch: false,
                padding: "{48/4}",
                corner_radius: "{48/2}",
                cross_align: "center",
                main_align: "center",

                onpress: move |_| {
                    let _ = open::that(crate::assets::GITHUB_REPO_LINK);
                },

                svg {
                    width: "24",
                    height: "24",
                    svg_data: static_bytes(crate::assets::GITHUB_ICON),
                    fill: THEME.read().text_primary,
                }
            }
        }

        rect {
            background: THEME.read().bg_primary,
            padding: "8",
            corner_radius: "8",
            ScrollView {
                spacing: "8",
                scrollbar_theme: theme_with!(ScrollBarTheme {
                    background: THEME.read().bg_tertiary.into(),
                }),

                TabBar {
                    selected: display_mode(),
                    onclick: move |mode| {
                        *display_mode.write() = mode;
                    },
                }

                match display_mode() {
                    SidebarDisplayMode::Profiles => {
                        rsx!(
                            {CONFIG.read().profiles.clone().into_iter().map(|(profile_id, profile)| {
                                rsx!(Button {
                                    width: "100%",
                                    selected: *CURRENT_PAGE.read() == AppPage::Profile(profile_id.clone()),

                                    onpress: move |_| {
                                        *CURRENT_PAGE.write() = AppPage::Profile(profile_id.clone());
                                    },

                                    label {
                                        font_size: "16",
                                        "{profile.profile.name}"
                                    }
                                })
                            })}

                            Button {
                                width: "100%",
                                onpress: |_| {
                                    let uuid = uuid::Uuid::new_v4().to_string();
                                    let new_profile = config::ProfileConfig {
                                        profile: config::Profile {
                                            name: "New Profile".to_string(),
                                            use_default_profile: false,
                                        },
                                        discord: Default::default(),
                                        instances: Default::default(),
                                    };

                                    let toml_path = paths::config_profile_dir().join(format!("{uuid}.toml"));
                                    if let Ok(toml) = toml::to_string(&new_profile) {
                                        let _ = std::fs::write(&toml_path, toml);
                                    }

                                    *CONFIG.write() = config::Config::init();

                                    *CURRENT_PAGE.write() = AppPage::Profile(uuid);
                                },

                                svg {
                                        width: "24",
                                        height: "24",
                                        svg_data: static_bytes(crate::assets::PLUS_ICON),
                                        fill: "#ffffff",
                                    }

                                label {
                                    font_size: "16",
                                    "Add New Profile"
                                }
                            }
                        )
                    }
                    SidebarDisplayMode::Mods => {
                        rsx!(
                            {CONFIG.read().mods.clone().into_iter().map(|(mod_id, mod_)| {
                                rsx!(Button {
                                    width: "100%",
                                    selected: *CURRENT_PAGE.read() == AppPage::Mod(mod_id.clone()),

                                    onpress: move |_| {
                                        *CURRENT_PAGE.write() = AppPage::Mod(mod_id.clone());
                                    },

                                    label {
                                        font_size: "16",
                                        "{mod_.name}"
                                    }

                                })
                            })}

                            Button {
                                width: "100%",
                                onpress: |_| {
                                     *CURRENT_PAGE .write() = AppPage::ModFromTemplate;
                                },

                                svg {
                                        width: "24",
                                        height: "24",
                                        svg_data: static_bytes(crate::assets::PLUS_ICON),
                                        fill: "#ffffff",
                                    }

                                label {
                                    font_size: "16",
                                    "Add New Mod"
                                }
                            }
                        )
                    }
                }
            }
        }
    })
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SidebarDisplayMode {
    Profiles,
    Mods,
}

#[component]
fn TabBar(selected: SidebarDisplayMode, onclick: EventHandler<SidebarDisplayMode>) -> Element {
    let anim_underscore_offset =
        use_animation(move |_conf| AnimNum::new(0.0, 50.0).ease(Ease::InOut).time(100));

    let anim_underscore_width =
        use_animation(move |_conf| AnimNum::new(50.0, 100.0).ease(Ease::InOut).time(100));

    rsx!(rect {
        direction: "vertical",
        padding: "0 0 1 0",

        // The two tab buttons
        rect {
            direction: "horizontal",

            Button {
                width: "50%",
                padding: "8",
                main_align: "center",
                cross_align: "center",
                base_color: THEME.read().bg_primary,
                target_color: THEME.read().bg_primary,
                shadow: "0",

                onmouseenter: move |_e| {
                    if selected == SidebarDisplayMode::Mods {
                        anim_underscore_width.run(AnimDirection::Forward);
                        anim_underscore_offset.run(AnimDirection::Reverse);
                    }
                },
                onmouseleave: move |_e| {
                    if selected == SidebarDisplayMode::Mods {
                        anim_underscore_width.run(AnimDirection::Reverse);
                        anim_underscore_offset.run(AnimDirection::Forward);
                    }
                },
                onpress: move |_e| {
                    if anim_underscore_width.get().read().read() == 100.0 {
                        anim_underscore_width.run(AnimDirection::Reverse);
                    } else if selected == SidebarDisplayMode::Mods {
                        anim_underscore_width.run(AnimDirection::Reverse);
                        anim_underscore_offset.run(AnimDirection::Reverse);
                    }
                    onclick(SidebarDisplayMode::Profiles);
                },
                label {
                    font_size: "18",

                    "Profiles"
                }
            }
            Button {
                width: "50%",
                padding: "8",
                main_align: "center",
                cross_align: "center",
                base_color: THEME.read().bg_primary,
                target_color: THEME.read().bg_primary,
                shadow: "0",

                onmouseenter: move |_e| {
                    if selected == SidebarDisplayMode::Profiles {
                        anim_underscore_width.run(AnimDirection::Forward);
                    }
                },
                onmouseleave: move |_e| {
                    if selected == SidebarDisplayMode::Profiles {
                        anim_underscore_width.run(AnimDirection::Reverse);
                    }
                },
                onpress: move |_| {
                    if selected == SidebarDisplayMode::Profiles || anim_underscore_width.get().read().read() != 50.0 {
                        anim_underscore_offset.run(AnimDirection::Forward);
                        anim_underscore_width.run(AnimDirection::Reverse);
                    }
                    onclick(SidebarDisplayMode::Mods);
                },
                label {
                    font_size: "18",

                    "Mods"
                }
            }
        }

        rect {
            direction: "horizontal",
            // This moves the underline left/right
            rect {
                width: "{anim_underscore_offset.get().read().read()}%",
                height: "2",
            }

            // The underline
            rect {
                width: "{anim_underscore_width.get().read().read()}%",
                height: "2",
                background: THEME.read().blurple,
            }
        }
    })
}

#[component]
fn HomeIconButton() -> Element {
    let home_icon = static_bytes(crate::assets::HOME_ICON);

    let animation = hoverable!(move |_conf| {
        AnimColor::new(THEME.read().bg_tertiary, THEME.read().blurple)
            .ease(Ease::InOut)
            .time(100)
    });

    let bg_color = animation.animation.get().read().read();

    rsx!(rect {
        width: "48",
        height: "48",
        corner_radius: "{48/2}",

        background: bg_color,
        onmouseenter: animation.onmouseenter,
        onmouseleave: animation.onmouseleave,

        main_align: "center",
        cross_align: "center",

        svg {
            width: "24",
            height: "24",
            svg_data: home_icon,
            fill: "#ffffff",
        }
    })
}

#[component]
fn SettingsButton() -> Element {
    let gear_icon = static_bytes(crate::assets::GEAR_ICON);

    let animation = hoverable!(move |_conf| {
        AnimColor::new(THEME.read().bg_tertiary, THEME.read().blurple)
            .ease(Ease::InOut)
            .time(100)
    });

    let bg_color = animation.animation.get().read().read();

    rsx!(rect {
        width: "48",
        height: "48",
        corner_radius: "{48/2}",

        background: bg_color,
        onmouseenter: animation.onmouseenter,
        onmouseleave: animation.onmouseleave,

        main_align: "center",
        cross_align: "center",

        svg {
            width: "24",
            height: "24",
            svg_data: gear_icon,
            fill: "#ffffff",
        }
    })
}

#[component]
fn GithubButton() -> Element {
    let github_icon = static_bytes(crate::assets::GITHUB_ICON);

    let animation = hoverable!(move |_conf| {
        AnimColor::new(THEME.read().bg_tertiary, THEME.read().blurple)
            .ease(Ease::InOut)
            .time(100)
    });

    let bg_color = animation.animation.get().read().read();

    rsx!(rect {
        width: "48",
        height: "48",
        corner_radius: "{48/2}",

        background: bg_color,
        onmouseenter: animation.onmouseenter,
        onmouseleave: animation.onmouseleave,

        main_align: "center",
        cross_align: "center",

        onclick: move |_| {
            let _ = open::that(crate::assets::GITHUB_REPO_LINK);
        },

        svg {
            width: "24",
            height: "24",
            svg_data: github_icon,
            fill: "#ffffff",
        }
    })
}

#[component]
fn DiscordButton() -> Element {
    let discord_icon = static_bytes(crate::assets::DISCORD_ICON);

    let animation = hoverable!(move |_conf| {
        AnimColor::new(THEME.read().bg_tertiary, THEME.read().blurple)
            .ease(Ease::InOut)
            .time(100)
    });

    let bg_color = animation.animation.get().read().read();

    rsx!(rect {
        width: "48",
        height: "48",
        corner_radius: "{48/2}",

        background: bg_color,
        onmouseenter: animation.onmouseenter,
        onmouseleave: animation.onmouseleave,

        main_align: "center",
        cross_align: "center",

        onclick: move |_| {
            let _ = open::that(crate::assets::DISCORD_INVITE_LINK);
        },

        svg {
            width: "26",
            height: "26",
            svg_data: discord_icon,
            fill: "#ffffff",
        }
    })
}
