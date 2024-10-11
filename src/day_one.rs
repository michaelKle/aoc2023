use std::fs::read_to_string;

trait WordFind {
    fn find_list(&self, words: Vec<&str>) -> Option<usize>;
    fn rfind_list(&self, words: Vec<&str>) -> Option<usize>;
}

impl WordFind for str {
    fn find_list(&self, words: Vec<&str>) -> Option<usize> {
        let mut smallest: usize = self.len();
        for word in words {
            let idx = self.find(word);
            if let Some(pos) = idx {
                if pos < smallest {
                    smallest = pos
                }
            }
        }
        if smallest < self.len() {
            return Some(smallest);
        } else {
            return None;
        }
    }
    fn rfind_list(&self, words: Vec<&str>) -> Option<usize> {
        let mut highest: usize = 0;
        let mut found: bool = false;
        for word in words {
            let idx = self.rfind(word);
            if let Some(pos) = idx {
                if pos > highest {
                    highest = pos;
                    found = true;
                }
            }
        }
        if found {
            return Some(highest);
        } else {
            return None;
        }
    }
}

pub fn get_num_word_value(line: &str, pos: usize) -> char {
    let words = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    const LABELS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    let substr = &line[pos..];
    for (pos, e) in words.iter().enumerate() {
        if substr.starts_with(e) {
            return LABELS[pos];
        }
    }

    '0'
}

pub fn get_index_of_first_num_word(line: &str) -> Option<usize> {
    let words = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    line.find_list(words)
}
pub fn get_index_of_last_num_word(line: &str) -> Option<usize> {
    let words = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    line.rfind_list(words)
}

fn is_digit(c: char) -> bool {
    (c >= '0') && (c <= '9')
}

pub fn get_first_num(line: &str) -> char {
    let idx_digit = line.find(is_digit);
    let idx_word = get_index_of_first_num_word(line);

    let idx: usize;
    let mut is_word = false;
    if idx_digit.is_none() {
        idx = idx_word.unwrap();
        is_word = true;
    } else if idx_word.is_none() {
        idx = idx_digit.unwrap();
    } else {
        if idx_digit.unwrap() < idx_word.unwrap() {
            idx = idx_digit.unwrap();
        } else {
            idx = idx_word.unwrap();
            is_word = true;
        }
    }

    if !is_word {
        return line.chars().nth(idx).unwrap();
    }

    get_num_word_value(line, idx)
}

pub fn get_last_digit(line: &str) -> u16 {
    let idx1 = line.find(is_digit);
    let idx2 = line.rfind(is_digit);

    let c1 = line.chars().nth(idx1.unwrap()).unwrap();
    let c2 = line.chars().nth(idx2.unwrap()).unwrap();

    let mut erg = String::new();
    erg.push(c1);
    erg.push(c2);

    return erg.parse::<u16>().unwrap();
}

pub fn sum_line_digits(filename: &str) -> u32 {
    let mut result: u32 = 0;

    for line in read_to_string(filename).unwrap().lines() {
        //result += get_first_and_last_digt(line) as u32;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_find() {
        assert_eq!("ninethree4v1five".find_list(vec!["three", "five"]), Some(4));
        assert_eq!(
            "ninethree4v1five".rfind_list(vec!["three", "five"]),
            Some(12)
        );
    }

    #[test]
    fn test_digit_get() {
        assert_eq!(get_num_word_value("ninethree4v1five", 0), '9');
        assert_eq!(get_num_word_value("ninethree4v1five", 4), '3');
    }

    #[test]
    fn test_get() {
        // assert_eq!(get_first_and_last_digt("ninethree4v1five"), 41);
        // assert_eq!(get_first_and_last_digt("498879"), 49);
        // assert_eq!(get_first_and_last_digt("khn1lbmzhvlsix3"), 13);
        // assert_eq!(get_first_and_last_digt("treb7uchet"), 77);
    }

    #[test]
    fn test_word_pos() {
        assert_eq!(get_index_of_first_num_word("eightwothree"), Some(0));
        assert_eq!(get_index_of_first_num_word("khn1twolbmzhvlsix3"), Some(4));
        assert_eq!(get_index_of_first_num_word("zoneight234"), Some(1));
        assert_eq!(get_index_of_last_num_word("eightwothree"), Some(7));
        assert_eq!(get_index_of_last_num_word("khn1twolbmzhvlsix3"), Some(14));
        assert_eq!(get_index_of_last_num_word("zoneight234"), Some(3));
    }

    #[test]
    fn test_word_and_digits_first() {
        assert_eq!(get_first_num("two1nine"), '2');
        assert_eq!(get_first_num("eightwothree"), '8');
        assert_eq!(get_first_num("khn1lbmzhvlsix3"), '1');
        assert_eq!(get_first_num("zoneight234"), '1');
    }
}
