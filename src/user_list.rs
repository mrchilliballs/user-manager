use std::collections::HashMap;
use std::fmt::Display;
use std::io::{self, BufRead};

use crate::user::User;
use crate::username::Username;

use proptest_derive::Arbitrary;

use serde::{Deserialize, Serialize};

use anyhow::Result;

#[derive(Deserialize, Serialize, Debug, Default, Arbitrary)]
pub struct UserList {
    users: HashMap<Username, User>,
}

impl UserList {
    pub fn new() -> Self {
        todo!()
    }
    pub fn deserialize(file: &mut impl BufRead) -> Result<Self> {
        todo!()
    }
    // &self or self?
    pub fn serialize(&self) -> Result<String, serde_json::Error> {
        todo!()
    }
    pub fn insert(&mut self, user: User) {
        todo!()
    }
    pub fn add(&mut self, user: User) -> Option<()> {
        todo!()
    }
    pub fn get(&self, username: &str) -> Option<User> {
        todo!()
    }
    pub fn get_all(&self) -> &HashMap<String, User> {
        todo!()
    }
    pub fn remove(&mut self, username: &str) -> Option<User> {
        todo!()
    }
    pub fn replace(&mut self, username: &str, user: User) -> Option<User> {
        todo!()
    }
}

impl Display for UserList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

// Tests for operations with invalid values, like usernames, potentially needed
#[cfg(test)]
mod tests {
    #[test]
    fn test_new() {
        todo!()
    }

    #[test]
    fn test_serialize() {
        todo!()
    }

    #[test]
    fn test_deserialize() {
        todo!()
    }
    #[test]
    fn test_insert() {
        todo!()
        // Tests for repetition behavior too
    }

    #[test]
    fn test_add() {
        todo!()
        // Tests for repetition behavior too
    }

    #[test]
    fn test_get() {
        todo!()
    }

    #[test]
    fn test_get_all() {
        todo!()
    }

    #[test]
    fn test_remove() {
        todo!()
    }

    #[test]
    fn test_display() {
        todo!()
    }

    // TODO: Property based testing
    // use super::*;
    // use proptest::prelude::*;
    // proptest! {
    //     fn test_insert_arbitrary_user(user: User, user_list: UserList) {
    //         todo!()
    //     }
    //     fn test_add_arbitrary_user(user: User, user_list: UserList) {
    //         todo!()
    //     }
    //     fn test_get_valid_user_from_arbitrary_list(user_list: UserList) {
    //         todo!()
    //     }
    //     fn test_remove_valid_user_from_arbitrary_list(user_list: UserList) {
    //         todo!()
    //     }
    //     fn test_replace_valid_user_from_arbitrary_list(user_list: UserList) {
    //         todo!()
    //     }
    // }
}
