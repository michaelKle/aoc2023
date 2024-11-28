use dotenv::dotenv;

mod day_one;
mod day_three;
mod day_two;

fn main() {
    dotenv().ok();

    // day 1
    {
        let day01_path = std::env::var("AOC01_FILE").expect("AOC01_FILE must be set.");
        let sum = day_one::sum_line_digits(&day01_path);
        println!("Day 1: Sum of lines is {0}", sum);
    }
    // day 2
    {
        let day02_path = std::env::var("AOC02_FILE").expect("AOC02_FILE must be set.");
        let sum = day_two::sum_ids_of_possible_games(&day02_path, 12, 13, 14);
        println!("Day 2: Sum of IDs of possible games is {0}", sum);
        let sum = day_two::sum_power_of_all_games(&day02_path);
        println!("Day 2: Sum of power of all games is {0}", sum);
    }
    //day 3
    {
        let _day03_path = std::env::var("AOC03_FILE").expect("AOC03_FILE must be set.");
        let sum = day_three::sum_all_parts(&_day03_path);
        println!("Day 3: Sum of all all part numbers {0}", sum);
        let sum = day_three::get_all_connected_parts(&_day03_path);
        println!("Day 3: Sum of all all star connected parts {0}", sum);
    }
}
