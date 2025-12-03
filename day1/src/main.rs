use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

use day1::Dial;

// Todo: handle errors
fn main() -> Result<(), Error> {
    let mut dial = Dial::new(50);
    let filename = "input_codes.txt";
    let file = File::open(filename)?;

    let buffer = BufReader::with_capacity(64, file);

    let lines = buffer.lines();

    let container: Vec<(char, u32)> = lines
        .map(|line| {
            let line = line.unwrap(); // Todo: handle errors
            // println!("{line}");

            let line = line.trim();

            let mut characters: Vec<char> = line.chars().collect();

            characters.reverse();

            let direction = characters.pop().unwrap().to_ascii_uppercase();

            characters.reverse();

            let characters = characters.iter().collect::<String>();

            let move_amount = characters.parse::<u32>().unwrap();

            (direction, move_amount)
        })
        .collect();

    for (direction, move_amount) in &container {
        match direction {
            'L' => dial.move_left(*move_amount),
            'R' => dial.move_right(*move_amount),
            _ => (),
        }
    }

    // println!("{:?}", container);

    println!("sum is {}", dial.value());
    println!("password is {}", dial.zero_count());

    Ok(())
}
