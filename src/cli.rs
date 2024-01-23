use clap::Parser;

use crate::{commands::Commands, user_list::UserList, username::Username};

#[derive(Debug, Parser)]
#[command(about = "A CLI to manage users", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub fn parse_command(&self, users: &mut UserList) {
        // Save for later
        // match self.command {
        //     Commands::Add {
        //         username,
        //         name,
        //         money,
        //     } => todo!(),
        //     Commands::Insert {
        //         username,
        //         name,
        //         money,
        //     } => todo!(),
        //     Commands::Edit {
        //         username,
        //         name,
        //         money,
        //     } => todo!(),
        //     Commands::Withdraw { username, amount } => todo!(),
        //     Commands::Deposit { username, amount } => todo!(),
        //     Commands::Transfer { from, to, amount } => todo!(),
        //     Commands::Delete { username } => todo!(),
        // }
    }
}

fn add(users: &mut UserList, username: &Username, name: &str, money: f64) {
    todo!()
}
fn insert(users: &mut UserList, username: &Username, name: &str, money: f64) {
    todo!()
}
fn edit(users: &mut UserList, username: &Username, name: Option<&str>, money: Option<f64>) {
    todo!()
}
fn withdraw(users: &mut UserList, username: &Username, amount: f64) {
    todo!()
}
fn deposit(users: &mut UserList, username: &Username, amount: f64) {
    todo!()
}
fn transfer(users: &mut UserList, from: &Username, to: &Username, amount: f64) {
    todo!()
}
fn delete(users: &mut UserList, username: &Username) {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_command() {
        todo!()
    }
    #[test]
    fn test_add() {
        todo!()
    }
    #[test]
    fn test_insert() {
        todo!()
    }
    #[test]
    fn test_edit() {
        todo!()
    }
    #[test]
    fn test_withdraw() {
        todo!()
    }
    #[test]
    fn test_deposit() {
        todo!()
    }
    #[test]
    fn test_transfer() {
        todo!()
    }
    #[test]
    fn test_delete() {
        todo!()
    }
}
