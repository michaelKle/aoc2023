fn is_symbol(x: u8) -> bool {
    let non_symbols = "0123456789.";

    non_symbols.find(x as char) == None
}

// returns true if line has a character
// at pos or pos-1 or pos+2
// that is not a dot or a numeral
// assumes line is ASCII
fn has_symbol(line: &str, pos: u16) -> bool {
    let bytes = line.as_bytes();
    let mut left = pos as usize;
    let middle = pos as usize;
    let mut right = pos as usize;

    if pos > 0 {
        left = (pos - 1) as usize;
    }
    if ((pos + 1) as usize) < (bytes.len()) {
        right = (pos + 1) as usize;
    }

    is_symbol(bytes[left]) || is_symbol(bytes[middle]) || is_symbol(bytes[right])
}

fn get_part_numbers(upper_line: &str, middle_line: &str, lower_line: &str) -> Vec<u16> {
    for (i, c) in middle_line.char_indices().enumerate() {
        println!("{0} => {1}", i, c.1);
    }
    vec![1, 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_for_symbol() {
        assert_eq!(is_symbol(".".as_bytes()[0]), false);
        assert_eq!(is_symbol("1".as_bytes()[0]), false);
        assert_eq!(is_symbol("#".as_bytes()[0]), true);
        assert_eq!(is_symbol("*".as_bytes()[0]), true);
    }

    #[test]
    fn test_check_for_symbol_in_line() {
        assert_eq!(has_symbol("...*......", 3), true);
        assert_eq!(has_symbol("...*......", 2), true);
        assert_eq!(has_symbol("...*......", 4), true);
        assert_eq!(has_symbol("...*......", 0), false);
        assert_eq!(has_symbol("...*......", 9), false);
    }

    #[test]
    fn test_get_valid_part_numbers() {
        assert_eq!(get_part_numbers("", "467..114..", "...*......"), [467]);
    }
}
