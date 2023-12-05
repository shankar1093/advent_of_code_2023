use std::char;
use std::fs;
use std::io;
use std::env;
use std::process;

fn read_file(path: &str) -> io::Result<String> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    // println!("File contents: \n{}", contents);

    Ok(contents)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a file path");
        process::exit(1);
    }

    let path = &args[1];
    let contents = read_file(path).unwrap();
    let mut holder: Vec<i32> = Vec::new();
    for line in contents.lines() {
        let mut temp: Vec<i32> = Vec::new();
        for character in line.chars() {
            if character.is_numeric() {

                temp.push(character.to_digit(10).expect("Char is not a digit") as i32);
                break;
            } 
        }
        for character in line.chars().rev() {
            if character.is_numeric() {
                temp.push(character.to_digit(10).expect("Char is not a digit") as i32);
                break;
            }
        }
        holder.push(temp[0] *10+temp[1]);
    }
    let sum: i32 = holder.iter().sum();
    println!("Sum: {}", sum)
}
