use std::collections::BTreeMap;
use std::fmt::Display;
use std::io::{prelude::*, BufWriter};

use crate::user::User;
use crate::username::Username;

use proptest_derive::Arbitrary;

use serde::{Deserialize, Serialize};

use anyhow::Result;

#[derive(Deserialize, Serialize, Debug, Default, Arbitrary, PartialEq, Eq, Clone)]
pub struct UserList {
    users: BTreeMap<Username, User>,
}

impl UserList {
    pub fn new() -> Self {
        Self::default()
    }
    // Maybe: Replace with BufReader and generics for Read
    pub fn load(reader: &mut impl BufRead) -> Result<Self, serde_json::Error> {
        serde_json::from_reader(reader)
    }
    // &self or self?
    // Maybe: Replace with BufReader and generics for Read
    pub fn save(&self, writer: &mut (impl Write + std::fmt::Debug)) -> Result<(), anyhow::Error> {
        let mut buf_writer = BufWriter::new(writer);
        serde_json::to_writer(&mut buf_writer, self)?;
        Ok(())
    }
    pub fn insert(&mut self, username: Username, user: User) {
        self.users.insert(username, user);
    }
    pub fn add(&mut self, username: Username, user: User) -> Option<()> {
        if !self.users.contains_key(&username) {
            self.insert(username, user);
            Some(())
        } else {
            None
        }
    }
    pub fn get(&self, username: &Username) -> Option<&User> {
        self.users.get(username)
    }
    pub fn get_mut(&mut self, username: &Username) -> Option<&mut User> {
        self.users.get_mut(username)
    }
    pub fn get_all(&self) -> &BTreeMap<Username, User> {
        &self.users
    }
    pub fn remove(&mut self, username: &Username) -> Option<User> {
        self.users.remove(username)
    }
}

impl Display for UserList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.get_all().iter().peekable();
        while let Some((username, user)) = iter.next() {
            write!(f, "({username}) {user}")?;
            if iter.peek().is_some() {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

// Destructure instead of using .1 and .0 all the time
#[cfg(test)]
pub(crate) mod tests {
    use std::{
        io::{Cursor, Read},
        str::FromStr,
    };

    use crate::money::Money;

    use super::*;

    pub fn example_user_list() -> UserList {
        let mut map = BTreeMap::new();
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
        assert_eq!(users, UserList::default());
    }

    #[test]
    fn test_get() {
        let users = example_user_list();
        let username: Username = "WildSir".parse().unwrap();
        assert_eq!(users.get(&username).unwrap(), &example_user_1().1);
    }

    #[test]
    fn test_get_mut() {
        let mut users = example_user_list();
        let username: Username = "WildSir".parse().unwrap();
        assert_eq!(users.get_mut(&username).unwrap(), &mut example_user_1().1);
    }

    #[test]
    fn test_insert() {
        let mut user_list = example_user_list();
        let user = example_user_3();
        user_list.insert(user.0.clone(), example_user_3().1);
        assert_eq!(user_list.get(&user.0).unwrap(), &user.1);
    }

    #[test]
    fn test_add_normal() {
        let mut user_list = example_user_list();
        let user = example_user_3();
        user_list.add(user.0.clone(), example_user_3().1);
        assert_eq!(user_list.get(&user.0).unwrap(), &user.1);
    }

    #[test]
    fn test_add_existing() {
        let mut user_list = example_user_list();
        let mut user = example_user_1();
        user.1 = User {
            name: String::from("Sir"),
            money: Money::new(0.0),
        };
        user_list.add(user.0.clone(), user.1.clone());
        assert_ne!(user_list.get(&user.0).unwrap(), &user.1);
    }

    #[test]
    fn test_insert_replace() {
        let mut user_list = example_user_list();
        let mut user = example_user_1();
        user.1.name = String::from("Mr. WildSir");
        user_list.insert(user.0.clone(), user.1.clone());
        assert_eq!(user_list.get(&user.0).unwrap(), &user.1);
        assert_ne!(user_list.get(&user.0).unwrap(), &example_user_1().1);
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

        cursor.set_position(0);

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
            UserList::load(&mut contents.as_bytes()).unwrap()
        );
    }

    #[test]
    fn test_display() {
        let user_list = example_user_list();
        let expected_string = "\
(Sir) Sir | $10.00
(WildSir) Wild Sir | $0.00";

        assert!(
            user_list.to_string() == expected_string || user_list.to_string() == expected_string
        );
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
