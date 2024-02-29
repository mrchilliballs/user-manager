//! # User Manager
//! `user-manger` provides utilities to manage users and a fully featured CLI.

// Maybe one day...
// #![deny(missing_docs)]
// #![forbid(unsafe_code)]
pub mod cli;
pub mod command;
mod macros;
pub mod money;
pub mod user;
pub mod user_list;
pub mod username;

use anyhow::{Context, Result};
pub use user::User;
pub use user_list::UserList;

use std::fs::File;
use std::io::{prelude::*, BufWriter};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    users_path: PathBuf,
}

impl Settings {
    const DEFAULT_SETTINGS_PATH: &'static str = "settings.toml";
    const DEFAULT_USERS_PATH: &'static str = "users.json";

    pub fn new(users_path: PathBuf) -> Self {
        Self { users_path }
    }
    pub fn save(&self, user_list: &mut UserList) -> Result<()> {
        let mut settings_file =
            BufWriter::new(File::create(PathBuf::from(Self::DEFAULT_SETTINGS_PATH))?);
        settings_file.write_all(
            toml::to_string_pretty(user_list)
                .context(format!("Failed to save to {}", Self::DEFAULT_SETTINGS_PATH))?
                .as_bytes(),
        )?;

        let users_file = BufWriter::new(File::create(PathBuf::from(&self.users_path))?);
        serde_json::to_writer_pretty(users_file, user_list)?;

        Ok(())
    }
}
impl Default for Settings {
    fn default() -> Self {
        Settings {
            users_path: PathBuf::from(Self::DEFAULT_USERS_PATH),
        }
    }
}
