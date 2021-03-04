use serde::Deserialize;

#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct CommandOutcome {
    pub success: bool,
    #[serde(flatten)]
    pub error: Option<CommandError>,
}

#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct CommandError {
    pub parse_error: bool,
    #[serde(rename = "error")]
    pub message: String,
}

#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct Workspace {
    pub id: i64,
    pub num: i32,
    pub name: String,
    pub layout: String,
    pub visible: bool,
    pub focused: bool,
    pub urgent: bool,
    pub representation: Option<String>,
    pub orientation: String,
    pub rect: Rect,
    pub output: String,
    #[serde(default)]
    pub focus: Vec<i32>,
}

#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct Success {
    pub success: bool,
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Deserialize)]
pub struct Mode {
    pub width: i32,
    pub height: i32,
    pub refresh: i32,
}

#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct Output {
    pub id: Option<i64>, // Sway doesn't give disabled outputs ids
    pub name: String,
    pub make: String,
    pub model: String,
    pub serial: String,
    pub active: bool,
    pub dpms: bool,
    pub primary: bool,
    pub scale: Option<f64>,
    pub subpixel_hinting: Option<String>,
    pub transform: Option<String>,
    pub current_workspace: Option<String>,
    #[serde(default)]
    pub modes: Vec<Mode>,
    pub current_mode: Option<Mode>,
    pub rect: Rect,
    #[serde(default)]
    pub focus: Vec<i32>,
    pub focused: bool,
}

#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct Libinput {
    pub send_events: Option<SendEvents>,
    pub tap: Option<EnabledOrDisabled>,
    pub tap_button_mapping: Option<ButtonMapping>,
    pub tap_drag: Option<EnabledOrDisabled>,
    pub tap_drag_lock: Option<EnabledOrDisabled>,
    pub accel_speed: Option<f64>,
    pub natural_scroll: Option<EnabledOrDisabled>,
    pub left_handed: Option<EnabledOrDisabled>,
    pub click_method: Option<ClickMethod>,
    pub middle_emulation: Option<EnabledOrDisabled>,
    pub scroll_method: Option<ScrollMethod>,
    pub scroll_button: Option<i32>,
    pub dwt: Option<EnabledOrDisabled>,
    pub calibration_matrix: Option<[f32; 6]>,
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SendEvents {
    Enabled,
    Disabled,
    DisabledOnExternalMouse,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EnabledOrDisabled {
    Enabled,
    Disabled,
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClickMethod {
    ButtonAreas,
    Clickfinger,
    None,
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScrollMethod {
    TwoFinger,
    Edge,
    OnButtonDown,
    None,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ButtonMapping {
    LMR,
    LRM,
}

#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct Input {
    pub identifier: String,
    pub name: String,
    pub vendor: i32,
    pub product: i32,
    #[serde(rename = "type")]
    pub input_type: String,
    pub xkb_active_layout_name: Option<String>,
    #[serde(default)]
    pub xkb_layout_names: Vec<String>,
    pub xkb_active_layout_index: Option<i32>,
    pub libinput: Option<Libinput>,
}

#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct Seat {
    pub name: String,
    pub capabilities: i32,
    pub focus: i32,
    #[serde(default)]
    pub devices: Vec<Input>,
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Deserialize)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Deserialize)]
pub struct WindowProperties {
    pub title: Option<String>,
    pub instance: Option<String>,
    pub class: Option<String>,
    pub window_role: Option<String>,
    pub transient_for: Option<i32>,
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserIdleInhibitType {
    Focus,
    Fullscreen,
    Open,
    Visible,
    None,
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ApplicationIdleInhibitType {
    Enabled,
    None,
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Deserialize)]
pub struct IdleInhibitors {
    pub application: ApplicationIdleInhibitType,
    pub user: UserIdleInhibitType,
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Deserialize)]
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
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NodeBorder {
    Normal,
    Pixel,
    Csd,
    None,
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NodeLayout {
    SplitH,
    SplitV,
    Stacked,
    Tabbed,
    Output,
    Dockarea, // i3-specific
    None,
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Deserialize)]
pub struct Node {
    pub id: i64,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub node_type: NodeType,
    pub border: NodeBorder,
    pub current_border_width: i32,
    pub layout: NodeLayout,
    pub percent: Option<f64>,
    pub rect: Rect,
    pub window_rect: Rect,
    pub deco_rect: Rect,
    pub geometry: Rect,
    pub urgent: bool,
    pub focused: bool,
    pub focus: Vec<i64>,
    #[serde(default)]
    pub nodes: Vec<Node>,
    pub floating_nodes: Vec<Node>,
    pub sticky: bool,
    pub representation: Option<String>,
    pub fullscreen_mode: Option<u8>,
    pub app_id: Option<String>,
    pub pid: Option<i32>,
    pub window: Option<i64>,
    pub num: Option<i32>, //workspace number if `node_type` == `NodeType::Workspace`
    pub window_properties: Option<WindowProperties>,
    #[serde(default)]
    pub marks: Vec<String>,
    pub inhibit_idle: Option<bool>,
    pub idle_inhibitors: Option<IdleInhibitors>,
    pub shell: Option<ShellType>,
    pub visible: Option<bool>,
}

