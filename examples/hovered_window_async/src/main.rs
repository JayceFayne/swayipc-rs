use swayipc::async_std::stream::StreamExt;
use swayipc::reply::Event;
use swayipc::{Connection, EventType, Fallible};

#[async_std::main]
async fn main() -> Fallible<()> {
    let mut events = Connection::new()
        .await?
        .subscribe(&[EventType::Window])
        .await?;
    while let Some(event) = events.next().await.transpose()? {
        match event {
            Event::Window(w) => println!("{}", w.container.name.unwrap_or("unnamed".to_owned())),
            _ => unreachable!(),
        }
    }
    Ok(())
}
