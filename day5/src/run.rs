use std::error;
use std::fs;
use std::io::{BufRead, BufReader};

pub type Result<T> = ::std::result::Result<T, Box<dyn error::Error>>;

// O(n^2) version
#[allow(dead_code)]
fn shrink_u8_1(input: &[u8]) -> Vec<u8> {
    let mut old = input.to_vec();
    let mut new: Vec<u8> = Vec::new();
    loop {
        let mut i = 0;
        while i < old.len() {
            if i < old.len() - 1 && (old[i] as i32 - old[i + 1] as i32).abs() == 32 {
                i += 1;
            } else {
                new.push(old[i]);
            }
            i += 1;
        }
        if new.len() == old.len() {
            break;
        }
        old = new.clone();
        new.clear();
    }
    new
}

// O(n) version
fn shrink_u8_2(input: &[u8]) -> Vec<u8> {
    if input.len() <= 1 {
        return input.to_vec();
    }
    let mut new: Vec<u8> = Vec::new();
    for next in 0..input.len() {
        if new.len() == 0 {
            new.push(input[next]);
        } else {
            let is_match = (*new.last().unwrap() as i32 - input[next] as i32).abs() == 32;

            if is_match {
                new.pop().unwrap();
            } else {
                new.push(input[next]);
            }
        }
    }
    new
}

fn u8_lowercase(ch: u8) -> u8 {
    if ch >= b'A' && ch <= b'Z' {
        return ch + 32;
    }
    ch
}

fn task1(input: &[u8]) {
    let shrunk = shrink_u8_2(input);
    println!("Shrunk length: {}", shrunk.len());
    assert_eq!(shrunk.len(), 10250);
}

fn task2(input: &[u8]) {
    let mut lengths_by_unit_removed: Vec<(u8, usize)> = Vec::new();

    for unit_to_remove in b'a'..=b'z' {
        let cleaned: Vec<u8> = input
            .iter()
            .filter(|&&ch| u8_lowercase(ch) != unit_to_remove)
            .cloned()
            .collect();

        let l = shrink_u8_2(&cleaned).len();

        lengths_by_unit_removed.push((unit_to_remove, l));

        println!("{} -> {}", unit_to_remove, l);
    }

    let min = lengths_by_unit_removed.iter().min_by_key(|x| x.1).unwrap();

    println!("{} -> {}", min.0 as char, min.1);
    assert_eq!(min.1, 6188);
}

fn run() -> Result<()> {
    let mut input_fn = std::env::current_exe()?.parent().unwrap().to_path_buf();
    input_fn.push("../../data/input.txt");

    let file = fs::File::open(&input_fn)?;
    let mut buffer = BufReader::new(file);
    let mut input = String::new();
    buffer.read_line(&mut input)?;
    let input = input.trim_end().as_bytes();

    task1(input);
    task2(input);

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("ERROR: {}", e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::{shrink_u8_1, shrink_u8_2};

    fn test(input: &str, expected: &str) {
        let r = shrink_u8_1(input.as_bytes());
        assert_eq!(r, expected.as_bytes());
        let r = shrink_u8_2(input.as_bytes());
        assert_eq!(r, expected.as_bytes());
    }

    #[test]
    fn empty_input() {
        test("", "");
    }

    #[test]
    fn single_char_input() {
        test("A", "A");
    }

    #[test]
    fn no_reduction() {
        test("Ab", "Ab");
    }

    #[test]
    fn reduction_0() {
        test("Aa", "");
    }
    #[test]
    fn reduction_1() {
        test("aAa", "a");
    }
    #[test]
    fn reduction_2() {
        test("zAa", "z");
    }
    #[test]
    fn reduction_3() {
        test("ABba", "");
    }
    #[test]
    fn reduction_4() {
        test("zAak", "zk");
    }
    #[test]
    fn reduction_5() {
        test("ABbak", "k");
    }
    #[test]
    fn reduction_6() {
        test("zABba", "z");
    }
    #[test]
    fn reduction_7() {
        test("zABbak", "zk");
    }
    #[test]
    fn reduction_8() {
        test("zABCcbak", "zk");
    }
    #[test]
    fn reduction_9() {
        test("bBtrlaALR", "t");
    }
    #[test]
    fn reduction_10() {
        test("EekK", "");
    }
}
