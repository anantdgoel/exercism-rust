/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let Some(digits) = code
        .chars()
        .filter(|&c| c != ' ')
        .map(|c| c.to_digit(10))
        .collect::<Option<Vec<_>>>()
    else {
        return false;
    };

    if digits.len() <= 1 {
        return false;
    }

    let sum: u32 = digits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &digit)| {
            if i % 2 == 0 {
                digit
            } else {
                let doubled = digit * 2;
                if doubled > 9 { doubled - 9 } else { doubled }
            }
        })
        .sum();

    sum % 10 == 0
}
