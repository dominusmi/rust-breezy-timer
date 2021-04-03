use criterion::black_box;
use breezy_timer_lib::{BreezyTimer, Timer};

fn main(){
    let mut btimer = BreezyTimer::new();
    let mut vectors = Vec::new();

    btimer.start("total");
    for _ in 0..10 {
        btimer.start("allocations");
        let vec: Vec<u8> = (0..102400).map(|_| { rand::random::<u8>() }).collect();
        vectors.push(vec);
        btimer.stop("allocations");

        btimer.start("sum");
        let mut total = 0;
        for v in vectors.iter() {
            total += v.iter().map(|x| *x as u32).sum::<u32>();
        }
        // used so that compiler doesn't simply remove the loop because nothing is done with total
        black_box(total);
        btimer.stop("sum");
    }
    btimer.stop("total");

    println!("{:?}", btimer);
}