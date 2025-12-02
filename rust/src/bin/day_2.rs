use std::ops::Range;

use rust::{get_input_file_path, read_input};

fn main() {
    let input_path = get_input_file_path().unwrap();
    let input = read_input(&input_path).unwrap();

    let ranges: Vec<&str> = input
        .strip_suffix("\n")
        .unwrap_or(&input)
        .split(",")
        .collect();

    let ranges: Vec<Range<u64>> = ranges
        .iter()
        .map(|range| {
            let range_split: Vec<&str> = range.split("-").collect();
            let start: u64 = range_split[0].parse().unwrap();
            let end: u64 = range_split[1].parse().unwrap();
            // Range end is exclusive, so make sure we get that last ID
            let end = end + 1;

            Range { start, end }
        })
        .collect();

    let mut invalid_ids = vec![];
    for range in ranges {
        for current_id in range {
            let id = current_id.to_string();

            let mut id_substrs = vec![];
            for i in 0..(id.len() / 2) {
                for j in (i + 1)..=(id.len() / 2) {
                    id_substrs.push(&id[i..j]);
                }
            }

            for substr in id_substrs {
                let occurences = id
                    .as_bytes() // Yeah I'm assuming it's only going to be UTF characters
                    // that don't go beyond 1 code point. Sue me
                    .chunks(substr.len())
                    .filter(|window| *window == substr.as_bytes())
                    .count();
                if occurences >= 2 && substr.len() * occurences == id.len() {
                    invalid_ids.push(current_id);
                    break;
                }
            }
        }
    }

    let sum_of_invalid_ids: u64 = invalid_ids.iter().sum();
    println!("Sum of invalid IDs: {}", sum_of_invalid_ids);
}
