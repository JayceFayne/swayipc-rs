use crate::{Event, EventType};

impl Event {
    pub fn event_type(&self) -> EventType {
        match self {
            Event::Workspace(_) => EventType::Workspace,
            Event::Mode(_) => EventType::Mode,
            Event::Window(_) => EventType::Window,
            Event::BarConfigUpdate(_) => EventType::BarConfigUpdate,
            Event::Binding(_) => EventType::Binding,
            Event::Shutdown(_) => EventType::Shutdown,
            Event::Tick(_) => EventType::Tick,
            Event::BarStateUpdate(_) => EventType::BarStateUpdate,
            Event::Input(_) => EventType::Input,
            Event::Output(_) => EventType::Output,
        }
    }
}
