use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Couldn't read the file");
    let numbers: Vec<i32> = contents
        .split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| i32::from_str_radix(s, 10))
        .map(|result| result.expect("Not a number"))
        .collect();

    for (i, number1) in numbers.iter().enumerate() {
        for number2 in &numbers[i..] {
            if number1 + number2 == 2020 {
                println!("{} + {} = 2020", number1, number2);
                println!("{} * {} = {}", number1, number2, number1 * number2);
            }
        }
    }
}
