#![allow(dead_code)]
use std::boxed::Box;
use std::error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub type Result<T> = ::std::result::Result<T, Box<dyn error::Error>>;

pub fn read_box_ids(filename: &Path) -> Result<Vec<Vec<u8>>> {
    let f = File::open(filename).map_err(|e| {
        format!("Unable to open input file '{}': {}", filename.display(), e).to_string()
    })?;
    let f = BufReader::new(f);
    let box_ids: Vec<String> = f.lines().collect::<std::result::Result<_, _>>()?;
    let box_ids: Vec<Vec<u8>> = box_ids.into_iter().map(|s| s.into_bytes()).collect();
    Ok(box_ids)
}

pub fn count_ids(ids: &Vec<Vec<u8>>) -> (usize, usize) {
    let mut nums = (0, 0);

    for id in ids {
        let mut histogram = [0i32; 256];
        //for b in id.into_iter() {
        for b in id {
            histogram[*b as usize] += 1;
        }

        for x in histogram.iter() {
            if *x == 2i32 {
                nums.0 += 1;
            } else if *x == 3i32 {
                nums.1 += 1
            }
        }
    }
    nums
}

// SLOW: 1975 ns
pub fn find_matching_boxes1(ids: &Vec<Vec<u8>>) -> Option<Vec<u8>> {
    for i in 0..ids.len() {
        for j in i + 1..ids.len() {
            let tups: Vec<_> = ids[i].iter().zip(&ids[j]).map(|(&a, &b)| (a, b)).collect();
            if 1 == tups.iter().map(|(a, b)| (a != b) as usize).sum::<usize>() {
                return Some(tups.iter().filter(|(a, b)| a == b).map(|x| x.0).collect());
            }
        }
    }
    None
}

// FASTEST: 177 ns
pub fn find_matching_boxes2(ids: &Vec<Vec<u8>>) -> Option<Vec<u8>> {
    for i in 0..ids.len() {
        for j in i + 1..ids.len() {
            let tups = ids[i].iter().zip(&ids[j]).map(|(&a, &b)| (a, b));
            if 1 == tups.clone().map(|(a, b)| (a != b) as i32).sum() {
                return Some(tups.filter(|(a, b)| a == b).map(|x| x.0).collect());
            }
        }
    }
    None
}

// C: 380 ns
pub fn find_matching_boxes3(ids: &Vec<Vec<u8>>) -> Option<Vec<u8>> {
    for i in 0..ids.len() {
        for j in i + 1..ids.len() {
            let mut num_diffs = 0;
            for (a, b) in ids[i].iter().zip(&ids[j]) {
                if *a != *b {
                    num_diffs += 1;
                    if num_diffs > 1 {
                        break;
                    }
                }
            }
            if num_diffs == 1 {
                return Some(ids[i].clone());
            }
        }
    }
    None
}
