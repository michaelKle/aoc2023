extern crate regex;
use regex::bytes::Regex;
use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

#[derive(Hash, Eq, PartialEq, Debug)]
struct Key {
    start: usize,
    end: usize,
    line: usize,
}
impl Key {
    /// Creates a new Viking.
    fn new(start: usize, end: usize, line: usize) -> Key {
        Key {
            start: start,
            end: end,
            line: line,
        }
    }
}

fn is_symbol(x: u8) -> bool {
    let non_symbols = "0123456789.";

    non_symbols.find(x as char) == None
}

fn is_number(x: u8) -> bool {
    let numbers = "0123456789";

    numbers.find(x as char) != None
}

fn get_num_in_ascii_slice(bytes: &[u8]) -> usize {
    let mut erg: usize = 0;
    let mut exp: usize = 1;
    for x in (0..bytes.len()).rev() {
        erg += (bytes[x] - '0' as u8) as usize * exp;
        exp *= 10;
    }
    erg
}

fn numbers_in_line(bytes: &[u8], line: usize) -> HashMap<Key, usize> {
    let mut ret = HashMap::new();

    let re = Regex::new(r"(\d+)").unwrap();
    for cap in re.find_iter(bytes) {
        ret.insert(
            Key::new(cap.start(), cap.end(), line),
            get_num_in_ascii_slice(cap.as_bytes()),
        );
    }

    ret
}

// checks for symbol and handles out ouf bounds gracefully
// end is one beyond the last byte to be checked
fn line_has_symbol(line: &[u8], start: usize, end: usize) -> bool {
    let end = min(end, line.len());
    for x in start..end {
        let c = line[x];
        if is_symbol(c) {
            return true;
        }
    }
    false
}

// checks for symbol around the position
// end is one beyond the last byte to be checked
fn symbol_around(
    prev_line: &[u8],
    current_line: &[u8],
    next_line: &[u8],
    start: usize,
    end: usize,
) -> bool {
    let start = max(start as isize - 1, 0) as usize;
    line_has_symbol(prev_line, start, end + 1)
        || line_has_symbol(current_line, start, end + 1)
        || line_has_symbol(next_line, start, end + 1)
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
    use std::collections::HashMap;

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
    fn test_returns_numbers_in_line() {
        let expect1 = HashMap::from([(Key::new(0, 3, 3), 467), (Key::new(6, 8, 3), 14)]);
        assert_eq!(numbers_in_line("467...14..".as_bytes(), 3), expect1);

        let expect2 = HashMap::from([(Key::new(3, 7, 77), 4633), (Key::new(9, 11, 77), 55)]);
        assert_eq!(numbers_in_line("..*4633..55".as_bytes(), 77), expect2);
    }

    #[test]
    fn test_check_symbol_in_line() {
        assert_eq!(line_has_symbol("...*......".as_bytes(), 1, 4), true);
        assert_eq!(line_has_symbol("...*......".as_bytes(), 0, 2), false);
        assert_eq!(line_has_symbol("...*......".as_bytes(), 3, 4), true);
        assert_eq!(line_has_symbol("...*......".as_bytes(), 4, 3), false);
    }

    #[test]
    fn test_check_for_symbol_in_lines() {
        assert_eq!(
            symbol_around(
                "".as_bytes(),
                "467..114..".as_bytes(),
                "....*.....".as_bytes(),
                0,
                3
            ),
            false
        );
        assert_eq!(
            symbol_around(
                "".as_bytes(),
                "467..114..".as_bytes(),
                "....*.....".as_bytes(),
                5,
                8
            ),
            true
        );
        assert_eq!(
            symbol_around(
                "".as_bytes(),
                "467*.114..".as_bytes(),
                "....*.....".as_bytes(),
                0,
                3
            ),
            true
        );
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
