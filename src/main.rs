use std::{
    fs::File,
    io::{BufReader, BufWriter, ErrorKind},
};

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use user_manager::{cli::Cli, command::logger::DefaultLogger, UserList};
fn main() -> Result<()> {
    let cli = Cli::parse();
    let file_outer = File::open("users.json");
    // let path = PathBuf::from_str("users.json").unwrap();
    let mut users = match &file_outer {
        Ok(file) => {
            let mut buf = BufReader::new(file);
            UserList::load(&mut buf).context("Failed to read users.json")?
        }
        Err(err) => {
            if let ErrorKind::NotFound = err.kind() {
                let file = File::create("users.json").context("Failed to create users.json")?;
                let mut buf = BufWriter::new(file);
                let users = UserList::new();
                users
                    .save(&mut buf)
                    .context("Failed to write to users.json")?;
                users
            } else {
                return Err(anyhow!("Failed to open users.json: {}", err));
            }
        }
    };
    let logger = DefaultLogger;
    cli.parse_command(&mut users, &logger);

    let mut file = File::create("users.json").unwrap();
    users.save(&mut file)?;
    Ok(())
}

// struct App {
//     settings: Settings,
// }
