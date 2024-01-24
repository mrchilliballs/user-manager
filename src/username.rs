use anyhow::{Error, Result};
use proptest_derive::Arbitrary;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Arbitrary, Hash, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Username(String);
impl Username {
    pub fn build(value: &str) -> Result<Self> {
        todo!()
    }
    pub fn get(&self) -> &str {
        todo!()
    }
    pub fn set(&mut self, new_username: &str) -> Result<Self> {
        todo!()
    }
    fn is_valid(string: &str) -> bool {
        todo!()
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for Username {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl Display for Username {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let username = Username::build("WildSir").unwrap();
        assert_eq!(username.0, "Wildsir");
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

    // TODO: Property based testing
}
