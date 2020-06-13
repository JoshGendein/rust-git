mod error;
mod init;
mod index;
mod types;
mod file;
mod add;

use clap::{App, Arg, SubCommand};

fn main() {
    let m = App::new("gitr")
        .subcommand(SubCommand::with_name("init").about("Initialize the repo"))
        .subcommand(SubCommand::with_name("add")
                .about("Add a file")
                .arg(
                    Arg::with_name("file")
                    .help("File to add")
                    .index(1)
                    .multiple(true)
                    .required(true),
                ),
    ).get_matches();

    match m.subcommand() {
        ("init", Some(..)) =>
            match init::init() {
                Ok(()) => println!("Repo Initialized."),
                Err(..) => println!("Already Initialized!"),
            },
        ("add", Some(submatch)) => {
            match add::add_all(&submatch.values_of("file").unwrap().collect()) {
                Ok(()) => (),
                Err(e) => println!("Error: {}", e),
            }
        }
        _ => println!("Command not recognized.")
    }
}
