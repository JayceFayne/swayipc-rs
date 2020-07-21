use swayipc::CommandBuilder;
use swayipc::{Connection, Fallible};

fn main() -> Fallible<()> {
    let mut connection = Connection::new()?;
    let command = CommandBuilder::new()
        .fullscreen()
        .toggle()
        .filter("[shell=\"xwayland\"]");
    println!("executing '{}'", command.as_ref());
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
