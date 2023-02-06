use serde::Serialize;

#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    /// Sent whenever an event involving a workspace occurs such as
    /// initialization of a new workspace or a different workspace gains focus.
    Workspace,
    /// Sent whenever an output is added, removed, or its configuration is changed.
    Output,
    /// Sent whenever the binding mode changes.
    Mode,
    /// Sent whenever an event involving a view occurs such as being reparented,
    /// focused, or closed.
    Window,
    /// Sent whenever a bar config changes.
    #[serde(rename = "barconfig_update")]
    BarConfigUpdate,
    /// Sent when a configured binding is executed.
    Binding,
    /// Sent when the ipc shuts down because sway is exiting.
    Shutdown,
    /// Sent when an ipc client sends a SEND_TICK message.
    Tick,
    /// Sent when the visibility of a bar should change due to a modifier.
    BarStateUpdate,
    /// Sent when something related to input devices changes.
    Input,
}
