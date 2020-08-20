use swayipc::{Connection, Event, EventType, Fallible};

fn main() -> Fallible<()> {
    for event in Connection::new()?.subscribe(&[EventType::Window])? {
        match event? {
            Event::Window(w) => println!("{}", w.container.name.unwrap_or("unnamed".to_owned())),
            _ => unreachable!(),
        }
    }
    Ok(())
}
