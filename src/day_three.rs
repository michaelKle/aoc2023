use std::{collections::HashSet, fs::read_to_string};

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

fn get_number(line: &str, pos: usize) -> Option<u64> {
    let bytes = line.as_bytes();
    if pos >= bytes.len() {
        return None;
    }

    let c = bytes[pos as usize];
    let numbers = "0123456789";
    let erg = numbers.find(c as char);
    if erg == None {
        return None;
    }

    let mut end: usize = bytes.len();
    for x in pos..bytes.len() {
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

fn insert_if_number(col: &mut HashSet<u64>, num: Option<u64>) {
    if num.is_some() {
        // if col.contains(&num.unwrap()) {
        //     println!("Collection already contains {}", num.unwrap());
        // }
        col.insert(num.unwrap());
    }
}

fn get_part_numbers(upper_line: &str, middle_line: &str, lower_line: &str) -> HashSet<u64> {
    let mut numbers = HashSet::new();

    let middle_line_bytes = middle_line.as_bytes();

    for x in 1..middle_line_bytes.len() - 1 {
        let c = middle_line_bytes[x];

        if is_symbol(c) {
            insert_if_number(&mut numbers, get_number(upper_line, x - 1));
            insert_if_number(&mut numbers, get_number(upper_line, x));
            insert_if_number(&mut numbers, get_number(upper_line, x + 1));
            insert_if_number(&mut numbers, get_number(middle_line, x - 1));
            insert_if_number(&mut numbers, get_number(middle_line, x + 1));
            insert_if_number(&mut numbers, get_number(lower_line, x - 1));
            insert_if_number(&mut numbers, get_number(lower_line, x));
            insert_if_number(&mut numbers, get_number(lower_line, x + 1));
        }
    }
    numbers
}

pub fn sum_power_of_all_parts(filename: &str) -> u64 {
    let one_line = read_to_string(filename).unwrap();
    let lines: Vec<_> = one_line.lines().collect();
    let mut all_sets: HashSet<u64> = HashSet::new();
    for y in 1..lines.len() - 1 {
        let one_set = get_part_numbers(lines[y - 1], lines[y], lines[y + 1]);
        let line_nums = one_set
            .clone()
            .into_iter()
            .map(|i| i.to_string() + ", ")
            .collect::<String>();
        println!("Part numbers adjacent to line {}: {}", y + 1, line_nums);

        for x in one_set.clone().into_iter() {
            if all_sets.contains(&x) {
                println!("{} already in set", x);
            }
        }
        all_sets.extend(one_set);
    }

    all_sets.into_iter().fold(0, |acc, num| acc + num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_for_symbol() {
        assert_eq!(is_symbol('.' as u8), false);
        assert_eq!(is_symbol('1' as u8), false);
        assert_eq!(is_symbol('#' as u8), true);
        assert_eq!(is_symbol('*' as u8), true);
        assert_eq!(is_symbol('*' as u8), true);
        assert_eq!(is_symbol('&' as u8), true);
        assert_eq!(is_symbol('/' as u8), true);
        assert_eq!(is_symbol('$' as u8), true);
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
        assert_eq!(get_number("", 0), None);
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
        assert_eq!(
            get_part_numbers("467..114..", "...*......", ""),
            HashSet::from([467])
        );
        assert_eq!(
            get_part_numbers("467..114..", "...*..*...", "467..114.."),
            HashSet::from([467, 114])
        );
        assert_eq!(
            get_part_numbers("467..114..", "..........", "467..114.."),
            HashSet::from([])
        );
        assert_eq!(get_part_numbers("", "467..114*.", ""), HashSet::from([114]));
        assert_eq!(
            get_part_numbers("467..114*.", "..........", "467..114.."),
            HashSet::from([114])
        );
    }
    #[test]
    fn check_sum() {
        assert_eq!(
            HashSet::from([467, 114])
                .into_iter()
                .fold(0, |acc, num| acc + num),
            581
        );
    }
}
