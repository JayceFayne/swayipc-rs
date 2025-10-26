use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CommandOutcome {
    /// A boolean indicating whether the command was successful.
    pub success: bool,
    /// An error object if the command failed, and None otherwise.
    #[serde(flatten)]
    pub error: Option<CommandError>,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CommandError {
    /// A boolean indicating whether the reason the command failed was because
    /// the command was unknown or not able to be parsed.
    pub parse_error: bool,
    /// A human readable error message.
    #[serde(rename = "error")]
    pub message: String,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Workspace {
    pub id: i64,
    /// The workspace number or -1 for workspaces that do not start with a
    /// number.
    pub num: i32,
    /// The name of the workspace.
    pub name: String,
    #[serde(default)]
    pub layout: NodeLayout,
    /// Whether the workspace is currently visible on any output.
    pub visible: bool,
    /// Whether the workspace is currently focused by the default seat (seat0).
    pub focused: bool,
    /// Whether a view on the workspace has the urgent flag set.
    pub urgent: bool,
    pub representation: Option<String>,
    /// The workspace orientation. It can be vertical, horizontal, or none
    #[serde(default)]
    pub orientation: Orientation,
    /// The bounds of the workspace. It consists of x, y, width, and height.
    pub rect: Rect,
    /// The name of the output that the workspace is on.
    pub output: String,
    #[serde(default)]
    pub focus: Vec<i64>,
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Success {
    /// A boolean value indicating whether the operation was successful or not.
    pub success: bool,
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Mode {
    pub width: i32,
    pub height: i32,
    pub refresh: i32,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Output {
    pub id: Option<i64>, // Sway doesn't give disabled outputs ids
    /// The name of the output. On DRM, this is the connector.
    pub name: String,
    /// The make of the output.
    pub make: String,
    /// The model of the output.
    pub model: String,
    /// The output's serial number as a hexa‐ decimal string.
    pub serial: String,
    /// Whether this output is active/enabled.
    #[serde(default)]
    pub active: bool,
    /// Whether this is a non-desktop output (e.g. VR headset).
    #[serde(default)]
    pub non_desktop: bool,
    /// Whether this output is on/off (via DPMS).
    #[serde(default)]
    pub dpms: bool,
    /// Whether this output is on/off
    #[serde(default)]
    pub power: bool,
    /// For i3 compatibility, this will be false. It does not make sense in
    /// Wayland.
    pub primary: bool,
    /// The scale currently in use on the output or -1 for disabled outputs.
    pub scale: Option<f64>,
    /// The subpixel hinting current in use on the output. This can be rgb, bgr,
    /// vrgb, vbgr, or none.
    pub subpixel_hinting: Option<String>,
    /// The transform currently in use for the output. This can be normal, 90,
    /// 180, 270, flipped-90, flipped-180, or flipped-270.
    pub transform: Option<String>,
    /// Status of adaptive sync
    pub adaptive_sync_status: Option<EnabledOrDisabled>,
    /// The workspace currently visible on the output or null for disabled
    /// outputs.
    pub current_workspace: Option<String>,
    /// An array of supported mode objects. Each object contains width, height,
    /// and refresh.
    #[serde(default)]
    pub modes: Vec<Mode>,
    /// An object representing the current mode containing width, height, and
    /// refresh.
    pub current_mode: Option<Mode>,
    /// The bounds for the output consisting of x, y, width, and height.
    #[serde(default)]
    pub rect: Rect,
    #[serde(default)]
    pub focus: Vec<i64>,
    #[serde(default)]
    pub focused: bool,
    /// Whether HDR is enabled
    #[serde(default)]
    pub hdr: bool,
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Libinput {
    /// Whether events are being sent by the device. It can be enabled,
    /// disabled, or disabled_on_external_mouse.
    pub send_events: Option<SendEvents>,
    /// Whether tap to click is enabled.  It can be enabled or disabled.
    pub tap: Option<EnabledOrDisabled>,
    /// The finger to button mapping in use.  It can be lmr or lrm.
    pub tap_button_mapping: Option<ButtonMapping>,
    /// Whether tap-and-drag is enabled. It can be enabled or disabled.
    pub tap_drag: Option<EnabledOrDisabled>,
    /// Whether drag-lock is enabled. It can be enabled, disabled or enabled_sticky.
    pub tap_drag_lock: Option<DragLock>,
    /// The pointer-acceleration in use.
    pub accel_speed: Option<f64>,
    /// Whether natural scrolling is enabled. It can be enabled or disabled.
    pub natural_scroll: Option<EnabledOrDisabled>,
    /// Whether left-handed mode is enabled. It can be enabled or disabled.
    pub left_handed: Option<EnabledOrDisabled>,
    /// The click method in use. It can be none, button_areas, or clickfinger.
    pub click_method: Option<ClickMethod>,
    /// The finger to button mapping in use for clickfinger.
    pub click_button_map: Option<ButtonMapping>,
    /// Whether middle emulation is enabled.  It can be enabled or disabled.
    pub middle_emulation: Option<EnabledOrDisabled>,
    /// The scroll method in use. It can be none, two_finger, edge, or
    /// on_button_down.
    pub scroll_method: Option<ScrollMethod>,
    /// The scroll button to use when scroll_method is on_button_down.  This
    /// will be given as an input event code.
    pub scroll_button: Option<i32>,
    /// Whether scroll button lock is enabled.
    pub scroll_button_lock: Option<EnabledOrDisabled>,
    /// Whether disable-while-typing is enabled. It can be enabled or disabled.
    pub dwt: Option<EnabledOrDisabled>,
    /// An array of 6 floats representing the calibration matrix for absolute
    /// devices such as touchscreens.
    pub calibration_matrix: Option<[f32; 6]>,
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SendEvents {
    Enabled,
    Disabled,
    DisabledOnExternalMouse,
}

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum EnabledOrDisabled {
    Enabled,
    Disabled,
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClickMethod {
    ButtonAreas,
    Clickfinger,
    None,
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ScrollMethod {
    TwoFinger,
    Edge,
    OnButtonDown,
    None,
}

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ButtonMapping {
    LMR,
    LRM,
}

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DragLock {
    Enabled,
    Disabled,
    EnabledSticky,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Input {
    /// The identifier for the input device.
    pub identifier: String,
    /// The human readable name for the device.
    pub name: String,
    /// The device type.  Currently this can be keyboard, pointer, touch,
    /// tablet_tool, tablet_pad, or switch.
    #[serde(rename = "type")]
    pub input_type: String,
    /// (Only keyboards) The name of the active keyboard layout in use.
    pub xkb_active_layout_name: Option<String>,
    /// (Only keyboards) A list a layout names configured for the keyboard.
    #[serde(default)]
    pub xkb_layout_names: Vec<String>,
    /// (Only keyboards) The index of the active keyboard layout in use.
    pub xkb_active_layout_index: Option<i32>,
    /// (Only pointers) Multiplier applied on scroll event values.
    #[serde(default)]
    pub scroll_factor: f64,
    /// (Only libinput devices) An object describing the current device
    /// settings. See below for more information.
    pub libinput: Option<Libinput>,
    /// (Only libinput devices) The vendor code for the input device.
    pub vendor: Option<i32>,
    /// (Only libinput devices) The product code for the input device.
    pub product: Option<i32>,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Seat {
    /// The unique name for the seat.
    pub name: String,
    /// The number of capabilities that the seat has.
    pub capabilities: i32,
    /// The id of the node currently focused by the seat or 0 when the seat is
    /// not currently focused by a node (i.e. a surface layer or xwayland
    /// unmanaged has focus).
    pub focus: i64,
    /// An array of input devices that are attached to the seat. Currently, this
    /// is an array of objects that are identical to those returned by
    /// GET_INPUTS.
    #[serde(default)]
    pub devices: Vec<Input>,
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WindowProperties {
    pub title: Option<String>,
    pub instance: Option<String>,
    pub class: Option<String>,
    pub window_role: Option<String>,
    pub window_type: Option<String>,
    pub transient_for: Option<i32>,
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum UserIdleInhibitType {
    Focus,
    Fullscreen,
    Open,
    Visible,
    None,
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ApplicationIdleInhibitType {
    Enabled,
    None,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct IdleInhibitors {
    pub application: ApplicationIdleInhibitType,
    pub user: UserIdleInhibitType,
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeType {
    Root,
    Output,
    Workspace,
    Con,
    FloatingCon,
    Dockarea, // i3-specific
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum NodeBorder {
    Normal,
    Pixel,
    Csd,
    None,
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum NodeLayout {
    SplitH,
    SplitV,
    Stacked,
    Tabbed,
    Output,
    Dockarea, // i3-specific
    #[default]
    None,
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Orientation {
    Vertical,
    Horizontal,
    #[default]
    None,
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Floating {
    AutoOn,
    AutoOff,
    UserOn,
    UserOff,
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ScratchpadState {
    None,
    Fresh,
    Changed,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Node {
    /// The internal unique ID for this node.
    pub id: i64,
    /// The name of the node such as the output name or window title. For the
    /// scratchpad, this will be __i3_scratch for compatibility with i3.
    pub name: Option<String>,
    /// The node type. It can be root, output, workspace, con, or floating_con.
    #[serde(rename = "type")]
    pub node_type: NodeType,
    /// The border style for the node. It can be normal, none, pixel, or csd.
    pub border: NodeBorder,
    /// Number of pixels used for the border width.
    pub current_border_width: i32,
    /// The node's layout.  It can either be splith, splitv, stacked, tabbed, or
    /// output.
    pub layout: NodeLayout,
    /// The node's orientation. It can be vertical, horizontal, or none
    pub orientation: Orientation,
    /// The percentage of the node's parent that it takes up or null for the
    /// root and other special nodes such as the scratchpad.
    pub percent: Option<f64>,
    /// The absolute geometry of the node.  The window decorations are excluded
    /// from this, but borders are included.
    pub rect: Rect,
    /// The geometry of the contents inside the node. The window decorations are
    /// excluded from this calculation, but borders are included.
    pub window_rect: Rect,
    /// The geometry of the decorations for the node relative to the parent
    /// node.
    pub deco_rect: Rect,
    /// The natural geometry of the contents if it were to size itself.
    pub geometry: Rect,
    /// Whether the node or any of its descendants has the urgent hint set.
    /// Note: This may not exist when compiled without xwayland support.
    pub urgent: bool,
    /// Whether the node is currently focused by the default seat (seat0).
    pub focused: bool,
    /// Array of child node IDs in the current focus order.
    pub focus: Vec<i64>,
    /// Floating state of container, i3 specific property
    pub floating: Option<Floating>,
    /// The tiling children nodes for the node.
    #[serde(default)]
    pub nodes: Vec<Node>,
    /// The floating children nodes for the node.
    pub floating_nodes: Vec<Node>,
    /// Whether the node is sticky (shows on all workspaces).
    pub sticky: bool,
    /// (Only workspaces) A string representation of the layout of the workspace
    /// that can be used as an aid in submitting reproduction steps for bug
    /// reports.
    pub representation: Option<String>,
    /// (Only views) The fullscreen mode of the node. 0 means
    /// none, 1 means full workspace, and 2 means global fullscreen.
    pub fullscreen_mode: Option<u8>,
    /// (Only views) For an xdg-shell and xwayland view, whether the window is in the scratchpad.
    /// Otherwise, null.
    pub scratchpad_state: Option<ScratchpadState>,
    /// (Only views) For an xdg-shell view, the name of the application, if set.
    /// Otherwise, null.
    pub app_id: Option<String>,
    /// (Only views) The PID of the application that owns the view.
    pub pid: Option<i32>,
    /// (Only xwayland views) The X11 window ID for the xwayland view.
    pub window: Option<i64>,
    pub num: Option<i32>, //workspace number if `node_type` == `NodeType::Workspace`
    /// (Only xwayland views) An object containing the title, class, instance,
    /// window_role, window_type, and transient_for for the view.
    pub window_properties: Option<WindowProperties>,
    /// List of marks assigned to the node.
    #[serde(default)]
    pub marks: Vec<String>,
    /// (Only views) Whether the view is inhibiting the idle state.
    pub inhibit_idle: Option<bool>,
    /// (Only views) An object containing the state of the application and user
    /// idle inhibitors.  application can be enabled or none.  user can be
    /// focus, fullscreen, open, visible or none.
    pub idle_inhibitors: Option<IdleInhibitors>,
    /// (Only views) The associated sandbox engine.
    pub sandbox_engine: Option<String>,
    /// Only views) The app ID provided by the associated sandbox engine.
    pub sandbox_app_id: Option<String>,
    /// (Only views) The instance ID provided by the associated sandbox engine.
    pub sandbox_instance_id: Option<String>,
    /// (Only windows) For an xdg-shell window, tag of the toplevel, if set.
    pub tag: Option<String>,
    /// (Only views) The shell of the view, such as xdg_shell or xwayland.
    pub shell: Option<ShellType>,
    /// (Only views) The ext-foreign-toplevel-list-v1 toplevel identifier of this node.
    pub foreign_toplevel_identifier: Option<String>,
    /// (Only views) Whether the node is visible.
    pub visible: Option<bool>,
    /// (Only workspaces) Name of the output the node is located on.
    pub output: Option<String>,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct ColorableBarPart {
    /// The color to use for the bar background on unfocused outputs.
    pub background: String,
    /// The color to use for the status line text on unfocused outputs.
    pub statusline: String,
    /// The color to use for the separator text on unfocused outputs.
    pub separator: String,
    /// The color to use for the background of the bar on the focused output.
    pub focused_background: String,
    /// The color to use for the status line text on the focused output.
    pub focused_statusline: String,
    /// The color to use for the separator text on the focused output.
    pub focused_separator: String,
    /// The color to use for the text of the focused workspace button.
    pub focused_workspace_text: String,
    /// The color to use for the background of the focused workspace button.
    pub focused_workspace_bg: String,
    /// The color to use for the border of the focused workspace button.
    pub focused_workspace_border: String,
    /// The color to use for the text of the workspace buttons for the visible
    /// workspaces on unfocused outputs.
    pub active_workspace_text: String,
    /// The color to use for the background of the workspace buttons for the
    /// visible workspaces on unfocused outputs.
    pub active_workspace_bg: String,
    /// The color to use for the border of the workspace buttons for the visible
    /// workspaces on unfocused outputs.
    pub active_workspace_border: String,
    /// The color to use for the text of the workspace buttons for workspaces
    /// that are not visible.
    pub inactive_workspace_text: String,
    /// The color to use for the background of the workspace buttons for
    /// workspaces that are not visible.
    pub inactive_workspace_bg: String,
    /// The color to use for the border of the workspace buttons for workspaces
    /// that are not visible.
    pub inactive_workspace_border: String,
    /// The color to use for the text of the workspace buttons for workspaces
    /// that contain an urgent view.
    pub urgent_workspace_text: String,
    /// The color to use for the background of the workspace buttons for
    /// workspaces that contain an urgent view.
    pub urgent_workspace_bg: String,
    /// The color to use for the border of the workspace buttons for workspaces
    /// that contain an urgent view.
    pub urgent_workspace_border: String,
    /// The color to use for the text of the binding mode indicator.
    pub binding_mode_text: String,
    /// The color to use for the background of the binding mode indicator.
    pub binding_mode_bg: String,
    /// The color to use for the border of the binding mode indicator.
    pub binding_mode_border: String,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BarConfig {
    /// The bar ID.
    pub id: String,
    /// The mode for the bar. It can be dock, hide, or invisible.
    pub mode: BarMode,
    /// The bar's position.  It can currently either be bottom or top.
    pub position: Position,
    /// The command which should be run to generate the status line.
    pub status_command: Option<String>,
    /// The font to use for the text on the bar.
    pub font: String,
    /// Whether to display the workspace buttons on the bar.
    pub workspace_buttons: bool,
    /// Minimum width in px for the workspace buttons on the bar
    #[serde(default)]
    pub workspace_min_width: usize,
    /// Whether to display the current binding mode on the bar.
    pub binding_mode_indicator: bool,
    /// For i3 compatibility, this will be the boolean value false.
    pub verbose: bool,
    /// An object containing the #RRGGBBAA colors to use for the bar. See below
    /// for more information.
    pub colors: ColorableBarPart,
    /// An object representing the gaps for the bar consisting of top, right,
    /// bottom, and left.
    pub gaps: Gaps,
    /// The absolute height to use for the bar or 0 to automatically size based
    /// on the font.
    pub bar_height: usize,
    /// The vertical padding to use for the status line.
    pub status_padding: usize,
    /// The horizontal padding to use for the status line when at the end of an
    /// output.
    pub status_edge_padding: usize,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BindingState {
    /// The currently active binding mode, as a string.
    pub name: String,
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BarMode {
    Dock,
    Hide,
    Invisible,
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Gaps {
    pub top: usize,
    pub bottom: usize,
    pub right: usize,
    pub left: usize,
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Position {
    Bottom,
    Top,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Version {
    /// The major version of the sway process.
    pub major: i32,
    /// The minor version of the sway process.
    pub minor: i32,
    /// The patch version of the sway process.
    pub patch: i32,
    /// A human readable version string that will likely contain more useful
    /// information such as the git commit short hash and git branch.
    pub human_readable: String,
    /// The path to the loaded config file.
    pub loaded_config_file_name: String,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    /// A single string property containing the contents of the config.
    pub config: String,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Event {
    /// Sent whenever an event involving a workspace occurs such as
    /// initialization of a new workspace or a different workspace gains focus.
    Workspace(Box<WorkspaceEvent>),
    /// Sent whenever an output is added, removed, or its configuration is changed.
    Output(OutputEvent),
    /// Sent whenever the binding mode changes.
    Mode(ModeEvent),
    /// Sent whenever an event involving a view occurs such as being reparented,
    /// focused, or closed.
    Window(Box<WindowEvent>),
    /// Sent whenever a bar config changes.
    BarConfigUpdate(Box<BarConfig>),
    /// Sent when a configured binding is executed.
    Binding(BindingEvent),
    /// Sent when the ipc shuts down because sway is exiting.
    Shutdown(ShutdownEvent),
    /// Sent when an ipc client sends a SEND_TICK message.
    Tick(TickEvent),
    /// Send when the visibility of a bar should change due to a modifier.
    BarStateUpdate(BarStateUpdateEvent),
    /// Sent when something related to input devices changes.
    Input(Box<InputEvent>),
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InputEvent {
    /// What has changed.
    pub change: InputChange,
    /// An object representing the input that is identical the ones GET_INPUTS
    /// gives.
    pub input: Input,
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InputChange {
    /// The input device became available.
    Added,
    /// The input device is no longer available.
    Removed,
    /// (Keyboards only) The keymap for the keyboard has changed.
    XkbKeymap,
    /// (Keyboards only) The effective layout in the keymap has changed.
    XkbLayout,
    /// (libinput device only) A libinput config option for the device changed.
    LibinputConfig,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BarStateUpdateEvent {
    /// The bar ID effected.
    pub id: String,
    /// Whether the bar should be made visible due to a modifier being pressed.
    pub visible_by_modifier: bool,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TickEvent {
    /// Whether this event was triggered by subscribing to the tick events.
    pub first: bool,
    /// The payload given with a SEND_TICK message, if any.  Otherwise, an empty
    /// string.
    pub payload: String,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WorkspaceEvent {
    /// The type of change that occurred.
    pub change: WorkspaceChange,
    /// An object representing the workspace effected or null for reload
    /// changes.
    pub current: Option<Node>, //Only None if WorkspaceChange::Reload
    /// For a focus change, this is will be an object representing the workspace
    /// being switched from. Otherwise, it is null.
    pub old: Option<Node>, //Only None if WorkspaceChange::Reload
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputChange {
    /// We don't know what exactly changed.
    Unspecified,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OutputEvent {
    /// The type of change that occurred.
    pub change: OutputChange,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ModeEvent {
    /// The binding mode that became active.
    pub change: String,
    /// Whether the mode should be parsed as pango markup.
    pub pango_markup: bool,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WindowEvent {
    /// The type of change that occurred.
    pub change: WindowChange,
    /// An object representing the view effected.
    pub container: Node,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BindingEvent {
    /// The change that occurred for the binding. Currently this will only be
    /// run.
    pub change: BindingChange,
    /// Details about the binding event.
    pub binding: BindingEventOps,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BindingEventOps {
    /// The command associated with the binding.
    pub command: String,
    /// An array of strings that correspond to each modifier key for the
    /// binding.
    #[serde(default)]
    pub event_state_mask: Vec<String>,
    /// For keyboard bindcodes, this is the key code for the binding. For mouse
    /// bindings, this is the X11 button number, if there is an equivalent. In
    /// all other cases, this will be 0.
    pub input_code: u8,
    /// For keyboard bindsyms, this is the bindsym for the binding. Otherwise,
    /// this will be null.
    pub symbol: Option<String>,
    /// The input type that triggered the binding. This is either keyboard or
    /// mouse.
    pub input_type: InputType,
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct ShutdownEvent {
    /// The reason for the shutdown.
    pub change: ShutdownChange,
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum WorkspaceChange {
    /// The workspace was created.
    Init,
    /// The workspace is empty and is being destroyed since it is not visible.
    Empty,
    /// The workspace was focused.  See the old property for the previous focus.
    Focus,
    /// The workspace was moved to a different output.
    Move,
    /// The workspace was renamed.
    Rename,
    /// A view on the workspace has had their urgency hint set or all urgency
    /// hints for views on the workspace have been cleared.
    Urgent,
    /// The configuration file has been reloaded.
    Reload,
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowChange {
    /// The view was created.
    New,
    /// The view was closed.
    Close,
    /// The view was focused.
    Focus,
    /// The view's title has changed.
    Title,
    /// The view's fullscreen mode has changed.
    FullscreenMode,
    /// The view has been reparented in the tree.
    Move,
    /// The view has become floating or is no longer floating.
    Floating,
    /// The view's urgency hint has changed status.
    Urgent,
    /// A mark has been added or.
    Mark,
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum InputType {
    Keyboard,
    Mouse,
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BindingChange {
    Run,
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ShutdownChange {
    Exit,
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ShellType {
    XdgShell,
    Xwayland,
    Unknown,
}
