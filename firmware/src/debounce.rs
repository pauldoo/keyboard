// Simple debounce.
const MIN: u8 = 0;
const MAX: u8 = 10;

// Derives copy for easy array initialization.
pub(crate) struct DebounceState {
    state_level: u8,
    last_transmitted_state: bool,
}

impl DebounceState {
    pub fn update(&mut self, signal_input: bool) -> bool {
        match (self.state_level, signal_input) {
            (MIN, false) => {
                // saturated
            }
            (MAX, true) => {
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
            (false, MAX) => {
                // value has saturated upward, press the key
                self.last_transmitted_state = true;
            }
            (true, MIN) => {
                // value has saturated downward, press the key
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
