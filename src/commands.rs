use clap::{ArgGroup, Subcommand};

use crate::username::Username;

#[derive(Debug, Subcommand)]
pub enum Commands {
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
