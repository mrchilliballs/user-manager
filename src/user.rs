use std::fmt::Display;

use crate::money::Money;

use proptest_derive::Arbitrary;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Arbitrary)]
pub struct User {
    pub name: String,
    pub money: Money,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_display() {
        todo!()
    }
}
