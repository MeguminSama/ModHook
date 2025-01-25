use crate::pages::{CreateNewProfile, Home, SelectedProfile, Settings};

use freya::prelude::*;

#[derive(Clone, PartialEq)]
pub enum CurrentPage {
    Home,
    SelectedProfile(String),
    CreateNewProfile(Option<String>),
    Settings,
}

#[component]
pub fn MainContent(page: CurrentPage) -> Element {
    match page {
        CurrentPage::Home => rsx!(Home {}),
        CurrentPage::SelectedProfile(id) => rsx!(SelectedProfile { id }),
        CurrentPage::CreateNewProfile(profile_id) => rsx!(CreateNewProfile { profile_id }),
        CurrentPage::Settings => rsx!(Settings {}),
    }
}
