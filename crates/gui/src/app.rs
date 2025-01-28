use freya::prelude::*;
use modloader_core::utils::{find_running_instances, kill_pids, launch_detached_instance};
use modloader_core::{config, paths};

use crate::{AppPage, CONFIG, CURRENT_PAGE, POPUP_STATE, THEME};
use crate::{PopupState, components};

#[component]
pub fn app() -> Element {
    use components::sidebar::Sidebar;

    rsx!(
        rect {
            height: "100%",
            width: "100%",
            background: THEME.read().bg_secondary,
            color: THEME.read().text_primary,
            direction: "horizontal",

            match POPUP_STATE() {
                PopupState::ConfirmDeleteMod(mod_id) => rsx!(DeleteModPopup { mod_id }),
                PopupState::ConfirmDeleteProfile(profile_id) => rsx!(DeleteProfilePopup { profile_id }),
                PopupState::InstanceAlreadyRunning(profile_id, new_instance_id, args) => rsx!(InstanceAlreadyRunning { profile_id, new_instance_id, args }),
                PopupState::ConfirmDeleteInstance(profile_id, instance_id) => rsx!(DeleteInstancePopup { profile_id, instance_id }),
                PopupState::ConfirmKillProfile(profile_id) => rsx!(KillInstancesPopup { profile_id }),
                _ => rsx!(),
            }

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
    let dependents: Vec<String> = CONFIG
        .read()
        .profiles
        .values()
        .filter(|profile| {
            profile
                .instances
                .values()
                .any(|instance| instance.mod_id == mod_id)
        })
        .map(|profile| profile.profile.name.clone())
        .collect();

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

            *CURRENT_PAGE.write() = AppPage::Home;
        },
        on_cancel: move |_| {
            *POPUP_STATE.write() = PopupState::None;
        }
    })
}

#[component]
fn DeleteInstancePopup(profile_id: String, instance_id: String) -> Element {
    rsx!(components::popup::ConfirmationPopup {
        title: "Delete Instance",
        body: format!(
            "Are you sure you want to delete this instance?\nAny active instances of the current profile will be stopped."
        ),
        on_confirm: move |_| {
            *POPUP_STATE.write() = PopupState::None;

            let pids = find_running_instances(
                &profile_id,
                &CONFIG.read().profiles.get(&profile_id).unwrap().clone(),
            );

            kill_pids(pids);

            // Delete instance

            if let Some(profile) = CONFIG.write().profiles.get_mut(&profile_id) {
                profile.instances.remove(&instance_id);
                let Ok(profile_toml) = toml::to_string::<config::ProfileConfig>(profile) else {
                    return;
                };

                let profile_path = paths::config_profile_dir().join(format!("{profile_id}.toml"));
                let _ = std::fs::write(profile_path, profile_toml);
            }

            *CONFIG.write() = config::Config::init();
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
        },
        on_cancel: move |_| {
            *POPUP_STATE.write() = PopupState::None;
        }
    })
}
