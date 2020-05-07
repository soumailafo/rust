use std::io::{self, stdin};
use std::io::Write;
use std::io::stdout;
use std::process::{Command, Stdio};
use std::process::Output;

fn main() -> std::io::Result<()> {
    loop {
        print!("> ~> ");
        stdout().flush();

        let mut user_input = String::with_capacity(256);
        stdin().read_line(&mut user_input)?;

        let mut splits = user_input.trim().split_whitespace();
        let command = splits.next().unwrap();
        let args = splits;

        let mut child = Command::new( command)
            .args(args)
            .spawn()?;

        let output = child.wait_with_output()?;

        println!("output = {:?}", output);
        println!("status: {}", output.status);

       //Ok(());
    }
}
