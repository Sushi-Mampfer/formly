use std::ops::Range;

pub fn parse(mut data: String, pattern: String) -> Option<String> {
    let mut temp = String::new();
    let mut temp2 = String::new();
    let mut expect_block = false;
    let mut block = false;
    let mut opened = 0;
    let mut range: Option<Range<usize>> = None;
    let mut repetitions: Option<usize> = None;
    let mut slice = false;
    let mut repeat = false;
    let mut dot = false;
    let mut equals = false;
    if !verify_alphanumerical(&data) {
        return None;
    }
    for c in pattern.chars() {
        if expect_block {
            if c == '{' {
                block = true;
                expect_block = false;
            } else {
                return None;
            }
        } else if block {
            match c {
                '}' => {
                    if opened > 0 {
                        opened -= 1;
                        temp.push('}');
                    } else if let Some(ref r) = range {
                        let mut chars: Vec<char> = data.chars().collect();
                        let slice: &[char] = &chars[r.clone()];
                        let value = parse(slice.iter().collect(), temp.clone())?;
                        chars.splice(r.clone(), value.chars());
                        data = chars.iter().collect();

                        block = false;
                        temp = String::new();
                        range = None;
                    } else if let Some(r) = repetitions {
                        for _ in 0..r {
                            data = parse(data.clone(), temp.clone())?;
                        }

                        block = false;
                        temp = String::new();
                        repetitions = None;
                    } else {
                        return None;
                    }
                }
                '{' => {
                    opened += 1;
                    temp.push('{');
                }
                c => temp.push(c)
            }
        } else if slice {
            match c {
                ')' => {
                    if dot {
                        if temp2.is_empty() {
                            temp2 = data.len().to_string();
                        }
                        let mut start = temp.parse::<usize>().ok()?;
                        let mut end = temp2.parse::<usize>().ok()?;
                        if end < start {
                            return None;
                        }
                        if start > data.len() - 1 {
                            start = 0;
                        }
                        if end > data.len() {
                            end = data.len();
                        }
                        range = Some(start..end)
                    } else {
                        let mut index = temp.parse::<usize>().ok()?;
                        if index > data.len() - 1 {
                            index = data.len();
                        }
                        range = Some(index..index + 1);
                    }
                    dot = false;
                    temp = String::new();
                    temp2 = String::new();
                    expect_block = true;
                    slice = false;
                }
                '.' => {
                    if dot {
                        return None
                    }
                    if temp.is_empty() {
                        temp = String::from("0");
                    }
                    dot = true
                }
                c => {
                    if dot {
                        temp2.push(c);
                    } else {
                        temp.push(c);
                    }
                }
            }
        } else if repeat {
            match c {
                ']' => {
                    repetitions = Some(temp.parse::<usize>().ok()?);
                    expect_block = true;

                    temp = String::new();
                    repeat = false;
                }
                c => temp.push(c),
            }
        } else if equals {
            data = data.replace(&temp, &c.to_string());
            temp = String::new();
            equals = false;
        }
        else {
            match c {
                '(' => slice = true,
                '[' => repeat = true,
                '>' => shift_up(&mut data),
                '<' => shift_down(&mut data),
                '=' => equals = true,
                '^' => data = data.to_ascii_uppercase(),
                '_' => data = data.to_ascii_lowercase(),
                c => {
                    if !verify_alphanumerical_char(c) {
                        return None;
                    }
                    if !temp.is_empty() {
                        return None;
                    }
                    temp.push(c);
                }
            }
        }
    }
    if block || expect_block || slice || repeat || opened != 0 {
        None
    } else {
        Some(data)
    }
}

pub fn verify_alphanumerical(data: &String) -> bool {
    data.chars().all(|c| verify_alphanumerical_char(c))
}

pub fn verify_alphanumerical_char(data: char) -> bool {
    match data {
        'a'..='z' | 'A'..='Z' | '0'..='9' => true,
        _ => false
    }
} 

pub fn shift_up(data: &mut String) {
    *data = data.chars().map(|c| match c {
        'z' => 'a',
        'Z' => 'A',
        '9' => '0',
        c => char::from_u32(c as u32 + 1).unwrap()
    }).collect();
}

pub fn shift_down(data: &mut String) {
    *data = data.chars().map(|c| match c {
        'a' => 'z',
        'A' => 'Z',
        '0' => '9',
        c => char::from_u32(c as u32 - 1).unwrap()
    }).collect();
}