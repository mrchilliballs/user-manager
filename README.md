# User Management System
`user-manager` provides utilities to manage users and a fully featured CLI.

# Usage
```bash
user-manager --help
```
```
A CLI for managing users.

Usage: user-manager <COMMAND>

Commands:
  insert    Adds a new user, replacing any existing user with the same username
  edit      Modify fields for an existing user
  get       Display the fields of a specific user or list all users if none is specified
  withdraw  Deduct the specified amount from a user's account
  deposit   Credit the specified amount to a user's account
  transfer  Move funds between two user accounts
  delete    Permanently remove a user
  clear     Permanently remove all users
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

# Building
You will need Rust's toolchain installed. On Unix-like OSes, you can enter the following in your terminal, and follow the instructions.
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Then, clone this repo with git like the following:
```bash
git clone https://gitlab.com/WildSir/user-manager.git
```
Finally, build the code with cargo:
```bash
cargo build --release
```
The binary should be found at `/target/release`.