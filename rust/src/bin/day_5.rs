use std::ops::Range;

use rust::{get_input_file_path, read_input};

fn main() {
    let input_path = get_input_file_path().unwrap();
    let input = read_input(&input_path).unwrap();

    let mut lines: Vec<_> = input.lines().collect();
    let split_index = lines.iter().position(|&line| line.is_empty()).unwrap();

    let mut available_ingredients = lines.split_off(split_index);
    available_ingredients.remove(0);
    let fresh_ingredients = lines;

    dbg!(&fresh_ingredients);

    let fresh_ingredients: Vec<Range<u64>> = fresh_ingredients
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
    dbg!(&fresh_ingredients);
    dbg!(&available_ingredients);

    let available_ingredients: Vec<u64> = available_ingredients
        .iter()
        .map(|&s| s.parse().unwrap())
        .collect();

    let mut fresh_ingredient_count = 0;
    for ingredient in &available_ingredients {
        for range in &fresh_ingredients {
            if range.contains(ingredient) {
                fresh_ingredient_count += 1;
                break;
            }
        }
    }

    println!("Fresh ingredient count: {}", fresh_ingredient_count);
}
