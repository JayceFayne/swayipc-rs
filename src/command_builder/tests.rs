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
