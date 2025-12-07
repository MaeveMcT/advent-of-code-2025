use std::{
    cmp::{max, min, Ordering},
    collections::HashSet,
    ops::{Range, RangeBounds},
};

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

    let mut fresh_ingredients: Vec<Range<u64>> = fresh_ingredients
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

    fresh_ingredients.sort_by(|a, b| {
        let start_bound_cmp = a.start.cmp(&b.start);
        if start_bound_cmp == Ordering::Equal {
            return a.end.cmp(&b.end);
        } else {
            return start_bound_cmp;
        }
    });

    let mut i = 0;
    let mut start_bound = 0;
    let mut end_bound = 0;
    while i < fresh_ingredients.len() {
        println!("On range {}", i);
        let current = &fresh_ingredients[i];

        i += 1;

        if current.start <= start_bound {
            if current.end < end_bound {
                // Nothing unique in the range
                continue;
            } else {
                start_bound = end_bound;
            }
        } else {
            start_bound = current.start;
        }
        end_bound = current.end;

        fresh_ingredient_count += end_bound - start_bound;
        start_bound = end_bound - 1;
    }
    println!("Fresh ingredient count: {}", fresh_ingredient_count);
}
