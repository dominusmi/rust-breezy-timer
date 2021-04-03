
# Breezy timer ⏲️

Breezy timer's objective is to be a very simple timing library, which can be put into 
production code without changing the final performance. See section 
[how does it work](#how-does-it-work) for further information.

[![](https://img.shields.io/crates/v/breezy-timer.svg)](https://crates.io/crates/breezy-timer)
[![](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/breezy-timer/latest/breezy_timer/)

## Aim
- simple & fast
- use directly in production code, 
- no need to modify code when releasing, simply de-activate feature!

## Usage
Add these lines to your `Cargo.toml`:
```
[dependencies]
breezy-timer = "1.0.0"

[features]
breezy_timer = ["breezy-timer/breezy_timer"]
```
When compiling, simply add the `feature` `breezy_timer` if you want to have the times, e.g.

``` cargo build foocrate --release --features breezy_timer ``` 

if the feature is not explicitely provided, all timers will disappear at compilation.

## API
`start("foo")`: creates or updates timer called `foo` to `ProcessTime::now()`

`stop("foo")`: computes the `ProcessTime` since the last `start("foo")` was called, and adds it to the timer state

`elapsed("foo")`: returns `Option<Duration>`, the summed duration of all intervals of timer `foo`. When feature not active, returns `None`


## Example
Taken from `examples/basic_example.rs`
```rust

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
```

### Benchmarks
There is also a benchmark file to test the difference when feature is enabled and
disabled. You will notice that when disabled, the timings are identical to non-timed
code.

```
Usage:
cargo bench --features breezy_timer
cargo bench
``` 


## How does it work
[`features`](https://doc.rust-lang.org/cargo/reference/features.html) are a rust compilation mechanism 
which allows you to do conditional compilation. This crate makes use of that together
with the compiler's ability to optimise "useless" code. When the feature is not 
active, all the functions become dummy, and so the compiler will simply remove
them. Hence, there is no performance drop when releasing, making the transition 
between development to release painless.

### Structure
The `BreezyTimer` typer is just an alias for `HashMap<&'static str, TimerState>`. The
`TimerState` object is used to keep track of the current interval, as well as the 
sum of the durations of all previous ones. 

## Future work
- Add `get_json()` function, to get the timers formatted in `json` of shape `{"timer-name": total_elapsed_ns}
- Add GlobalBreezyTimer, together with function based timing using 
procedural macro and a global BreezyTimer
- Check performance gain with simpler hasher (by default, `HashMap` uses DOS-safe hasher) 

## License

This project is licensed under either of

    Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
    MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

## Contribution

PR requests are welcome highly welcome! 

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in globals by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
