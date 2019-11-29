use std::io::{stdin, stdout, Write};
use swayipc::{Connection, Fallible};

fn main() -> Fallible<()> {
    println!("Executes sway commands in a loop. Enter \"q\" at any time to quit.");
    let mut connection = Connection::new()?;
    let stdin = stdin();
    let mut stdout = stdout();
    loop {
        print!(">>> ");
        stdout.flush()?;
        let mut command_text = String::new();
        stdin.read_line(&mut command_text)?;
        command_text.pop(); // throw away the \n
        if command_text == "q" {
            break;
        }
        for outcome in connection.run_command(&command_text)? {
            if outcome.success {
                println!("success");
            } else {
                println!("failure");
                if let Some(e) = outcome.error {
                    println!("{}", e);
                }
            }
        }
    }
    Ok(())
}
