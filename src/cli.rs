use clap::Parser;

use crate::{commands::Command, user_list::UserList};

#[derive(Debug, Parser)]
#[command(about = "A CLI to manage users", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

impl Cli {
    pub fn parse_command(&self, users: &mut UserList) {
        todo!()
    }
}
