use std::collections::BTreeMap;
use std::fmt::Display;
use std::io::{self, prelude::*, BufWriter};

use crate::user::User;
use crate::username::Username;

#[cfg(test)]
use mockall::mock;

use serde::{Deserialize, Serialize};

use anyhow::Result;

/// Holds a list of users identified by usernames. No duplicates are held and entries are sorted.
#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Eq, Clone)]
pub struct UserList {
    users: BTreeMap<Username, User>,
}

impl UserList {
    /// Creates an empty UserList.
    pub fn new() -> Self {
        Self {
            users: BTreeMap::new(),
        }
    }
    // Maybe: Replace with BufReader and generics for Read
    /// Loads a UserList from JSON. If provided, will save to the provided file path.
    pub fn load(reader: &mut impl BufRead) -> Result<Self, io::Error> {
        let users: UserList = serde_json::from_reader(reader).map_err(Into::<io::Error>::into)?;
        // TEST THIS
        Ok(users)
    }
    // &self or self?
    // Maybe: Replace with BufReader and generics for Read
    /// Parses UserList to JSON.
    pub fn save(&self, writer: &mut impl Write) -> Result<(), io::Error> {
        let mut buf_writer = BufWriter::new(writer);
        serde_json::to_writer(&mut buf_writer, self)?;
        Ok(())
    }
    /// Inserts a new entry to UserList, overwriting any entries with the same username.
    pub fn insert(&mut self, username: Username, user: User) {
        self.users.insert(username, user);
    }
    /// Adds a new entry to UserList, returning it if it already exists
    pub fn add(&mut self, username: Username, user: User) -> &User {
        self.users.entry(username).or_insert(user)
    }
    /// Gets a user from UserList, returning None if it does not exist.
    pub fn get(&self, username: &Username) -> Option<&User> {
        self.users.get(username)
    }
    /// Gets a mutable reference to a user in UserList, returning None if it does not exist.
    pub fn get_mut(&mut self, username: &Username) -> Option<&mut User> {
        self.users.get_mut(username)
    }
    /// Gets all users from UserList.
    pub fn get_all(&self) -> &BTreeMap<Username, User> {
        &self.users
    }
    /// Removes a user permanently, and returns the removed user. It returns None if it does not exist.
    pub fn remove(&mut self, username: &Username) -> Option<User> {
        self.users.remove(username)
    }

    // TODO: Clear
}

impl Display for UserList {
    /// Will display user sorted in the format "({username}): {user}", with new users separated by newlines.
    /// # Examples
    /// ```text
    /// (JoeD): John Doe | $0.00
    /// (WildSir): Wild Sir | $10.00
    /// (Zach): Zachary Johnson | $100.000
    /// ```
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

impl From<BTreeMap<Username, User>> for UserList {
    fn from(value: BTreeMap<Username, User>) -> Self {
        Self { users: value }
    }
}
// impl Drop for UserList {
//     fn drop(&mut self) {
//         let save_location = if let Some(save_location) = self.save_location.clone() {
//             save_location
//         } else {
//             return;
//         };
//         let save_loc_str = save_location
//             .as_os_str()
//             .to_str()
//             .unwrap_or("(path is not valid UTF-8; cannot be displayed)");
//         let file = match File::options()
//             .write(true)
//             .truncate(true)
//             .open(&save_location)
//         {
//             Ok(file) => file,
//             Err(err) => {
//                 eprintln!(
//                     "Failed to save to \"{}\" because of error: {}",
//                     save_loc_str, err
//                 );
//                 return;
//             }
//         };
//         let mut buf = BufWriter::new(file);
//         if let Err(err) = self.save(&mut buf, None) {
//             eprintln!(
//                 "Failed to write to file \"{}\" because of error: {}",
//                 save_loc_str, err
//             );
//             return;
//         }
//     }
// }
#[cfg(test)]
mock! {
    #[derive(Debug)]
    pub UserList {
        pub fn new() -> Self;
        pub fn load<T: 'static + BufRead>(reader: &mut T) -> Result<Self, serde_json::Error>;
        pub fn save<T: 'static + Write>(&self, writer: &mut T) -> Result<(), serde_json::Error>;
        pub fn insert(&mut self, username: Username, user: User);
        pub fn add(&mut self, username: Username, user: User) -> Option<()>;
        pub fn get<'a>(&'a self, username: &Username) -> Option<&'a User>;
        pub fn get_mut(&mut self, username: &Username) -> Option<&'static mut User>;
        pub fn get_all(&self) -> &BTreeMap<Username, User>;
        pub fn remove(&mut self, username: &Username) -> Option<User>;
    }
    impl PartialEq for UserList {
        fn eq(&self, other: &Self) -> bool;
    }
    impl Display for UserList {
        fn fmt<'a>(&self, f: &mut std::fmt::Formatter<'a>) -> std::fmt::Result;
    }
}

// Destructure instead of using .1 and .0 all the time
#[cfg(test)]
mod tests {
    use std::{
        io::{Cursor, Read},
        str::FromStr,
    };

    use crate::money::Money;

    use super::*;

    fn example_user_list() -> UserList {
        let mut map = BTreeMap::new();
        let user1 = example_user_1();
        let user2 = example_user_2();
        map.insert(user1.0, user1.1);
        map.insert(user2.0, user2.1);
        UserList { users: map.into() }
    }

    fn example_user_1() -> (Username, User) {
        (
            Username::from_str("WildSir").unwrap(),
            User {
                name: String::from("Wild Sir"),
                ..Default::default()
            },
        )
    }
    fn example_user_2() -> (Username, User) {
        (
            Username::from_str("Sir").unwrap(),
            User {
                name: String::from("Sir"),
                ..Default::default()
            },
        )
    }
    fn example_user_3() -> (Username, User) {
        (
            Username::from_str("Wild").unwrap(),
            User {
                name: String::from("Wild"),
                ..Default::default()
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
            money: Money::default(),
            ..Default::default()
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

    #[ignore]
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
        // Make JSON PRETTY AND CHECK FOR THAT
        todo!()
    }

    #[ignore]
    #[test]
    fn test_load() {
        let expected_user_list = example_user_list();
        let contents = serde_json::to_string(&expected_user_list).unwrap();

        assert_eq!(
            expected_user_list,
            UserList::load(&mut contents.as_bytes()).unwrap()
        );
        // Make JSON PRETTY AND CHECK FOR THAT
        todo!()
    }

    #[test]
    fn test_display() {
        let user_list = example_user_list();
        let expected_string = "\
(Sir) Sir | $0.00
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
