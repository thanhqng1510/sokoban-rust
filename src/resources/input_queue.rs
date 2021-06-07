use ggez::event::KeyCode;
use std::collections::VecDeque;


pub struct InputQueue {
    keys_pressed: VecDeque<KeyCode>
}

impl InputQueue {
    const MAX_LEN: usize = 20;

    pub fn new() -> Self {
        InputQueue {
            keys_pressed: VecDeque::new()
        }
    }

    pub fn push(&mut self, keycode: KeyCode) {
        if self.keys_pressed.len() >= Self::MAX_LEN { self.keys_pressed.pop_front().unwrap(); }
        self.keys_pressed.push_back(keycode);
    }

    pub fn pop(&mut self) -> Option<KeyCode> {
        self.keys_pressed.pop_front()
    }

    pub fn clear(&mut self) {
        self.keys_pressed.clear();
    }
}
