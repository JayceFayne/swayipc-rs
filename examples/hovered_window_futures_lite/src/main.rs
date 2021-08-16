use futures_lite::{future::block_on, stream::StreamExt};
use swayipc_async::{Connection, Event, EventType, Fallible};

fn main() -> Fallible<()> {
    block_on(async {
        let mut events = Connection::new()
            .await?
            .subscribe([EventType::Window])
            .await?;
        while let Some(event) = events.next().await.transpose()? {
            match event {
                Event::Window(w) => {
                    println!(
                        "{}",
                        w.container.name.unwrap_or_else(|| "unnamed".to_owned())
                    )
                }
                _ => unreachable!(),
            }
        }
        Ok(())
    })
}
