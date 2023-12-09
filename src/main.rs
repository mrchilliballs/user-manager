use std::process;

fn main() {
    user_manager::run().unwrap_or_else(|error| {
        eprintln!("{error}");
        process::exit(0);
    })
}
