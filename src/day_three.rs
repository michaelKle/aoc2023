extern crate regex;
use regex::bytes::Regex;
use std::{
    cmp::{max, min},
    collections::HashMap,
    fs::read_to_string,
};

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
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

// checks the current line for numbers and
// then these numbers whether they are symbols
fn valid_parts(
    prev_line: &[u8],
    current_line: &[u8],
    next_line: &[u8],
    line: usize,
) -> HashMap<Key, usize> {
    let mut ret = HashMap::new();

    let nums = numbers_in_line(current_line, line);
    for (key, num) in nums {
        if symbol_around(prev_line, current_line, next_line, key.start, key.end) {
            ret.insert(key, num);
        }
    }

    ret
}

// fn print_hash(parts: &HashMap<Key, usize>) -> () {
//     for (key, num) in parts.into_iter() {
//         print!("{:?} = {},", key, num);
//     }
//     println!();
// }

fn get_all_parts(filename: &str) -> HashMap<Key, usize> {
    let one_line = read_to_string(filename).unwrap();
    let lines: Vec<_> = one_line.lines().collect();

    //println!("Line 0");
    let mut all_sets = valid_parts("".as_bytes(), lines[0].as_bytes(), lines[1].as_bytes(), 0);
    //print_hash(&all_sets);

    for y in 1..lines.len() - 1 {
        let one_set = valid_parts(
            lines[y - 1].as_bytes(),
            lines[y].as_bytes(),
            lines[y + 1].as_bytes(),
            y,
        );

        //println!("Line {}", y);
        //print_hash(&one_set);

        all_sets.extend(one_set.into_iter());
    }

    let last_set = valid_parts(
        lines[lines.len() - 2].as_bytes(),
        lines[lines.len() - 1].as_bytes(),
        "".as_bytes(),
        lines.len() - 1,
    );
    //println!("Line {}", lines.len() - 1);
    //print_hash(&last_set);

    all_sets.extend(last_set.into_iter());

    all_sets
}

fn get_stars_in_line(line: &[u8], linenum: usize) -> Vec<Key> {
    let mut ret = Vec::new();

    let re = Regex::new(r"(\*)").unwrap();
    for cap in re.find_iter(line) {
        ret.push(Key::new(cap.start(), cap.end(), linenum));
    }

    ret
}

fn is_symbol_close_to_num(symbol: &Key, num: &Key) -> bool {
    if symbol.line.abs_diff(num.line) >= 2 {
        return false;
    }

    if symbol.end > num.end + 1 {
        return false;
    }

    if symbol.start + 1 < num.start {
        return false;
    }

    return true;
}

// gets numbers connected to star symbol
fn check_symbol_in_num_hashes(
    symbol: &Key,
    nums: &HashMap<Key, usize>,
) -> Option<(Key, usize, usize)> {
    let mut found = Vec::new();
    //found.push(symbol.clone());
    for (k, val) in nums {
        if is_symbol_close_to_num(symbol, k) {
            found.push(*val);
        }
    }
    if found.len() != 2 {
        return None;
    }
    Some((symbol.clone(), found[0], found[1]))
}

// finds all stars
fn get_all_star_symbols(filename: &str) -> Vec<Key> {
    let mut ret = Vec::new();

    let one_line = read_to_string(filename).unwrap();
    let lines: Vec<_> = one_line.lines().collect();

    for y in 0..lines.len() {
        let line = lines[y].as_bytes();
        ret.extend(get_stars_in_line(line, y));
    }

    ret
}

pub fn get_all_connected_parts(filename: &str) -> usize {
    let parts = get_all_parts(filename);
    let stars = get_all_star_symbols(filename);

    let mut sum: usize = 0;
    for sym in stars {
        let part_match = check_symbol_in_num_hashes(&sym, &parts);
        if part_match.is_some() {
            let mat = part_match.unwrap();
            let erg = mat.1 * mat.2;
            //println!("{:?} => {}*{} = {}", mat, mat.1, mat.2, erg);
            sum += erg;
        }
    }

    sum
}

pub fn sum_all_parts(filename: &str) -> usize {
    let parts = get_all_parts(filename);

    parts.into_iter().fold(0, |acc, (_, num)| acc + num)
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
    fn test_check_finds_star_symbols_in_line() {
        let expect: Vec<Key> = vec![Key::new(3, 4, 17), Key::new(9, 10, 17)];
        assert_eq!(get_stars_in_line("./.*4633.*.55#".as_bytes(), 17), expect);
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
    fn test_get_valid_parts() {
        let expect1 = HashMap::from([(Key::new(5, 8, 2), 114)]);
        assert_eq!(
            valid_parts(
                "".as_bytes(),
                "467..114..".as_bytes(),
                "....*.....".as_bytes(),
                2
            ),
            expect1
        );
        let expect2 = HashMap::from([(Key::new(5, 8, 6), 114), (Key::new(0, 3, 6), 467)]);
        assert_eq!(
            valid_parts(
                "".as_bytes(),
                "467*.114..".as_bytes(),
                "....*.....".as_bytes(),
                6
            ),
            expect2
        );
    }

    #[test]
    fn test_get_valid_nums_to_symbol() {
        let sym = Key {
            start: 0,
            end: 1,
            line: 0,
        };
        assert!(is_symbol_close_to_num(
            &sym,
            &Key {
                start: 0,
                end: 3,
                line: 0
            }
        ));
        assert!(is_symbol_close_to_num(
            &sym,
            &Key {
                start: 1,
                end: 3,
                line: 0
            }
        ));
        assert!(!is_symbol_close_to_num(
            &sym,
            &Key {
                start: 2,
                end: 3,
                line: 0
            }
        ));

        assert!(is_symbol_close_to_num(
            &sym,
            &Key {
                start: 0,
                end: 3,
                line: 1
            }
        ));
        assert!(!is_symbol_close_to_num(
            &sym,
            &Key {
                start: 0,
                end: 3,
                line: 2
            }
        ));

        let sym_mid = Key {
            start: 10,
            end: 11,
            line: 0,
        };
        assert!(is_symbol_close_to_num(
            &sym_mid,
            &Key {
                start: 8,
                end: 11,
                line: 0
            }
        ));
        assert!(is_symbol_close_to_num(
            &sym_mid,
            &Key {
                start: 8,
                end: 10,
                line: 0
            }
        ));
        assert!(!is_symbol_close_to_num(
            &sym_mid,
            &Key {
                start: 8,
                end: 9,
                line: 0
            }
        ));
    }
}
