use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;

fn main()
{
    let in_file = File::open("2.in").unwrap();
    let buf_reader = BufReader::new(in_file);

    let mut result: u32 = 0;

    let rows = buf_reader.lines()
        .map(|r| r.unwrap())
        .filter(|r| !r.is_empty());

    for row in rows {
        let numbers: Vec<u32> = row.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        result += even_division_result(&numbers);
    }
    
    println!("Result: {}", result);

}

fn even_division_result(numbers: &Vec<u32>) -> u32
{
    for left in numbers {
        for right in numbers {
            if left != right && left % right == 0 {
                return left / right;
            }
        }
    }
    return 0;
}
