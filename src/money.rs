use std::{fmt::Display, num::ParseFloatError, str::FromStr};

#[cfg(test)]
use mockall::mock;
use serde::{Deserialize, Serialize};

/// Wrapper of i64 used for holding money with basic methods provided, where each unit represents a cent.
/// This allows for accurate and precise handling of currency values.
#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
pub struct Money(i64);

impl Money {
    /// Creates an instance of Money holding `amount` (in cents).
    pub fn new(amount: i64) -> Self {
        Money(amount)
    }
    /// Returns the amount of money.
    pub fn val(&self) -> i64 {
        self.0
    }
    /// Sets the amount of money.
    pub fn set(&mut self, amount: i64) {
        self.0 = amount
    }
    /// Withdraws (subtracts) the amount from the money.
    /// # Panics
    /// * Will panic if computation overflows.
    pub fn withdraw(&mut self, amount: i64) {
        self.0 -= amount;
    }
    /// Deposits (adds) the amount to the money.
    /// # Panics
    /// * Will panic if computation overflows.
    pub fn deposit(&mut self, amount: i64) {
        self.0 += amount;
    }
}

impl From<i64> for Money {
    /// Creates a new instance of money in cents.
    fn from(value: i64) -> Self {
        Money::new(value)
    }
}

impl FromStr for Money {
    type Err = ParseFloatError;
    // Input may start with a dollar sign ($), and the value should be formatted as float.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let amount_str = if s.starts_with("$") { &s[1..] } else { s };
        Ok(Money::from((amount_str.parse::<f64>()? * 100.0) as i64))
    }
}

impl Display for Money {
    /// Displays the money followed by a dollar sign and round to the nearest tenth.
    /// Examples:
    /// `$100.00`
    /// `$99.99`
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Maybe custom currencies?
        write!(f, "${:.2}", self.0 as f64 / 100.0)
    }
}

impl PartialEq for Money {
    fn eq(&self, other: &Self) -> bool {
        self.val() == other.val()
    }
}
impl PartialEq<i64> for Money {
    fn eq(&self, other: &i64) -> bool {
        self.val() == *other
    }
}
impl Eq for Money {}

#[derive(Clone)]
pub struct MoneyParser;

#[cfg(test)]
mock! {
    #[derive(Debug)]
    pub Money {
        pub fn new(amount: i64) -> Self;
        pub fn val(&self) -> i64;
        pub fn set(&mut self, amount: i64);
        pub fn withdraw(&mut self, amount: i64);
        pub fn deposit(&mut self, amount: i64);
    }
    impl FromStr for Money {
        type Err = anyhow::Error;
        fn from_str(s: &str) -> Result<Self, <MockMoney as FromStr>::Err>;
    }
    impl PartialEq for Money {
        fn eq(&self, other: &Self) -> bool;
    }
    impl Clone for Money {
        fn clone(&self) -> Self;
    }
    impl From<i64> for Money {
        fn from(value: i64) -> Self;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let money = Money::new(1000);
        assert_eq!(money.0, 1000);
    }
    #[test]
    fn test_val() {
        let money = Money::new(1000);
        assert_eq!(money.0, money.val());
    }

    #[test]
    fn test_set() {
        let mut money = Money::new(500);
        money.set(1000);
        assert_eq!(money.val(), 1000);
    }

    #[test]
    fn test_deposit() {
        let mut money = Money::new(500);
        money.deposit(500);
        assert_eq!(money.val(), 1000);
    }

    #[test]
    fn test_withdraw() {
        let mut money = Money::new(1500);
        money.withdraw(500);
        assert_eq!(money.val(), 1000);
    }

    #[test]
    fn test_from_str() {
        let money_str = "100.50";
        let money = Money::from_str(money_str).unwrap();

        assert_eq!(money.val(), 10050);
    }

    #[test]
    fn test_from_str_with_currency() {
        let money_str = "$100.50";
        let money = Money::from_str(money_str).unwrap();

        assert_eq!(money.val(), 10050);
    }

    #[test]
    fn test_from_f64() {
        let money: Money = 1000.into();
        assert_eq!(money.val(), 1000);
    }

    #[test]
    fn test_display() {
        let money = Money::new(1000);
        assert_eq!(money.to_string(), "$10.00");
    }

    #[test]
    fn test_partial_eq() {
        let money1 = Money::new(1000);
        let money2 = Money::new(1000);

        assert!(money1 == money2);
    }

    #[test]
    fn test_partial_eq_f64() {
        let money = Money::new(1000);
        let money_float = 1000;

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
