use anyhow::{Error, Result};
use proptest_derive::Arbitrary;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};
use thiserror::Error;

use crate::user::User;

#[derive(Error, Debug)]
#[error("invalid username: must not be empty and must only contain ASCII-alphanumeric characters")]
pub struct UsernameError;

#[derive(Debug, Arbitrary, Hash, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Username(String);
impl Username {
    pub fn build(username: &str) -> Result<Self, UsernameError> {
        if Self::is_valid(username) {
            Ok(Username(username.to_string()))
        } else {
            Err(UsernameError)
        }
    }
    pub fn get(&self) -> &str {
        &self.0
    }
    pub fn set(&mut self, new_username: &str) -> Result<(), UsernameError> {
        if Self::is_valid(new_username) {
            self.0 = new_username.to_string();
            Ok(())
        } else {
            Err(UsernameError)
        }
    }
    pub fn is_valid(candidate: &str) -> bool {
        candidate.chars().all(|c| c.is_ascii_alphanumeric()) && !candidate.is_empty()
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for Username {
    type Err = UsernameError;
    fn from_str(candiate: &str) -> Result<Self, Self::Err> {
        Username::build(candiate)
    }
}

impl Display for Username {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
