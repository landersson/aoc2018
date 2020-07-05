extern crate aoc;
use aoc::Result;

fn run() -> Result<()> {
    let mut input_file = std::env::current_exe()?.parent().unwrap().to_path_buf();
    input_file.push("../../data/input.txt");
    let box_ids = aoc::read_box_ids(input_file.as_path())?;
    //println!("n={}", box_ids.len());
    let (num_2, num_3) = aoc::count_ids(&box_ids);
    let checksum = num_2 * num_3;
    println!("{} * {} -> checksum {}", num_2, num_3, checksum);
    assert_eq!(checksum, 18060);
    let v = aoc::find_matching_boxes2(&box_ids).ok_or("KAKA")?;
    let s = String::from_utf8(v).unwrap();
    println!("{:?}", s);
    assert_eq!(s, "srijafjzloguvlntqmphenbkd");
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("ERROR: {}", e);
        std::process::exit(1);
    }
}
