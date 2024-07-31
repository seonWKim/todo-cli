use colored::Colorize;

pub fn log(text: &str) {
    println!("{} {}", "[tc]".green(), text);
}

pub fn user_input(text: &str) {
    print!("{} {}", "[tc]".blue(), text.blue());
}
