use std::io::{stdin};
use std::io::Write;
use std::io::stdout;
use std::process::{Command, Stdio, Child};
use std::path::Path;
use std::env;

fn main() {

}

    fn cmdProcess()-> std::io::Result<()> {
        loop {
            print!("> ");
            stdout().flush();

            let mut user_input = String::with_capacity(256);
            stdin().read_line(&mut user_input)?;

            let mut splits = user_input.trim().split_whitespace();
           // let command = splits.next().unwrap();
            let command = splits.next()?;
            let args = splits;

            match command {
                "cd" => {
                    let new_rep = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(new_rep);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }
                },
                command => {
                    let mut child = Command::new(command)
                        .args(args)
                        .spawn();

                    match child {
                        Ok(mut child) => { child.wait(); },
                        Err(e) => eprintln!("{}", e),
                    }
                }
            }
        }
    }

    fn basic_version()-> stdio::io::Result<()>{

        let mut spcmd = Command::new("ls");
        spcmd.arg("-c")
             .arg("echo basic version");

        let vers_1 = spcmd.output();
        let vers_2 = spcmd.output();

    }

    fn version_avance()-> std::io::Result<()> {
        loop {
            print!("> ");
            stdout().flush();

            let mut user_input = String::with_capacity(256);
            stdin().read_line(&mut user_input)?;

            // must be peekable so we know when we are on the last command
            let mut cmds = user_input.trim().split(" | ").peekable();
            let mut prev_comd = None;

            while let Some(comd) = cmds.next() {
                let mut parts = comd.trim().split_whitespace();
               // let command = parts.next().unwrap();
                let comd = parts.next()?;
                let args = parts;

                match comd {
                    "cd" => {
                        let new_dir = args.peekable().peek()
                            .map_or("/", |x| *x);
                        let root = Path::new(new_dir);
                        if let Err(e) = env::set_current_dir(&root) {
                            eprintln!("{}", e);
                        }

                        prev_comd = None;
                    },
                    comd => {
                        let stdin = prev_comd
                            .map_or(
                                Stdio::inherit(),
                                |output: Child| Stdio::from(output.stdout.unwrap())
                            );

                        let stdout = if cmds.peek().is_some() {
                            // there is another command piped behind this one
                            // prepare to send output to the next command
                            Stdio::piped()
                        } else {
                            // there are no more commands piped behind this one
                            // send output to shell stdout
                            Stdio::inherit()
                        };

                        let output = Command::new(comd)
                            .args(args)
                            .stdin(stdin)
                            .stdout(stdout)
                            .spawn();

                        match output {
                            Ok(output) => { prev_comd = Some(output); },
                            Err(e) => {
                                prev_comd = None;
                                eprintln!("{}", e);
                            },
                        };
                    }
                }
            }

            if let Some(mut comd_final) = prev_comd {
                // block until the final command has finished
                comd_final.wait();
            }
        }
    }

