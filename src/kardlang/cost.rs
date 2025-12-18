pub fn effective_len(source: &str) -> usize {
    source
        .chars()
        .map(|c| match c {
            '0' => 1,
            '1'..='9' => c.to_digit(10).unwrap_or(1) as usize,
            _ => 1,
        })
        .sum()
}
