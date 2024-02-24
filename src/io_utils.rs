use std::io;

use dialoguer::FuzzySelect;

//make return option
pub fn read_input(prompt: &str) -> Option<String> {
    println!("{prompt}");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    let input = input.trim().to_string();
    if !input.is_empty() {
        Some(input)
    } else {
        None
    }
}
pub fn require_input(prompt: &str) -> String {
    loop {
        let input = read_input(prompt);
        match input {
            Some(input) => break input,
            None => continue,
        };
    }
}

pub fn fuzzy_select(prompt: &str, items: &[String]) -> usize {
    FuzzySelect::new()
        .with_prompt(prompt)
        .items(items)
        .interact()
        .unwrap()
}
