
# Breezy timer ⏲️

Breezy timer's objective is to be a very simple timing library, which can be put into 
production code without changing the final performance. See section 
[how does it work](#how-does-it-work) for further information.

[![](https://img.shields.io/crates/v/breezy-timer.svg)](https://crates.io/crates/breezy-timer)
[![](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/breezy-timer/latest/breezy_timer/)

## Aim
- simple to use
- use directly in production code, 
- no need to modify code when releasing, simply de-activate feature!

__Note__: due to the structure, and the use of locking mechanism (thread safety), this package is not suitable for 
very high performance timings. It takes, on my average machine, about `500ns` per update (whether start or stop). 
If you try to time blocks of code which are of this order of magnitude of speed, the readings will be 
quite useless. Generally, if you want to benchmark pieces of code of that order of speed, you probably want to use individual benchmark files, with tools such 
as [Criterion](https://github.com/bheisler/criterion.rs).


## Usage
Add these lines to your `Cargo.toml`:
```
[dependencies]
breezy-timer = "0.1.2"

[features]
breezy_timer = ["breezy-timer/breezy_timer"]
```
When compiling, simply add the `feature` `breezy_timer` if you want to have the times, e.g.

``` cargo build foocrate --release --features breezy_timer ``` 

if the feature is not explicitely provided, all timers will disappear at compilation.

## API
`prepare_timer!()`: must be called before any other timer related function

`start_timer!("foo")`: creates or updates timer called `foo` to `ProcessTime::now()`

`stop_timer!("foo")`: computes the `ProcessTime` since the last `start_timer!("foo")` was called, and adds it to the timer state

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

## Future work
- Add `get_json!()`: macro to get the timers formatted in `json` of shape `{"timer-name": total_elapsed_ns}
- Check performance gain with simpler hasher (by default, `HashMap` uses DOS-safe hasher) 

## License

This project is licensed under either of

    Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
    MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

## Contribution

PR requests are welcome highly welcome! 

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in globals by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
