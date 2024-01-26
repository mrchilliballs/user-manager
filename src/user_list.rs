use std::collections::HashMap;
use std::fmt::Display;
use std::io::prelude::*;

use crate::user::User;
use crate::username::Username;

use proptest_derive::Arbitrary;

use serde::{Deserialize, Serialize};

use anyhow::Result;

#[derive(Deserialize, Serialize, Debug, Default, Arbitrary, PartialEq, Eq, Clone)]
pub struct UserList {
    users: HashMap<Username, User>,
}

impl UserList {
    pub fn new() -> Self {
        todo!()
    }
    // Maybe: Replace with BufReader and generics for Read
    pub fn load(reader: &impl Read) -> Result<Self> {
        todo!()
    }
    // &self or self?
    // Maybe: Replace with BufReader and generics for Read
    pub fn save(&self, writer: &impl Write) -> Result<String, serde_json::Error> {
        todo!()
    }
    pub fn insert(&mut self, username: Username, user: User) {
        todo!()
    }
    pub fn add(&mut self, user: User) -> Option<()> {
        todo!()
    }
    pub fn get(&self, username: &Username) -> Option<User> {
        todo!()
    }
    pub fn get_all(&self) -> &HashMap<Username, User> {
        todo!()
    }
    pub fn remove(&mut self, username: &Username) -> Option<User> {
        todo!()
    }
    pub fn replace(&mut self, username: &Username, user: User) -> Option<User> {
        todo!()
    }
}

impl Display for UserList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

// Destructure instead of using .1 and .0 all the time
#[cfg(test)]
pub(crate) mod tests {
    use std::{
        io::{Cursor, Read},
        str::FromStr,
    };

    use super::*;

    pub fn example_user_list() -> UserList {
        let mut map = HashMap::new();
        let user1 = example_user_1();
        let user2 = example_user_2();
        map.insert(user1.0, user1.1);
        map.insert(user2.0, user2.1);
        UserList { users: map.into() }
    }

    pub fn example_user_1() -> (Username, User) {
        (
            Username::from_str("WildSir").unwrap(),
            User {
                name: String::from("Wild Sir"),
                ..Default::default()
            },
        )
    }
    pub fn example_user_2() -> (Username, User) {
        (
            Username::from_str("Sir").unwrap(),
            User {
                name: String::from("Sir"),
                money: 10.0.into(),
            },
        )
    }
    pub fn example_user_3() -> (Username, User) {
        (
            Username::from_str("Wild").unwrap(),
            User {
                name: String::from("Wild"),
                money: 10.0.into(),
            },
        )
    }

    #[test]
    fn test_new() {
        let users = UserList::new();
        assert_eq!(users.users, HashMap::new());
    }

    #[test]
    fn test_get() {
        let users = example_user_list();
        let username: Username = "WildSir".parse().unwrap();
        assert_eq!(users.get(&username).unwrap(), example_user_1().1);
    }

    #[test]
    fn test_insert() {
        let mut user_list = example_user_list();
        let user = example_user_3();
        user_list.insert(user.0.clone(), example_user_3().1);
        assert_eq!(user_list.get(&user.0).unwrap(), user.1);
    }

    #[test]
    fn test_insert_replace() {
        let mut user_list = example_user_list();
        let mut user = example_user_1();
        user.1.name = String::from("Mr. WildSir");
        user_list.insert(user.0.clone(), user.1.clone());
        assert_eq!(user_list.get(&user.0).unwrap(), user.1);
        assert_ne!(user_list.get(&user.0).unwrap(), example_user_1().1);
    }

    #[test]
    fn test_get_all() {
        let user_list = example_user_list();

        assert_eq!(user_list.get_all(), &user_list.users);
    }

    #[test]
    fn test_remove() {
        let mut user_list = example_user_list();
        let removed_user = example_user_1();

        assert_eq!(&user_list.remove(&removed_user.0).unwrap(), &removed_user.1);
        assert!(user_list.get(&removed_user.0).is_none())
    }

    #[test]
    fn test_save() {
        let user_list = example_user_list();
        let expected_contents = serde_json::to_string(&user_list).unwrap();
        let mut cursor = Cursor::new(vec![]);

        user_list.save(&mut cursor).unwrap();

        let mut string = String::new();
        cursor.read_to_string(&mut string).unwrap();

        assert_eq!(string, expected_contents);
    }

    #[test]
    fn test_load() {
        let expected_user_list = example_user_list();
        let contents = serde_json::to_string(&expected_user_list).unwrap();

        assert_eq!(
            expected_user_list,
            UserList::load(&contents.as_bytes()).unwrap()
        );
    }

    #[test]
    fn test_display() {
        let user_list = example_user_list();
        let expected_string = "\
         (WildSir) Wild Sir | $0.00
         (Sir) Sir | $10.00
         (Wild) Wild | $10.00";

        assert_eq!(user_list.to_string(), expected_string);
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
