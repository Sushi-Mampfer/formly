use std::collections::HashSet;

pub fn parse(data: String, pattern: String) -> Option<String> {
    if !verify_alphanumerical(data) {
        return None;
    }
    for i in pattern.chars() {
        match i {
            '>' => shift_up(&mut data),

        }
    }
    None
}

pub fn verify_alphanumerical(data: String) -> bool {
    data.chars().all(|c| match c {
        'a'..='z' | 'A'..='Z' | '0'..='9' => true,
        _ => false
    })
}

pub fn shift_up(mut data: &String) {
    data = data.chars().map(|c| match c {
        'z' => 'a',
        'Z' => 'A',
        '9' => '0',
        c => char::from_u32(c as u32 + 1).unwrap()
    }).collect();
}

pub fn shift_down(mut data: String) {
    data = data.chars().map(|c| match c {
        'a' => 'z',
        'A' => 'Z',
        '0' => '9',
        c => char::from_u32(c as u32 - 1).unwrap()
    }).collect();
}