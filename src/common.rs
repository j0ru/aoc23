pub fn human_readable_numbers<T: Into<u64>>(value: T) -> String {
    let value: u64 = value.into();
    match value {
        n if value > 1_000_000_000_000 => format!("{}T", n / 1_000_000_000_000),
        n if value > 1_000_000_000 => format!("{}G", n / 1_000_000_000),
        n if value > 1_000_000 => format!("{}M", n / 1_000_000),
        n if value > 1_000 => format!("{}K", n / 1_000),
        n => n.to_string(),
    }
}