#[non_exhaustive]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct ColorableBarPart {
    pub background: String,
    pub statusline: String,
    pub separator: String,
    pub focused_background: String,
    pub focused_statusline: String,
    pub focused_separator: String,
    pub focused_workspace_text: String,
    pub focused_workspace_bg: String,
    pub focused_workspace_border: String,
    pub active_workspace_text: String,
    pub active_workspace_bg: String,
    pub active_workspace_border: String,
    pub inactive_workspace_text: String,
    pub inactive_workspace_bg: String,
    pub inactive_workspace_border: String,
    pub urgent_workspace_text: String,
    pub urgent_workspace_bg: String,
    pub urgent_workspace_border: String,
    pub binding_mode_text: String,
    pub binding_mode_bg: String,
    pub binding_mode_border: String,
}

#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct BarConfig {
    pub id: String,
    pub mode: BarMode,
    pub position: Position,
    pub status_command: String,
    pub font: String,
    pub workspace_buttons: bool,
    pub binding_mode_indicator: bool,
    pub verbose: bool,
    pub colors: ColorableBarPart,
    pub gaps: Gaps,
    pub bar_height: usize,
    pub status_padding: usize,
    pub status_edge_padding: usize,
}

#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct BindingState {
    pub name: String,
}

#[non_exhaustive]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BarMode {
    Dock,
    Hide,
    Invisible,
}

#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct Gaps {
    pub top: usize,
    pub bottom: usize,
    pub right: usize,
    pub left: usize,
}

#[non_exhaustive]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Position {
    Bottom,
    Top,
}

#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct Version {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
    pub human_readable: String,
    pub loaded_config_file_name: String,
}

#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct Config {
    pub config: String,
}

#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub enum Event {
    Workspace(Box<WorkspaceEvent>),
    Mode(ModeEvent),
    Window(Box<WindowEvent>),
    BarConfigUpdate(Box<BarConfig>),
    Binding(BindingEvent),
    Shutdown(ShutdownEvent),
    Tick(TickEvent),
    BarStateUpdate(BarStateUpdateEvent),
    Input(Box<InputEvent>),
}

#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct InputEvent {
    pub change: InputChange,
    pub input: Input,
}

#[non_exhaustive]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InputChange {
    Added,
    Removed,
    XkbKeymap,
    XkbLayout,
    LibinputConfig,
}

#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct BarStateUpdateEvent {
    pub id: String,
    pub visible_by_modifier: bool,
}

#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct TickEvent {
    pub first: bool,
    pub payload: String,
}

#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct WorkspaceEvent {
    pub change: WorkspaceChange,
    pub current: Option<Node>, //Only None if WorkspaceChange::Reload
    pub old: Option<Node>,     //Only None if WorkspaceChange::Reload
}

#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct ModeEvent {
    pub change: String,
    pub pango_markup: bool,
}

#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct WindowEvent {
    pub change: WindowChange,
    pub container: Node,
}

#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct BindingEvent {
    pub change: BindingChange,
    pub binding: BindingEventOps,
}

#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct BindingEventOps {
    pub command: String,
    #[serde(default)]
    pub event_state_mask: Vec<String>,
    pub input_code: u8,
    pub symbol: Option<String>,
    pub input_type: InputType,
}

#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct ShutdownEvent {
    pub change: ShutdownChange,
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WorkspaceChange {
    Init,
    Empty,
    Focus,
    Move,
    Rename,
    Urgent,
    Reload,
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowChange {
    New,
    Close,
    Focus,
    Title,
    FullscreenMode,
    Move,
    Floating,
    Urgent,
    Mark,
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InputType {
    Keyboard,
    Mouse,
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BindingChange {
    Run,
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ShutdownChange {
    Exit,
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShellType {
    XdgShell,
    Xwayland,
    Unknown,
}
