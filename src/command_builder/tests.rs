use super::{Command, Filter};

#[test]
fn filter_single_criteria() {
    let filter = Filter::new().workspace("c");
    let reference = "[workspace=c]";
    assert_eq!(filter.as_ref(), reference)
}

#[test]
fn filter_multiple_criteria() {
    let filter = Filter::new().pid("1").shell("xwayland");
    let reference = "[pid=1 shell=xwayland]";
    assert_eq!(filter.as_ref(), reference)
}

#[test]
fn border_csd() {
    let command = Command::new().border().csd();
    let reference = "border csd";
    assert_eq!(command.as_ref(), reference)
}

#[test]
fn border_none() {
    let command = Command::new().border().none();
    let reference = "border none";
    assert_eq!(command.as_ref(), reference)
}

#[test]
fn border_toggle() {
    let command = Command::new().border().toggle();
    let reference = "border toggle";
    assert_eq!(command.as_ref(), reference)
}

#[test]
fn border_normal() {
    let command = Command::new().border().normal();
    let reference = "border normal";
    assert_eq!(command.as_ref(), reference)
}

#[test]
fn border_pixel() {
    let command = Command::new().border().pixel();
    let reference = "border pixel";
    assert_eq!(command.as_ref(), reference)
}

#[test]
fn border_normal_with() {
    let command = Command::new().border().normal().with(10);
    let reference = "border normal 10";
    assert_eq!(command.as_ref(), reference)
}

#[test]
fn border_pixel_with() {
    let command = Command::new().border().pixel().with(10);
    let reference = "border pixel 10";
    assert_eq!(command.as_ref(), reference)
}

#[test]
fn exit() {
    let command = Command::new().exit();
    let reference = "exit";
    assert_eq!(command.as_ref(), reference)
}

#[test]
fn floating_enable() {
    let command = Command::new().floating().enable();
    let reference = "floating enable";
    assert_eq!(command.as_ref(), reference)
}

#[test]
fn floating_disable() {
    let command = Command::new().floating().disable();
    let reference = "floating disable";
    assert_eq!(command.as_ref(), reference)
}

#[test]
fn floating_toggle() {
    let command = Command::new().floating().toggle();
    let reference = "floating toggle";
    assert_eq!(command.as_ref(), reference)
}

#[test]
fn fullscreen_enable() {
    let command = Command::new().fullscreen().enable();
    let reference = "fullscreen enable";
    assert_eq!(command.as_ref(), reference)
}

#[test]
fn fullscreen_disable() {
    let command = Command::new().fullscreen().disable();
    let reference = "fullscreen disable";
    assert_eq!(command.as_ref(), reference)
}

#[test]
fn fullscreen_toggle() {
    let command = Command::new().fullscreen().toggle();
    let reference = "fullscreen toggle";
    assert_eq!(command.as_ref(), reference)
}

#[test]
fn gaps_inner_all_set() {
    let command = Command::new().gaps().inner().all().set().amount(1);
    let reference = "gaps inner all set 1";
    assert_eq!(command.as_ref(), reference)
}

#[test]
fn gaps_outer_current_plus() {
    let command = Command::new().gaps().outer().current().plus().amount(1);
    let reference = "gaps outer current plus 1";
    assert_eq!(command.as_ref(), reference)
}

#[test]
fn gaps_horizontal_all_minus() {
    let command = Command::new().gaps().horizontal().all().minus().amount(1);
    let reference = "gaps horizontal all minus 1";
    assert_eq!(command.as_ref(), reference)
}

#[test]
fn inhibit_idle_focus() {
    let command = Command::new().inhibit_idle().focus();
    let reference = "inhibit_idle focus";
    assert_eq!(command.as_ref(), reference)
}

#[test]
fn inhibit_idle_fullscreen() {
    let command = Command::new().inhibit_idle().fullscreen();
    let reference = "inhibit_idle fullscreen";
    assert_eq!(command.as_ref(), reference)
}

#[test]
fn inhibit_idle_open() {
    let command = Command::new().inhibit_idle().open();
    let reference = "inhibit_idle open";
    assert_eq!(command.as_ref(), reference)
}

#[test]
fn inhibit_idle_none() {
    let command = Command::new().inhibit_idle().none();
    let reference = "inhibit_idle none";
    assert_eq!(command.as_ref(), reference)
}

#[test]
fn inhibit_idle_visible() {
    let command = Command::new().inhibit_idle().visible();
    let reference = "inhibit_idle visible";
    assert_eq!(command.as_ref(), reference)
}
