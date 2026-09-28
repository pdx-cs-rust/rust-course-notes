pub fn reply(message: &str) -> &str {
    let message = message.trim();
    if message == "" {
        return "Fine. Be that way!";
    }
    let yelling =
        message.chars().any(|c| c.is_alphabetic()) &&
        ! message.chars().any(|c| c.is_lowercase());
    let question = message.chars().last().unwrap() == '?';
    match (yelling, question) {
        (false, false) => "Whatever.",
        (true, false) => "Whoa, chill out!",
        (false, true) => "Sure.",
        (true, true) => "Calm down, I know what I'm doing!",
    }
}
