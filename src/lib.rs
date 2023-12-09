pub mod io_utils;
pub mod menu;
pub mod user;

use std::error::Error;

use io_utils::fuzzy_select;
use user::{User, UserList};

use console::Term;

use crate::io_utils::read_input;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut user_list = UserList::new();
    loop {
        if show_menu(&mut user_list).is_none() {
            break;
        }
    }
    Ok(())
}

fn show_menu(user_list: &mut UserList) -> Option<()> {
    let terminal = Term::stdout();
    terminal.clear_screen().unwrap();
    // TODO: Stderr should be red
    terminal.style().red().for_stderr();
    let items: Vec<String> = [
        "1. Add User",
        "2. Edit User",
        "3. Transfer Money",
        "4. Remove User",
        "5. Display Users",
        "6. Exit Program",
    ]
    .iter()
    .map(|i| i.to_string())
    .collect();
    let action = fuzzy_select("What would you like to do?", &items);
    println!();
    match action {
        0 => menu::add_user(user_list),
        1 => menu::edit_user(user_list),
        2 => menu::transfer_money(user_list),
        3 => todo!(),
        4 => menu::display_users(user_list),
        5 => return None,
        _ => return None,
    };

    // Some need more time
    read_input("\nPlease enter to continue...");

    Some(())
}
