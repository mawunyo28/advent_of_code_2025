use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

fn main() -> Result<(), std::io::Error> {
    let filename = "input_ranges.txt";

    let mut invalid_ids_sum: u64 = 0;

    let file = File::open(filename)?;

    let buffer = BufReader::new(file);

    for lines in buffer.lines() {
        let line = lines?;

        //println!("line: {line}");

        let ranges: Vec<&str> = line.split(',').collect();

        //println!("\tRanges: {ranges:?}");

        for range in ranges {
            //println!("\t\trange: {range}");
            let range = range.to_string();

            let range: Vec<&str> = range.split('-').collect();

            //println!("\t\t\tRange slitted: {range:?}");

            if let (Ok(upper), Ok(lower)) = (
                range.first().unwrap().parse::<u64>(),
                range.last().unwrap().parse::<u64>(),
            ) {
                //println!("\t\t\t\tRange upper: {upper}");
                //println!("\t\t\t\tRange lower: {lower}");
                for id in upper..=lower {
                    // //println!("\t\t\t\t\tId: {id}");
                    let id_string = id.to_string();

                    if let Some(count) = count_of_repeated_string(id_string)
                        && count >= 2
                    {
                        println!("\t\t\t\t\tId: {id}");
                        invalid_ids_sum += id;
                    }
                }
            }
        }
    }

    println!("Sum of invalid ids {invalid_ids_sum}");

    Ok(())
}

fn count_of_repeated_string(str: String) -> Option<usize> {
    let length = str.len();
    if length < 2 {
        return None;
    }
    let mut sub_str = String::new();

    let half = &str[0..(length / 2)];

    let half: Vec<char> = half.chars().collect();

    for i in half {
        sub_str.push(i);

        let cleared_from_substring = str.replace(sub_str.as_str(), "");

        if cleared_from_substring.is_empty() {
            let re = Regex::new(&sub_str).unwrap();

            let occurence_count: Vec<&str> = re.find_iter(&str).map(|mat| mat.as_str()).collect();

            let occurence_count = occurence_count.len();

            return Some(occurence_count);
        }
    }

    None
}
