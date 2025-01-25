// TODO: Implement theme switching.
// #[derive(Clone)]
// pub enum Theme {
//     Dark,
//     // Light,
// }

// impl Theme {
//     pub fn get(&self) -> &'static ThemeDef {
//         match self {
//             Theme::Dark => &DARK_THEME,
//             // Theme::Light => &LIGHT_THEME,
//         }
//     }
// }

pub const DARK_THEME: ThemeDef = ThemeDef {
    bg_primary: "rgb(46, 46, 52)",
    bg_secondary: "rgb(52, 52, 58)",
    bg_tertiary: "rgb(56, 56, 62)",

    text_primary: "white",
    text_secondary: "rgb(219, 219, 219)",

    bg_success: "rgb(68, 162, 91)",
    bg_danger: "rgb(210, 45, 57)",

    blurple: "rgb(88, 101, 242)",
};

pub struct ThemeDef {
    pub bg_primary: &'static str,
    pub bg_secondary: &'static str,
    pub bg_tertiary: &'static str,

    pub text_primary: &'static str,
    pub text_secondary: &'static str,

    pub bg_success: &'static str,
    pub bg_danger: &'static str,

    pub blurple: &'static str,
}
