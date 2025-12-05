use std::collections::VecDeque;

use rust::{get_input_file_path, read_input};

fn main() {
    let input_path = get_input_file_path().unwrap();
    let input = read_input(&input_path).unwrap();

    let banks: Vec<&str> = input.lines().collect();

    let num_batteries_to_combine = 12;
    let battery_sequences: Vec<_> = banks
        .iter()
        .map(|bank| {
            let biggest_sequence = biggest_sequence_for_bank(bank, num_batteries_to_combine);
            return biggest_sequence;
        })
        .collect();

    let battery_sequences: Vec<_> = battery_sequences
        .iter()
        .map(|chars| {
            let s: String = chars.iter().collect();
            return s;
        })
        .collect();
    println!("{} sequences", battery_sequences.len());
    for sequence in &battery_sequences {
        println!("{}", &sequence);
    }

    let output_joltage: u64 = battery_sequences
        .iter()
        .map(|s| s.parse::<u64>().unwrap())
        .sum();

    println!("Output joltage: {}", output_joltage);
}

fn biggest_sequence_for_bank(bank: &str, num_batteries_to_combine: u32) -> VecDeque<char> {
    let mut biggest_sequence: VecDeque<char> = VecDeque::new();

    let batteries: Vec<char> = bank.chars().collect();

    for &battery in batteries.iter().rev() {
        let sequence_so_far: String = biggest_sequence.iter().collect();
        println!("Sequence so far {}", sequence_so_far);
        println!("Considering {}", battery);
        if biggest_sequence.len() == num_batteries_to_combine as usize {
            println!("Sequence is full, gonna evict");
            // Time to evict something!
            let mut i = 0;
            let mut smallest_seen = ('9', 0);
            while i < biggest_sequence.len() - 1 {
                let joltage = biggest_sequence[i];
                println!("Joltage {}", joltage);
                if joltage < smallest_seen.0 {
                    smallest_seen = (joltage, i);
                    println!("Smallest seen {} at {}", smallest_seen.0, smallest_seen.1);
                }
                i += 1;
            }
            if battery < biggest_sequence[0] {
                // Sequence is only going to get smaller if we move the first number in the
                // sequence for the sake of a smaller joltage;
                continue;
            }
            if battery > smallest_seen.0 {
                println!(
                    "Removing joltage {} from index {}",
                    smallest_seen.0, smallest_seen.1
                );
                biggest_sequence.remove(smallest_seen.1);
                biggest_sequence.push_front(battery);
            }
        } else {
            // Plenty of room. Fill it up
            biggest_sequence.push_front(battery);
            println!("Adding {} to the sequence", battery);
            continue;
        }
        // If the sequence is filled up, whenever we find a new biggest, we can just go
        //
        // We know that the current battery we're evaluating is the furthest left we've
        // seen, so we can look into the sequence and remove the smallest thing from it,
        // then push the current battery to the front
        //
        // 818181911112111
    }
    return biggest_sequence;
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::biggest_sequence_for_bank;

    #[test]
    fn it_finds_biggest_sequence_in_between_start_and_end() {
        let bank = "234234234234278";
        let sequence = biggest_sequence_for_bank(bank, 12);

        let expected: VecDeque<char> = "434234234278".chars().collect();

        assert_eq!(sequence, expected);
    }

    #[test]
    fn it_finds_biggest_sequence_at_start() {
        let bank = "811111111111119";
        let sequence = biggest_sequence_for_bank(bank, 12);

        let expected: VecDeque<char> = "811111111119".chars().collect();

        assert_eq!(sequence, expected);
    }

    #[test]
    fn it_finds_biggest_sequence_at_end() {
        let bank = "111191111111119";
        let sequence = biggest_sequence_for_bank(bank, 12);

        let expected: VecDeque<char> = "191111111119".chars().collect();

        assert_eq!(sequence, expected);
    }
}
