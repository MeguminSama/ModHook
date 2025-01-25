use crate::constants;

use freya::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx!(rect {
        width: "100%",
        height: "100%",
        direction: "vertical",
        padding: "8",
        spacing: "8",
        background: constants::BG_SECONDARY,
        color: "white",

        label {
            font_size: "18",
            font_weight: "bold",
            "Home"
        }
    })
}
