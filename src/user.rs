use std::fmt::Display;

use crate::money::Money;

use proptest_derive::Arbitrary;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Arbitrary, PartialEq, Eq, Clone)]
pub struct User {
    pub name: String,
    pub money: Money,
    // Potential new feature: currency symbols
}

impl Display for User {
    /// Formatted as "{name} | ${money}". Money is rounded to the nearest hundreth.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.name.is_empty() {
            write!(f, "Unnamed User | {}", self.money)
        } else {
            write!(f, "{} | {}", self.name, self.money)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let user = User {
            name: String::from("Wild Sir"),
            money: Money::from(10.0),
        };
        assert_eq!(user.to_string(), "Wild Sir | $10.00");
    }

    #[test]
    fn test_display_missing_name() {
        let user = User {
            money: Money::from(10.0),
            ..Default::default()
        };
        assert_eq!(user.to_string(), "Unnamed User | $10.00");
    }
}
