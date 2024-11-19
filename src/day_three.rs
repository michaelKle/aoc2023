fn is_symbol(x: u8) -> bool {
    let non_symbols = "0123456789.";

    non_symbols.find(x as char) == None
}

fn is_number(x: u8) -> bool {
    let numbers = "0123456789";

    numbers.find(x as char) != None
}

fn is_symbol_char(x: char) -> bool {
    let non_symbols = "0123456789.";

    non_symbols.find(x) == None
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

fn get_number(line: &str, pos: u16) -> Option<u64> {
    let bytes = line.as_bytes();
    let c = bytes[pos as usize];
    let numbers = "0123456789";
    let erg = numbers.find(c as char);
    if erg == None {
        return None;
    }

    let mut end: usize = bytes.len();
    for x in pos..(bytes.len() as u16) {
        if !is_number(bytes[x as usize]) {
            end = x as usize;
            break;
        }
    }

    let mut num: u64 = 0;
    let mut exp: u64 = 1;
    for x in (0..end).rev() {
        if !is_number(bytes[x]) {
            break;
        }

        num += (bytes[x] - '0' as u8) as u64 * exp;
        exp *= 10;
    }

    Some(num)
}

fn get_part_numbers(upper_line: &str, middle_line: &str, lower_line: &str) -> Vec<u16> {
    for (i, c) in middle_line.char_indices().enumerate() {
        println!("{0} => {1}", i, c.1);

        if is_symbol_char(c.1) {
            println!("{0} has symbol", i);
        }
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
    fn test_check_for_symbol_char() {
        assert_eq!(is_symbol_char('.'), false);
        assert_eq!(is_symbol_char('1'), false);
        assert_eq!(is_symbol_char('#'), true);
        assert_eq!(is_symbol_char('*'), true);
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
    fn test_get_number_in_line() {
        assert_eq!(get_number("467...114..", 0), Some(467));
        assert_eq!(get_number("467...114..", 1), Some(467));
        assert_eq!(get_number("467...114..", 2), Some(467));
        assert_eq!(get_number("467...114..", 3), None);
        assert_eq!(get_number("467...114..", 4), None);
        assert_eq!(get_number("467...114..", 5), None);
        assert_eq!(get_number("467...114..", 6), Some(114));
        assert_eq!(get_number("467...114..", 7), Some(114));
        assert_eq!(get_number("467...114..", 8), Some(114));
        assert_eq!(get_number("467...114..", 9), None);
        assert_eq!(get_number("467...114..", 10), None);
        assert_eq!(get_number("467...114", 8), Some(114));
    }

    #[test]
    fn test_get_valid_part_numbers() {
        assert_eq!(get_part_numbers("467..114..", "...*......", ""), [467]);
    }
}
