use swayipc::reply::Event;
use swayipc::{block_on, Connection, EventType, Fallible};

fn main() -> Fallible<()> {
    block_on(async {
        let mut events = Connection::new()
            .await?
            .subscribe(&[EventType::Window])
            .await?;
        loop {
            let event = events.next().await?;
            match event {
                Event::Window(w) => {
                    println!("{}", w.container.name.unwrap_or("unnamed".to_owned()))
                }
                _ => unreachable!(),
            }
        }
    })
}
