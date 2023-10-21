use colored::{Colorize, ColoredString};

pub fn print_hangman(attempts: u32) {
    let stages = [
        r#"
        +---+
        |   |
            |
            |
            |
            |
        =============
        "#,
        r#"
        +---+
        |   |
        O   |
            |
            |
            |
        =============
        "#,
        r#"
        +---+
        |   |
        O   |
        |   |
            |
            |
        =============
        "#,
        r#"
        +---+
        |   |
        O   |
       /|   |
            |
            |
        =============
        "#,
        r#"
        +---+
        |   |
        O   |
       /|\  |
            |
            |
        =============
        "#,
        r#"
        +---+
        |   |
        O   |
       /|\  |
       /    |
            |
        =============
        "#,
        r#"
        +---+
        |   |
        O   |
       /|\  |
       / \  |
            |
        =============
        "#
    ];

    println!("{}", stages[attempts as usize]);
}

pub fn error_message(message: &str) -> ColoredString {
    let printable_message = message.red();
    return printable_message;
}