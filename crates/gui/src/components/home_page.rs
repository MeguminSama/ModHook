use freya::prelude::*;
use modloader_core::config;

use crate::{CONFIG, THEME, utils::icons::GetIcon};

fn get_starred_profiles() -> Vec<(String, config::Profile, Vec<(String, config::Instance)>)> {
    CONFIG
        .read()
        .profiles
        .clone()
        .into_iter()
        .filter_map(|(profile_id, profile)| {
            let instances = profile
                .instances
                .into_iter()
                .filter(|(_, instance)| instance.starred)
                .collect::<Vec<_>>();

            if instances.is_empty() {
                None
            } else {
                Some((profile_id, profile.profile, instances))
            }
        })
        .collect::<Vec<_>>()
}

#[component]
pub fn HomePage() -> Element {
    let starred = use_signal(get_starred_profiles);

    dbg!(&starred);

    rsx!(
        rect {
            direction: "vertical",
            padding: "8",
            spacing: "8",

            Header { },

            ScrollView {
                direction: "vertical",
                spacing: "8",

                for (profile_id, profile, instances) in starred() {
                    ProfileBlock { profile_id, profile, instances }
                }
            }
        }
    )
}

#[component]
fn Header() -> Element {
    rsx!(rect {
        width: "100%",
        padding: "12 8 12 12",
        height: "{48 + 12 + 12}",
        corner_radius: "8",
        border: "0 0 2 0 outer {THEME.read().bg_secondary}",

        background: THEME.read().bg_primary,

        direction: "horizontal",
        cross_align: "center",

        spacing: "12",

        label {
            font_size: "18",

            "Home"
        }
    })
}

#[component]
fn ProfileBlock(
    profile_id: String,
    profile: config::Profile,
    instances: Vec<(String, config::Instance)>,
) -> Element {
    rsx!(
        rect {
            padding: "8",
            background: THEME.read().bg_primary,
            corner_radius: "8",

            label {
                "{profile.name}"
            }

            ScrollView {
                direction: "horizontal",
                spacing: "8",
                padding: "8",
                width: "auto",
                height: "auto",

                for (instance_id, instance) in instances {
                    InstanceBlock { instance_id, instance }
                }
            }
        }
    )
}

#[component]
fn InstanceBlock(instance_id: String, instance: config::Instance) -> Element {
    let icon = instance.get_icon().map(dynamic_bytes);

    rsx!(
        rect {
            padding: "8",
            corner_radius: "8",
            width: "128",
            background: THEME.read().bg_secondary,
            direction: "vertical",
            cross_align: "center",
            main_align: "center",
            overflow: "clip",


            if icon.is_some() {
                image {
                    width: "48",
                    height: "48",
                    image_data: icon,
                }
            }

            label {
                max_lines: "1",
                text_overflow: "ellipsis",
                "{instance.name}"
            }
        }
    )
}
