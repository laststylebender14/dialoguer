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
    options: Vec<String>,
}

impl MyCompletion {
    fn new() -> Self {
        Self {
            options: vec![
                "orange".to_string(),
                "apple".to_string(),
                "banana".to_string(),
                "apricot".to_string(),
                "avocado".to_string(),
            ],
        }
    }
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
        self.options
            .iter()
            .filter(|option| option.starts_with(input))
            .cloned()
            .collect()
    }
}