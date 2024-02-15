use clap::Parser;

#[cfg_attr(test, mockall_double::double)]
use crate::user_list::UserList;
use crate::{
    command::parser::CommandParser,
    command::{Command, Logger},
};

#[derive(Debug, Parser)]
#[command(about = "A CLI to manage users", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

impl Cli {
    pub fn parse_command(self, users: &mut UserList, logger: &impl Logger) {
        CommandParser::new(self.command, users, logger).parse();
    }
}
