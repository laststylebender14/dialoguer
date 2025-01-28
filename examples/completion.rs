use dialoguer::{theme::ColorfulTheme, Completion, Input};

fn main() {
    println!("Type to search. Use Tab to complete, Up/Down arrows to navigate suggestions.");

    let completion = MyCompletion::default();

    Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("fruit")
        .completion_with(&completion)
        .interact_text()
        .unwrap();
}

struct MyCompletion {
    options: Vec<String>,
}

impl Default for MyCompletion {
    fn default() -> Self {
        MyCompletion {
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
    fn get_suggestions(&self, input: &str) -> Vec<String> {
        self.options
            .iter()
            .filter(|option| option.starts_with(input))
            .cloned()
            .collect()
    }

    // The default implementation from the trait will be used for get()
}