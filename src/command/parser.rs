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
            Command::Get => self.get(),
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
            let changed_user = user.to_original(original_user);
            self.users.insert(username, changed_user);
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
    // Notes: Do mocking expects before running the actual code being tested.

    use super::*;
    use crate::command::logger::MockLogger;
    use crate::user::User;
    use crate::username::Username;
    use mockall::predicate::eq;

    macro_rules! define {
        ($username:ident, $user:ident) => {
            let $username = Username::build("WildSir").unwrap();
            let $user = User::default();
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
    }
    #[test]
    fn test_edit() {
        todo!()
    }
    #[test]
    fn test_get() {
        todo!()
    }
    #[test]
    fn test_withdraw() {
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
