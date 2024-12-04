mod day1;
mod day2;
mod day3;

use std::{env, fs, path::PathBuf};

fn main() {
    let day = 3;
    let filename = format!("day{}.txt", day);
    let content = get_input(&filename);
    if day == 1 {
        day1::run1(content.clone());
        day1::run2(content);
    } else if day == 2 {
        day2::run1(content.clone());
        day2::run2(content);
    } else if day == 3 {
        day3::run1(content.clone());
        day3::run2(content);
    }
}


pub fn get_input(file_name: &str) -> String {
    let mut relative_input_path: String = "input_files\\".to_owned();
    relative_input_path.push_str(file_name);
    let mut path_to_input = PathBuf::from(env::current_dir().unwrap().as_path());
    path_to_input.push(relative_input_path.clone());
    println!("{}", path_to_input.display());
    let content = fs::read_to_string(relative_input_path).unwrap();
    return content;
}