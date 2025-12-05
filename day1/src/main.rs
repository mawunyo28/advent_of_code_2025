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

    let _: Vec<_> = lines
        .map(|line| {
            let line = line.unwrap(); // Todo: handle errors
            // println!("{line}");

            let line = line.trim().to_string();
            if let (Some(direction), Some(number_str)) = (line.get(0..1), line.get(1..)) {
                let move_amount = number_str.to_string().parse::<i16>().unwrap();
                let direction = direction.to_ascii_uppercase();

                match direction.as_str() {
                    "L" => dial.move_left(move_amount),
                    "R" => dial.move_right(move_amount),
                    _ => (),
                }
            }
        })
        .collect();

    println!("sum is {}", dial.value());
    println!("password is {}", dial.zero_count());

    Ok(())
}
