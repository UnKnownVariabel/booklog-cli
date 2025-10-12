pub mod api;

fn check_10_digit_isbn(isbn: &str) -> bool {
    let mut sum = 0;
    for i in 0..10 {
        sum += (10 - i as u32) * isbn.chars().nth(i).unwrap().to_digit(10).unwrap();
    }
    sum % 11 == 0
}

fn check_13_digit_isbn(isbn: &str) -> bool {
    let mut sum = 0;
    for i in 0..12 {
        let x = isbn.chars().nth(i).unwrap().to_digit(10).unwrap();
        sum += (((i % 2) * 2 + 1) as u32) * x;
    }
    (10 - (sum % 10 as u32)) % 10 == isbn.chars().nth(12).unwrap().to_digit(10).unwrap()
}

pub fn check_isbn(isbn: &str) -> bool {
    if !isbn.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    if isbn.len() == 10 {
        return check_10_digit_isbn(&isbn);
    }
    if isbn.len() == 13 {
        return check_13_digit_isbn(&isbn);
    }
    false
}
