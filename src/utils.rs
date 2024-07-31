use std::io;
use std::io::Write;
use colored::Colorize;

pub fn log(text: &str) {
    println!("{} {}", "[tc]".green(), text);
}

pub fn user_input(text: &str) -> Result<String, ()> {
    print!("{} {}", "[tc]".blue(), text.blue());
    io::stdout().flush().map_err(|_| ())?;

    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|_| ())?;
    Ok(input.trim().to_string())
}
