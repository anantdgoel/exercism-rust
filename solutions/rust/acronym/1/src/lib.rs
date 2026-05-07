pub fn abbreviate(phrase: &str) -> String {
    let mut acronym = String::new();
    let mut starts_word = true;
    let mut previous_was_lowercase = false;

    for c in phrase.chars() {
        match c {
            '-' => starts_word = true,
            c if c.is_whitespace() => starts_word = true,
            c if c.is_ascii_punctuation() => {}
            c => {
                if starts_word || previous_was_lowercase && c.is_uppercase() {
                    acronym.extend(c.to_uppercase());
                }

                starts_word = false;
                previous_was_lowercase = c.is_lowercase();
            }
        }

        if starts_word {
            previous_was_lowercase = false;
        }
    }

    acronym
}
