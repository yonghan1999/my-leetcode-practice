
use std::collections::HashMap;

struct Logger {
    logs: HashMap<String, i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Logger {

    fn new() -> Self {
        Self {
            logs: HashMap::new(),
        }
    }

    fn should_print_message(&mut self, timestamp: i32, message: String) -> bool {
        if let Some(t) = self.logs.get(&message) {
            if timestamp - t < 10 {
                return false;
            }
        }

        self.logs.insert(message, timestamp);

        true
    }
}
/**
 * Your Logger object will be instantiated and called as such:
 * let obj = Logger::new();
 * let ret_1: bool = obj.should_print_message(timestamp, message);
 */

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
