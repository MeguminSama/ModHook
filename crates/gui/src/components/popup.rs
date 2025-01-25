use freya::prelude::*;

use crate::components::button::Button;
use crate::{THEME, utils::hoverable::hoverable};

#[component]
pub fn ConfirmationPopup(
    title: String,
    body: String,
    on_confirm: EventHandler<()>,
    on_cancel: EventHandler<()>,
) -> Element {
    use_focus().focus();

    rsx!(Popup {
        theme: theme_with!(PopupTheme {
            background: THEME.read().bg_primary.into(),
            color: THEME.read().text_primary.into(),
            height: "auto".into(),
        }),
        close_on_escape_key: true,

        oncloserequest: move |_| {
            on_cancel.call(());
        },
        PopupTitle {
            label {
                "{title}"
            },
        },
        PopupContent {
            label {
                "{body}"
            },

            rect {
                margin: "8",
                spacing: "8",
                direction: "horizontal",
                width: "fill",
                height: "auto",
                cross_align: "end",
                main_align: "end",

                Button {
                    stretch: false,
                    onpress: move |_| {
                        on_cancel.call(());
                    },
                    label {
                        "Cancel"
                    },
                },

                Button {
                    stretch: false,
                    base_color: THEME.read().bg_danger,
                    onpress: move |_| {
                        on_confirm.call(());
                    },

                    label {
                        "Confirm"
                    }
                }
            },
        },
    })
}

#[component]
fn PopupButton(
    name: String,
    onclick: EventHandler<MouseEvent>,
    icon: Option<&'static [u8]>,
    color: Option<String>,
) -> Element {
    let animation = hoverable!(move |_conf| {
        AnimColor::new(
            if let Some(ref color) = color {
                color
            } else {
                THEME.read().bg_secondary
            },
            THEME.read().blurple,
        )
        .ease(Ease::InOut)
        .time(100)
    });

    let bg_color = animation.animation.get().read().read();

    let icon = icon.map(static_bytes);

    rsx!(rect {
        // width: "100%",
        padding: "8 16",
        corner_radius: "8",
        direction: "horizontal",
        cross_align: "center",
        spacing: "8",
        height: "42",
        color: THEME.read().text_primary,
        background: bg_color,
        onmouseenter: animation.onmouseenter,
        onmouseleave: animation.onmouseleave,

        onclick: {
            #[allow(clippy::redundant_closure)]
            move |e| onclick(e)
        },

        if icon.is_some() {
            svg {
                width: "24",
                height: "24",
                svg_data: icon,
                fill: "#ffffff",
            }
        }

        label {
            font_size: "16",

            "{name}"
        }
    })
}
