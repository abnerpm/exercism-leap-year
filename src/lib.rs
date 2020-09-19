pub fn is_leap_year(year: u64) -> bool {
    if is_divisible_by_four(&year) && (is_not_divisible_by_one_hundred(&year) ||
        is_divisible_by_one_hundred_and_four_hundred(&year)) {
            true
        } else {
            false
        }
}

fn is_divisible_by_four(year: &u64) -> bool {
    if year % 4 == 0 {
        true
    } else {
        false
    }
}

fn is_not_divisible_by_one_hundred(year: &u64) -> bool {
    if year % 100 != 0 {
        true
    } else {
        false
    }
}

fn is_divisible_by_one_hundred_and_four_hundred(year: &u64) -> bool {
    if year % 100 == 0 && year % 400 == 0 {
        true
    } else {
        false
    }
}