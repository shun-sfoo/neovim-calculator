extern crate neovim_lib;
use neovim_lib::{Neovim, NeovimApi, Session};

struct Calculator;
impl Calculator {
    fn new() -> Self {
        Calculator {}
    }

    // Add a vector of numbers.
    fn add(&self, num: Vec<i64>) -> i64 {
        num.iter().sum::<i64>()
    }

    // Multiply two numbers
    fn multiply(&self, p: i64, q: i64) -> i64 {
        p * q
    }
}

enum Messages {
    Add,
    Multiply,
    Unknow(String),
}

impl From<String> for Messages {
    fn from(event: String) -> Self {
        match &event[..] {
            "add" => Messages::Add,
            "multiply" => Messages::Multiply,
            _ => Messages::Unknow(event),
        }
    }
}

struct EventHandler {
    nvim: Neovim,
    calculator: Calculator,
}
impl EventHandler {
    fn new() -> EventHandler {
        let session = Session::new_parent().unwrap();
        let nvim = Neovim::new(session);
        let calculator = Calculator::new();

        EventHandler { nvim, calculator }
    }

    fn recv(&mut self) {
        let receiver = self.nvim.session.start_event_loop_channel();

        for (event, values) in receiver {
            match Messages::from(event) {
                // Handle 'Add'
                Messages::Add => {
                    let nums = values
                        .iter()
                        .map(|v| v.as_i64().unwrap())
                        .collect::<Vec<i64>>();
                    let sum = self.calculator.add(nums);
                    self.nvim
                        .command(&format!("echo \"Sum: {}\"", sum.to_string()))
                        .unwrap();
                }

                // Handle 'Multiply'
                Messages::Multiply => {
                    let mut nums = values.iter();
                    let p = nums.next().unwrap().as_i64().unwrap();
                    let q = nums.next().unwrap().as_i64().unwrap();

                    let product = self.calculator.multiply(p, q);

                    self.nvim
                        .command(&format!("echo \"Product: {}\"", product.to_string()))
                        .unwrap();
                }

                Messages::Unknow(event) => {
                    self.nvim
                        .command(&format!("echo \"Unkonw command: {}\"", event))
                        .unwrap();
                }
            }
        }
    }
}

fn main() {
    let mut event_handler = EventHandler::new();
    event_handler.recv();
}
