use futures_util::stream::StreamExt;
use swayipc_async::{Connection, EventType, Fallible};

#[tokio::main]
async fn main() -> Fallible<()> {
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
    let mut events = Connection::new().await?.subscribe(subs).await?;
    while let Some(event) = events.next().await {
        println!("{:?}\n", event?)
    }
    Ok(())
}
