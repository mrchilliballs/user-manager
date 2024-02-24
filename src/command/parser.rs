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

    use super::*;
    use crate::command::logger::MockLogger;
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

    #[test]
    fn test_parse() {
        // Is it even possible?
        todo!()
    }

    #[test]
    fn test_new() {
        define! {
            mut users: UserList,
            mut logger: MockLogger,
            command = Command::Get {
                username: None
            },
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
            user: User
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
}
