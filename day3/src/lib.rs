#![allow(dead_code)]
use std::boxed::Box;
use std::error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[macro_use]
extern crate scan_fmt;
extern crate nalgebra as na;

pub type Result<T> = ::std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct Claim {
    id: u32,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

pub fn read_claims(filename: &Path) -> Result<Vec<Claim>> {
    let file = File::open(filename).map_err(|e| {
        format!("Unable to open input file '{}': {}", filename.display(), e).to_string()
    })?;
    let file = BufReader::new(file);
    let mut claims = Vec::new();
    for line in file.lines() {
        let line = line?;
        match scan_fmt!(&line, "#{d} @ {d},{d}: {d}x{d}", u32, u32, u32, u32, u32) {
            Ok((id, x, y, w, h)) => claims.push(Claim { id, x, y, w, h }),
            Err(e) => return Err(e.into()),
        }
    }
    Ok(claims)
}

fn find_fabric_size(claims: &[Claim]) -> Option<(usize, usize)> {
    let max_right = claims.iter().max_by_key(|c| c.x + c.w)?;
    let max_bottom = claims.iter().max_by_key(|c| c.y + c.h)?;
    Some((
        (max_right.x + max_right.w) as usize,
        (max_bottom.y + max_bottom.h) as usize,
    ))
}

pub fn layout_claims(claims: &[Claim]) -> na::DMatrix<i32> {
    let (fw, fh) = find_fabric_size(claims).unwrap();
    println!("size: {}x{}", fw, fh);

    let mut fabric = na::DMatrix::<i32>::zeros(fh, fw);
    for c in claims {
        let (cx, cy) = (c.x as usize, c.y as usize);
        let (cw, ch) = (c.w as usize, c.h as usize);
        fabric.slice_mut((cy, cx), (ch, cw)).apply(|x| x + 1);
    }
    fabric
}

pub fn find_overallocated_cells(fabric: &na::DMatrix<i32>) -> usize {
    fabric.iter().map(|&x| (x > 1) as usize).sum()
}

pub fn find_nonoverlappig_rect(claims: &[Claim], fabric: &na::DMatrix<i32>) -> u32 {
    for c in claims {
        let (cx, cy) = (c.x as usize, c.y as usize);
        let (cw, ch) = (c.w as usize, c.h as usize);
        if fabric
            .slice((cy, cx), (ch, cw))
            .map(|x| x == 1)
            .fold(true, |acc, x| acc & x)
        {
            return c.id;
        }
    }
    0
}
