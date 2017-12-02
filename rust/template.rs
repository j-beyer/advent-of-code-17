use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;

fn main()
{
    let mut in_file = File::open("X.in").unwrap();
    let mut string = String::new();
    in_file.read_to_string(&mut string).unwrap();

    let mut result: u32 = 0;

    println!("Result: {}", result);

}
