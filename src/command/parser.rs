use super::logger::Logger;
use crate::command::Command;
#[cfg_attr(test, mockall_double::double)]
use crate::user_list::UserList;

#[derive(Debug, PartialEq)]
pub struct CommandParser<'a, T>
where
    T: Logger,
{
    command: Command,
    users: &'a mut UserList,
    logger: &'a T,
}

impl<'a, T> CommandParser<'a, T>
where
    T: Logger,
{
    pub fn new(
        command: Command,
        users: &'a mut UserList,
        logger: &'a T,
        // Normally a file
    ) -> Self {
        CommandParser {
            command,
            users,
            logger,
        }
    }
    pub fn parse(self) {
        match self.command {
            Command::Insert { .. } => self.insert(),
            Command::Edit { .. } => self.edit(),
            Command::Get { .. } => self.get(),
            Command::Withdraw { .. } => self.withdraw(),
            Command::Deposit { .. } => self.deposit(),
            Command::Transfer { .. } => self.transfer(),
            Command::Delete { .. } => self.delete(),
            Command::Clear { .. } => self.clear(),
        }
    }
    // user_list.insert(...)
    pub fn insert(self) {
        if let Command::Insert { username, user } = self.command {
            self.users.insert(username, user);
            dbg!(self.users);
            self.logger.println("Sucessfully inserted user.");
        } else {
            panic!("insert was called with wrong command variant");
        }
    }
    // user_list.get_mut(...)
    fn edit(self) {
        if let Command::Edit { username, user } = self.command {
            let original_user = if let Some(user) = self.users.get(&username) {
                user
            } else {
                self.logger
                    .eprintln(&format!("User \"{username}\" not found."));
                return;
            };
            // TODO: This should maybe be tested for? meh
            let changed_user = user.to_original(original_user);

            self.users.insert(username, changed_user);
            self.logger.println("Sucessfully edited user.")
        }
    }
    // user_list.get(...)
    fn get(self) {
        todo!()
    }
    // user_list.get(...)
    fn withdraw(self) {
        todo!()
    }
    // user_list.deposit(...)
    fn deposit(self) {
        todo!()
    }
    // user_list.deposit(...)
    fn transfer(self) {
        todo!()
    }
    // user_list.delete(...)
    fn delete(self) {
        todo!()
    }
    // user_list.clear(...)
    fn clear(self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    // Note: Do mocking expects before running the actual code being tested.

    use std::collections::BTreeMap;
    use std::str::FromStr;

    use super::*;
    use crate::command::logger::MockLogger;
    use crate::money::Money;
    use crate::user::{OptionalUser, User};
    use crate::username::Username;
    use mockall::predicate::eq;

    macro_rules! define {
        ($username:ident $(, $user:ident)?) => {
            let $username = Username::build("WildSir").unwrap();
            $(let $user = User::default();)?
        };
        ($username:ident, $optional_user:ident? $(,$user:ident)?) => {
            let $username = Username::build("WildSir").unwrap();
            let $optional_user = OptionalUser::default();
            $(let $user = User::default();)?
        };
    }

    struct CmdParserBuilder {
        users: UserList,
        logger: MockLogger,
        command: Command,
    }
    impl CmdParserBuilder {
        fn new(command: Command) -> Self {
            CmdParserBuilder {
                users: UserList::default(),
                logger: MockLogger::default(),
                command,
            }
        }
        fn parser<'a>(&'a mut self) -> CommandParser<'a, MockLogger> {
            CommandParser {
                users: &mut self.users,
                command: self.command.clone(),
                logger: &self.logger,
            }
        }
    }
    #[test]
    fn test_new() {
        todo!()
    }
    #[test]
    fn test_parse() {
        todo!()
    }
    // Template for other tests.
    #[test]
    fn test_insert() {
        define!(username, user);

        let mut builder = CmdParserBuilder::new(Command::Insert {
            username: username.clone(),
            user: user.clone(),
        });

        builder
            .users
            .expect_insert()
            .once()
            .with(eq(username), eq(user))
            .return_const(());

        builder
            .logger
            .expect_println()
            .with(eq("Sucessfully inserted user."))
            .once()
            .return_const(());

        let parser = builder.parser();
        parser.insert();
        drop(builder);
    }

    #[test]
    fn test_edit_valid_username() {
        define!(username, optional_user?, user);

        // TODO: fix memory leak
        let user = Box::leak(Box::new(user));

        let mut builder = CmdParserBuilder::new(Command::Edit {
            username: username.clone(),
            user: optional_user,
        });

        builder
            .users
            .expect_insert()
            .with(eq(username.clone()), eq(user.clone()))
            .once()
            .return_const(());

        builder
            .users
            .expect_get()
            .with(eq(username))
            .once()
            .return_const(Some(&*user));

        builder
            .logger
            .expect_println()
            .with(eq("Sucessfully edited user."))
            .once()
            .return_const(());

        let parser = builder.parser();
        parser.edit();
    }

    #[test]
    fn test_edit_invalid_username() {
        define!(username, optional_user?);

        let mut builder = CmdParserBuilder::new(Command::Edit {
            username,
            user: optional_user,
        });

        builder.users.expect_get().return_const(None);

        builder
            .logger
            .expect_eprintln()
            .with(eq("User \"WildSir\" not found."))
            .return_const(());

        let parser = builder.parser();
        parser.edit();
    }

    #[test]
    fn test_get_all_no_users() {
        let mut builder = CmdParserBuilder::new(Command::Get { username: None });

        builder
            .users
            .expect_get_all()
            .once()
            .return_const(BTreeMap::new());

        builder
            .logger
            .expect_println()
            .once()
            .with(eq("No users found."))
            .return_const(());

        let parser = builder.parser();

        parser.get();
    }

    fn user_1() -> (Username, User) {
        (
            Username::from_str("WildSir").unwrap(),
            User {
                name: String::from("Wild Sir"),
                ..Default::default()
            },
        )
    }

    fn user_2() -> (Username, User) {
        (
            Username::from_str("Sir").unwrap(),
            User {
                name: String::from("Sir"),
                money: 10.0.into(),
                ..Default::default()
            },
        )
    }
    #[test]
    fn test_get_all() {
        let mut builder = CmdParserBuilder::new(Command::Get { username: None });

        let mut map = BTreeMap::new();
        let user_1 = user_1();
        let user_2 = user_2();

        map.insert(user_1.0, user_1.1);
        map.insert(user_2.0, user_2.1);
        let display_contents = format!(
            "Users:\n{}",
            crate::UserList::from(map.clone())
                .to_string()
                .lines()
                .map(|line| format!("   {line}"))
                .collect::<String>()
        );

        builder.users.expect_get_all().once().return_const(map);

        builder
            .logger
            .expect_println()
            .with(eq(display_contents))
            .once()
            .return_const(());

        let parser = builder.parser();

        parser.get();
    }
    #[test]
    fn test_get_one_valid() {
        define!(username);

        let user = Box::leak(Box::new(user_1().1));

        let mut builder = CmdParserBuilder::new(Command::Get {
            username: Some(username.clone()),
        });

        builder
            .users
            .expect_get()
            .with(eq(username))
            .once()
            .return_const(&*user);

        builder
            .logger
            .expect_println()
            .with(eq(user.to_string()))
            .once()
            .return_const(());

        let parser = builder.parser();
        parser.get();
    }
    #[test]
    fn test_get_one_invalid() {
        define!(username);
        let mut builder = CmdParserBuilder::new(Command::Get {
            username: Some(username.clone()),
        });

        builder
            .users
            .expect_get()
            .with(eq(username))
            .once()
            .return_const(None);

        builder
            .logger
            .expect_eprintln()
            .with(eq("User not found."))
            .once()
            .return_const(());

        let parser = builder.parser();
        parser.get();
    }

    #[test]
    fn test_withdraw_negative() {
        define!(username);
        let mut builder = CmdParserBuilder::new(Command::Withdraw {
            username: username.clone(),
            amount: Money::from(-10.0),
        });

        // builder
        //     .users
        //     .expect_get()
        //     .with(eq(username))
        //     .once()
        //     .return_const(Some(&*user));

        builder
            .logger
            .expect_eprintln()
            .with(eq(
                "Cannot withdraw a negative value. Please enter a positive float literal.",
            ))
            .once()
            .return_const(());

        let parser = builder.parser();
        parser.withdraw();
    }

    #[test]
    fn test_withdraw_invalid_user() {
        define!(username, user);
        let user = Box::leak(Box::new(user));
        let mut builder = CmdParserBuilder::new(Command::Withdraw {
            username: username.clone(),
            amount: Money::from(10.0),
        });

        builder
            .users
            .expect_get()
            .once()
            .with(eq(username))
            .return_const(&*user);

        builder
            .logger
            .expect_eprintln()
            .once()
            .with(eq("User not found."))
            .return_const(());

        let parser = builder.parser();
        parser.withdraw();
    }

    #[test]
    fn test_withdraw_valid() {
        todo!()
    }

    #[test]
    fn test_deposit() {
        todo!()
    }
    #[test]
    fn test_transfer() {
        todo!()
    }
    #[test]
    fn test_delete() {
        todo!()
    }
    #[test]
    fn test_clear() {
        todo!()
    }
}
