pub use super::states::*;
pub use super::Command;

mod border;
mod default_border;
mod floating;
mod floating_modifier;
mod floating_size;
mod focus_follows_mouse;
mod focus_on_window_activation;
mod focus_wrapping;
mod fullscreen;
mod gaps;
mod hide_edge_borders;
mod inhibit_idle;
mod layout;
mod mark;
mod max_render_time;
mod mouse_wraping;
mod opacity;
mod popup_during_fullscreen;
mod rename;
mod scratchpad;
mod shortcuts_inhibitor;
mod show_marks;
mod smart_borders;
mod smart_gaps;
mod split;
mod sticky;
mod swap;
mod sway_move;
mod tiling_drag;
mod title_align;
mod titlebar_padding;
mod unbindswitch;
mod unmark;
mod urgent;
mod workspace;
mod workspace_auto_back_and_forth;

impl Command {
    //TODO: bar

    pub fn border(self) -> Command<Border<()>> {
        self.push("border").transmute()
    }

    //TODO: create_output

    pub fn exit(self) -> Command<Final> {
        self.push("exit").transmute()
    }

    pub fn floating(self) -> Command<Floating<()>> {
        self.push("floating").transmute()
    }

    //TODO: focus

    //TODO: force_display_urgency_hint

    pub fn fullscreen(self) -> Command<Fullscreen<()>> {
        self.push("fullscreen").transmute()
    }

    pub fn gaps(self) -> Command<Gaps<()>> {
        self.push("gaps").transmute()
    }

    pub fn inhibit_idle(self) -> Command<InhibitIdle<()>> {
        self.push("inhibit_idle").transmute()
    }

    pub fn layout(self) -> Command<Layout<()>> {
        self.push("layout").transmute()
    }

    pub fn max_render_time(self) -> Command<MaxRenderTime<()>> {
        self.push("max_render_time").transmute()
    }

    pub fn sway_move(self) -> Command<Move<()>> {
        self.push("move").transmute()
    }

    pub fn nop(self) -> Command<Final> {
        self.push("nop").transmute()
    }

    pub fn reload(self) -> Command<Final> {
        self.push("reload").transmute()
    }

    pub fn rename(self) -> Command<Rename<()>> {
        self.push("rename").transmute()
    }

    //TODO: resize

    pub fn scratchpad(self) -> Command<Scratchpad<()>> {
        self.push("scratchpad").transmute()
    }

    pub fn shortcuts_inhibitor(self) -> Command<ShortcutsInhibitor<()>> {
        self.push("shortcuts_inhibitor").transmute()
    }

    pub fn split(self) -> Command<Split<()>> {
        self.push("split").transmute()
    }

    pub fn sticky(self) -> Command<Sticky<()>> {
        self.push("sticky").transmute()
    }

    pub fn swap(self) -> Command<Swap<()>> {
        self.push("swap").transmute()
    }

    pub fn title_format(self, title_format: impl AsRef<str>) -> Command<Final> {
        self.push("title_format").push(title_format).transmute()
    }

    //TODO: assign

    //TODO: bindsym

    //TODO: bindcode

    //TODO: bindswitch

    //TODO: client

    pub fn default_border(self) -> Command<DefaultBorder<()>> {
        self.push("default_border").transmute()
    }

    pub fn default_floating_border(self) -> Command<DefaultBorder<()>> {
        self.push("default_floating_border").transmute()
    }

    pub fn exec(self, exec: impl AsRef<str>) -> Command<Final> {
        self.push("exec").push(exec).transmute()
    }

    pub fn exec_always(self, exec_always: impl AsRef<str>) -> Command<Final> {
        self.push("exec_always").push(exec_always).transmute()
    }

    pub fn floating_maximum_size(self) -> Command<FloatingSize<()>> {
        self.push("floating_maximum_size").transmute()
    }

    pub fn floating_minimum_size(self) -> Command<FloatingSize<()>> {
        self.push("floating_minimum_size").transmute()
    }

    pub fn floating_modifier(self) -> Command<FloatingModifier<()>> {
        self.push("floating_modifier").transmute()
    }

    pub fn focus_follows_mouse(self) -> Command<FocusFollowMouse<()>> {
        self.push("focus_follows_mouse").transmute()
    }

    pub fn focus_on_window_activation(self) -> Command<FocusOnWindowActivation<()>> {
        self.push("focus_on_window_activation").transmute()
    }

    pub fn focus_wrapping(self) -> Command<FocusWrapping<()>> {
        self.push("focus_wrapping").transmute()
    }

    //TODO: font

    pub fn titlebar_border_thickness(self, px: usize) -> Command<Final> {
        self.push("titlebar_border_thickness")
            .push(px.to_string())
            .transmute()
    }

    pub fn titlebar_padding(self) -> Command<TitlebarPadding<()>> {
        self.push("titlebar_padding").transmute()
    }

    pub fn hide_edge_borders(self) -> Command<HideEdgeBorders<()>> {
        self.push("hide_edge_borders").transmute()
    }

    //TODO: input

    //TODO: seat

    pub fn kill(self) -> Command<Gaps<()>> {
        self.push("kill").transmute()
    }

    pub fn smart_borders(self) -> Command<SmartBoarders<()>> {
        self.push("smart_borders").transmute()
    }

    pub fn smart_gaps(self) -> Command<SmartGaps<()>> {
        self.push("smart_gaps").transmute()
    }

    pub fn mark(self) -> Command<Mark<()>> {
        self.push("mark").transmute()
    }

    //TODO: add missing subcommands
    pub fn mode(self, mode: impl AsRef<str>) -> Command<Final> {
        self.push("mode").push(mode).transmute()
    }

    pub fn mouse_warping(self) -> Command<MouseWraping<()>> {
        self.push("mouse_warping").transmute()
    }

    pub fn no_focus(self, criteria: impl AsRef<str>) -> Command<Final> {
        self.push("no_focus").push(criteria).transmute()
    }

    //TODO: output

    pub fn popup_during_fullscreen(self) -> Command<PopupDuringFullscreen<()>> {
        self.push("popup_during_fullscreen").transmute()
    }

    pub fn set(self, key: impl AsRef<str>, value: impl AsRef<str>) -> Command<Final> {
        self.push(key).push(value).transmute()
    }

    pub fn show_marks(self) -> Command<ShowMarks<()>> {
        self.push("show_marks").transmute()
    }

    pub fn opacity(self) -> Command<Opacity<()>> {
        self.push("opacity").transmute()
    }

    pub fn tiling_drag(self) -> Command<TilingDrag<()>> {
        self.push("smart_borders").transmute()
    }

    pub fn tiling_drag_threshold(self, threshold: usize) -> Command<TilingDrag<()>> {
        self.push("tiling_drag_threshold")
            .push(threshold.to_string())
            .transmute()
    }

    pub fn title_align(self) -> Command<TitleAlign<()>> {
        self.push("title_align").transmute()
    }

    pub fn unbindswitch(self) -> Command<Unbindswitch<()>> {
        self.push("unbindswitch").transmute()
    }

    //TODO: unbindsym

    //TODO: unbindcode

    pub fn unmark(self) -> Command<Unmark<()>> {
        self.push("unmark").transmute()
    }

    pub fn urgent(self) -> Command<Urgent<()>> {
        self.push("urgent").transmute()
    }

    pub fn workspace(self) -> Command<Workspace<()>> {
        self.push("workspace").transmute()
    }

    pub fn workspace_auto_back_and_forth(self) -> Command<WorkspaceAutoBackAndForth<()>> {
        self.push("workspace_auto_back_and_forth").transmute()
    }
}
