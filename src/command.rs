use crate::{money::Money, username::Username};
use clap::{ArgGroup, Subcommand};

mod logger;
mod parser;

#[derive(Debug, Subcommand, PartialEq)]
pub enum Command {
    Insert {
        username: Username,
        name: String,
        money: Money,
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
        money: Option<Money>,
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
