mod cpu;
mod font;

use cpu::Chip8;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Chip8Emulator {
    cpu: Chip8,
}

#[wasm_bindgen]
impl Chip8Emulator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { cpu: Chip8::new() }
    }

    pub fn load_rom(&mut self, data: &[u8]) -> Result<(), JsValue> {
        self.cpu.load_rom(data).map_err(|e| JsValue::from_str(e))
    }

    pub fn tick(&mut self) {
        self.cpu.tick();
    }

    pub fn tick_timers(&mut self) {
        self.cpu.tick_timers();
    }

    pub fn set_key(&mut self, key: usize, pressed: bool) {
        self.cpu.set_key(key, pressed);
    }

    pub fn is_beeping(&self) -> bool {
        self.cpu.is_beeping()
    }

    pub fn display_ptr(&self) -> *const u8 {
        self.cpu.display_ptr()
    }

    pub fn display_len(&self) -> usize {
        self.cpu.display_len()
    }
}