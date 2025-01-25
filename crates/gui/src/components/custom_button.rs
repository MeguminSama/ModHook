use freya::prelude::*;

use crate::utils::hoverable::hoverable;

#[component]
pub fn CustomButton(
    bg_anim: (String, String),
    onclick: EventHandler<MouseEvent>,
    children: Element,
    height: Option<String>,
    padding: Option<String>,
) -> Element {
    let mut bg_anim = hoverable!(move |_conf| {
        AnimColor::new(&bg_anim.0, &bg_anim.1)
            .ease(Ease::InOut)
            .time(100)
    });

    let bg_color = bg_anim.animation.get();

    let onmouseenter = move |ctx| {
        (bg_anim.onmouseenter)(ctx);
    };

    let onmouseleave = move |ctx| {
        (bg_anim.onmouseleave)(ctx);
    };

    let height = height.unwrap_or("64".to_string());
    let padding = padding.unwrap_or("0 0 0 8".to_string());

    let background = bg_color.read().read();

    rsx!(rect {
        padding: "{padding}",

        onclick: move |data| {
            #[allow(clippy::redundant_closure)]
            onclick(data)
        },
        rect {
            height: "{height}",
            min_width: "64",

            main_align: "center",
            cross_align: "center",

            padding: "0 14",
            corner_radius: "8",

            background: "{background}",
            onmouseenter: onmouseenter,
            onmouseleave: onmouseleave,

            {children}
        }
    })
}
