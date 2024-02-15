use std::{
    fs::File,
    io::{BufReader, BufWriter, ErrorKind},
};

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use console::Term;
use user_manager::{cli::Cli, UserList};

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
    let term = Term::stdout();
    cli.parse_command(&mut users, &term);
    Ok(())
}

// struct App {
//     settings: Settings,
// }
