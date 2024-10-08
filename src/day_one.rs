use std::fs::read_to_string;

pub fn get_first_and_last_digt(line: &str) -> u16 {
    let is_digit = |c: char| (c >= '0') && (c <= '9');
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
        result += get_first_and_last_digt(line) as u32;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit() {
        assert_eq!(get_first_and_last_digt("ninethree4v1five"), 41);
        assert_eq!(get_first_and_last_digt("498879"), 49);
        assert_eq!(get_first_and_last_digt("khn1lbmzhvlsix3"), 13);
        assert_eq!(get_first_and_last_digt("treb7uchet"), 77);
    }
}
