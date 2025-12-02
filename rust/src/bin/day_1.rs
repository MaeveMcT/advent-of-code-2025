use rust::{get_input_file_path, read_input};

fn main() {
    let input_path = get_input_file_path().unwrap();
    let input = read_input(&input_path).unwrap();

    let turns: Vec<&str> = input.lines().collect();

    //let turns = vec!["R50"]; // Should be 1 click
    //let turns = vec!["R51"]; // Should be 1 click
    //let turns = vec!["L50", "L100"]; // Should be 2 click
    //let turns = vec!["L50", "L200"]; // Should be 3 click
    //let turns = vec!["L50", "L5"];
    //let turns = vec!["R1000"]; // Should be 10 clicks

    let mut times_hit_0 = 0;
    let mut current_combination = 50;
    for turn in turns {
        let (direction, rotation_amount) = turn.split_at(1);
        let rotation_amount: i32 = rotation_amount.parse().unwrap();

        match direction {
            "R" => {
                current_combination += rotation_amount;
                times_hit_0 += current_combination / 100;
                current_combination = current_combination.rem_euclid(100);
            }
            "L" => {
                let old_combination = current_combination;
                current_combination -= rotation_amount;
                current_combination = current_combination.rem_euclid(100);
                if old_combination == 0 {
                    times_hit_0 += rotation_amount / 100;
                } else if rotation_amount > old_combination {
                    times_hit_0 += 1;
                    times_hit_0 += (rotation_amount - old_combination - 1) / 100;
                    if current_combination == 0 {
                        times_hit_0 += 1;
                    }
                } else if rotation_amount == old_combination {
                    times_hit_0 += 1;
                }
            }
            _ => {
                unreachable!()
            }
        }
    }

    println!("{}", times_hit_0);
}
