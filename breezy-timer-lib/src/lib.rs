use std::collections::HashMap;
use cpu_time::ProcessTime;
use std::time::Duration;

pub type BreezyTimer = HashMap<&'static str, TimerState>;

/// Structure used to keep track of the current process time since the last `start_timer!()` call,
/// as well as the sum of all previously calculated times.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TimerState {
    process_time: ProcessTime,
    total_elapsed: Duration
}

impl TimerState {
    pub fn new() -> Self {
        TimerState {
            process_time: ProcessTime::now(),
            total_elapsed: Duration::new(0, 0)
        }
    }

    /// Helper function to add time up to a certain ProcessTime
    pub fn add_time(&mut self, up_to: ProcessTime){
        self.total_elapsed += up_to.duration_since(self.process_time);
    }

    /// Setter function to reset time to the given ProcessTime
    pub fn reset_time(&mut self, new_time: ProcessTime){
        self.process_time = new_time;
    }

    pub fn get_total_elapsed(&self) -> Duration {
        self.total_elapsed.clone()
    }
}

pub trait Timer {
    /// Creates or updates a timer with the provided name. The same timer can be started and stopped
    /// as many times as needed, and will keep track of the sum of all the time spent
    fn start(&mut self, _: &'static str) { return }

    /// Stops the timer with the provided name. The timer must already exist, or this call will panic.
    /// The same timer can be started and stopped as many times as needed, and will keep track of the
    /// sum of all the intervals
    fn stop(&mut self, _: &'static str) { return }

    fn elapsed(&self, _: &'static str) -> Option<Duration> { return None }
}

impl Timer for HashMap<&'static str, TimerState> {
    #[cfg(feature="breezy_timer")]
    fn start(&mut self, name: &'static str) {
        self.entry(name)
        .and_modify(|entry| entry.reset_time(ProcessTime::now()))
        .or_insert(TimerState::new());
    }

    #[cfg(feature="breezy_timer")]
    fn stop(&mut self, name: &'static str) {
        let before = ProcessTime::now();
        match self.get_mut(name) {
            // todo: is there no better way than this?
            //  problem with using the `log` crate is that the logger must be initialised upstream,
            //  which is  a big assumption. Given that the feature should not be active for production
            //  environment but only development, a print seems fair enough?
            None => println!("Warning: timer {} was stopped but does not exist", name),
            Some(entry) => {
                entry.add_time(before);
            }
        }
    }

    #[cfg(feature="breezy_timer")]
    fn elapsed(&self, name: &'static str) -> Option<Duration> {
        match self.get(&name) {
            None => None,
            Some(ts) => Some(ts.total_elapsed)
        }
    }
}