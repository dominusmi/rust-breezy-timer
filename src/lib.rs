pub use breezy_timer_lib::{TimerState, BreezyTimer, Timer};

pub use cpu_time::ProcessTime;
pub use std::collections::HashMap;


#[cfg(all(test, feature="breezy_timer"))]
mod tests {
    use cpu_time::ProcessTime;
    use criterion::black_box;
    use std::collections::HashMap;
    use breezy_timer_lib::{BreezyTimer, TimerState, Timer};

    #[test]
    fn check() {
        let mut btimer = BreezyTimer::new();

        let start = ProcessTime::now();
        btimer.start("loop");

        assert!(btimer.contains_key("loop"));
        let mut total = 0;
        for _ in 0..100 {
            total += black_box(1);
        }
        black_box(total);
        btimer.stop("loop");

        let elapsed_ns = start.elapsed().as_nanos();

        let elapsed_breezy = btimer.elapsed("loop");
        assert!(elapsed_breezy.is_some());
        let elapsed_breezy = elapsed_breezy.unwrap().as_nanos();

        assert!(elapsed_breezy > 0);
        assert!(elapsed_breezy < elapsed_ns);
    }

    #[test]
    fn sanity_check(){
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
        assert!(btimer.elapsed("allocations").unwrap() < btimer.elapsed("total").unwrap());
        assert!(btimer.elapsed("sum").unwrap() < btimer.elapsed("total").unwrap())
    }
}

#[cfg(all(test, not(feature="breezy_timer")))]
mod tests {
    use criterion::black_box;
    use breezy_timer_lib::{BreezyTimer, Timer};

    #[test]
    fn check_no_feature() {
        let mut btimer = BreezyTimer::new();
        btimer.start("loop");
        let mut total = 0;
        for _ in 0..100 {
            total += 1;
        }
        black_box(total);
        btimer.stop("loop");
        assert!(btimer.elapsed("loop").is_none());
    }
}
