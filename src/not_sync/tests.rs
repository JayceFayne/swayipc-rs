use crate::{ensure, Connection, EventType, Fallible};

#[async_std::test]
async fn connect() -> Fallible<()> {
    Connection::new().await?;
    Ok(())
}

#[async_std::test]
async fn run_command_nothing() -> Fallible<()> {
    let mut connection = Connection::new().await?;
    let result = connection.run_command("").await?;
    ensure!(result.is_empty());
    Ok(())
}

#[async_std::test]
async fn run_command_single_sucess() -> Fallible<()> {
    let mut connection = Connection::new().await?;
    let result = connection.run_command("exec /bin/true").await?;
    ensure!(result.len() == 1);
    ensure!(result[0].success);
    Ok(())
}

#[async_std::test]
async fn run_command_multiple_success() -> Fallible<()> {
    let mut connection = Connection::new().await?;
    let result = connection
        .run_command("exec /bin/true; exec /bin/true")
        .await?;
    ensure!(result.len() == 2);
    ensure!(result[0].success);
    ensure!(result[1].success);
    Ok(())
}

#[async_std::test]
async fn run_command_fail() -> Fallible<()> {
    let mut connection = Connection::new().await?;
    let result = connection.run_command("somerandomcommand").await?;
    ensure!(result.len() == 1);
    ensure!(!result[0].success);
    Ok(())
}

#[async_std::test]
async fn get_workspaces() -> Fallible<()> {
    Connection::new().await?.get_workspaces().await?;
    Ok(())
}

#[async_std::test]
async fn get_outputs() -> Fallible<()> {
    Connection::new().await?.get_outputs().await?;
    Ok(())
}

#[async_std::test]
async fn get_tree() -> Fallible<()> {
    Connection::new().await?.get_tree().await?;
    Ok(())
}

#[async_std::test]
async fn get_marks() -> Fallible<()> {
    Connection::new().await?.get_marks().await?;
    Ok(())
}

#[async_std::test]
async fn get_bar_ids() -> Fallible<()> {
    Connection::new().await?.get_bar_ids().await?;
    Ok(())
}

#[async_std::test]
async fn get_bar_ids_and_one_config() -> Fallible<()> {
    let mut connection = Connection::new().await?;
    let ids = connection.get_bar_ids().await?;
    connection.get_bar_config(&ids[0]).await?;
    Ok(())
}

#[async_std::test]
async fn get_version() -> Fallible<()> {
    Connection::new().await?.get_version().await?;
    Ok(())
}

#[async_std::test]
async fn get_binding_modes() -> Fallible<()> {
    Connection::new().await?.get_binding_modes().await?;
    Ok(())
}

#[async_std::test]
async fn get_config() -> Fallible<()> {
    Connection::new().await?.get_config().await?;
    Ok(())
}

#[async_std::test]
async fn send_tick() -> Fallible<()> {
    let success = Connection::new().await?.send_tick("").await?;
    ensure!(success);
    Ok(())
}

#[async_std::test]
async fn send_sync() -> Fallible<()> {
    let success = Connection::new().await?.send_sync().await?;
    ensure!(!success, "sync should always return false");
    Ok(())
}

#[async_std::test]
async fn get_inputs() -> Fallible<()> {
    Connection::new().await?.get_inputs().await?;
    Ok(())
}

#[async_std::test]
async fn get_seats() -> Fallible<()> {
    Connection::new().await?.get_seats().await?;
    Ok(())
}

#[async_std::test]
async fn event_subscribe_all() -> Fallible<()> {
    Connection::new()
        .await?
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
        .await?;
    Ok(())
}

#[async_std::test]
async fn find_in_tree_as_ref() -> Fallible<()> {
    ensure!(Connection::new()
        .await?
        .get_tree()
        .await?
        .find(|n| n.focused)
        .is_some());
    Ok(())
}

#[async_std::test]
async fn find_in_tree() -> Fallible<()> {
    ensure!(Connection::new()
        .await?
        .get_tree()
        .await?
        .find_as_ref(|n| n.focused)
        .is_some());
    Ok(())
}

#[async_std::test]
async fn find_in_tree_comp() -> Fallible<()> {
    ensure!(
        Connection::new()
            .await?
            .get_tree()
            .await?
            .find_as_ref(|n| n.focused)
            == Connection::new()
                .await?
                .get_tree()
                .await?
                .find(|n| n.focused)
                .as_ref()
    );
    Ok(())
}

#[async_std::test]
async fn find_focused_as_ref() -> Fallible<()> {
    ensure!(Connection::new()
        .await?
        .get_tree()
        .await?
        .find_focused_as_ref(|n| n.focused)
        .is_some());
    Ok(())
}

#[async_std::test]
async fn find_focused() -> Fallible<()> {
    ensure!(Connection::new()
        .await?
        .get_tree()
        .await?
        .find_focused(|n| n.focused)
        .is_some());
    Ok(())
}

#[async_std::test]
async fn find_in_tree_comp_find_focused() -> Fallible<()> {
    ensure!(
        Connection::new()
            .await?
            .get_tree()
            .await?
            .find_focused(|n| n.focused)
            == Connection::new()
                .await?
                .get_tree()
                .await?
                .find(|n| n.focused)
    );
    Ok(())
}
