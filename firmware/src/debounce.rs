// Simple debounce.
// approx 10ms given the scan period of 1ms
const COOLDOWN_TICKS: u64 = 50; 

// Derives copy for easy array initialization.
pub(crate) struct DebounceState {
    state: bool,
    earliest_next_change_clock: u64
}

impl DebounceState {
    pub fn update<F : FnMut()>(&mut self, signal_input: bool, clock: u64, mut press_action: F) -> bool {
        if signal_input != self.state && clock >= self.earliest_next_change_clock {
            // State has changed, and cooldown period since last change has expired.
            self.state = signal_input;
            self.earliest_next_change_clock = clock + COOLDOWN_TICKS;
            if self.state {
                press_action();
            }
        }
        self.state
    }
}

impl Default for DebounceState {
    fn default() -> Self {
        Self {
            state: false,
            earliest_next_change_clock: 0
        }
    }
}
