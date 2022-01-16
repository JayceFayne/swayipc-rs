use crate::{Connection, EventType};

#[cfg(all(feature = "default-io", not(feature = "tokio")))]
use async_io::block_on;
#[cfg(feature = "tokio")]
use tokio_test::block_on;

#[test]
fn connect() {
    block_on(async {
        Connection::new().await.unwrap();
    });
}

#[test]
fn run_command_nothing() {
    block_on(async {
        let mut connection = Connection::new().await.unwrap();
        let result = connection.run_command("").await.unwrap();
        assert!(result.is_empty());
    });
}

#[test]
fn run_command_single_success() {
    block_on(async {
        let mut connection = Connection::new().await.unwrap();
        let result = connection.run_command("exec /bin/true").await.unwrap();
        assert_eq!(result.len(), 1);
        result[0].as_ref().unwrap();
    });
}

#[test]
fn run_command_multiple_success() {
    block_on(async {
        let mut connection = Connection::new().await.unwrap();
        let result = connection
            .run_command("exec /bin/true; exec /bin/true")
            .await
            .unwrap();
        assert_eq!(result.len(), 2);
        result[0].as_ref().unwrap();
        result[1].as_ref().unwrap();
    });
}

#[test]
fn run_command_fail() {
    block_on(async {
        let mut connection = Connection::new().await.unwrap();
        let result = connection.run_command("somerandomcommand").await.unwrap();
        assert_eq!(result.len(), 1);
        assert!(result[0].as_ref().is_err());
    });
}

#[test]
fn get_workspaces() {
    block_on(async {
        Connection::new()
            .await
            .unwrap()
            .get_workspaces()
            .await
            .unwrap();
    });
}

#[test]
fn get_outputs() {
    block_on(async {
        Connection::new()
            .await
            .unwrap()
            .get_outputs()
            .await
            .unwrap();
    });
}

#[test]
fn get_tree() {
    block_on(async {
        Connection::new().await.unwrap().get_tree().await.unwrap();
    });
}

#[test]
fn get_marks() {
    block_on(async {
        Connection::new().await.unwrap().get_marks().await.unwrap();
    });
}

#[test]
fn get_bar_ids() {
    block_on(async {
        Connection::new()
            .await
            .unwrap()
            .get_bar_ids()
            .await
            .unwrap();
    });
}

#[test]
fn get_bar_ids_and_one_config() {
    block_on(async {
        let mut connection = Connection::new().await.unwrap();
        let ids = connection.get_bar_ids().await.unwrap();
        connection.get_bar_config(&ids[0]).await.unwrap();
    });
}

#[test]
fn get_version() {
    block_on(async {
        Connection::new()
            .await
            .unwrap()
            .get_version()
            .await
            .unwrap();
    });
}

#[test]
fn get_binding_modes() {
    block_on(async {
        Connection::new()
            .await
            .unwrap()
            .get_binding_modes()
            .await
            .unwrap();
    });
}

#[test]
fn get_config() {
    block_on(async {
        Connection::new().await.unwrap().get_config().await.unwrap();
    });
}

#[test]
fn send_tick() {
    block_on(async {
        let success = Connection::new()
            .await
            .unwrap()
            .send_tick("")
            .await
            .unwrap();
        assert!(success);
    });
}

#[test]
fn sync() {
    block_on(async {
        let success = Connection::new().await.unwrap().sync().await.unwrap();
        assert!(!success, "sync should always return false on sway");
    });
}

#[test]
fn get_binding_state() {
    block_on(async {
        Connection::new()
            .await
            .unwrap()
            .get_binding_state()
            .await
            .unwrap();
    });
}

#[test]
fn get_inputs() {
    block_on(async {
        Connection::new().await.unwrap().get_inputs().await.unwrap();
    });
}

#[test]
fn get_seats() {
    block_on(async {
        Connection::new().await.unwrap().get_seats().await.unwrap();
    });
}

#[test]
fn event_subscribe_all() {
    block_on(async {
        Connection::new()
            .await
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
            .await
            .unwrap();
    });
}

#[test]
fn find_in_tree() {
    block_on(async {
        assert!(Connection::new()
            .await
            .unwrap()
            .get_tree()
            .await
            .unwrap()
            .find_as_ref(|n| n.focused)
            .is_some());
    });
}

#[test]
fn find_in_tree_comp() {
    block_on(async {
        assert_eq!(
            Connection::new()
                .await
                .unwrap()
                .get_tree()
                .await
                .unwrap()
                .find_as_ref(|n| n.focused),
            Connection::new()
                .await
                .unwrap()
                .get_tree()
                .await
                .unwrap()
                .find(|n| n.focused)
                .as_ref()
        );
    });
}

#[test]
fn find_focused_as_ref() {
    block_on(async {
        assert!(Connection::new()
            .await
            .unwrap()
            .get_tree()
            .await
            .unwrap()
            .find_focused_as_ref(|n| n.focused)
            .is_some());
    });
}

#[test]
fn find_focused() {
    block_on(async {
        assert!(Connection::new()
            .await
            .unwrap()
            .get_tree()
            .await
            .unwrap()
            .find_focused(|n| n.focused)
            .is_some());
    });
}

#[test]
fn find_in_tree_comp_find_focused() {
    block_on(async {
        assert_eq!(
            Connection::new()
                .await
                .unwrap()
                .get_tree()
                .await
                .unwrap()
                .find_focused(|n| n.focused),
            Connection::new()
                .await
                .unwrap()
                .get_tree()
                .await
                .unwrap()
                .find(|n| n.focused)
        );
    });
}
