use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::Display,
};

#[derive(Debug, PartialEq, Eq, Default)]
pub struct User {
    // should not be the same
    money: i32,
    name: String,
}
impl User {
    pub fn new(name: &str, initial_money: i32) -> Self {
        User {
            money: initial_money,
            name: name.to_string(),
        }
    }
    pub fn money(&self) -> i32 {
        self.money
    }
    pub fn edit_money(&mut self, amount: i32) {
        self.money += amount;
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn set_name(&mut self, new_name: &str) {
        self.name = new_name.to_string();
    }
}
impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "(Name: {}, Money: {})", self.name, self.money)
    }
}

#[derive(Debug, Default)]
pub struct UserList {
    users: HashMap<String, User>,
}
impl UserList {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn get_users(&self) -> &HashMap<String, User> {
        &self.users
    }
    pub fn get_user(&self, username: &str) -> Option<&User> {
        self.users.get(&username.to_string())
    }
    pub fn add_user(&mut self, username: &str, user: User) -> &User {
        self.users.entry(username.to_lowercase()).or_insert(user)
    }
    pub fn edit_user(&mut self, username: &str, new_user: User) -> Option<User> {
        if let Entry::Occupied(mut entry) = self.users.entry(username.to_lowercase()) {
            Some(entry.insert(new_user))
        } else {
            None
        }
    }
    pub fn remove_user(&mut self, username: &str) -> Option<User> {
        self.users.remove(&username.to_lowercase())
    }
    pub fn username_taken(&self, username: &str) -> bool {
        self.users.contains_key(&username.to_string())
    }
}

impl Display for UserList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for user in &self.users {
            writeln!(f, "{} {}", user.0, user.1)?;
        }
        if self.users.is_empty() {
            write!(f, "No users found")?;
        }
        Ok(())
    }
}
