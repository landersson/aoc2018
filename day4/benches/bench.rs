#[macro_use]
extern crate bencher;
extern crate aoc;

use bencher::Bencher;


fn perf_2(b: &mut Bencher) {
    let box_ids = aoc::read_box_ids("../input.txt").unwrap();
    b.iter(|| {
        aoc::find_matching_boxes2(&box_ids);
    })
}
fn perf_3(b: &mut Bencher) {
    let box_ids = aoc::read_box_ids("../input.txt").unwrap();
    b.iter(|| {
        aoc::find_matching_boxes3(&box_ids);
    })
}

benchmark_group!(benches, perf_2, perf_3);
benchmark_main!(benches);
