use crate::{ensure, Connection, EventType, Fallible};
use async_std::task::block_on;

#[test]
fn connect() -> Fallible<()> {
    block_on(async {
        Connection::new().await?;
        Ok(())
    })
}

#[test]
fn run_command_nothing() -> Fallible<()> {
    block_on(async {
        let mut connection = Connection::new().await?;
        let result = connection.run_command("").await?;
        ensure!(result.len() == 0);
        Ok(())
    })
}

#[test]
fn run_command_single_sucess() -> Fallible<()> {
    block_on(async {
        let mut connection = Connection::new().await?;
        let result = connection.run_command("exec /bin/true").await?;
        ensure!(result.len() == 1);
        ensure!(result[0].success);
        Ok(())
    })
}

#[test]
fn run_command_multiple_success() -> Fallible<()> {
    block_on(async {
        let mut connection = Connection::new().await?;
        let result = connection
            .run_command("exec /bin/true; exec /bin/true")
            .await?;
        ensure!(result.len() == 2);
        ensure!(result[0].success);
        ensure!(result[1].success);
        Ok(())
    })
}

#[test]
fn run_command_fail() -> Fallible<()> {
    block_on(async {
        let mut connection = Connection::new().await?;
        let result = connection.run_command("somerandomcommand").await?;
        ensure!(result.len() == 1);
        ensure!(!result[0].success);
        Ok(())
    })
}

#[test]
fn get_workspaces() -> Fallible<()> {
    block_on(async {
        Connection::new().await?.get_workspaces().await?;
        Ok(())
    })
}

#[test]
fn get_outputs() -> Fallible<()> {
    block_on(async {
        Connection::new().await?.get_outputs().await?;
        Ok(())
    })
}

#[test]
fn get_tree() -> Fallible<()> {
    block_on(async {
        Connection::new().await?.get_tree().await?;
        Ok(())
    })
}

#[test]
fn get_marks() -> Fallible<()> {
    block_on(async {
        Connection::new().await?.get_marks().await?;
        Ok(())
    })
}

#[test]
fn get_bar_ids() -> Fallible<()> {
    block_on(async {
        Connection::new().await?.get_bar_ids().await?;
        Ok(())
    })
}

#[test]
fn get_bar_ids_and_one_config() -> Fallible<()> {
    block_on(async {
        let mut connection = Connection::new().await?;
        let ids = connection.get_bar_ids().await?;
        connection.get_bar_config(&ids[0]).await?;
        Ok(())
    })
}

#[test]
fn get_version() -> Fallible<()> {
    block_on(async {
        Connection::new().await?.get_version().await?;
        Ok(())
    })
}

#[test]
fn get_binding_modes() -> Fallible<()> {
    block_on(async {
        Connection::new().await?.get_binding_modes().await?;
        Ok(())
    })
}

#[test]
fn get_config() -> Fallible<()> {
    block_on(async {
        Connection::new().await?.get_config().await?;
        Ok(())
    })
}

#[test]
fn send_tick() -> Fallible<()> {
    block_on(async {
        let success = Connection::new().await?.send_tick("").await?;
        ensure!(success);
        Ok(())
    })
}

#[test]
fn send_sync() -> Fallible<()> {
    block_on(async {
        let success = Connection::new().await?.send_sync().await?;
        ensure!(!success, "sync should always return false");
        Ok(())
    })
}

#[test]
fn get_inputs() -> Fallible<()> {
    block_on(async {
        Connection::new().await?.get_inputs().await?;
        Ok(())
    })
}

#[test]
fn get_seats() -> Fallible<()> {
    block_on(async {
        Connection::new().await?.get_seats().await?;
        Ok(())
    })
}

#[test]
fn event_subscribe_all() -> Fallible<()> {
    block_on(async {
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
                //EventType::BarStatusUpdate, //FIXME: Fails for some reason :shrug:
                EventType::Input,
            ])
            .await?;
        Ok(())
    })
}

#[test]
fn find_in_tree_as_ref() -> Fallible<()> {
    block_on(async {
        ensure!(Connection::new()
            .await?
            .get_tree()
            .await?
            .find(|n| n.focused)
            .is_some());
        Ok(())
    })
}

#[test]
fn find_in_tree() -> Fallible<()> {
    block_on(async {
        ensure!(Connection::new()
            .await?
            .get_tree()
            .await?
            .find_as_ref(|n| n.focused)
            .is_some());
        Ok(())
    })
}

#[test]
fn find_in_tree_comp() -> Fallible<()> {
    block_on(async {
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
    })
}

#[test]
fn find_focused_as_ref() -> Fallible<()> {
    block_on(async {
        ensure!(Connection::new()
            .await?
            .get_tree()
            .await?
            .find_focused_as_ref(|n| n.focused)
            .is_some());
        Ok(())
    })
}

#[test]
fn find_focused() -> Fallible<()> {
    block_on(async {
        ensure!(Connection::new()
            .await?
            .get_tree()
            .await?
            .find_focused(|n| n.focused)
            .is_some());
        Ok(())
    })
}

#[test]
fn find_in_tree_comp_find_focused() -> Fallible<()> {
    block_on(async {
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
    })
}
