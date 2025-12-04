use rust::{get_input_file_path, read_input};

fn main() {
    let input_path = get_input_file_path().unwrap();
    let input = read_input(&input_path).unwrap();

    let banks: Vec<&str> = input.lines().collect();

    let battery_pairs: Vec<_> = banks
        .iter()
        .map(|bank| {
            let mut biggest_seen = '0';
            let mut next_biggest = '0';

            // iterate through chars
            // take note of biggest current
            // take note of biggest since latest biggest
            let batteries: Vec<char> = bank.chars().collect();
            for (i, battery) in batteries.iter().enumerate() {
                if *battery > next_biggest {
                    next_biggest = *battery;
                }

                if i == batteries.len() - 1 {
                    break;
                }

                if *battery > biggest_seen {
                    biggest_seen = *battery;
                    next_biggest = batteries[i + 1];
                }
            }
            return (biggest_seen, next_biggest);
        })
        .collect();

    for pair in &battery_pairs {
        println!("{}{}", pair.0, pair.1);
    }

    let output_joltage: u32 = battery_pairs
        .iter()
        .map(|(a, b)| {
            // Could we have done this digit conversion earlier? Perhaps. Is it a crime?
            let mut s = String::new();
            s.push(*a);
            s.push(*b);
            let combined: u32 = s.parse().unwrap();
            return combined;
        })
        .sum();

    println!("Total output joltage: {}", output_joltage);
}
