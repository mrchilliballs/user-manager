use std::{fmt::Display, ops::Deref};

use proptest_derive::Arbitrary;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Arbitrary, Clone, Copy)]
pub struct Money(f64);

impl Money {
    pub fn new(amount: f64) -> Self {
        todo!()
    }
    pub fn val(&self) -> f64 {
        todo!()
    }
    pub fn set(&mut self, amount: f64) {
        todo!()
    }
    pub fn withdraw(&mut self, amount: f64) {
        todo!()
    }
    pub fn deposit(&mut self, amount: f64) {
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

impl PartialEq for Money {
    fn eq(&self, other: &Self) -> bool {
        self.val() == other.val()
    }
}
impl Eq for Money {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let money = Money::new(10.0);
        assert_eq!(money.0, 10.0);
    }
    #[test]
    fn test_val() {
        let money = Money::new(10.0);
        assert_eq!(money.0, money.val());
    }

    #[test]
    fn test_set() {
        let mut money = Money::new(5.0);
        money.set(10.0);
        assert_eq!(money.val(), 10.0);
    }

    #[test]
    fn test_deposit() {
        let mut money = Money::new(5.0);
        money.deposit(5.0);
        assert_eq!(money.val(), 10.0);
    }

    #[test]
    fn test_withdraw() {
        let mut money = Money::new(15.0);
        money.withdraw(5.0);
        assert_eq!(money.val(), 10.0);
    }

    #[test]
    fn test_from_f64() {
        let money: Money = 10.0.into();
        assert_eq!(money.val(), 10.0);
    }

    #[test]
    fn test_display() {
        let money = Money::new(10.0);
        assert_eq!(money.to_string(), "$10.00");
    }

    #[test]
    fn test_partial_eq() {
        let money1 = Money::new(10.00);
        let money2 = Money::new(10.00);

        assert!(money1.val() == money2.val());
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
