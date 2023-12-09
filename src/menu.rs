use std::num::IntErrorKind;

use crate::{io_utils, User, UserList};

pub fn add_user(user_list: &mut UserList) {
    let username =
        io_utils::require_input("Please enter the user's username (enter \"cancel\" to cancel):");
    if username == "cancel" {
        println!("Canceled adding a user.");
        return;
    }
    let name = io_utils::require_input("Please enter the user's name:");
    let money = loop {
        if let Some(input) =
            io_utils::read_input("Please enter the user's money (leave blank for default):")
        {
            if let Ok(val) = input.parse() {
                break val;
            } else {
                println!("Please enter an integer.");
                continue;
            }
        } else {
            break 0;
        }
    };
    let user = User::new(&name, money);
    user_list.add_user(&username, user);
    println!("User succesfully added!")
}
pub fn edit_user(user_list: &mut UserList) {
    let mut users_usernames: Vec<String> =
        user_list.get_users().iter().map(|b| b.0).cloned().collect();
    if users_usernames.is_empty() {
        println!("No users found.");
        return;
    }
    users_usernames.insert(0, String::from("Cancel"));
    let username_index = io_utils::fuzzy_select(
        "Please enter the user you would like to modify's username (choose \"Cancel\" to cancel):",
        &users_usernames,
    );
    if username_index == 0 {
        println!("Cancelled editing a user.");
        return;
    }
    let username = &users_usernames[username_index];
    let user = match user_list.get_user(username) {
        Some(user) => user,
        None => {
            println!("User not found.");
            return;
        }
    };
    let name = io_utils::read_input("Please enter the new name (leave blank to keep the same):")
        .unwrap_or(user.name().to_string());
    let money = loop {
        if let Some(money) = io_utils::read_input("Please enter the amount of money you wish to increase (use negatives to decrease; leave blank to keep the same):") {
            match money.parse::<i32>() {
                Ok(val) => break val,
                Err(error) => {
                    match error.kind() {
                        IntErrorKind::NegOverflow | IntErrorKind::PosOverflow => eprintln!("The value entered is too large or too small."),
                        IntErrorKind::InvalidDigit => eprintln!("Please enter a valid integer."),
                        _ => eprintln!("An unkown error has occured: {}", error),
                    }
                    continue;
                }
            }
        } else {
            break user.money()
        }
    };
    let user = User::new(&name, money);
    user_list.edit_user(username, user);
    println!("User succesfully edited.")
}

pub fn display_users(user_list: &UserList) {
    println!("Users");
    println!("{}", user_list)
}

pub fn transfer_money(_user_list: &mut UserList) {
    todo!()
}
