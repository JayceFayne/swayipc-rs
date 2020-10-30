use serde::Serialize;

#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    Workspace,
    Mode,
    Window,
    #[serde(rename = "barconfig_update")]
    BarConfigUpdate,
    Binding,
    Shutdown,
    Tick,
    BarStateUpdate,
    Input,
}
