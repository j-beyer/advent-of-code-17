use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;

fn main()
{
    let mut in_file = File::open("1.in").expect("File not found!");
    let mut captcha = String::new();
    in_file.read_to_string(&mut captcha).expect("Reading the file failed!");

    let mut result: u32 = 0;
    let numbers: Vec<u32> = captcha.chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    let length = numbers.len();
    let halfway = length / 2;

    for i in 0..length - 1 {
        if numbers[i] == numbers[(i + halfway) % length] {
            result += numbers[i];
        }
    }

    println!("Result: {}", result);

}
