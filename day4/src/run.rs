extern crate aoc;
use aoc::{Action, Note, Result};

use itertools::Itertools;
use std::collections::HashMap;

type MinuteHistogram = [u32; 60];

//struct MinuteHistogram
type MinuteRange = (usize, usize);

//impl Default for MinuteRange {
//fn default() -> [0; 60]
//}

fn analyse_shift<'a>(it: impl Iterator<Item = &'a Note>) -> Vec<MinuteRange> {
    //let mut min_hist: MinuteHistogram = [0; 60];
    let mut ranges: Vec<MinuteRange> = Vec::new();
    let v: Vec<&'a Note> = it.take_while(|n| !n.action.is_begin_shift()).collect();

    assert!(v.len() % 2 == 0);
    for (start, end) in v.iter().tuples() {
        assert_eq!(start.action, Action::FallAsleep);
        assert_eq!(end.action, Action::WakeUp);
        //println!("{:?} - {:?}", start.minute, end.minute);
        ranges.push((start.minute as usize, end.minute as usize));
    }
    ranges
}

fn make_minute_histograms(notes: Vec<Note>) -> HashMap<u32, MinuteHistogram> {
    let mut minute_histograms: HashMap<u32, MinuteHistogram> = HashMap::new();

    let mut it = notes.iter();
    while let Some(note) = it.next() {
        if let Action::BeginShift(id) = note.action {
            let ranges = analyse_shift(it.clone());

            for (start, end) in ranges {
                let entry = minute_histograms.entry(id).or_insert([0; 60]);
                for m in &mut entry[start..end] {
                    *m += 1;
                }
            }
        }
    }
    minute_histograms
}

fn find_max_index<'a>(mins: &MinuteHistogram) -> u32 {
    let (index, _) = mins.iter().enumerate().max_by_key(|x| x.1).unwrap();
    index as u32
}

struct HistogramItem {
    index: u32,
    value: u32,
}

fn find_max_item<'a>(mins: &MinuteHistogram) -> HistogramItem {
    let index = find_max_index(mins);
    HistogramItem {
        index,
        value: mins[index as usize],
    }
}

fn task_1(minute_histograms: &HashMap<u32, MinuteHistogram>) -> u32 {
    struct MinuteSum {
        id: u32,
        sum: u32,
    }
    let sums = minute_histograms.iter().map(|(id, minutes)| MinuteSum {
        id: *id,
        sum: minutes.iter().sum::<u32>(),
    });

    let sleepiest = sums.max_by_key(|x| x.sum).unwrap();
    let minutes = minute_histograms.get(&sleepiest.id).unwrap();
    let max_index = find_max_index(minutes);
    max_index * sleepiest.id
}

fn task_2(minute_histograms: &HashMap<u32, MinuteHistogram>) -> u32 {
    let maxs = minute_histograms
        .iter()
        .map(|(id, minutes)| (id, find_max_item(minutes)));

    let max_id = maxs.max_by_key(|x| (x.1).value).unwrap();

    let guard_id = max_id.0;
    let max_minute = (max_id.1).index;
    guard_id * max_minute
}

fn run() -> Result<()> {
    let mut input_file = std::env::current_exe()?.parent().unwrap().to_path_buf();
    input_file.push("../../data/input.txt");
    let notes = aoc::read_notes(input_file.as_path())?;

    let minute_histograms = make_minute_histograms(notes);

    let answer_1 = task_1(&minute_histograms);
    println!("Answer 1: {}", answer_1);
    assert_eq!(answer_1, 106710);

    let answer_2 = task_2(&minute_histograms);
    println!("Answer 2: {}", answer_2);
    assert_eq!(answer_2, 10491);

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("ERROR: {}", e);
        std::process::exit(1);
    }
}
