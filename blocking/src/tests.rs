use crate::{Connection, EventType};

#[test]
fn connect() {
    Connection::new().unwrap();
}

#[test]
fn run_command_nothing() {
    let mut connection = Connection::new().unwrap();
    let result = connection.run_command("").unwrap();
    assert!(result.is_empty());
}

#[test]
fn run_command_single_success() {
    let mut connection = Connection::new().unwrap();
    let result = connection.run_command("exec /bin/true").unwrap();
    assert_eq!(result.len(), 1);
    result[0].as_ref().unwrap();
}

#[test]
fn run_command_multiple_success() {
    let mut connection = Connection::new().unwrap();
    let result = connection
        .run_command("exec /bin/true; exec /bin/true")
        .unwrap();
    assert_eq!(result.len(), 2);
    result[0].as_ref().unwrap();
    result[1].as_ref().unwrap();
}

#[test]
fn run_command_fail() {
    let mut connection = Connection::new().unwrap();
    let result = connection.run_command("somerandomcommand").unwrap();
    assert_eq!(result.len(), 1);
    assert!(result[0].as_ref().is_err());
}

#[test]
fn get_workspaces() {
    Connection::new().unwrap().get_workspaces().unwrap();
}

#[test]
fn get_outputs() {
    Connection::new().unwrap().get_outputs().unwrap();
}

#[test]
fn get_tree() {
    Connection::new().unwrap().get_tree().unwrap();
}

#[test]
fn get_marks() {
    Connection::new().unwrap().get_marks().unwrap();
}

#[test]
fn get_bar_ids() {
    Connection::new().unwrap().get_bar_ids().unwrap();
}

#[test]
fn get_bar_ids_and_one_config() {
    let mut connection = Connection::new().unwrap();
    let ids = connection.get_bar_ids().unwrap();
    connection.get_bar_config(&ids[0]).unwrap();
}

#[test]
fn get_version() {
    Connection::new().unwrap().get_version().unwrap();
}

#[test]
fn get_binding_modes() {
    Connection::new().unwrap().get_binding_modes().unwrap();
}

#[test]
fn get_config() {
    Connection::new().unwrap().get_config().unwrap();
}

#[test]
fn send_tick() {
    let success = Connection::new().unwrap().send_tick("").unwrap();
    assert!(success);
}

#[test]
fn sync() {
    let success = Connection::new().unwrap().sync().unwrap();
    assert!(!success, "sync should always return false on sway");
}

#[test]
fn get_binding_state() {
    Connection::new().unwrap().get_binding_state().unwrap();
}

#[test]
fn get_inputs() {
    Connection::new().unwrap().get_inputs().unwrap();
}

#[test]
fn get_seats() {
    Connection::new().unwrap().get_seats().unwrap();
}

#[test]
fn event_subscribe_all() {
    Connection::new()
        .unwrap()
        .subscribe(&[
            EventType::Workspace,
            EventType::Mode,
            EventType::Window,
            EventType::BarConfigUpdate,
            EventType::Binding,
            EventType::Shutdown,
            EventType::Tick,
            EventType::BarStateUpdate,
            EventType::Input,
        ])
        .unwrap();
}

#[test]
fn find_in_tree() {
    assert!(Connection::new()
        .unwrap()
        .get_tree()
        .unwrap()
        .find_as_ref(|n| n.focused)
        .is_some());
}

#[test]
fn find_in_tree_comp() {
    assert_eq!(
        Connection::new()
            .unwrap()
            .get_tree()
            .unwrap()
            .find_as_ref(|n| n.focused),
        Connection::new()
            .unwrap()
            .get_tree()
            .unwrap()
            .find(|n| n.focused)
            .as_ref()
    );
}

#[test]
fn find_focused_as_ref() {
    assert!(Connection::new()
        .unwrap()
        .get_tree()
        .unwrap()
        .find_focused_as_ref(|n| n.focused)
        .is_some());
}

#[test]
fn find_focused() {
    assert!(Connection::new()
        .unwrap()
        .get_tree()
        .unwrap()
        .find_focused(|n| n.focused)
        .is_some());
}

#[test]
fn find_in_tree_comp_find_focused() {
    assert_eq!(
        Connection::new()
            .unwrap()
            .get_tree()
            .unwrap()
            .find_focused(|n| n.focused),
        Connection::new()
            .unwrap()
            .get_tree()
            .unwrap()
            .find(|n| n.focused)
    );
}
