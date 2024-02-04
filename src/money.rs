use std::{fmt::Display, str::FromStr};

use proptest_derive::Arbitrary;
use serde::{Deserialize, Serialize};

/// Wrapper of f64 used for holding money with basic methods provided.
#[derive(Serialize, Deserialize, Debug, Default, Arbitrary, Clone, Copy)]
pub struct Money(f64);

impl Money {
    /// Creates an instance of Money holding `amount`.
    pub fn new(amount: f64) -> Self {
        Money(amount)
    }
    /// Returns the amount of money.
    pub fn val(&self) -> f64 {
        self.0
    }
    /// Sets the amount of money.
    pub fn set(&mut self, amount: f64) {
        self.0 = amount
    }
    /// Withdraws (subtracts) the amount from the money.
    /// # Panics
    /// * Will panic if computation overflows.
    pub fn withdraw(&mut self, amount: f64) {
        self.0 -= amount;
    }
    /// Deposits (adds) the amount to the money.
    /// # Panics
    /// * Will panic if computation overflows.
    pub fn deposit(&mut self, amount: f64) {
        self.0 += amount;
    }
}

impl From<f64> for Money {
    fn from(value: f64) -> Self {
        Money(value)
    }
}

impl FromStr for Money {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Money::from(s.parse::<f64>()?))
    }
}

impl Display for Money {
    /// Displays the money followed by a dollar sign and round to the nearest tenth.
    /// Examples:
    /// `$100.00`
    /// `$99.99`
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${:.2}", self.0)
    }
}

impl PartialEq for Money {
    fn eq(&self, other: &Self) -> bool {
        self.val() == other.val()
    }
}
impl PartialEq<f64> for Money {
    fn eq(&self, other: &f64) -> bool {
        self.val() == *other
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
    fn test_from_str() {
        let money_str = "100.50";
        assert!(Money::from_str(money_str).is_ok());
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
        let money1 = Money::new(10.0);
        let money2 = Money::new(10.0);

        assert!(money1 == money2);
    }

    #[test]
    fn test_partial_eq_f64() {
        let money = Money::new(10.0);
        let money_float = 10.0;

        assert!(money == money_float);
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
