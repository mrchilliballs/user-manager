use super::logger::Logger;
use crate::command::Command;
use crate::user_list::UserList;

#[derive(Debug, PartialEq)]
pub struct CommandParser<'a, T>
where
    T: Logger,
{
    command: &'a Command,
    users: &'a mut UserList,
    logger: &'a T,
}

impl<'a, T> CommandParser<'a, T>
where
    T: Logger,
{
    pub fn new(
        command: &'a Command,
        users: &'a mut UserList,
        logger: &'a T,
        // Normally a file
    ) -> Self {
        todo!()
    }
    pub fn parse(self) {
        todo!()
    }
    // user_list.insert(...)
    fn insert(&mut self) {
        todo!()
    }
    // user_list.get_mut(...)
    fn edit(&mut self) {
        todo!()
    }
    // user_list.get(...)
    fn get(&mut self) {
        todo!()
    }
    // user_list.get(...)
    fn withdraw(&mut self) {
        todo!()
    }
    // user_list.deposit(...)
    fn deposit(&mut self) {
        todo!()
    }
    // user_list.deposit(...)
    fn transfer(&mut self) {
        todo!()
    }
    // user_list.delete(...)
    fn delete(&mut self) {
        todo!()
    }
}

// #[cfg(test)]
// mod tests {
//     use std::str::FromStr;

//     use crate::{money::Money, user, username::Username};

//     use super::*;

//     #[derive(Debug, Default, PartialEq)]
//     struct DummyLogger(String);
//     impl Logger for DummyLogger {
//         fn print(&mut self, value: &str) {
//             self.0.push_str(value);
//         }
//     }
//     struct CmdParserHelper {
//         command: Command,
//         users: UserList,
//         logger: DummyLogger,
//     }
//     impl CmdParserHelper {
//         fn new(command: Command) -> Self {
//             let users = UserList::default();
//             let logger = DummyLogger::default();
//             CmdParserHelper {
//                 command,
//                 users,
//                 logger,
//             }
//         }
//         fn command(&self) -> &Command {
//             &self.command
//         }
//         fn users(&mut self) -> &mut UserList {
//             &mut self.users
//         }
//         fn logger(&self) -> &DummyLogger {
//             &self.logger
//         }
//         fn parser(&mut self) -> CommandParser<DummyLogger> {
//             CommandParser {
//                 command: &self.command,
//                 users: &mut self.users,
//                 logger: &mut self.logger,
//             }
//         }
//     }

//     fn test_user() -> (Username, String, Money) {
//         (
//             Username::from_str("Sir").unwrap(),
//             String::from("Sir"),
//             Money::from(10.0),
//         )
//     }

//     #[test]
//     fn test_logger_default_impl() {
//         let mut logger = DummyLogger::default();
//         logger.println("This is a test!");

//         assert_eq!(logger.0, "This is a test!\n");
//     }

//     #[test]
//     fn test_parser_new() {
//         let mut helper = CmdParserHelper::new(Command::Get);

//         let (mut user1, mut user2) = (helper.users().clone(), helper.users().clone());

//         let parser = CommandParser::new(helper.command(), &mut user1, helper.logger());
//         let expected_parser = CommandParser {
//             command: helper.command(),
//             users: &mut user2,
//             logger: helper.logger(),
//         };

//         assert_eq!(parser, expected_parser);
//     }

//     #[test]
//     fn test_insert_user() {
//         let (username, name, money) = test_user();
//         let mut helper = CmdParserHelper::new(Command::Insert {
//             username,
//             name,
//             money,
//         });
//         let mut parser = helper.parser();

//         parser.insert();

//         assert_eq!(helper.logger().0, "Sucessfully inserted user.")
//     }

//     #[test]
//     fn test_edit_none() {
//         let username = test_user().0;
//         let mut helper = CmdParserHelper::new(Command::Edit {
//             username,
//             name: None,
//             money: None,
//         });
//         let mut parser = helper.parser();

//         parser.edit();

//         assert_eq!(
//             helper.logger().0,
//             "Error: You must change at least one field."
//         );
//     }

//     #[test]
//     fn test_edit_one() {
//         let (username, _, money) = test_user();
//         let mut helper = CmdParserHelper::new(Command::Edit {
//             username: username.clone(),
//             name: Some(String::from("BigSir")),
//             money: None,
//         });
//         let mut parser = helper.parser();

//         parser.edit();

//         let user = parser.users.get(&username).unwrap();

//         assert_eq!(user.name, "BigSir");
//         assert_eq!(helper.logger().0, "Succesfully edited field \"name.\"");
//     }

//     #[test]
//     fn test_edit_all() {
//         let username = test_user().0;
//         let mut helper = CmdParserHelper::new(Command::Edit {
//             username: username.clone(),
//             name: Some(String::from("BigSir")),
//             money: Some(Money::from(0.0)),
//         });
//         let mut parser = helper.parser();

//         parser.edit();

//         let user = parser.users.get(&username).unwrap();

//         assert_eq!(user.name, "BigSir");
//         assert_eq!(user.money, 0.0);
//         assert_eq!(
//             helper.logger().0,
//             "Succesfully edited fields \"name,\" \"money.\""
//         );
//     }
// }
