#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate regex;

use chrono::NaiveDateTime;
use chrono::Timelike;
use regex::Regex;
use std::boxed::Box;
use std::error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub type Result<T> = ::std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, PartialEq)]
pub enum Action {
    BeginShift(u32),
    FallAsleep,
    WakeUp,
}
#[derive(Debug)]
pub struct Note {
    pub timestamp: i64,
    pub minute: u32,
    pub action: Action,
}

impl Action {
    pub fn is_begin_shift(&self) -> bool {
        match self {
            Action::BeginShift(_) => true,
            _ => false,
        }
    }
}

fn parse_action(line: &str) -> Result<Action> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"Guard #(\S+) begins.*").unwrap();
    }
    if let Some(caps) = RE.captures(line) {
        let id: u32 = caps.get(1).unwrap().as_str().parse()?;
        return Ok(Action::BeginShift(id));
    } else if line == "wakes up" {
        return Ok(Action::WakeUp);
    } else if line == "falls asleep" {
        return Ok(Action::FallAsleep);
    }
    Err("Invalid action".into())
}

fn parse_note(line: &str) -> Result<Note> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\[(.+)\] (.+)").unwrap();
    }
    let caps = RE.captures(line).ok_or("Parse error")?;
    let date = &caps[1];
    let action = &caps[2];
    let date_time = NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M")?;
    let action = parse_action(action)?;
    Ok(Note {
        timestamp: date_time.timestamp(),
        minute: date_time.minute(),
        action: action,
    })
}

pub fn read_notes(filename: &Path) -> Result<Vec<Note>> {
    let file = File::open(filename).map_err(|e| {
        format!("Unable to open input file '{}': {}", filename.display(), e).to_string()
    })?;
    let file = BufReader::new(file);
    let mut notes = Vec::new();
    for line in file.lines() {
        let line = line?;

        match parse_note(&line) {
            Ok(n) => notes.push(n),
            Err(e) => return Err(e.into()),
        }
    }
    notes.sort_by_key(|note| note.timestamp);

    Ok(notes)
}
