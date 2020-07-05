extern crate aoc;
extern crate nalgebra as na;
use aoc::Result;

fn run() -> Result<()> {
    let mut input_file = std::env::current_exe()?.parent().unwrap().to_path_buf();
    input_file.push("../../data/input.txt");
    let claims = aoc::read_claims(input_file.as_path())?;
    let fabric = aoc::layout_claims(&claims);
    let num_overallocated = aoc::find_overallocated_cells(&fabric);
    println!("n={}", num_overallocated);
    let cell_index = aoc::find_nonoverlappig_rect(&claims, &fabric);

    println!("Cell #{} does not overlap", cell_index);
    assert_eq!(cell_index, 658);

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("ERROR: {}", e);
        std::process::exit(1);
    }
}
