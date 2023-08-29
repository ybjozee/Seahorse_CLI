use regex::Regex;

pub fn number_of_rotations(input: isize) -> Option<i32> {
    if input > 0 {
        return Some(input as i32);
    }
    None
}

pub fn recipient_phone_number(input: &str) -> Option<&str> {
    if let Ok(regex) = Regex::new(r"^\+[1-9]\d{12,14}$") {
        if regex.is_match(input) {
            return Some(input);
        }
    };
    None
}