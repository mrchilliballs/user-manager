#[cfg(test)]
use mockall::mock;

/// Used to specify logging behavior. Not only does this allow you to keep track of logs, but also use custom loggers from other crates.
pub trait Logger {
    /// Print the specefied sring
    fn print(&self, value: &str);
    /// Print the specified string to stderr; by default will use `print`
    fn eprint(&self, value: &str) {
        self.print(value);
    }
    /// Same as `eprint`, with a new line appended
    fn eprintln(&self, value: &str) {
        self.println(value);
        self.println("\n");
    }
    /// Same as `print`, with a new line appended
    fn println(&self, value: &str) {
        self.print(value);
        self.print("\n");
    }
}

#[cfg(test)]
mock! {
    #[derive(Debug)]
    pub Logger {}
    impl Logger for Logger {
        fn print(&self, value: &str);
        fn println(&self, value: &str);
        fn eprintln(&self, value: &str);
        fn eprint(&self, value: &str);
    }
    impl PartialEq for Logger {
        fn eq(&self, other: &Self) -> bool;
    }
}

/// Default logger using std macros
pub struct DefaultLogger;
impl Logger for DefaultLogger {
    /// Uses default `print` macro
    fn print(&self, value: &str) {
        print!("{value}")
    }
    /// Uses default `eprint` macro
    fn eprint(&self, value: &str) {
        eprint!("{value}")
    }
}
