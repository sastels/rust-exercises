pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }
    let proverb = (0..list.len() - 1)
        .map(|i| format!("For want of a {} the {} was lost.\n", list[i], list[i + 1]))
        .fold("".to_string(), |sum, value| sum + &value);
    format!("{}And all for the want of a {}.", proverb, list[0])
}
