use swayipc::async_std::io::prelude::WriteExt;
use swayipc::async_std::io::{stdin, stdout};
use swayipc::{Connection, Fallible};

#[async_std::main]
async fn main() -> Fallible<()> {
    println!("Executes sway commands in a loop. Enter \"q\" at any time to quit.");
    let mut connection = Connection::new().await?;
    let stdin = stdin();
    let mut stdout = stdout();
    loop {
        print!(">>> ");
        stdout.flush().await?;
        let mut command_text = String::new();
        stdin.read_line(&mut command_text).await?;
        command_text.pop(); // throw away the \n
        if command_text == "q" {
            break;
        }
        for outcome in connection.run_command(&command_text).await? {
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
