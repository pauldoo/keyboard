// Simple debounce.
const DEBOUNCE_WIDTH: u8 = 5u8;
const MIN: u8 = DEBOUNCE_WIDTH * 0;
const LOWER: u8 = DEBOUNCE_WIDTH * 1;
const UPPER: u8 = DEBOUNCE_WIDTH * 2;
const MAX: u8 = DEBOUNCE_WIDTH * 3;

// Derives copy for easy array initialization.
#[derive(Clone, Copy)]
pub(crate) struct DebounceState {
    state_level: u8,
    last_transmitted_state: bool,
}

impl DebounceState {
    pub fn update(&mut self, signal_input: bool) -> bool {
        match (self.state_level, signal_input) {
            (n, false) if n == MIN => {
                // saturated
            }
            (n, true) if n == MAX => {
                // saturated
            }
            (_, false) => {
                self.state_level -= 1;
            }
            (_, true) => {
                self.state_level += 1;
            }
        }

        match (self.last_transmitted_state, self.state_level) {
            (false, n) if (n >= UPPER) => {
                // value is in the upper 1/3 now, press the key
                self.last_transmitted_state = true;
            }
            (true, n) if (n <= LOWER) => {
                // value is in the lower 1/3 now, release the key
                self.last_transmitted_state = false;
            }
            _ => {}
        }
        self.last_transmitted_state
    }
}

impl Default for DebounceState {
    fn default() -> Self {
        Self {
            state_level: MIN,
            last_transmitted_state: false,
        }
    }
}
