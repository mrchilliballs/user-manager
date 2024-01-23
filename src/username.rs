use anyhow::Error;
use proptest_derive::Arbitrary;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Arbitrary, Hash, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Username(String);
impl Username {
    pub fn new(value: &str) -> Self {
        todo!()
    }
    pub fn get(&self) -> &str {
        todo!()
    }
    pub fn set(&mut self) {
        todo!()
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
    #[test]
    fn test_new() {
        todo!()
    }

    #[test]
    fn test_new_not_ascii_alphabetic() {
        todo!()
    }

    #[test]
    fn test_new_empty() {
        todo!()
    }

    #[test]
    fn test_get() {
        todo!()
    }

    #[test]
    fn test_set() {
        todo!()
    }

    #[test]
    fn test_from_str() {
        todo!()
    }

    // TODO: Property based testing
}
