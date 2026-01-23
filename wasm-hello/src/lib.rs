use wasm_bindgen::prelude::*;
use std::sync::{LazyLock, Mutex};

/// Simple state structure for the hello-wasm template
/// This demonstrates the state management pattern used throughout the project.
///
/// **Learning Point**: In Rust WASM, we can't have global mutable state directly.
/// Instead, we use `LazyLock<Mutex<State>>` which:
/// - `LazyLock`: Initializes the value on first access (lazy initialization)
/// - `Mutex`: Provides thread-safe access to mutable data
///
/// Even though WASM runs single-threaded, `Mutex` satisfies Rust's borrow checker
/// when we need mutable access to shared state across function calls.
struct HelloState {
    /// Counter value that can be incremented
    counter: i32,
    /// Message string that can be set and retrieved
    message: String,
    /// Favorite soccer team string that can be set and retrieved
    favorite_soccer_team: String,
}

impl HelloState {
    /// Create a new HelloState with default values
    fn new() -> Self {
        HelloState {
            counter: 0,
            message: String::from("Hello from WASM!"),
            favorite_soccer_team: String::from("FC Barcelona"),
        }
    }

    /// Get the current counter value
    fn get_counter(&self) -> i32 {
        self.counter
    }

    /// Increment the counter by 1
    fn increment_counter(&mut self) {
        self.counter += 1;
    }

    /// Get the current message
    fn get_message(&self) -> String {
        self.message.clone()
    }

    /// Set a new message
    fn set_message(&mut self, message: String) {
        self.message = message;
    }

    /// Get the current favorite soccer team
    fn get_favorite_soccer_team(&self) -> String {
        self.favorite_soccer_team.clone()
    }

    /// Set a new favorite soccer team
    fn set_favorite_soccer_team(&mut self, team: String) {
        self.favorite_soccer_team = team;
    }
}

/// Global state using the LazyLock<Mutex<State>> pattern
static HELLO_STATE: LazyLock<Mutex<HelloState>> = LazyLock::new(|| Mutex::new(HelloState::new()));

/// Initialize the WASM module
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

/// Initialize the hello-wasm module
///
/// @param initial_counter - Optional starting value for the counter (defaults to 0)
#[wasm_bindgen]
pub fn wasm_init(initial_counter: i32) {
    let mut state = HELLO_STATE.lock().unwrap();
    state.counter = initial_counter;
}

/// Get the current counter value
#[wasm_bindgen]
pub fn get_counter() -> i32 {
    let state = HELLO_STATE.lock().unwrap();
    state.get_counter()
}

/// Increment the counter by 1
#[wasm_bindgen]
pub fn increment_counter() {
    let mut state = HELLO_STATE.lock().unwrap();
    state.increment_counter();
}

/// Get the current message
#[wasm_bindgen]
pub fn get_message() -> String {
    let state = HELLO_STATE.lock().unwrap();
    state.get_message()
}

/// Set a new message
#[wasm_bindgen]
pub fn set_message(message: String) {
    let mut state = HELLO_STATE.lock().unwrap();
    state.set_message(message);
}

/// Get the current favorite soccer team
#[wasm_bindgen]
pub fn get_favorite_soccer_team() -> String {
    let state = HELLO_STATE.lock().unwrap();
    state.get_favorite_soccer_team()
}

/// Set a new favorite soccer team
#[wasm_bindgen]
pub fn set_favorite_soccer_team(team: String) {
    let mut state = HELLO_STATE.lock().unwrap();
    state.set_favorite_soccer_team(team);
}
