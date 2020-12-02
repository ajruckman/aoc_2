use std::io;
use std::fs::File;
use std::io::{BufReader, BufRead};
use itertools::Itertools;

fn main() -> io::Result<()> {
    println!("Hello, world!");

    let file = File::open("input/day1.txt")?;
    let reader = BufReader::new(file);

    let mut vec: Vec<String> = Vec::new();

    for line in reader.lines() {
        // println!("{}", line?);
        vec.push(line?);
    }

    let s = vec.as_slice();

    for vpair in s.into_iter().combinations(2) {
        let left = vpair[0].parse::<u32>().unwrap();
        let right = vpair[1].parse::<u32>().unwrap();

        if left + right == 2020 {
            println!("{} {} -> {}", left, right, left * right);
            break;
        }
    }

    for vpair in s.into_iter().combinations(3) {
        let left = vpair[0].parse::<u32>().unwrap();
        let mid = vpair[1].parse::<u32>().unwrap();
        let right = vpair[2].parse::<u32>().unwrap();

        if left + mid + right == 2020 {
            println!("{} {} {} -> {}", left, mid, right, left * mid * right);
            break;
        }
    }

    Ok(())
}
