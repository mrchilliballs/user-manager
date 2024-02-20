use crate::user::User;
use crate::username::Username;
use crate::{money::Money, user::OptionalUser};
use clap::Subcommand;

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
    Edit {
        username: Username,
        #[command(flatten)]
        user: OptionalUser,
    },
    Get {
        username: Option<Username>,
    },
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
    // Confirmation required
    Clear,
}
