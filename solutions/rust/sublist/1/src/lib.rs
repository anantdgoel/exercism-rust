#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    match (
        contains(first_list, second_list),
        contains(second_list, first_list),
    ) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Sublist,
        (false, true) => Comparison::Superlist,
        (false, false) => Comparison::Unequal,
    }
}

fn contains(small: &[i32], large: &[i32]) -> bool {
    small.is_empty() || large.windows(small.len()).any(|w| w == small)
}
