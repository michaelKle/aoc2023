use dotenv::dotenv;

mod day_one;

fn main() {
    dotenv().ok();

    {
        let day01_path = std::env::var("AOC01_FILE").expect("AOC01_FILE must be set.");
        let sum = day_one::sum_line_digits(&day01_path);
        println!("Sum of lines is {0}", sum);
    }
}
