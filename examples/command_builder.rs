use swayipc::{Command, Connection, Fallible, Filter};

fn main() -> Fallible<()> {
    let filter = Filter::new().shell("xwayland");
    let command = Command::new().filter(filter).fullscreen().enable();
    println!("executing '{}'", command);
    exec_command(command)?;
    std::thread::sleep(std::time::Duration::new(2, 0));
    let filter = "[shell=xwayland]";
    let command = Command::new().filter(filter).fullscreen().disable();
    println!("executing '{}'", command);
    exec_command(command)?;
    Ok(())
}

fn exec_command(command: impl AsRef<str>) -> Fallible<()> {
    let mut connection = Connection::new()?;
    for outcome in connection.run_command(command)? {
        if outcome.success {
            println!("success");
        } else {
            println!("failure");
            if let Some(e) = outcome.error {
                println!("{}", e);
            }
        }
    }
    Ok(())
}
