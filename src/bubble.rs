use textwrap::wrap;

pub fn make_bubble(message: &str) -> String {
    let text = wrap(message, 28);
    let border_length = text.iter().map(|line| line.len()).max().unwrap_or(0);
    let border = format!(" {}", "-".repeat(border_length + 2));
    let mut bubble = String::new();
    bubble.push_str(&border);
    bubble.push('\n');

    for line in text {
        bubble.push_str(&format!(
            "| {}{} |\n",
            line,
            " ".repeat(border_length - line.len())
        ));
    }
    bubble.push_str(&border);
    bubble
}
