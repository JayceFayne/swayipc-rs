use serde_derive::Deserialize;
//TODO: reduce execive use of option with serde flatten and extra struct while preserving semantics (e.g. BindingEvent)

#[derive(Debug, Deserialize)]
pub struct CommandOutcome {
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Workspace {
    pub num: i32,
    pub name: String,
    pub visible: bool,
    pub focused: bool,
    pub urgent: bool,
    pub rect: Rect,
    pub output: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Success {
    pub success: bool,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Mode {
    pub width: i32,
    pub height: i32,
    pub refresh: i32,
}

#[derive(Debug, Deserialize)]
pub struct Output {
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
}

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

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClickMethod {
    ButtonAreas,
    Clickfinger,
    None,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScrollMethod {
    TwoFingerEdge,
    OnButtonDown,
    None,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ButtonMapping {
    LMR,
    LRm,
}

#[derive(Debug, Deserialize)]
pub struct Input {
    pub identifier: String,
    pub name: String,
    pub vendor: i32,
    pub product: i32,
    #[serde(rename = "type")]
    pub input_type: String,
    pub xkb_active_layout_name: Option<String>, //TODO: implement this with custom logic since this is only None if its not a keyboard
    #[serde(default)]
    pub xkb_layout_names: Vec<String>,
    pub xkb_active_layout_name_index: Option<u32>,
    pub libinput: Option<Libinput>,
}

#[derive(Debug, Deserialize)]
pub struct Seat {
    pub name: String,
    pub capabilities: i32,
    pub focus: u32,
    #[serde(default)]
    pub devices: Vec<Input>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Rect {
    pub x: i64,
    pub y: i64,
    pub width: i64,
    pub height: i64,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct WindowProperties {
    pub title: Option<String>,
    pub instance: String,
    pub class: String,
    pub window_role: Option<String>,
    pub transient_for: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeType {
    Root,
    Output,
    Workspace,
    Con,
    FloatingCon,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NodeBorder {
    Normal,
    Pixel,
    Csd,
    None,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NodeLayout {
    SplitH,
    SplitV,
    Stacked,
    Tabbed,
    Output,
    None,
}

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
    #[serde(default)]
    pub floating_nodes: Vec<Node>,
    pub representation: Option<String>,
    pub app_id: Option<String>,
    pub pid: Option<i32>,
    pub window: Option<i32>,
    pub window_properties: Option<WindowProperties>,
}

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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BarMode {
    Dock,
    Hide,
    Invisible,
}

#[derive(Debug, Deserialize)]
pub struct Gaps {
    pub top: usize,
    pub bottom: usize,
    pub right: usize,
    pub left: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Position {
    Bottom,
    Top,
}

#[derive(Debug, Deserialize)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub human_readable: String,
    pub loaded_config_file_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub config: String,
}

#[derive(Debug, Deserialize)]
pub enum Event {
    Workspace(WorkspaceEvent),
    Mode(ModeEvent),
    Window(WindowEvent),
    BarConfigUpdate(BarConfig),
    Binding(BindingEvent),
    Shutdown(ShutdownEvent),
    Tick(TickEvent),
    BarStateUpdate(BarStateUpdateEvent),
    Input(InputEvent),
}

#[derive(Debug, Deserialize)]
pub struct InputEvent {
    pub change: InputChange,
    pub input: Input,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InputChange {
    Added,
    Removed,
    XkbKeymap,
    XkbLayout,
    LibinputConfig,
}

#[derive(Debug, Deserialize)]
pub struct BarStateUpdateEvent {
    pub id: String,
    pub visible_by_modifier: bool,
}

#[derive(Debug, Deserialize)]
pub struct TickEvent {
    pub first: bool,
    pub payload: String,
}

#[derive(Debug, Deserialize)]
pub struct WorkspaceEvent {
    pub change: WorkspaceChange,
    pub current: Option<Node>, //Only None if WorkspaceChange::Reload
    pub old: Option<Node>,
}

#[derive(Debug, Deserialize)]
pub struct ModeEvent {
    pub change: String,
    pub pango_markup: bool,
}

#[derive(Debug, Deserialize)]
pub struct WindowEvent {
    pub change: WindowChange,
    pub container: Node,
}

#[derive(Debug, Deserialize)]
pub struct BindingEvent {
    pub change: BindingChange,
    #[serde(flatten)]
    pub optional: Option<BindingEventOps>,
}

#[derive(Debug, Deserialize)]
pub struct BindingEventOps {
    pub command: String,
    pub event_state_mask: Vec<String>,
    pub input_code: u8,
    pub symbol: String,
    pub input_type: InputType,
}

#[derive(Debug, Deserialize)]
pub struct ShutdownEvent {
    pub change: ShutdownChange,
}

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

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InputType {
    Keyboard,
    Mouse,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BindingChange {
    Run,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ShutdownChange {
    Exit,
}
