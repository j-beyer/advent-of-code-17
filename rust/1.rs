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

    for i in 0..numbers.len() - 2 {
        if numbers[i] == numbers[i + 1] {
            result += numbers[i];
        }
    }

    if numbers[0] == numbers[numbers.len() - 1] {
        result += numbers[0];
    }

    println!("Result: {}", result);

}
