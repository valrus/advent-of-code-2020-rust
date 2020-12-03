use std::env;
use std::fs;
use utils;

fn print_pairs(numbers: &Vec<i32>) {
    for (i, number1) in numbers.iter().enumerate() {
        for number2 in &numbers[i..] {
            if number1 + number2 == 2020 {
                println!("{} + {} = 2020", number1, number2);
                println!("{} * {} = {}", number1, number2, number1 * number2);
            }
        }
    }
}

fn print_triples(numbers: &Vec<i32>) {
    for (i, number1) in numbers.iter().enumerate() {
        for (j, number2) in numbers[i..].iter().enumerate() {
            for number3 in &numbers[i+j..] {
                if number1 + number2 + number3 == 2020 {
                    println!("{} + {} + {} = 2020", number1, number2, number3);
                    println!("{} * {} * {} = {}", number1, number2, number3, number1 * number2 * number3);
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Couldn't read the file");
    let numbers: Vec<i32> = utils::split_lines(&contents)
        .map(|s| i32::from_str_radix(s, 10))
        .map(|result| result.expect("Not a number"))
        .collect();

    print_pairs(&numbers);
    print_triples(&numbers);
}
