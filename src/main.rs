use std::io;
use std::fs::File;
use std::io::{BufReader, BufRead, Error, ErrorKind};
use itertools::Itertools;
use regex::Regex;

fn main() -> io::Result<()> {
    day2_p2().unwrap();

    return Ok(());
}

fn day1() -> io::Result<u32> {
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

            return Ok(left * mid * right);
        }
    }

    return Err(Error::new(ErrorKind::NotFound, "No results"))
    // Ok(())
}

fn day2_p1() -> io::Result<u16> {
    let file = File::open("input/day2.txt")?;
    let reader = BufReader::new(file);
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): (\w+)$").unwrap();
    let mut last: String;
    let mut valid: u16 = 0;

    for line in reader.lines() {
        last = line.unwrap();

        let cap = re.captures(&last).unwrap();

        let min = cap[1].parse::<u8>().unwrap();
        let max = cap[2].parse::<u8>().unwrap();
        let target = cap[3].parse::<char>().unwrap();
        let password = &cap[4];

        //

        let mut count: u8 = 0;

        for v in password.chars() {
            if v == target {
                count += 1;
            }
        }

        if min <= count && count <= max {
            println!("{}", password);
            valid += 1;
        }
    }

    println!("Valid passwords: {}", valid);

    //

    Ok(valid)
}

fn day2_p2() -> io::Result<u16> {
    let file = File::open("input/day2.txt")?;
    let reader = BufReader::new(file);
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): (\w+)$").unwrap();
    let mut last: String;
    let mut valid: u16 = 0;

    for line in reader.lines() {
        last = line.unwrap();

        let cap = re.captures(&last).unwrap();

        let pos1 = cap[1].parse::<u8>().unwrap();
        let pos2 = cap[2].parse::<u8>().unwrap();
        let target = cap[3].parse::<char>().unwrap();
        let password = &cap[4];

        //

        let mut count: u8 = 0;

        if password.len() < pos2 as usize {
            continue;
        }

        if password.as_bytes()[(pos1 - 1) as usize] as char == target {
            count += 1;
        }

        if password.as_bytes()[(pos2 - 1) as usize] as char == target {
            count += 1;
        }

        if count == 1 {
            valid += 1;
        }
    }

    println!("Valid passwords: {}", valid);

    //

    Ok(valid)
}
