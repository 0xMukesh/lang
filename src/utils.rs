pub fn is_alpha(str: &String) -> bool {
    str.chars().all(|c| c.is_ascii_alphabetic())
}

pub fn is_num(str: &String) -> bool {
    str.chars().all(|c| c.is_ascii_digit())
}

pub fn is_skippable(str: &String) -> bool {
    str.chars()
        .all(|c| c.is_whitespace() || c == '\n' || c == '\t')
}

pub fn parse_num_literals(chars: &mut Vec<char>) -> String {
    let mut literal = String::new();

    while chars.len() > 0 && is_num(&chars[0].to_string()) {
        literal.push_str(chars.remove(0).to_string().as_str());
    }

    literal
}

pub fn parse_str_literals(chars: &mut Vec<char>) -> String {
    let mut literal = String::new();

    while chars.len() > 0 && is_alpha(&chars[0].to_string()) {
        literal.push_str(chars.remove(0).to_string().as_str());
    }

    literal
}
