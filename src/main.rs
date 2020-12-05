use std::io;
use std::fs::File;
use std::io::{BufReader, BufRead, Error, ErrorKind};
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    // day2_p2().unwrap();
    day4_p1().unwrap();

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

    return Err(Error::new(ErrorKind::NotFound, "No results"));
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

fn day3_p1() -> io::Result<u32> {
    let file = File::open("input/day3.txt")?;
    let reader = BufReader::new(file);

    let mut rows: Vec<String> = Vec::new();

    for line in reader.lines() {
        rows.push(line?);
    }

    let width = rows[0].len();
    let height = rows.len();

    println!("{} {}", width, height);
    let mut grid = vec![vec![' '; width]; height];

    for y in 0..height {
        let row = rows[y].as_bytes();
        for x in 0..width {
            grid[y][x] = row[x] as char;
        }
    }

    let mut x: usize = 0;
    let mut y: usize = 0;

    let mut count: u32 = 0;

    loop {
        x = (x + 3) % width;
        y += 1;

        if y >= height {
            break;
        }

        let c = grid[y][x];

        println!("{} {} {}", x, y, c);

        if c == '#' {
            count += 1;
        }
    }

    println!("Trees: {}", count);

    //

    Ok(count)
}

fn day3_p2() -> io::Result<u32> {
    let file = File::open("input/day3.txt")?;
    let reader = BufReader::new(file);

    let mut rows: Vec<String> = Vec::new();

    for line in reader.lines() {
        rows.push(line?);
    }

    let width = rows[0].len();
    let height = rows.len();

    println!("{} {}", width, height);
    let mut grid = vec![vec![' '; width]; height];

    for y in 0..height {
        let row = rows[y].as_bytes();
        for x in 0..width {
            grid[y][x] = row[x] as char;
        }
    }

    let slopes: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut result: u32 = 0;

    for slope in slopes.iter() {
        let mut x: usize = 0;
        let mut y: usize = 0;

        let mut count: u32 = 0;

        loop {
            x = (x + slope.0) % width;
            y += slope.1;

            if y >= height {
                break;
            }

            let c = grid[y][x];

            // println!("{} {} {}", x, y, c);

            if c == '#' {
                count += 1;
            }
        }

        println!("Trees ({}, {}): {}", slope.0, slope.1, count);

        result = if result == 0 { count } else { result * count };
    }

    println!("Product: {}", result);

    Ok(result)
}

fn day4_p1() -> io::Result<u16> {
    let file = File::open("input/day4.txt")?;
    let reader = BufReader::new(file);
    let re = Regex::new(r"([^\s]+):([^\s]+)").unwrap();

    let mut fields: HashMap<String, String> = HashMap::new();

    let mut count: u16 = 0;
    
    fn is_valid(fields: &HashMap<String, String>) -> bool {
        fields.contains_key("byr") &&
            fields.contains_key("iyr") &&
            fields.contains_key("eyr") &&
            fields.contains_key("hgt") &&
            fields.contains_key("hcl") &&
            fields.contains_key("ecl") &&
            fields.contains_key("pid")
    }

    for line in reader.lines() {
        let s = line.unwrap();

        if s == "" {
            if is_valid(&fields) {
                count += 1;
            }

            fields.clear();
            continue;
        }

        for x in re.captures_iter(&s) {
            fields.insert(x[1].to_string(), x[2].to_string());
        }
    }

    if is_valid(&fields) {
        count += 1;
    }

    println!("Valid: {}", count);

    Ok(count)
}
