//! # User Manager
//! `user-manger` provides utilities to manage users and a fully featured CLI.
pub mod cli;
pub mod command;
pub mod macros;
pub mod money;
pub mod user;
pub mod user_list;
pub mod username;

pub use user::User;
pub use user_list::UserList;
