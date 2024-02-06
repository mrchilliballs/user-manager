use std::io::{BufWriter, Write};

use console::Term;
#[cfg(test)]
use mockall::mock;

pub trait Logger {
    fn print(&self, value: &str);
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
    }
    impl PartialEq for Logger {
        fn eq(&self, other: &Self) -> bool;
    }
}

// TESTS
impl Logger for Term {
    fn print(&self, value: &str) {
        let mut buf_writer = BufWriter::new(self);
        buf_writer
            .write(value.as_bytes())
            .expect("Failed to write to terminal");
    }
}
