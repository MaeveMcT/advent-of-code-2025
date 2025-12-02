use std::env;

pub fn get_input_file_path() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide an input file path");
        return None;
    }
    return Some(args[1].clone());
}

pub fn read_input(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}
