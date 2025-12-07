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
        .map(|line| -> u64 {
            let line = line.unwrap().trim().to_string();

            let line = line.chars().collect::<Vec<char>>();

            let numbers = line
                .iter()
                .map(|n| (*n).to_string().parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            print!("{numbers:?}");

            let largest = construct_largest_n_digit(numbers, 12);

            println!("\t{largest:?}");
            largest.unwrap()
        })
        .collect::<Vec<u64>>()
        .iter()
        .fold(0, |acc, val| acc + *val);

    println!("Sum: {sum}");

    Ok(())
}

fn construct_largest_n_digit(line: Vec<u32>, ndigit: u16) -> Option<u64> {
    let mut largest: u64 = 0;

    let len = line.len() as u16;

    if ndigit == 0 {
        return Some(0);
    }

    if ndigit > len {
        return None;
    }
    if ndigit == len {
        largest = line.iter().fold(0, |acc, val| acc * 10 + (*val) as u64)
    }

    if ndigit < len {
        let mut remainder = line.clone();

        let search_end = (remainder.len() - (ndigit as usize) + 1).min(remainder.len());

        let first = remainder[..search_end].to_vec();
        if let Some((index, &max_number)) = first
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(&a.0)))
        {
            let _ = remainder.drain(0..=index).collect::<Vec<u32>>();

            if let Some(remaining_largest) = construct_largest_n_digit(remainder, ndigit - 1) {
                largest = (max_number as u64) * 10_u64.pow((ndigit - 1) as u32) + remaining_largest;
            }
        }

        // largest = construct_largest_n_digit(remainder, ndigit - 1).unwrap();

        // println!("first: {first:?}",);
        // println!("Largest: {largest}");

        // println!("Remainder: {remainder:?}");
    }

    Some(largest)
}
