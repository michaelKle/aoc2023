use regex::Regex;
use std::fs::read_to_string;

fn extract_colors_from_group(line: &str) -> (u8, u8, u8) {
    let v: Vec<&str> = line.trim().split(',').collect();
    let mut result: (u8, u8, u8) = (0, 0, 0);

    for c in v {
        let num_color: Vec<&str> = c.trim().split(' ').collect();
        let num = num_color[0].parse::<u8>().unwrap();
        let color = num_color[1];
        if color == "red" {
            result.0 += num;
            continue;
        }
        if color == "green" {
            result.1 += num;
            continue;
        }
        if color == "blue" {
            result.2 += num;
            continue;
        }
    }

    result
}

fn extract_max_colors(line: &str) -> (u16, u8, u8, u8) {
    let re = Regex::new(r"Game (?<id>\d+):(?<rest>.*)").unwrap();
    let caps = re.captures(line).unwrap();
    let id = caps["id"].parse::<u16>().unwrap();

    let mut result = (id, 0u8, 0u8, 0u8);
    for group in caps["rest"].split(';') {
        let colors = extract_colors_from_group(group);
        if colors.0 > result.1 {
            result.1 = colors.0
        }
        if colors.1 > result.2 {
            result.2 = colors.1
        }
        if colors.2 > result.3 {
            result.3 = colors.2
        }
    }

    result
}

pub fn sum_ids_of_possible_games(filename: &str, red: u8, green: u8, blue: u8) -> u32 {
    let mut result: u32 = 0;
    for line in read_to_string(filename).unwrap().lines() {
        let (id, max_red, max_green, max_blue) = extract_max_colors(line);
        if max_red > red || max_green > green || max_blue > blue {
            continue;
        }
        result += id as u32;
    }

    result
}

pub fn sum_power_of_all_games(filename: &str) -> u32 {
let mut result: u32 = 0;
    for line in read_to_string(filename).unwrap().lines() {
        let (min_red, min_green, min_blue) = extract_min_colors(line);
        
        result += id as u32;
    }

    result
}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_colors_from_group() {
        assert_eq!(
            extract_colors_from_group(" 4 red, 1 green, 2 blue"),
            (4, 1, 2)
        );
        assert_eq!(extract_colors_from_group(" 4 red"), (4, 0, 0));
        assert_eq!(extract_colors_from_group("2 blue, 4 red"), (4, 0, 2));
    }

    #[test]
    fn test_extract_tuple_vec() {
        assert_eq!(
            extract_max_colors("Game 41: 4 red, 1 green, 2 blue; 4 red; 4 red"),
            (41, 4, 1, 2)
        );
        assert_eq!(
            extract_max_colors("Game 54: 7 blue, 13 red, 7 green; 1 red, 2 green; 11 red, 10 green, 5 blue; 10 red, 8 green, 5 blue; 8 green, 12 blue, 12 red"),
            (54, 13, 10, 12)
        );
    }
}
