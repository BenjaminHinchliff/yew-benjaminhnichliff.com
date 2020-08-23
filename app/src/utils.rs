/// takes an input string and replaces any larger whitespace like tabs and
/// multiple spaces and replaces it with a single space
pub fn normalize_whitespace(string: &str) -> String {
    String::from(string).split_whitespace().collect::<Vec<&str>>().join(" ")
}
