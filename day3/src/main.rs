use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), std::io::Error> {
    let filename = "input_codes.txt";

    let file = File::open(filename)?;

    let buffer = BufReader::new(file);

    let lines = buffer.lines();

    let sum = lines
        .map(|line| {
            let line = line.unwrap().trim().to_string();

            let mut line = line.chars().collect::<Vec<char>>();

            print!("Line: {line:?}");
            let mut largest: u32 = 0;

            line.reverse();

            while let Some(digit1) = line.pop() {
                line.reverse();

                // println!("\tLine: {line:?}");
                let mut remainder = line.clone();

                remainder.reverse();

                while let Some(digit2) = remainder.pop() {
                    let number = format!("{digit1}{digit2}").parse::<u32>().unwrap();

                    largest = if number > largest { number } else { largest };
                }

                line.reverse();
            }

            println!("\tLargest: {largest}");

            largest
        })
        .collect::<Vec<u32>>()
        .iter()
        .fold(0, |acc, val| acc + *val);

    println!("Sum: {sum}");

    Ok(())
}
