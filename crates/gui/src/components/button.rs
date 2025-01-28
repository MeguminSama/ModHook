use freya::prelude::*;

use crate::{THEME, utils::hoverable::hoverable};

#[component]
pub fn Button(
    children: Element,
    onpress: Option<EventHandler<PressEvent>>,
    selected: Option<bool>,
    base_color: Option<String>,
    target_color: Option<String>,
    selected_color: Option<String>,
    width: Option<String>,
    height: Option<String>,
    corner_radius: Option<String>,
    padding: Option<String>,
    main_align: Option<String>,
    cross_align: Option<String>,
    onmouseenter: Option<EventHandler<MouseEvent>>,
    onmouseleave: Option<EventHandler<MouseEvent>>,
    shadow: Option<String>,
    stretch: Option<bool>,
    margin: Option<String>,
    direction: Option<String>,
    spacing: Option<String>,
) -> Element {
    let mut animation = hoverable!(move |_conf| {
        let base_color = base_color.as_deref().unwrap_or(THEME.read().bg_tertiary);
        let target_color = target_color.as_deref().unwrap_or(THEME.read().blurple);
        AnimColor::new(base_color, target_color)
            .ease(Ease::InOut)
            .time(100)
    });

    let background = match selected {
        Some(true) => selected_color
            .as_deref()
            .unwrap_or(THEME.read().blurple)
            .to_string(),
        _ => animation.animation.get().read().read(),
    };

    let corner_radius = match corner_radius {
        Some(corner_radius) => corner_radius,
        None => "8".to_string(),
    };

    let width = match width {
        Some(width) => width,
        None => "auto".to_string(),
    };

    let height = match height {
        Some(height) => height,
        None => "40".to_string(),
    };

    let padding = match padding {
        Some(padding) => padding,
        None => "8 16".to_string(),
    };

    let shadow = match shadow {
        Some(shadow) => shadow,
        None => freya::hooks::DARK_THEME.button.shadow.into(),
    };

    let stretch = stretch.unwrap_or(true);
    let button_width = if stretch { "100%" } else { "auto" };
    let button_height = if stretch { "auto" } else { "100%" };

    rsx!(rect {
        width: width,
        height: height,
        margin: margin,
        onmouseenter: move |e| {
            (animation.onmouseenter)(e.clone());
            if let Some(onmouseenter) = onmouseenter {
                onmouseenter(e);
            }
        },
        onmouseleave: move |e| {
            (animation.onmouseleave)(e.clone());
            if let Some(onmouseleave) = onmouseleave {
                onmouseleave(e);
            }
        },
        freya::prelude::Button {
            onpress: onpress,
            theme: theme_with!(ButtonTheme {
                background: background.clone().into(),
                hover_background: background.clone().into(),
                border_fill: background.clone().into(),
                focus_border_fill: THEME.read().blurple.into(),
                padding: padding.clone().into(),
                shadow: shadow.into(),
                width: button_width.into(),
                height: button_height.into(),
                corner_radius: corner_radius.clone().into(),
            }),
            rect {
                // content: "fit",
                corner_radius: corner_radius.clone(),
                direction: direction.as_deref().unwrap_or("horizontal"),
                cross_align: cross_align.as_deref().unwrap_or("center"),
                main_align: main_align.as_deref().unwrap_or("start"),
                spacing: spacing.as_deref().unwrap_or("8"),
                width: if stretch { "calc(100% - 4)" } else { "auto" },
                height: if stretch { "calc(100% - 4)" } else { "auto" },
                color: THEME.read().text_primary,
                background: background,

                {children}
            }
        }
    })
}
