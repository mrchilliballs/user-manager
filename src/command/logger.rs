pub trait Logger {
    fn print(&mut self, value: &str);
    fn println(&mut self, value: &str) {
        self.print(value);
        self.print("\n");
    }
}
// impl Logger for Term {
//     fn print(&mut self, value: &str) {
//         write!(&self, "{}", value);
//     }
// }
