use crate::font::FONTSET;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
pub const RAM_SIZE: usize = 4096;
pub const ROM_START_ADDR: usize = 0x200;

pub struct Chip8 {
    ram: [u8; RAM_SIZE],
    display: [u8; SCREEN_WIDTH * SCREEN_HEIGHT],
    pc: u16,
    v: [u8; 16],
    i: u16,
    stack: [u16; 16],
    sp: u8,
    delay_timer: u8,
    sound_timer: u8,
    keypad: [bool; 16],
    rng_state: u32,
}

impl Chip8 {
    pub fn new() -> Self {
        let mut ram = [0u8; RAM_SIZE];
        ram[..FONTSET.len()].copy_from_slice(&FONTSET);

        Self {
            ram,
            display: [0u8; SCREEN_WIDTH * SCREEN_HEIGHT],
            pc: ROM_START_ADDR as u16,
            v: [0u8; 16],
            i: 0,
            stack: [0u16; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: [false; 16],
            rng_state: 0x1234_5678,
        }
    }

    pub fn load_rom(&mut self, data: &[u8]) -> Result<(), &'static str> {
        let end_addr = ROM_START_ADDR + data.len();
        if end_addr > self.ram.len() {
            return Err("ROM is too large to fit in CHIP-8 RAM");
        }

        self.display.fill(0);
        self.pc = ROM_START_ADDR as u16;
        self.sp = 0;
        self.i = 0;
        self.v.fill(0);
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.keypad.fill(false);
        self.ram[ROM_START_ADDR..].fill(0);

        self.ram[ROM_START_ADDR..end_addr].copy_from_slice(data);
        Ok(())
    }

    pub fn set_key(&mut self, key: usize, pressed: bool) {
        if key < self.keypad.len() {
            self.keypad[key] = pressed;
        }
    }

    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    pub fn is_beeping(&self) -> bool {
        self.sound_timer > 0
    }

    pub fn display_ptr(&self) -> *const u8 {
        self.display.as_ptr()
    }

    pub fn display_len(&self) -> usize {
        self.display.len()
    }

    fn next_random(&mut self) -> u8 {
        // Simple Linear Congruential Generator (LCG)
        self.rng_state = self.rng_state.wrapping_mul(1664525).wrapping_add(1013904223);
        (self.rng_state >> 24) as u8
    }

