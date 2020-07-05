#![allow(dead_code)]
use std::boxed::Box;
use std::collections::HashSet;
use std::error;
use std::fs::File;
use std::io::{BufRead, BufReader};

extern crate itertools;
use itertools::Itertools;

type Result<T> = ::std::result::Result<T, Box<dyn error::Error>>;

fn parse_int((line_no, text): (usize, String)) -> Result<i32> {
    text.parse::<i32>()
        .map_err(|e| format!("Can't parse int from '{}' on line {}: {}", text, line_no, e).into())
}

// iterator with error, Result Sum version
fn sum_up_file_map(reader: impl BufRead) -> Result<i32> {
    reader
        .lines()
        .map(|x| Ok(x?.parse::<i32>()?))
        //.map(|res| res.map(|x| x + 1))
        .sum()
}

fn sum_up_file_fold(reader: impl BufRead) -> Result<i32> {
    reader
        .lines()
        .try_fold(0, |acc, line| Ok(acc + line?.parse::<i32>()?))
}

fn sum_up_file_loop(reader: impl BufRead) -> Result<i32> {
    let mut sum = 0;
    for line in reader.lines() {
        let x = line?.parse::<i32>()?;
        sum += x + 1;
    }
    Ok(sum)
}

pub fn div_rem<T: std::ops::Div<Output = T> + std::ops::Rem<Output = T> + Copy>(
    x: T,
    y: T,
) -> (T, T) {
    let quot = x / y;
    let rem = x % y;
    (quot, rem)
}

fn find_first_repeat_1(numbers: &Vec<i32>) -> Option<i32> {
    let mut sum = 0;
    let mut seen_numbers = HashSet::new();
    let N = numbers.len();
    for n in numbers.iter().cycle() {
        //println!("{} {}", n, sum);
        if seen_numbers.contains(&sum) {
            return Some(sum);
        //println!("First twice = {}", sum);
        //break;
        } else {
            seen_numbers.insert(sum);
        }
        sum += n;
    }
    return None;
}

fn find_first_repeat_2(numbers: &Vec<i32>) -> Option<i32> {
    // compute cumulative sum
    let sums: Vec<i32> = numbers
        .iter()
        .scan(0, |sum, n| {
            *sum += n;
            Some(*sum)
        })
        .collect();

    let total_sum = *sums.last()?;
    let mut rep_val = None;
    let mut rep_ind = std::usize::MAX;

    for i in 0..sums.len() {
        for j in 0..sums.len() {
            if i == j {
                continue;
            }
            let (q, r) = div_rem(sums[i] - sums[j], total_sum);
            if r == 0 && q >= 0 {
                //println!("{} {} {} {}", i, j, q, r);
                let ind = numbers.len() * q as usize + j;
                if ind < rep_ind {
                    rep_ind = ind;
                    rep_val = Some(sums[i]);
                }
            }
        }
    }
    return rep_val;
}

fn run() -> Result<()> {
    let mut input_file = std::env::current_exe()?.parent().unwrap().to_path_buf();
    input_file.push("../../data/input.txt");
    let f = File::open(&input_file).map_err(|e| {
        format!(
            "Unable to open input file '{}': {}",
            input_file.display(),
            e
        )
        .to_string()
    })?;
    let f = BufReader::new(f);
    //for line in f.lines().map(|x| x.unwrap()).enumerate() {
    let numbers: Vec<i32> = f
        .lines()
        .map(|x| x.unwrap().parse::<i32>().unwrap())
        .collect();
    match find_first_repeat_2(&numbers) {
        Some(n) => println!("First repeated sum is: {}", n),
        None => println!("There is no repeated sum!"),
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("ERROR: {}", e);
        std::process::exit(1);
    }
}
