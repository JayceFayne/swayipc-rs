pub use super::states::*;
pub use super::Command;

mod default_border;
mod floating_modifier;
mod floating_size;
mod focus_follows_mouse;
mod focus_on_window_activation;
mod focus_wrapping;
mod fullscreen;
mod gaps;
mod hide_edge_borders;
mod mark;
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
mod unmark;
mod urgent;

impl Command {
    pub fn fullscreen(self) -> Command<Fullscreen<()>> {
        self.push("fullscreen").transmute()
    }

    pub fn sway_move(self) -> Command<Move<()>> {
        self.push("move").transmute()
    }

    pub fn nop(self, comment: impl AsRef<str>) -> Command<Final> {
        self.push("nop").push(comment.as_ref()).transmute()
    }

    pub fn reload(self) -> Command<Final> {
        self.push("reload").transmute()
    }

    pub fn rename(self) -> Command<Rename<()>> {
        self.push("rename").transmute()
    }

    //TODO: impl resize

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
        self.push("title_format")
            .push(title_format.as_ref())
            .transmute()
    }

    pub fn default_border(self) -> Command<DefaultBorder<()>> {
        self.push("default_border").transmute()
    }

    pub fn default_floating_border(self) -> Command<DefaultBorder<()>> {
        self.push("default_floating_border").transmute()
    }

    pub fn exec(self, exec: impl AsRef<str>) -> Command<Final> {
        self.push("exec").push(exec.as_ref()).transmute()
    }

    pub fn exec_always(self, exec_always: impl AsRef<str>) -> Command<Final> {
        self.push("exec_always")
            .push(exec_always.as_ref())
            .transmute()
    }

    pub fn floating_maximum_size(self) -> Command<FloatingSize<()>> {
        self.push("floating_maximum_size").transmute()
    }

    pub fn floating_minimum_size(self) -> Command<FloatingSize<()>> {
        self.push("floating_minimum_size").transmute()
    }

    pub fn floating_modifier(self) -> Command<FloatingModifier<()>> {
        self.push("floating_minimum_size").transmute()
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

    pub fn titlebar_border_thickness(self, px: usize) -> Command<Final> {
        self.push("titlebar_border_thickness")
            .push(px.to_string().as_str())
            .transmute()
    }

    pub fn titlebar_padding(self) -> Command<TitlebarPadding<()>> {
        self.push("titlebar_padding").transmute()
    }

    pub fn gaps(self) -> Command<Gaps<()>> {
        self.push("gaps").transmute()
    }

    pub fn hide_edge_borders(self) -> Command<HideEdgeBorders<()>> {
        self.push("hide_edge_borders").transmute()
    }

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

    pub fn mode(self, mode: impl AsRef<str>) -> Command<Final> {
        self.push("mode").push(mode.as_ref()).transmute()
    }

    pub fn mouse_warping(self) -> Command<MouseWraping<()>> {
        self.push("mouse_warping").transmute()
    }

    pub fn no_focus(self, criteria: impl AsRef<str>) -> Command<Final> {
        self.push("no_focus").push(criteria.as_ref()).transmute()
    }

    //TODO: impl output

    pub fn popup_during_fullscreen(self) -> Command<PopupDuringFullscreen<()>> {
        self.push("popup_during_fullscreen").transmute()
    }

    pub fn set(self, key: impl AsRef<str>, value: impl AsRef<str>) -> Command<Final> {
        self.push(key.as_ref()).push(value.as_ref()).transmute()
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
            .push(threshold.to_string().as_str())
            .transmute()
    }

    pub fn title_align(self) -> Command<TitleAlign<()>> {
        self.push("title_align").transmute()
    }

    pub fn unmark(self) -> Command<Unmark<()>> {
        self.push("unmark").transmute()
    }

    pub fn urgent(self) -> Command<Urgent<()>> {
        self.push("urgent").transmute()
    }
}
