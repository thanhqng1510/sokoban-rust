use ggez::event::KeyCode;
use std::collections::VecDeque;


#[derive(Default)]
pub struct InputQueue {
    keys_pressed: VecDeque<KeyCode>
}

impl InputQueue {
    const MAX_LEN: usize = 50;

    pub fn push(&mut self, keycode: KeyCode) {
        if self.keys_pressed.len() >= Self::MAX_LEN { self.keys_pressed.pop_front().unwrap(); }
        self.keys_pressed.push_back(keycode);
    }

    pub fn pop(&mut self) -> Option<KeyCode> {
        self.keys_pressed.pop_front()
    }
}
