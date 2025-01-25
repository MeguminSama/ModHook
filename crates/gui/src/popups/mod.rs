pub mod already_launched;
pub mod create_instance;
pub mod delete_instance;
pub mod delete_profile;
pub mod launching;

use discord_modloader::config;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PopupState {
    Hidden,
    Launching(config::Instance),
    // Launched,
    AlreadyLaunched(String, String),
    DeleteProfile(String),
    DeleteInstance(String, String),
    CreateNewInstance(String),
    // Error,
}

pub use already_launched::*;
pub use create_instance::*;
pub use delete_instance::*;
pub use delete_profile::*;
pub use launching::*;
