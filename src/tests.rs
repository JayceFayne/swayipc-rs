use crate::{ensure, Connection, EventType, Fallible};

#[test]
fn connect() -> Fallible<()> {
    Connection::new()?;
    Ok(())
}

#[test]
fn run_command_nothing() -> Fallible<()> {
    let mut connection = Connection::new()?;
    let result = connection.run_command("")?;
    ensure!(result.len() == 0);
    Ok(())
}

#[test]
fn run_command_single_sucess() -> Fallible<()> {
    let mut connection = Connection::new()?;
    let result = connection.run_command("exec /bin/true")?;
    ensure!(result.len() == 1);
    ensure!(result[0].success);
    Ok(())
}

#[test]
fn run_command_multiple_success() -> Fallible<()> {
    let mut connection = Connection::new()?;
    let result = connection.run_command("exec /bin/true; exec /bin/true")?;
    ensure!(result.len() == 2);
    ensure!(result[0].success);
    ensure!(result[1].success);
    Ok(())
}

#[test]
fn run_command_fail() -> Fallible<()> {
    let mut connection = Connection::new()?;
    let result = connection.run_command("somerandomcommand")?;
    ensure!(result.len() == 1);
    ensure!(!result[0].success);
    Ok(())
}

#[test]
fn get_workspaces() -> Fallible<()> {
    Connection::new()?.get_workspaces()?;
    Ok(())
}

#[test]
fn get_outputs() -> Fallible<()> {
    Connection::new()?.get_outputs()?;
    Ok(())
}

#[test]
fn get_tree() -> Fallible<()> {
    Connection::new()?.get_tree()?;
    Ok(())
}

#[test]
fn get_marks() -> Fallible<()> {
    Connection::new()?.get_marks()?;
    Ok(())
}

#[test]
fn get_bar_ids() -> Fallible<()> {
    Connection::new()?.get_bar_ids()?;
    Ok(())
}

#[test]
fn get_bar_ids_and_one_config() -> Fallible<()> {
    let mut connection = Connection::new()?;
    let ids = connection.get_bar_ids()?;
    connection.get_bar_config(&ids[0])?;
    Ok(())
}

#[test]
fn get_version() -> Fallible<()> {
    Connection::new()?.get_version()?;
    Ok(())
}

#[test]
fn get_binding_modes() -> Fallible<()> {
    Connection::new()?.get_binding_modes()?;
    Ok(())
}

#[test]
fn get_config() -> Fallible<()> {
    Connection::new()?.get_config()?;
    Ok(())
}

#[test]
fn send_tick() -> Fallible<()> {
    let success = Connection::new()?.send_tick("")?;
    assert!(success);
    Ok(())
}

#[test]
fn send_sync() -> Fallible<()> {
    let success = Connection::new()?.send_sync()?;
    assert!(!success);
    Ok(())
}

#[test]
fn get_inputs() -> Fallible<()> {
    Connection::new()?.get_inputs()?;
    Ok(())
}

#[test]
fn get_seats() -> Fallible<()> {
    Connection::new()?.get_seats()?;
    Ok(())
}

#[test]
fn event_subscribe_all() -> Fallible<()> {
    Connection::new()?.subscribe(&[
        EventType::Workspace,
        EventType::Mode,
        EventType::Window,
        EventType::BarConfigUpdate,
        EventType::Binding,
        EventType::Shutdown,
        EventType::Tick,
        //EventType::BarStatusUpdate, //FIXME: Fails for some reason :shrug:
        EventType::Input,
    ])?;
    Ok(())
}

#[test]
fn find_in_tree_as_ref() -> Fallible<()> {
    ensure!(Connection::new()?.get_tree()?.find(|n| n.focused).is_some());
    Ok(())
}

#[test]
fn find_in_tree() -> Fallible<()> {
    ensure!(Connection::new()?
        .get_tree()?
        .find_as_ref(|n| n.focused)
        .is_some());
    Ok(())
}

#[test]
fn find_in_tree_comp() -> Fallible<()> {
    ensure!(
        Connection::new()?.get_tree()?.find_as_ref(|n| n.focused)
            == Connection::new()?.get_tree()?.find(|n| n.focused).as_ref()
    );
    Ok(())
}

#[test]
fn find_focused_as_ref() -> Fallible<()> {
    ensure!(Connection::new()?
        .get_tree()?
        .find_focused_as_ref(|n| n.focused)
        .is_some());
    Ok(())
}

#[test]
fn find_focused() -> Fallible<()> {
    ensure!(Connection::new()?
        .get_tree()?
        .find_focused(|n| n.focused)
        .is_some());
    Ok(())
}

#[test]
fn find_in_tree_comp_find_focused() -> Fallible<()> {
    ensure!(
        Connection::new()?.get_tree()?.find_focused(|n| n.focused)
            == Connection::new()?.get_tree()?.find(|n| n.focused)
    );
    Ok(())
}
