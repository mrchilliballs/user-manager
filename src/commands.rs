use clap::{ArgGroup, Subcommand};

use crate::{user_list::UserList, username::Username};

use std::fmt::Debug;
use std::io::prelude::*;

#[derive(Debug, Subcommand)]
pub enum Command {
    Insert {
        username: Username,
        name: String,
        money: f64,
    },
    #[clap(group(
        ArgGroup::new("editables")
            .required(true)
            .multiple(true)
            .args(&["name", "money"])
    ))]
    Edit {
        username: Username,
        name: Option<String>,
        money: Option<f64>,
    },
    Get,
    Withdraw {
        username: Username,
        amount: f64,
    },
    Deposit {
        username: Username,
        amount: f64,
    },
    Transfer {
        from: Username,
        to: Username,
        amount: f64,
    },
    // Confirmation required
    Delete {
        username: Username,
    },
}
pub trait Logger: Debug {
    fn print(&mut self, value: &str);
    fn println(&mut self, value: &str) {
        self.print(value);
        self.print("\n");
    }
}
// impl Logger for Term {
//     fn print(&mut self, value: &str) {
//         write!(&self, "{}", value);
//     }
// }

pub trait ReadWrite: Read + Write {}
pub struct CommandParser<'a> {
    command: &'a Command,
    users: &'a mut UserList,
    logger: &'a dyn Logger,
    buf: &'a mut (dyn ReadWrite),
}

impl<'a> CommandParser<'a> {
    pub fn new(
        command: &'a Command,
        users: &'a mut UserList,
        logger: &'a dyn Logger,
        buf: &'a mut (dyn ReadWrite),
    ) -> Self {
        todo!()
    }
    fn load_from_buf() {
        todo!()
    }
    fn save_to_buf() {
        todo!()
    }
    pub fn parse(self) {
        todo!()
    }
    fn insert(&mut self) {
        todo!()
    }
    fn edit(&mut self) {
        todo!()
    }
    fn get(&mut self) {
        todo!()
    }
    fn withdraw(&mut self) {
        todo!()
    }
    fn deposit(&mut self) {
        todo!()
    }
    fn transfer(&mut self) {
        todo!()
    }
    fn delete(&mut self) {
        todo!()
    }
}
