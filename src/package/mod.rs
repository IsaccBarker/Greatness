pub mod overload;
pub mod add;
pub mod rm;
pub mod jog;

use crate::manifest::State;

/// Get the default package manager on your system.
pub fn get_manager(state: &State) -> Option<String> {
    let managers = state.package_context.package_install_prefix.keys().collect::<Vec<&String>>();
    let mut ok_managers: Vec<String> = vec![];
    let mut winning = (0 as u8, "".to_owned());

    // Get all posible managers installed on the system.
    for manager in managers {
        if which::which(manager).is_ok() {
            ok_managers.push(manager.into());
        }
    }

    // Given a list of managers, get the one with the
    // highest priority.
    for ok_manager in ok_managers {
        if state.package_context.package_install_prefix.get_key_value(&ok_manager).unwrap().1.1 >= winning.0 {
            winning = (state.package_context.package_install_prefix.get_key_value(&ok_manager).unwrap().1.1, ok_manager);
        }
    }

    // None found? Return none.
    if winning.1 == "".to_owned() {
        return None;
    }

    Some(winning.1)
}

/// If your name is karen, the return value is always false.
pub fn manager_available(manager: String) -> bool {
    match which::which(manager) {
        Ok(_) => true,
        Err(_) => false,
    }
}
