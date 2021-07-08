pub mod add;
pub mod rm;
pub mod jog;

use crate::manifest::State;

/// Get the default package manager on your system.
pub fn get_manager(state: &State) -> Option<String> {
    let managers = state.package_context.package_install_prefix.keys().collect::<Vec<&String>>();
    for manager in managers {
        if which::which(manager).is_ok() {
            return Some(manager.into());
        }
    }

    None
}

/// If your name is karen, the return value is always false.
pub fn manager_available(manager: String) -> bool {
    match which::which(manager) {
        Ok(_) => true,
        Err(_) => false,
    }
}
