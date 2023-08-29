use colored::Colorize;

pub struct CLIResponse {
    message: String,
    is_error: bool,
}

impl CLIResponse {
    pub fn new() -> Self {
        Self {
            message: String::new(),
            is_error: false,
        }
    }

    pub fn success_update(&mut self, message: String) {
        self.message = message;
        self.is_error = false;
        self.print_result();
    }

    pub fn error_update(&mut self, message: String) {
        self.message = message;
        self.is_error = true;
        self.print_result();
    }

    fn print_result(&self) {
        if self.is_error {
            println!("\n{}", "Error!!".bright_white().on_bright_red().bold());
            println!("{}", self.message.red().bold());
        } else {
            println!("\n{}", "Success!!".bright_white().on_bright_green().italic());
            println!("{}", self.message.green().italic());
        }
    }

    pub fn is_success(&self) -> bool {
        !self.is_error
    }
}