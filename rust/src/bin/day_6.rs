use rust::{get_input_file_path, read_input};

fn main() {
    let input_path = get_input_file_path().unwrap();
    let input = read_input(&input_path).unwrap();

    let lines: &Vec<_> = &input.lines().collect();

    let rows_count = lines.len();

    // Dumb approach but maybe:
    // get the max number of digits per column
    // use max number of digits per column to go back to original lines and substr max number of
    // digits per column, repeat for each column

    let rows: Vec<_> = lines
        .iter()
        .map(|line| {
            let columns: Vec<_> = line.split_whitespace().collect();
            return columns;
        })
        .collect();

    let columns_count = rows[0].len(); // Assumes all columns are same width
    let mut equations = vec![];
    for col_idx in 0..columns_count {
        let mut equation = vec![];
        for row_idx in 0..rows_count {
            let row = &rows[row_idx];
            let column = row[col_idx];
            equation.push(column);
        }
        equations.push(equation);
    }

    let max_digit_places_for_columns: Vec<_> = equations
        .iter()
        .map(|equation| {
            let numbers = &equation[0..equation.len() - 1];
            let max_digit_places = numbers
                .iter()
                .max_by(|a, b| a.len().cmp(&b.len()))
                .unwrap()
                .len();
            return max_digit_places;
        })
        .collect();

    // Now we know how wide each column should be, reparse using these fixed width blocks so we can
    // columnize the numbers
    let lines: Vec<_> = input.lines().collect();
    let mut columns = vec![vec![]; columns_count];
    for line in &lines {
        let mut start_idx = 0;
        for column in 0..columns_count {
            let max_digits = &max_digit_places_for_columns[column];
            let column_substr = &line[start_idx..start_idx + *max_digits];
            columns[column].push(column_substr);
            start_idx += max_digits + 1;
        }
    }

    let mut equations = vec![];
    for (i, column) in columns.iter_mut().enumerate() {
        let max_digits = max_digit_places_for_columns[i];
        let mut equation: Vec<_> = vec![];
        for _ in 0..max_digits {
            let mut columnized_num = String::new();

            for num_idx in 0..column.len() - 1 {
                let num = &column[num_idx];
                let mut num = num.chars();
                let remainder = num.next_back().unwrap();
                if remainder != ' ' {
                    columnized_num.push(remainder);
                }

                column[num_idx] = num.as_str();
            }
            equation.push(columnized_num);
        }
        let operator: Vec<_> = column[column.len() - 1].split_whitespace().collect();
        let operator = operator[0];
        equation.push(operator.to_string());
        equations.push(equation);
    }
    let mut answers = vec![];
    for equation in &equations {
        let numbers: Vec<i64> = equation[0..equation.len() - 1]
            .iter()
            .map(|num| num.parse().unwrap())
            .collect();
        let operation = &equation[equation.len() - 1];
        let numbers = numbers.into_iter();
        let answer = match operation.as_str() {
            "+" => numbers.reduce(|acc, n| acc + n).unwrap(),
            "-" => numbers.reduce(|acc, n| acc - n).unwrap(),
            "*" => numbers.reduce(|acc, n| acc * n).unwrap(),
            "/" => numbers.reduce(|acc, n| acc / n).unwrap(),
            _ => unreachable!("Operator should be limited so basic math operators"),
        };
        answers.push(answer);
    }

    println!("Total {}", answers.iter().sum::<i64>());
}
