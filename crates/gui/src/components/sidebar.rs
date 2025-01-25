use crate::{constants, utils::hoverable::hoverable};
use discord_modloader::config;

use freya::prelude::*;
use itertools::Itertools as _;

use super::main_content::CurrentPage;

pub const SIDEBAR_WIDTH: f32 = 256.;

#[component]
pub fn ProfileList(onpagechange: EventHandler<CurrentPage>) -> Element {
    let ctx = use_context::<Signal<crate::AppState>>();

    let profiles = use_memo(move || {
        let profiles = ctx().config.profiles.clone();
        profiles
            .into_iter()
            .sorted_by_key(|(id, _)| id.clone())
            .collect::<Vec<_>>()
    });

    let create_profile = hoverable!(move |_conf| {
        AnimColor::new(constants::BG_SECONDARY, constants::BLURPLE)
            .ease(Ease::InOut)
            .time(100)
    });

    let create_profile_anim = create_profile.animation.get();

    let bg_create_profile = if let CurrentPage::CreateNewProfile(_) = ctx().current_page {
        constants::BLURPLE
    } else {
        &create_profile_anim.read().read()
    };

    let settings = hoverable!(move |_conf| {
        AnimColor::new(constants::BG_SECONDARY, constants::BLURPLE)
            .ease(Ease::InOut)
            .time(100)
    });

    let settings_anim = settings.animation.get();

    let bg_settings = if ctx().current_page == CurrentPage::Settings {
        constants::BLURPLE
    } else {
        &settings_anim.read().read()
    };

    rsx!(rect {
        width: "{SIDEBAR_WIDTH}",
        height: "100%",
        direction: "vertical",
        background: constants::BG_PRIMARY,
        padding: "8",
        color: "white",
        corner_radius: "4",

        label {
            font_size: "18",
            font_weight: "bold",
            "Profiles"
        }

        ScrollView {
            spacing: "4",

            {
                profiles().into_iter().map(|(id, profile)| {
                    rsx!(ProfileListEntry {
                        profile: profile.clone(),
                        selected: ctx().current_page == CurrentPage::SelectedProfile(id.clone()),
                        onclick: move |_| {
                            onpagechange.call(CurrentPage::SelectedProfile(id.clone()));
                        }
                    })
                })
            }

            rect {
                width: "100%",
                padding: "8",
                color: "white",
                corner_radius: "4",

                background: bg_create_profile,
                onmouseenter: create_profile.onmouseenter,
                onmouseleave: create_profile.onmouseleave,

                onclick: move |_| {
                    onpagechange.call(CurrentPage::CreateNewProfile(None));
                },

                label {
                    font_size: "18",
                    font_weight: "bold",
                    "Create New Profile"
                }
            }

            rect {
                width: "100%",
                padding: "8",
                color: "white",
                corner_radius: "4",

                background: bg_settings,
                onmouseenter: settings.onmouseenter,
                onmouseleave: settings.onmouseleave,

                onclick: move |_| {
                    onpagechange.call(CurrentPage::Settings);
                },

                label {
                    font_size: "18",
                    font_weight: "bold",
                    "Settings"
                }
            }
        }

    })
}

#[component]
fn ProfileListEntry(
    profile: config::ProfileConfig,
    selected: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let bg_anim = hoverable!(move |_conf| {
        AnimColor::new(constants::BG_SECONDARY, constants::BLURPLE)
            .ease(Ease::InOut)
            .time(100)
    });

    let bg_color = bg_anim.animation.get();

    let bg_color = if selected {
        constants::BLURPLE
    } else {
        &bg_color.read().read()
    };

    rsx!(rect {
        width: "100%",
        // height: "32",
        padding: "8",
        color: "white",
        corner_radius: "4",

        background: bg_color,
        onmouseenter: bg_anim.onmouseenter,
        onmouseleave: bg_anim.onmouseleave,

        onclick: move |evt| onclick.call(evt),

        label {
            font_size: "18",
            font_weight: "bold",
            {profile.profile.name}
        }
    })
}
