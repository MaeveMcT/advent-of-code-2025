use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide an input file path");
        return;
    }
    let input_path = &args[1];
    let input = std::fs::read_to_string(input_path).unwrap();

    let turns: Vec<&str> = input.lines().collect();

    let mut times_hit_0 = 0;
    let mut current_combination = 50;
    for turn in turns {
        let (direction, rotation_amount) = turn.split_at(1);
        let rotation_amount: i32 = rotation_amount.parse().unwrap();

        match direction {
            "R" => current_combination += rotation_amount,
            "L" => current_combination -= rotation_amount,
            _ => {
                unreachable!()
            }
        }

        while current_combination < 0 {
            current_combination += 100
        }
        while current_combination >= 100 {
            current_combination -= 100
        }

        if current_combination == 0 {
            times_hit_0 += 1;
        }
    }

    println!("{}", times_hit_0);
}
