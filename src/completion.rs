/// Trait for completion handling.
///
/// ## Example
/// ```rust,no_run
/// use dialoguer::Completion;
/// 
/// struct MyCompletion;
/// 
/// impl Completion for MyCompletion {
///     fn get(&self, input: &str) -> Option<String> {
///         if input.starts_with("ha") {
///             Some("hello".to_string())
///         } else {
///             None
///         }
///     }
/// }
/// ```
pub trait Completion {
    /// Gets a single suggested completion for the input text.
    ///
    /// Returns `Some(String)` with the completion if there is exactly one match,
    /// or `None` if there are zero or multiple matches.
    ///
    /// * `input` - The current input text to generate a completion for
    fn get(&self, input: &str) -> Option<String> {
        let suggestions = self.get_suggestions(input);
        if suggestions.len() == 1 {
            Some(suggestions[0].clone())
        } else {
            None
        }
    }

    /// Gets all suggested completions for the input text.
    ///
    /// Returns a vector of completion suggestions.
    /// The suggestions should generally match or extend the input text.
    ///
    /// * `input` - The current input text to generate completions for
    fn get_suggestions(&self, input: &str) -> Vec<String> {
        self.get(input).map_or_else(Vec::new, |s| vec![s])
    }
}