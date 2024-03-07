use super::logger::Logger;
use crate::command::Command;
#[mockall_double::double]
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
            self.logger.println("Sucessfully inserted user.");
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
            let changed_user = user.to_original(original_user.clone());

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
        if let Command::Withdraw { username, amount } = self.command {
            let user = if let Some(user) = self.users.get_mut(&username) {
                user
            } else {
                self.logger
                    .eprintln(&format!("User \"{username}\" not found."));
                return;
            };

            user.money.withdraw(amount.val());

            self.logger.println("Sucessfully withdrew amount.");
        }
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

    use std::ops::{Deref, DerefMut};

    use super::*;
    use crate::command::logger::MockLogger;
    use crate::money::Money;
    use crate::user::{OptionalUser, User};
    use crate::username::Username;
    use mockall::predicate::eq;

    macro_rules! define {
        {} => {};
        { $var:ident = $value:expr $(, $($tail:tt)*)?} => {
            let $var = $value;
            define!($($($tail)*)?);
        };
        { $var:ident: $type:ty $(, $($tail:tt)*)?} => {
            let $var = <$type>::default();
            define!($($($tail)*)?)
        };
        { mut $var:ident = $value:expr $(, $($tail:tt)*)?} => {
            let mut $var = $value;
            define!($($($tail)*)?);
        };
        { mut $var:ident: $type:ty $(, $($tail:tt)*)?} => {
            let mut $var = <$type>::default();
            define!($($($tail)*)?)
        }
    }

    fn leak<T>(val: T) -> &'static T {
        Box::leak(Box::new(val))
    }

    fn leak_mut<T>(val: T) -> &'static mut T {
        Box::leak(Box::new(val))
    }
    fn leak_mut_option<T>(val: T) -> OptionStatic<T> {
        OptionStatic(val)
    }

    #[derive(Clone)]
    struct OptionStatic<T>(T);

    impl<T> Into<Option<&'static mut T>> for OptionStatic<T> {
        fn into(self) -> Option<&'static mut T> {
            Some(leak_mut(self.0))
        }
    }

    impl<T> Into<Option<&'static T>> for OptionStatic<T> {
        fn into(self) -> Option<&'static T> {
            Some(leak(self.0))
        }
    }

    impl<T> Deref for OptionStatic<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T> DerefMut for OptionStatic<T> {
        fn deref_mut(&mut self) -> &mut T {
            &mut self.0
        }
    }

    #[ignore]
    #[test]
    fn test_parse() {
        // Is it even possible?
        todo!()
    }

    #[test]
    fn test_new() {
        define! {
            command = Command::Get {
                username: None
            },
            mut users: UserList,
            mut logger: MockLogger,
        };

        users.expect_eq().once().return_const(true);
        logger.expect_eq().once().return_const(true);

        let parser = CommandParser::new(command.clone(), &mut users, &mut logger);

        assert_eq!(parser.command, command);
        assert_eq!(parser.users, &mut UserList::default());
        assert_eq!(parser.logger, &mut MockLogger::default());
    }

    #[test]
    fn test_insert() {
        define! {
            mut users: UserList,
            mut logger: MockLogger,
            username = Username::build("WildSir").unwrap(),
            user: User,
        };

        users
            .expect_insert()
            .once()
            .with(eq(username.clone()), eq(user.clone()))
            .return_const(());

        logger
            .expect_println()
            .once()
            .with(eq("Sucessfully inserted user."))
            .return_const(());

        let parser =
            CommandParser::new(Command::Insert { username, user }, &mut users, &mut logger);

        parser.insert();
    }

    #[test]
    fn test_edit_valid_username() {
        define! {
            username = Username::build("WildSir").unwrap(),
            optional_user: OptionalUser,
            user: User,
            mut users: UserList,
            mut logger: MockLogger,
        };

        users
            .expect_insert()
            .once()
            .with(
                eq(username.clone()),
                eq(optional_user.clone().to_original(user.clone())),
            )
            .return_const(());

        users
            .expect_get()
            .once()
            .with(eq(username.clone()))
            .return_const(leak(user));

        logger
            .expect_println()
            .once()
            .with(eq("Sucessfully edited user."))
            .return_const(());

        let parser = CommandParser::new(
            Command::Edit {
                username,
                user: optional_user,
            },
            &mut users,
            &mut logger,
        );

        parser.edit();
    }

    #[test]
    fn test_edit_invalid_username() {
        define! {
            username = Username::build("WildSir").unwrap(),
            optional_user: OptionalUser,
            mut users: UserList,
            mut logger: MockLogger,
        };

        users
            .expect_get()
            .once()
            .with(eq(username.clone()))
            .return_const(None);

        logger
            .expect_eprintln()
            .once()
            .with(eq("User \"WildSir\" not found."))
            .return_const(());

        let parser = CommandParser::new(
            Command::Edit {
                username,
                user: optional_user,
            },
            &mut users,
            &mut logger,
        );

        parser.edit();
    }

    #[test]
    fn test_get_valid_username() {
        define! {
            username = Username::build("WildSir").unwrap(),
            user: User,
            mut users: UserList,
            mut logger: MockLogger,
        };

        users
            .expect_get()
            .once()
            .with(eq(username.clone()))
            .return_const(leak(user.clone()));

        logger
            .expect_println()
            .once()
            .with(eq(user.to_string()))
            .return_const(());

        let parser = CommandParser::new(
            Command::Get {
                username: Some(username),
            },
            &mut users,
            &mut logger,
        );

        parser.get();
    }

    #[test]
    fn test_get_invalid_username() {
        define! {
            username = Username::build("WildSir").unwrap(),
            mut users: UserList,
            mut logger: MockLogger,
        };

        users
            .expect_get()
            .once()
            .with(eq(username.clone()))
            .return_const(None);

        logger
            .expect_eprintln()
            .once()
            .with(eq("User \"WildSir\" not found."))
            .return_const(());

        let parser = CommandParser::new(
            Command::Get {
                username: Some(username),
            },
            &mut users,
            &mut logger,
        );

        parser.get();
    }

    #[test]
    fn test_get_all() {
        define! {
            mut users: UserList,
            mut logger: MockLogger,
        };

        logger
            .expect_println()
            .once()
            .with(eq(users.to_string()))
            .return_const(());

        let parser = CommandParser::new(Command::Get { username: None }, &mut users, &mut logger);

        parser.get();
    }

    #[test]
    fn test_withdraw_valid_username() {
        define! {
            mut users: UserList,
            mut logger: MockLogger,
            username = Username::build("WildSir").unwrap(),
            amount = Money::from(500),
            user = User {
                money: Money::from(1000),
                ..Default::default()
            }
        }

        let user_ref = leak_mut_option(user.clone());

        users
            .expect_get_mut()
            .once()
            .with(eq(username.clone()))
            .return_const(user_ref.clone());

        logger
            .expect_println()
            .once()
            .with(eq("Sucessfully withdrew amount."))
            .return_const(());

        let parser = CommandParser::new(
            Command::Withdraw { username, amount },
            &mut users,
            &mut logger,
        );

        // assert_eq!((*user_ref).money, 500);
        parser.withdraw();
    }

    #[test]
    fn test_withdraw_invalid_username() {
        define! {
            mut users: UserList,
            mut logger: MockLogger,
            username = Username::build("WildSir").unwrap(),
            amount = Money::from(500),
            user: User,
        }

        users
            .expect_get_mut()
            .once()
            .with(eq(username.clone()))
            .returning(|_| None);

        logger
            .expect_eprintln()
            .once()
            .with(eq("User \"WildSir\" not found."))
            .return_const(());

        let parser = CommandParser::new(
            Command::Withdraw { username, amount },
            &mut users,
            &mut logger,
        );

        assert_eq!(user.money, 0);
        parser.withdraw();
    }

    #[test]
    fn test_desposit_valid_username() {
        define! {
            mut users: UserList,
            mut logger: MockLogger,
            username = Username::build("WildSir").unwrap(),
            amount = Money::from(500),
            user: User,
        }

        users
            .expect_get()
            .once()
            .with(eq(username.clone()))
            .return_const(leak_mut_option(user));

        logger
            .expect_println()
            .once()
            .with(eq("Sucessfully deposited amount."))
            .return_const(());

        let parser = CommandParser::new(
            Command::Deposit { username, amount },
            &mut users,
            &mut logger,
        );

        parser.deposit();
    }

    #[test]
    fn test_deposit_invalid_username() {
        define! {
            mut users: UserList,
            mut logger: MockLogger,
            username = Username::build("WildSir").unwrap(),
            amount: Money,
            user: User,
        }

        users
            .expect_get()
            .once()
            .with(eq(username.clone()))
            .return_const(leak_mut_option(user));

        logger
            .expect_eprintln()
            .once()
            .with(eq("User \"WildSir\" not found."))
            .return_const(());

        let parser = CommandParser::new(
            Command::Deposit { username, amount },
            &mut users,
            &mut logger,
        );

        parser.deposit();
    }

    #[test]
    fn test_transfer_valid_usernames() {
        define! {
            mut users: UserList,
            mut logger: MockLogger,
            from = Username::build("WildSir").unwrap(),
            to = Username::build("BigSir").unwrap(),
            user: User,
            amount: Money,
        }

        let (from_clone, to_clone) = (from.clone(), to.clone());

        users
            .expect_get()
            .times(2)
            .withf(move |username: &Username| (*username == from_clone) || (*username == to_clone))
            .return_const(leak_mut_option(user));

        logger
            .expect_println()
            .once()
            .with(eq("Sucessfully transfered amount."))
            .return_const(());

        let parser = CommandParser::new(
            Command::Transfer { from, to, amount },
            &mut users,
            &mut logger,
        );

        parser.transfer();
    }

    #[test]
    fn test_transfer_both_invalid_usernames() {
        define! {
            mut users: UserList,
            mut logger: MockLogger,
            from = Username::build("WildSir").unwrap(),
            to = Username::build("BigSir").unwrap(),
            amount: Money,
        }

        let (from_clone, to_clone) = (from.clone(), to.clone());

        users
            .expect_get()
            .times(2)
            .withf(move |username: &Username| (*username == from_clone) || (*username == to_clone))
            .return_const(None);

        logger
            .expect_eprintln()
            .once()
            .with(eq("User \"WildSir\" and \"BigSir\" not found."))
            .return_const(());

        let parser = CommandParser::new(
            Command::Transfer { from, to, amount },
            &mut users,
            &mut logger,
        );

        parser.transfer();
    }

    #[test]
    fn test_transfer_one_invalid_username() {
        define! {
            mut users: UserList,
            mut logger: MockLogger,
            from = Username::build("WildSir").unwrap(),
            to = Username::build("BigSir").unwrap(),
            amount: Money,
            user: User,
        }

        let (from_clone1, from_clone2, to_clone) = (from.clone(), from.clone(), to.clone());

        users
            .expect_get()
            .times(2)
            .withf(move |username: &Username| (*username == from_clone1) || (*username == to_clone))
            .returning(move |username: &Username| {
                if *username == from_clone2 {
                    let user = user.clone();
                    Some(leak(user))
                } else {
                    None
                }
            });

        logger
            .expect_eprintln()
            .once()
            .with(eq("User \"BigSir\" not found."))
            .return_const(());

        let parser = CommandParser::new(
            Command::Transfer { from, to, amount },
            &mut users,
            &mut logger,
        );

        parser.transfer();
    }

    #[test]
    fn test_delete_valid_username() {
        define! {
            mut logger: MockLogger,
            mut users: UserList,
            username = Username::build("WildSir").unwrap(),
            user: User,
        }

        users
            .expect_remove()
            .once()
            .with(eq(username.clone()))
            .return_const(Some(user));

        logger
            .expect_println()
            .once()
            .with(eq("Succesfully deleted user."))
            .return_const(());

        let parser = CommandParser::new(Command::Delete { username }, &mut users, &mut logger);

        parser.delete();
    }

    #[test]
    fn test_delete_invalid_username() {
        define! {
            mut logger: MockLogger,
            mut users: UserList,
            username = Username::build("WildSir").unwrap(),
        }

        users
            .expect_remove()
            .once()
            .with(eq(username.clone()))
            .return_const(None);

        logger
            .expect_eprintln()
            .once()
            .with(eq("User \"WildSir\" not found."))
            .return_const(());

        let parser = CommandParser::new(Command::Delete { username }, &mut users, &mut logger);

        parser.delete();
    }

    #[test]
    fn test_clear() {
        define! {
            mut logger: MockLogger,
            mut users: UserList,
        }

        users.expect_clear().once().return_const(());

        logger
            .expect_println()
            .once()
            .with(eq("Sucesfully cleared list."))
            .return_const(());

        let parser = CommandParser::new(Command::Clear, &mut users, &mut logger);
        parser.clear()
    }
}
