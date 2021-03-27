pub use std::collections::HashMap;
pub use global::Global;
use std::hash::Hash;
use cpu_time::ProcessTime;

/// Structure used to keep track of the current process time since the last `start_timer!()` call,
/// as well as the sum of all previously calculated times.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TimerState {
    process_time: ProcessTime,
    total_ns: u128
}

impl TimerState {
    pub fn new() -> Self {
        TimerState {
            process_time: ProcessTime::now(),
            total_ns: 0
        }
    }

    /// Helper function to add time up to a certain ProcessTime
    pub fn add_time(&mut self, up_to: ProcessTime){
        self.total_ns += up_to.duration_since(self.process_time).as_nanos();
    }

    /// Setter function to reset time to the given ProcessTime
    pub fn reset_time(&mut self, new_time: ProcessTime){
        self.process_time = new_time;
    }

    /// Getter function to the total_ns variable
    pub fn get_total_elapsed_ns(&self) -> u128 {
        self.total_ns
    }
}

/// Prepares the global variable `TIMERS`. Must be called before any other macro from this crate,
/// or the other calls will panic
#[macro_export]
macro_rules! prepare_timer {
    () => {
            #[cfg(feature="breezy_timer")]
            static TIMERS: global::Global<std::collections::HashMap<&'static str, $crate::TimerState>> = global::Global::new();
    }
}

/// Creates or updates a timer with the provided name. The same timer can be started and stopped
/// as many times as needed, and will keep track of the sum of all the time spent
#[macro_export]
macro_rules! start_timer {
    ( $x:expr ) => {
        #[cfg(feature="breezy_timer")]
        {
            TIMERS.with_mut(|hashmap| {
                hashmap.entry($x)
                    .and_modify(|entry| entry.reset_time(cpu_time::ProcessTime::now()))
                    .or_insert($crate::TimerState::new());
            });
        }
    };
}

/// Stops the timer with the provided name. The timer must already exist, or this call will panic.
/// The same timer can be started and stopped as many times as needed, and will keep track of the
/// sum of all the time spent
#[macro_export]
macro_rules! stop_timer {
    ( $x:expr ) => {
        #[cfg(feature="breezy_timer")]
        {
            let before = cpu_time::ProcessTime::now();
            TIMERS.with_mut(|hashmap| {
                let entry = hashmap.get_mut($x).unwrap();
                entry.add_time(before);
            });
        }
    };
}

/// Returns the amount of nanoseconds elapsed by the timer with the provided name
#[macro_export]
macro_rules! elapsed_ns {
    ( $x:expr) => {
        {
            #[cfg(feature="breezy_timer")]
            {
                TIMERS.with(|hashmap| hashmap[$x].get_total_elapsed_ns())
            }

            #[cfg(not(feature="breezy_timer"))]
            {
                0u128
            }
        }
    }
}

/// Helper function to clone a hashmap, needed by the macro `get_hashmap!()`
pub fn clone_hashmap<A: Clone+Eq+Hash, B: Clone+Eq>(hashmap: &HashMap<A, B>) -> HashMap<A, B> {
    let mut new_hashmap = HashMap::new();
    for (a,b) in hashmap.iter(){
        new_hashmap.insert(a.clone(), b.clone());
    }
    new_hashmap
}

/// Returns the hashmap containing each timer. The key corresponds to the timer name, and the value
/// is an instance of `TimerState`.
#[macro_export]
macro_rules! get_timers_map {
    () => {
        {
            #[cfg(feature="breezy_timer")]
            {
                TIMERS.with(|hashmap| $crate::clone_hashmap(hashmap))
            }
            #[cfg(not(feature="breezy_timer"))]
            {
                use std::collections::HashMap;
                HashMap::<&'static str, $crate::TimerState>::new()
            }
        }
    }
}

#[cfg(all(test, feature="breezy_timer"))]
mod tests {
    use cpu_time::ProcessTime;
    use criterion::black_box;

    #[test]
    fn check() {
        prepare_timer!();

        let start = ProcessTime::now();
        start_timer!("loop");
        assert!(TIMERS.lock().unwrap().contains_key("loop"));
        let mut total = 0;
        for _ in 0..100 {
            total += black_box(1);
        }
        black_box(total);
        stop_timer!("loop");
        let elapsed_ns = start.elapsed().as_nanos();

        let elapsed_macro = elapsed_ns!("loop");
        assert!(elapsed_macro > 0);
        assert!(elapsed_macro < elapsed_ns);
    }

    #[test]
    fn sanity_check(){
        let mut vectors = Vec::new();

        prepare_timer!();
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
        assert!(elapsed_ns!("allocations") < elapsed_ns!("total"));
        assert!(elapsed_ns!("sum") < elapsed_ns!("total"))
    }
}

#[cfg(all(test, not(feature="breezy_timer")))]
mod tests {
    use cpu_time::ProcessTime;
    use criterion::black_box;

    #[test]
    fn check_no_feature() {
        prepare_timer!();

        let start = ProcessTime::now();
        start_timer!("loop");
        let mut total = 0;
        for _ in 0..100 {
            total += black_box(1);
        }
        stop_timer!("loop");
        let elapsed_ns = start.elapsed().as_nanos();
        let elapsed_macro = elapsed_ns!("loop");
        assert!(elapsed_macro == 0);
    }
}
