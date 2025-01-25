use discord_modloader::utils::{find_running_instances, kill_pids, launch_detached_instance};
use discord_modloader::{config, paths};
use freya::prelude::*;

use crate::{AppPage, CONFIG, CURRENT_PAGE, POPUP_STATE, REFRESH_PIDS, THEME};
use crate::{PopupState, components};

#[component]
pub fn app() -> Element {
    use components::sidebar::Sidebar;

    rsx!(
        rect {
            height: "100%",
            width: "100%",
            background: "{THEME.read().bg_secondary}",
            color: "{THEME.read().text_primary}",
            direction: "horizontal",

            if let PopupState::ConfirmDeleteMod(mod_id) = POPUP_STATE() {
                DeleteModPopup { mod_id }
            } else if let PopupState::ConfirmDeleteProfile(profile_id) = POPUP_STATE() {
                DeleteProfilePopup { profile_id }
            } else if let PopupState::InstanceAlreadyRunning(profile_id, new_instance_id, args) = POPUP_STATE() {
                InstanceAlreadyRunning { profile_id, new_instance_id, args }
            } else if let PopupState::ConfirmKillProfile(profile_id) = POPUP_STATE() {
                KillInstancesPopup { profile_id }
            },

            Sidebar {},

            Router {}
        }
    )
}

#[component]
pub fn Router() -> Element {
    use components::home_page::HomePage;
    use components::mod_from_template_page::ModFromTemplatePage;
    use components::mod_page::ModPage;
    use components::profile_page::ProfilePage;
    use components::settings_page::SettingsPage;

    match CURRENT_PAGE.read().clone() {
        AppPage::Home => rsx!(HomePage {}),
        AppPage::Mod(mod_id) => rsx!(ModPage { mod_id }),
        AppPage::ModFromTemplate => rsx!(ModFromTemplatePage {}),
        AppPage::Profile(profile_id) => rsx!(ProfilePage { profile_id }),
        AppPage::Settings => rsx!(SettingsPage {}),
    }
}

#[component]
fn DeleteModPopup(mod_id: String) -> Element {
    let dependents = CONFIG
        .read()
        .profiles
        .iter()
        .filter(|(_k, v)| v.instances.iter().any(|(_k, v)| v.mod_id == mod_id))
        .map(|(_k, v)| v.profile.name.clone())
        .collect::<Vec<_>>();

    let dependent_str = if dependents.is_empty() {
        "No profiles depend on this mod.".to_string()
    } else {
        format!(
            "Some profiles depend on this mod and will need updating:\n - {}",
            dependents.join("\n - ")
        )
    };

    rsx!(components::popup::ConfirmationPopup {
        title: "Delete Mod",
        body: format!("Are you sure you want to delete this mod?\n{dependent_str}"),
        on_confirm: move |_| {
            *POPUP_STATE.write() = PopupState::None;

            // Delete mod
            let mod_path = paths::config_mods_dir().join(format!("{mod_id}.toml"));
            let _ = std::fs::remove_file(mod_path);

            *CONFIG.write() = config::Config::init();

            *CURRENT_PAGE.write() = AppPage::Home;
        },
        on_cancel: move |_| {
            *POPUP_STATE.write() = PopupState::None;
        }
    })
}

#[component]
fn DeleteProfilePopup(profile_id: String) -> Element {
    rsx!(components::popup::ConfirmationPopup {
        title: "Delete Mod",
        body: format!(
            "Are you sure you want to delete this profile?\nAny active instances of this profile will be stopped."
        ),
        on_confirm: move |_| {
            *POPUP_STATE.write() = PopupState::None;

            let pids = find_running_instances(
                &profile_id,
                &CONFIG.read().profiles.get(&profile_id).unwrap().clone(),
            );

            kill_pids(pids);

            // Delete mod
            let profile_path = paths::config_profile_dir().join(format!("{profile_id}.toml"));
            let _ = std::fs::remove_file(profile_path);

            *CONFIG.write() = config::Config::init();

            *REFRESH_PIDS.write() = ();

            *CURRENT_PAGE.write() = AppPage::Home;
        },
        on_cancel: move |_| {
            *POPUP_STATE.write() = PopupState::None;
        }
    })
}

#[component]
fn KillInstancesPopup(profile_id: String) -> Element {
    rsx!(components::popup::ConfirmationPopup {
        title: "Stop Running Instances",
        body: "Are you sure you want to stop all running instances of this profile?",
        on_confirm: move |_| {
            *POPUP_STATE.write() = PopupState::None;

            let pids = find_running_instances(
                &profile_id,
                &CONFIG.read().profiles.get(&profile_id).unwrap().clone(),
            );

            kill_pids(pids);

            *REFRESH_PIDS.write() = ();
        },
        on_cancel: move |_| {
            *POPUP_STATE.write() = PopupState::None;
        }
    })
}

#[component]
fn InstanceAlreadyRunning(profile_id: String, new_instance_id: String, args: String) -> Element {
    rsx!(components::popup::ConfirmationPopup {
        title: "Instance already running",
        body: "An instance of this profile is already running.\nDo you want to stop the running instance and start a new one?",
        on_confirm: move |_| {
            *POPUP_STATE.write() = PopupState::None;

            let pids = find_running_instances(
                &profile_id,
                &CONFIG.read().profiles.get(&profile_id).unwrap().clone(),
            );

            kill_pids(pids);

            launch_detached_instance(&profile_id, &new_instance_id, &args, false).unwrap();

            *REFRESH_PIDS.write() = ();
        },
        on_cancel: move |_| {
            *POPUP_STATE.write() = PopupState::None;
        }
    })
}
