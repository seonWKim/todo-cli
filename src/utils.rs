use colored::{ColoredString, Colorize};

pub fn log(text: &str) {
    println!("{} {}", "[tc]".green(), text);
}

pub fn log1(text: &ColoredString) {
    println!("{} {}", "[tc]".green(), text);
}

pub fn user_input(text: &str) {
    print!("{} {}", "[tc]".blue(), text.blue());
}
