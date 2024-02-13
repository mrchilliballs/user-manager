use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("invalid username: must not be empty and must only contain ASCII-alphanumeric characters")]
pub struct UsernameError;

/// Usernames can only hold ASCII-alphanumeric characters and must not be empty.
#[derive(Debug, Hash, PartialEq, Eq, Serialize, Deserialize, Clone, PartialOrd, Ord)]
pub struct Username(String);
impl Username {
    /// Will fail if provided username is not valid.
    pub fn build(username: &str) -> Result<Self, UsernameError> {
        if Self::is_valid(username) {
            Ok(Username(username.to_string()))
        } else {
            Err(UsernameError)
        }
    }
    /// Gets the username.
    pub fn get(&self) -> &str {
        &self.0
    }
    /// Sets the username to a value. The value must be a valid username.
    pub fn set(&mut self, new_username: &str) -> Result<(), UsernameError> {
        if Self::is_valid(new_username) {
            self.0 = new_username.to_string();
            Ok(())
        } else {
            Err(UsernameError)
        }
    }
    /// Checks if a value is ASCII-alphanumeric and is not empty.
    pub fn is_valid(candidate: &str) -> bool {
        candidate.chars().all(|c| c.is_ascii_alphanumeric()) && !candidate.is_empty()
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for Username {
    type Err = UsernameError;
    /// Candidate must be ASCII-alphanumeric and must not be empty.
    fn from_str(candiate: &str) -> Result<Self, Self::Err> {
        Username::build(candiate)
    }
}

impl Display for Username {
    /// Simply displays the username.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let username = Username::build("WildSir").unwrap();
        assert_eq!(username.0, "WildSir");
    }

    #[test]
    fn test_is_valid() {
        assert!(Username::is_valid("WildSir"));
    }
    #[test]
    fn test_is_valid_ascii_alphanumeric() {
        assert!(Username::is_valid("0"));
        assert!(Username::is_valid("K"));
    }

    #[test]
    fn test_is_invalid_ascii_alphanumeric() {
        assert!(!Username::is_valid("✅"));
        assert!(!Username::is_valid("①"));
        assert!(!Username::is_valid(""));
        assert!(!Username::is_valid(" "));
        assert!(!Username::is_valid("\n"));
        assert!(!Username::is_valid("%"));
    }

    #[test]
    fn test_get() {
        let username = Username::build("WildSir").unwrap();
        assert_eq!(username.0, username.get());
    }

    #[test]
    fn test_set() {
        let mut username = Username::build("WildSir").unwrap();
        username.set("Sir").unwrap();
        assert_eq!(username.as_str(), "Sir")
    }

    #[test]
    fn test_from_str() {
        let username = Username::from_str("WildSir").unwrap();
        assert_eq!(username.as_str(), "WildSir");
    }

    #[test]
    fn test_display() {
        let username = Username::from_str("WildSir").unwrap();
        assert_eq!(username.to_string(), username.get());
    }

    // TODO: Property based testing
}
