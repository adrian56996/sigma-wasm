use wasm_bindgen::prelude::*;
use std::sync::{LazyLock, Mutex};

/// Constant favorite soccer team (not part of mutable state)
const FAVORITE_SOCCER_TEAM: &str = "FC Barcelona";

struct HelloState {
    counter: i32,
    message: String,
    gum: String,
}

impl HelloState {
    fn new() -> Self {
        HelloState {
            counter: 0,
            message: String::from("Rust WASM is so Sigma!"),
            gum: String::from("Hubba Bubba"),
        }
    }

    fn get_counter(&self) -> i32 {
        self.counter
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }

    fn get_message(&self) -> String {
        self.message.clone()
    }

    fn set_message(&mut self, message: String) {
        self.message = message;
    }

    fn get_gum(&self) -> String {
        self.gum.clone()
    }

    fn set_gum(&mut self, gum: String) {
        self.gum = gum;
    }
}

static HELLO_STATE: LazyLock<Mutex<HelloState>> =
    LazyLock::new(|| Mutex::new(HelloState::new()));

#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn wasm_init(initial_counter: i32) {
    let mut state = HELLO_STATE.lock().unwrap();
    state.counter = initial_counter;
}

#[wasm_bindgen]
pub fn get_counter() -> i32 {
    HELLO_STATE.lock().unwrap().get_counter()
}

#[wasm_bindgen]
pub fn increment_counter() {
    HELLO_STATE.lock().unwrap().increment_counter();
}

#[wasm_bindgen]
pub fn get_message() -> String {
    HELLO_STATE.lock().unwrap().get_message()
}

#[wasm_bindgen]
pub fn set_message(message: String) {
    HELLO_STATE.lock().unwrap().set_message(message);
}

/// Get the favorite soccer team (constant)
#[wasm_bindgen]
pub fn get_favorite_soccer_team() -> String {
    FAVORITE_SOCCER_TEAM.to_string()
}

#[wasm_bindgen]
pub fn get_gum() -> String {
    HELLO_STATE.lock().unwrap().get_gum()
}

#[wasm_bindgen]
pub fn set_gum(gum: String) {
    HELLO_STATE.lock().unwrap().set_gum(gum);
}
