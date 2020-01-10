use swayipc::async_std::stream::StreamExt;
use swayipc::{block_on, Connection, EventType, Fallible};

fn main() -> Fallible<()> {
    block_on(async {
        let subs = [
            EventType::Workspace,
            EventType::Input,
            EventType::Tick,
            EventType::Shutdown,
            EventType::Mode,
            EventType::Window,
            EventType::BarStateUpdate,
            EventType::BarConfigUpdate,
            EventType::Binding,
        ];
        let mut events = Connection::new().await?.subscribe(&subs).await?;
        while let Some(event) = events.next().await {
            println!("{:?}\n", event?)
        }
        unreachable!();
    })
}
