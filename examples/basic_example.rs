use criterion::black_box;

use breezy_timer::{prepare_timer, start_timer, stop_timer, elapsed_ns, get_timers_map};

prepare_timer!();

fn main(){
    let mut vectors = Vec::new();

    start_timer!("total");
    for _ in 0..10 {
        start_timer!("allocations");
        let vec: Vec<u8> = (0..102400).map(|_| { rand::random::<u8>() }).collect();
        vectors.push(vec);
        stop_timer!("allocations");

        start_timer!("sum");
        let mut total = 0;
        for v in vectors.iter() {
            total += v.iter().map(|x| *x as u32).sum::<u32>();
        }
        // used so that compiler doesn't simply remove the loop because nothing is done with total
        black_box(total);
        stop_timer!("sum");
    }
    stop_timer!("total");

    println!("{:?}", get_timers_map!());
    println!("allocations: {}ns\nsum: {}ns\ntotal: {}ns", elapsed_ns!("allocations"), elapsed_ns!("sum"), elapsed_ns!("total"));
}