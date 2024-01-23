use std::fmt::Display;

use proptest_derive::Arbitrary;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Arbitrary)]
pub struct Money(f64);

impl Money {
    pub fn new(amount: f64) -> Self {
        todo!()
    }
    pub fn value(&self) -> f64 {
        todo!()
    }
    pub fn increase(&mut self, amount: f64) {
        todo!()
    }
    pub fn decrease(&mut self, amount: f64) {
        todo!()
    }
    pub fn set(&mut self, amount: f64) {
        todo!()
    }
}

impl From<f64> for Money {
    fn from(value: f64) -> Self {
        todo!()
    }
}

impl Display for Money {
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
    fn test_value() {
        todo!()
    }

    #[test]
    fn test_increase() {
        todo!()
    }

    #[test]
    fn test_decrease() {
        todo!()
    }

    #[test]
    fn test_set() {
        todo!()
    }

    #[test]
    fn test_from_f64() {
        todo!()
    }

    #[test]
    fn test_display() {
        todo!()
    }

    // TODO: Property-based testing
    // use proptest::prelude::*;
    // proptest! {
    //     #[test]
    //     fn test_new_all_amounts(amount: f64) {
    //         todo!()
    //     }
    //     #[test]
    //     fn test_value_all_amounts(amount: f64) {
    //         todo!()
    //     }
    //     #[test]
    //     fn test_increase_all_amounts(amount: f64) {
    //         todo!()
    //     }
    //     #[test]
    //     fn test_decrease_all_amounts(amount: f64) {
    //         todo!()
    //     }
    //     #[test]
    //     fn test_set_set_all_amounts(amount: f64) {
    //         todo!()
    //     }
    // }
}
