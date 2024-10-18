use std::fs::read_to_string;

fn extract_max_colors(line: &str) -> (u16, u8, u8, u8) {
    (0, 0, 0, 0)
}

pub fn sum_ids_of_possible_games(filename: &str, red: u8, green: u8, blue: u8) -> u32 {
    let result: u32 = 0;
    for line in read_to_string(filename).unwrap().lines() {
        let (id, max_red, max_green, max_blue) = extract_max_colors(line);
        if max_red > red || max_green > green || max_blue > blue {
            continue;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract() {
        assert_eq!(
            extract_max_colors("Game 41: 4 red, 1 green, 2 blue; 4 red; 4 red"),
            (41, 4, 1, 2)
        );
    }
}
