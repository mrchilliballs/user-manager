use crate::money::Money;
use crate::user::OptionalUser;
use crate::user::User;
use crate::username::Username;
use clap::Subcommand;

pub mod logger;
pub mod parser;
pub use logger::Logger;

/// Commands used to manage a `UserList`, parsed with `CommandParser`.
#[derive(Debug, Subcommand, PartialEq, Clone)]
pub enum Command {
    /// Adds a new user, replacing any existing user with the same username
    Insert {
        username: Username,
        #[command(flatten)]
        user: User,
    },
    /// Modify fields for an existing user
    Edit {
        username: Username,
        #[command(flatten)]
        user: OptionalUser,
    },
    /// Display the fields of a specific user or list all users if none is specified
    Get { username: Option<Username> },
    /// Deduct the specified amount from a user's account
    Withdraw { username: Username, amount: Money },
    /// Credit the specified amount to a user's account
    Deposit { username: Username, amount: Money },
    /// Move funds between two user accounts
    Transfer {
        from: Username,
        to: Username,
        amount: Money,
    },
    /// Permanently remove a user
    Delete {
        username: Username,
        /// Do not ask for confirmation
        #[arg(short, long)]
        force: bool,
    },
    /// Permanently remove all users
    Clear {
        /// Do not ask for confirmation
        #[arg(short, long)]
        force: bool,
    },
}
