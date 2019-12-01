use swayipc::{Connection, EventType, Fallible};

fn main() -> Fallible<()> {
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
    for event in Connection::new()?.subscribe(&subs)? {
        println!("{:?}\n", event?)
    }
    Ok(())
}
