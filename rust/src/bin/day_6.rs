use rust::{get_input_file_path, read_input};

fn main() {
    let input_path = get_input_file_path().unwrap();
    let input = read_input(&input_path).unwrap();

    let lines: Vec<_> = input.lines().collect();

    let rows_count = lines.len();
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
    dbg!(&equations);

    let mut answers = vec![];
    for equation in &equations {
        // Parse numbers up to columns_count - 1

        dbg!(&equation);

        let numbers: Vec<i64> = equation[0..rows_count - 1]
            .iter()
            .map(|num| num.parse().unwrap())
            .collect();
        let operation = equation[rows_count - 1];
        let numbers = numbers.into_iter();
        let answer = match operation {
            "+" => numbers.reduce(|acc, n| acc + n).unwrap(),
            "-" => numbers.reduce(|acc, n| acc - n).unwrap(),
            "*" => numbers.reduce(|acc, n| acc * n).unwrap(),
            "/" => numbers.reduce(|acc, n| acc / n).unwrap(),
            _ => unreachable!("Operator should be limited so basic math operators"),
        };
        answers.push(answer);
    }

    dbg!(&answers);
    println!("Total {}", answers.iter().sum::<i64>());
}
