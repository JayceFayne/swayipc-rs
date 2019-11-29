use swayipc::{Connection, EventType, Fallible};

fn main() -> Fallible<()> {
    let subs = [
        EventType::Workspace,
        EventType::Shutdown,
        EventType::Mode,
        EventType::Window,
        EventType::BarConfigUpdate,
        EventType::Binding,
    ];
    for event in Connection::new()?.subscribe(&subs)? {
        println!("{:?}\n", event?)
    }
    Ok(())
}
