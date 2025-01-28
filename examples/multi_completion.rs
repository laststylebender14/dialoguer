use std::io::Write;

use dialoguer::{theme::ColorfulTheme, Completion, Input};

fn main() {
    println!("Type to search. Use Tab to see suggestions, Up/Down arrows to navigate suggestions.");

    let completion = MyCompletion::new();

    let input = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("")
        .completion_with(&completion)
        .interact_text()
        .unwrap();

    println!("You selected: {}", input);
}

struct MyCompletion {
    files: Vec<String>,
    commands: Vec<String>,
}

impl MyCompletion {
    fn new() -> Self {
        Self {
            files: vec![
                "file1".to_string(),
                "file2".to_string(),
                "file3".to_string(),
            ],
            commands: vec!["list".to_string(), "search".to_string(), "quit".to_string()],
        }
    }
}

impl MyCompletion {
    fn find_last_trigger_position(input: &str) -> (&str, char) {
        if let Some(pos) = input.rfind('@').or_else(|| input.rfind('/')) {
            let trigger_char = input.chars().nth(pos).unwrap();
            let suggestion = &input[pos+1..];
            (suggestion, trigger_char)
        } else {
            // Return an empty suggestion and a default trigger character if none found
            ("", ' ') // or any other default character
        }
    }
}

#[derive(Debug)]
struct Data {
    suggestion: String,
    trigger_char: char,
    result: Vec<String>,
}

impl Completion for MyCompletion {
    fn get(&self, input: &str) -> Option<String> {
        let suggestions = self.get_suggestions(input);
        if suggestions.len() == 1 {
            Some(suggestions[0].clone())
        } else {
            None
        }
    }

    fn get_suggestions(&self, input: &str) -> Vec<String> {
        let (suggestion, trigger_char) = MyCompletion::find_last_trigger_position(input);
        let result = match trigger_char {
            '@' => self
                .files
                .iter()
                .filter(|file| file.starts_with(suggestion))
                .cloned()
                .collect(),
            '/' => self
                .commands
                .iter()
                .filter(|cmd| cmd.starts_with(suggestion))
                .cloned()
                .collect(),
            _ => self.commands.clone(),
        };

        let data = Data {
            suggestion: suggestion.to_string(),
            trigger_char,
            result: result.clone(),
        };

        let mut fs = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open("log.md")
            .unwrap();
        fs.write_all(format!("{:#?}\n", data).as_bytes()).unwrap();

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_last_trigger_position() {
        let input = "Hello @world";
        let (suggestion, trigger_char) = MyCompletion::find_last_trigger_position(input);
        assert_eq!(suggestion, "world");
        assert_eq!(trigger_char, '@');
    }

    #[test]
    fn test_find_last_trigger_position1() {
        let input = "Hello @";
        let (suggestion, trigger_char) = MyCompletion::find_last_trigger_position(input);
        assert_eq!(suggestion, "");
        assert_eq!(trigger_char, '@');
    }

    #[test]
    fn test_find_last_trigger_command() {
        let input = "Hello /s";
        let (suggestion, trigger_char) = MyCompletion::find_last_trigger_position(input);
        assert_eq!(suggestion, "s");
        assert_eq!(trigger_char, '/');
    }
}
