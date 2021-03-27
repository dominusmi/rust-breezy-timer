# Breezy timer ⏲️

Breezy timer's objective is to be a very simple timing library, which can be put into 
production code without changing the final performance. See section 
[how does it work](#how-does-it-work) for further information.

## Aim
- simple to use
- use directly in production code, 
- no need to modify code when releasing, simply de-activate feature!


## Usage
Add dependency `Cargo.toml`
```
breezy-timer = "0.1.0"
```
When compiling, simply add the `feature` `breezy_timer` if you want to have the times, e.g.

``` cargo build foocrate --release --feature breezy_timer ``` 

or simply do not put the feature to remove the times.

## API
`prepare_timer!()`: must be called before any other timer related function

`start_timer!("foo")`: creates or adds to timer called `foo`, depending if it already exists

`stop_timer!("foo")`: computes the `ProcessTime` since the last `start_timer!("foor")` was called, and adds it to the timer state

`elapsed_ns!("foo")`: returns the sum of the nano secondes spent in the timer `foo`. When feature not active, returns `0u128`

`get_timers_map!()`: returns a clone of the `HashMap` containing all the timers and their `TimerState`: When feature not active, returns an empty `HashMap`

## Example
Taken from `examples/basic_example.rs`
```rust


use cpu_time::ProcessTime;
use criterion::black_box;

use breezy_timer::{prepare_timer, start_timer, stop_timer, elapsed_ns, get_timers_map};

// must be called before 
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
    println!(" - allocations: {}ns\n - sum: {}ns\n - total: {}ns", elapsed_ns!("allocations"), elapsed_ns!("sum"), elapsed_ns!("total"));
}
```

## How does it work
[`features`](https://doc.rust-lang.org/cargo/reference/features.html) are a rust compilation mechanism 
which allows you to do conditional compilation. This crate makes use of this together with 
`macros`, in order to make it so that a normal compilation (without the feature activated) 
will leave no trace of the library in the final code. Hence, there is no performance drop 
when releasing, making the transition between development to release painless.

### Structure
When activated, the library creates a global thread safe `HashMap` containing the names 
of the timers and their current state. The states are a structure which containes the last
`ProcessTime`, and the total number of nano seconds in the timer. 

When we call `start_timer("foo")`, what happens entry `foo` in the global hash map is created if 
it doesn't exist, or updated, and its start time is set to `ProcessTime::now()`.

When `stop_timer!("foo")` is called, the entry `foo` is fetched the elapsed time
since the last `start_timer!("foo")` is added to the total nanoseconds variable.


## License

This project is licensed under either of

    Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
    MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

## Contribution

PR requests are welcome highly welcome! 
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in globals by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
