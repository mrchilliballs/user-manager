/*
*/

use dialoguer::Confirm;

fn main() {
    let confirmation = Confirm::new()
        .with_prompt("Do you want to continue?")
        .interact()
        .unwrap();

    if confirmation {
        println!("Looks like you want to continue");
    } else {
        println!("nevermind then :(");
    };
}
