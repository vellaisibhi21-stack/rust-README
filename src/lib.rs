/// Returns a greeting message for the given name.
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feat_1_greeter_returns_expected_message() {
        assert_eq!(greet("LaneSync"), "Hello, LaneSync!");
    }

    #[test]
    fn feat_1_greeter_handles_empty_string() {
        assert_eq!(greet(""), "Hello, !");
    }
}
