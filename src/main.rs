use std::io;
use std::fs::File;
use std::io::{BufReader, BufRead, Error, ErrorKind};
use itertools::Itertools;
use regex::{Regex, Captures};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    // day2_p2().unwrap();
    day5_p2().unwrap();

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

fn day4_p2() -> io::Result<u16> {
    let file = File::open("input/day4.txt")?;
    let reader = BufReader::new(file);
    let re = Regex::new(r"([^\s]+):([^\s]+)").unwrap();

    //

    let v_hgt = &Regex::new(r"^(\d+)(cm|in)$").unwrap();
    let v_hcl = &Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let v_ecl = &Regex::new(r"^(?:amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    let v_pid = &Regex::new(r"^[0-9]{9}$").unwrap();

    //

    let mut fields: HashMap<String, String> = HashMap::new();

    let mut count: u16 = 0;

    let is_valid = |fields: &HashMap<String, String>| -> bool {
        let has_all = fields.contains_key("byr") &&
            fields.contains_key("iyr") &&
            fields.contains_key("eyr") &&
            fields.contains_key("hgt") &&
            fields.contains_key("hcl") &&
            fields.contains_key("ecl") &&
            fields.contains_key("pid");

        if !has_all {
            return false;
        }

        let byr = fields["byr"].parse::<u16>().unwrap();
        if !(byr >= 1920 && byr <= 2002) { return false; }
        // if 1920 > byr || byr > 2002 { return false; }

        let iyr = fields["iyr"].parse::<u16>().unwrap();
        if !(iyr >= 2010 && iyr <= 2020) { return false; }
        // if 2010 > iyr || iyr > 2020 { return false; }

        let eyr = fields["eyr"].parse::<u16>().unwrap();
        if !(eyr >= 2020 && eyr <= 2030) { return false; }
        // if 2020 > eyr || eyr > 2030 { return false; }

        match v_hgt.captures(&fields["hgt"]) {
            None => return false,
            Some(m) => {
                let u = &m[2];
                let v = m[1].parse::<u16>().unwrap();

                if u == "cm" {
                    if 150 > v || v > 193 { return false; }
                } else if u == "in" {
                    if 59 > v || v > 76 { return false; }
                } else {
                    return false;
                }
                println!("{} {}", &m[1], &m[2])
            }
        }

        let pid = v_pid.is_match(&fields["pid"]);
        if !pid { return false; }

        let hcl = v_hcl.is_match(&fields["hcl"]);
        if !hcl { return false; }

        let ecl = v_ecl.is_match(&fields["ecl"]);
        if !ecl { return false; }

        true
    };


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
            if fields.contains_key(&x[1].to_string()) {
                println!("DUPE: {}", &x[1].to_string());
            }
            fields.insert(x[1].to_string(), x[2].to_string());
        }
    }

    if is_valid(&fields) {
        count += 1;
    }

    println!("Valid: {}", count);

    Ok(count)
}

fn day5_p1() -> io::Result<u32> {
    let file = File::open("input/day5.txt")?;
    let reader = BufReader::new(file);

    let r = reader.lines()
        .map(|v| v.unwrap())
        .map(|v| {
            let mut x = v;
            x = x.replace("F", "0");
            x = x.replace("B", "1");
            x = x.replace("L", "0");
            x = x.replace("R", "1");

            x
        })
        .map(|v| u32::from_str_radix(&v, 2).unwrap())
        .max()
        .unwrap();

    println!("Max seat ID: {}", r);

    Ok(r)
}

fn day5_p2() -> io::Result<u32> {
    let file = File::open("input/day5.txt")?;
    let reader = BufReader::new(file);

    let seat_ids = reader.lines()
        .map(|v| v.unwrap())
        .map(|v| {
            let mut x = v;
            x = x.replace("F", "0");
            x = x.replace("B", "1");
            x = x.replace("L", "0");
            x = x.replace("R", "1");

            x
        })
        .map(|v| u32::from_str_radix(&v, 2).unwrap())
        .sorted()
        .collect_vec();

    let min = seat_ids.iter().min().unwrap();
    let max = seat_ids.iter().max().unwrap();

    let mut last = min;

    for i in seat_ids.iter() {
        if i - last > 1 {
            println!("Seat: {}", i - 1);
            break;
        }

        if i == max {
            println!("No seat IDs found!");
            break;
        }

        last = i;
    }

    Ok(0)
}