    pub fn tick(&mut self) {
        // 1. FETCH: Read big-endian 16-bit opcode at PC and advance PC by 2
        let pc_idx = self.pc as usize;
        let high = self.ram[pc_idx] as u16;
        let low = self.ram[pc_idx + 1] as u16;
        let opcode = (high << 8) | low;
        self.pc = self.pc.wrapping_add(2);

        // Extract common nibbles & addresses
        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as u8;
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        // 2. DECODE & EXECUTE
        match opcode & 0xF000 {
            0x0000 => match opcode {
                // 00E0: Clear display
                0x00E0 => self.display.fill(0),
                // 00EE: Return from subroutine
                0x00EE => {
                    self.sp -= 1;
                    self.pc = self.stack[self.sp as usize];
                }
                _ => {}
            },
            // 1NNN: Jump to NNN
            0x1000 => self.pc = nnn,
            // 2NNN: Call subroutine at NNN
            0x2000 => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            }
            // 3XNN: Skip next instruction if VX == NN
            0x3000 => {
                if self.v[x] == nn {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            // 4XNN: Skip next instruction if VX != NN
            0x4000 => {
                if self.v[x] != nn {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            // 5XY0: Skip next instruction if VX == VY
            0x5000 if (opcode & 0x000F) == 0 => {
                if self.v[x] == self.v[y] {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            // 6XNN: Set VX = NN
            0x6000 => self.v[x] = nn,
            // 7XNN: Add NN to VX (no carry flag)
            0x7000 => self.v[x] = self.v[x].wrapping_add(nn),
            // 8XY_: Arithmetic and bitwise operations
            0x8000 => match opcode & 0x000F {
                0x0 => self.v[x] = self.v[y],
                0x1 => {
                    self.v[x] |= self.v[y];
                    self.v[0xF] = 0;
                }
                0x2 => {
                    self.v[x] &= self.v[y];
                    self.v[0xF] = 0;
                }
                0x3 => {
                    self.v[x] ^= self.v[y];
                    self.v[0xF] = 0;
                }
                0x4 => {
                    let (sum, carry) = self.v[x].overflowing_add(self.v[y]);
                    self.v[x] = sum;
                    self.v[0xF] = if carry { 1 } else { 0 };
                }
                0x5 => {
                    let (diff, borrow) = self.v[x].overflowing_sub(self.v[y]);
                    self.v[x] = diff;
                    self.v[0xF] = if !borrow { 1 } else { 0 };
                }
                0x6 => {
                    let lsb = self.v[x] & 0x1;
                    self.v[x] >>= 1;
                    self.v[0xF] = lsb;
                }
                0x7 => {
                    let (diff, borrow) = self.v[y].overflowing_sub(self.v[x]);
                    self.v[x] = diff;
                    self.v[0xF] = if !borrow { 1 } else { 0 };
                }
                0xE => {
                    let msb = (self.v[x] >> 7) & 0x1;
                    self.v[x] <<= 1;
                    self.v[0xF] = msb;
                }
                _ => {}
            },
            // 9XY0: Skip next instruction if VX != VY
            0x9000 if (opcode & 0x000F) == 0 => {
                if self.v[x] != self.v[y] {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            // ANNN: Set I = NNN
            0xA000 => self.i = nnn,
            // BNNN: Jump to NNN + V0
            0xB000 => self.pc = nnn.wrapping_add(self.v[0] as u16),
            // CXNN: VX = random_byte() & NN
            0xC000 => self.v[x] = self.next_random() & nn,
            // DXYN: Draw N-row sprite from RAM[I..] at (VX, VY)
            0xD000 => {
                let start_x = (self.v[x] as usize) % SCREEN_WIDTH;
                let start_y = (self.v[y] as usize) % SCREEN_HEIGHT;
                let rows = n as usize;
                self.v[0xF] = 0;

                for row in 0..rows {
                    let target_y = start_y + row;
                    if target_y >= SCREEN_HEIGHT {
                        break;
                    }

                    let sprite_byte = self.ram[(self.i as usize) + row];
                    for col in 0..8 {
                        let target_x = start_x + col;
                        if target_x >= SCREEN_WIDTH {
                            break;
                        }

                        let sprite_pixel = (sprite_byte >> (7 - col)) & 0x1;
                        if sprite_pixel == 1 {
                            let idx = target_y * SCREEN_WIDTH + target_x;
                            if self.display[idx] == 1 {
                                self.v[0xF] = 1;
                            }
                            self.display[idx] ^= 1;
                        }
                    }
                }
            }
            // EX9E / EXA1: Keypad skip instructions
            0xE000 => match opcode & 0x00FF {
                0x9E => {
                    let key = (self.v[x] & 0x0F) as usize;
                    if self.keypad[key] {
                        self.pc = self.pc.wrapping_add(2);
                    }
                }
                0xA1 => {
                    let key = (self.v[x] & 0x0F) as usize;
                    if !self.keypad[key] {
                        self.pc = self.pc.wrapping_add(2);
                    }
                }
                _ => {}
            },
            // FX__: Timers, memory, BCD, and key wait
            0xF000 => match opcode & 0x00FF {
                0x07 => self.v[x] = self.delay_timer,
                0x0A => {
                    let pressed_key = self.keypad.iter().position(|&k| k);
                    match pressed_key {
                        Some(k) => self.v[x] = k as u8,
                        None => {
                            // Repeat this instruction until a key is pressed
                            self.pc = self.pc.wrapping_sub(2);
                        }
                    }
                }
                0x15 => self.delay_timer = self.v[x],
                0x18 => self.sound_timer = self.v[x],
                0x1E => self.i = self.i.wrapping_add(self.v[x] as u16),
                0x29 => self.i = ((self.v[x] & 0x0F) as u16) * 5,
                0x33 => {
                    let val = self.v[x];
                    let base = self.i as usize;
                    self.ram[base] = val / 100;
                    self.ram[base + 1] = (val / 10) % 10;
                    self.ram[base + 2] = val % 10;
                }
                0x55 => {
                    let base = self.i as usize;
                    for idx in 0..=x {
                        self.ram[base + idx] = self.v[idx];
                    }
                }
                0x65 => {
                    let base = self.i as usize;
                    for idx in 0..=x {
                        self.v[idx] = self.ram[base + idx];
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}