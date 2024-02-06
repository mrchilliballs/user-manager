use crate::{money::Money, username::Username, User};
use clap::{ArgGroup, Subcommand};

pub mod logger;
pub mod parser;
pub use logger::Logger;

#[derive(Debug, Subcommand, PartialEq, Clone)]
pub enum Command {
    Insert {
        username: Username,
        #[command(flatten)]
        user: User,
    },
    #[clap(group(
        ArgGroup::new("editables")
            .required(true)
            .multiple(true)
            .args(&["name", "money"])
    ))]
    Edit {
        username: Username,
        #[command(flatten)]
        user: Option<User>,
    },
    Get,
    Withdraw {
        username: Username,
        amount: Money,
    },
    Deposit {
        username: Username,
        amount: Money,
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
