use std::rc::Rc;
use glow::{Context, HasContext, NativeTexture, TEXTURE_2D};
use imgui::{DrawListMut, Ui};
use rand::Rng;

static FONT_MAP: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80 // F
];

pub struct Chip8 {
    ram: [u8; 4096],
    pc: u16,
    i_reg: u16,
    stack: [u16; 16],
    stack_ptr: u8,
    delay_timer: u8,
    sound_timer: u8,
    v_regs: [u8; 16],
    screen: [u8; 64*32],
    display_screen: [u8; 64 * 32 * 3],
    pub keys: [bool; 16],
    loaded: bool,
    cycle: u16,
    graphics_update: bool
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            ram: [0u8; 4096],
            pc: 512,
            i_reg: 0,
            stack: [0; 16],
            stack_ptr: 0,
            delay_timer: 0,
            sound_timer: 0,
            v_regs: [0u8; 16],
            screen: [0u8; 64 * 32],
            display_screen: [0u8; 64 * 32 * 3],
            keys: [false; 16],
            loaded: false,
            cycle: 0,
            graphics_update: false,
        }
    }

    fn reset(&mut self) {
        self.ram = [0u8; 4096];
        self.pc = 512;
        self.i_reg = 0;
        self.stack = [0; 16];
        self.stack_ptr = 0;
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.v_regs = [0u8; 16];
        self.screen = [0u8; 64 * 32];
        self.display_screen = [0u8; 64 * 32 * 3];
        self.keys = [false; 16];
        self.cycle = 0;
        self.graphics_update = true;
    }

    pub fn load(&mut self, data: &[u8]) {
        // Making sure :3
        self.reset();

        self.ram[..FONT_MAP.len()].copy_from_slice(&FONT_MAP);

        self.ram[512..(data.len()+512)].copy_from_slice(data);

        self.loaded = true;
    }

    pub fn unload(&mut self) {
        if !self.loaded {return;}

        self.loaded = false;
        self.reset()
    }

    // TODO: Add self.pc += 2 on the top of cycle
    pub fn tick(&mut self) {
        if !self.loaded {return}

        self.cycle += 1;
        let current_instruction = ((self.ram[self.pc as usize] as u16) << 8) | self.ram[self.pc as usize + 1] as u16;

        let x = ((current_instruction & 0x0F00) >> 8) as usize;
        let y = ((current_instruction & 0x00F0) >> 4) as usize;

        match (current_instruction & 0xF000) >> 12 {
            0 => {
                match current_instruction & 0x000F {
                    0 => {
                        self.screen = [0u8; 64 * 32];
                        self.graphics_update = true;
                        self.pc += 2;
                    }
                    0xE => {
                        self.stack_ptr -= 1;
                        self.pc = self.stack[self.stack_ptr as usize];
                        self.pc += 2;
                    }
                    _ => {}
                }
            }
            1 => {
                self.pc = current_instruction & 0x0FFF;
            }
            2 => {
                self.stack[self.stack_ptr as usize] = self.pc;
                self.stack_ptr += 1;
                self.pc = current_instruction & 0x0FFF;
            }
            3 => {
                if self.v_regs[x] == (current_instruction & 0x00FF) as u8 {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            4 => {
                if self.v_regs[x] != (current_instruction & 0x00FF) as u8 {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            5 => {
                if self.v_regs[x] == self.v_regs[y] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            6 => {
                self.v_regs[x] = (current_instruction & 0x00FF) as u8;
                self.pc += 2;
            }
            7 => {
                self.v_regs[x] += (current_instruction & 0x00FF) as u8;
                self.pc += 2;
            }
            8 => {
                match current_instruction & 0x000F {
                    0 => {
                        self.v_regs[x] = self.v_regs[y];
                        self.pc += 2;
                    }
                    1 => {
                        self.v_regs[0xF] = 0;
                        self.v_regs[x] |= self.v_regs[y];
                        self.pc += 2;
                    }
                    2 => {
                        self.v_regs[0xF] = 0;
                        self.v_regs[x] &= self.v_regs[y];
                        self.pc += 2;
                    }
                    3 => {
                        self.v_regs[0xF] = 0;
                        self.v_regs[x] ^= self.v_regs[y];
                        self.pc += 2;
                    }
                    4 => {
                        let statement = ((self.v_regs[y] as u32) + (self.v_regs[x] as u32)) > 255;
                        self.v_regs[x] += self.v_regs[y];
                        if statement {
                            self.v_regs[0xF] = 1
                        } else {
                            self.v_regs[0xF] = 0
                        }
                        self.pc += 2;
                    }
                    5 => {
                        let statement = self.v_regs[y] > self.v_regs[x];
                        self.v_regs[x] -= self.v_regs[y];
                        if statement {
                            self.v_regs[0xF] = 0
                        } else {
                            self.v_regs[0xF] = 1
                        }
                        self.pc += 2;
                    }
                    6 => {
                        self.v_regs[x] = self.v_regs[y];
                        let vf = self.v_regs[x] & 0x1;
                        self.v_regs[x] = self.v_regs[x] >> 1;
                        self.v_regs[0xF] = vf;
                        self.pc += 2;
                    }
                    7 => {
                        let statement = self.v_regs[x] > self.v_regs[y];
                        self.v_regs[x] = self.v_regs[y] - self.v_regs[x];
                        if statement {
                            self.v_regs[0xF] = 0
                        } else {
                            self.v_regs[0xF] = 1
                        }
                        self.pc += 2;
                    }
                    0xE => {
                        self.v_regs[x] = self.v_regs[y];
                        let vf = self.v_regs[x] >> 7;
                        self.v_regs[x] = self.v_regs[x] << 1;
                        self.v_regs[0xF] = vf;
                        self.pc += 2;
                    }
                    _ => {}
                }
            }
            9 => {
                if self.v_regs[x] != self.v_regs[y] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0xA => {
                self.i_reg = current_instruction & 0x0FFF;
                self.pc += 2;
            }
            0xB => {
                self.pc = self.v_regs[0] as u16 + (current_instruction & 0x0FFF);
            }
            0xC => {
                let random_num = rand::thread_rng().gen_range(0..255) & ((current_instruction & 0x00FF) as u8);
                self.v_regs[x] = random_num;
                self.pc += 2;
            }
            0xD => {
                let vx = self.v_regs[x] % 64;
                let vy = self.v_regs[y] % 32;
                let rows = (current_instruction & 0x000F) as u8;
                self.v_regs[0xF] = 0;
                for yl in 0..rows {
                    let pixel = self.ram[self.i_reg as usize + yl as usize];
                    for xl in 0..8 {
                        if pixel & (0x80 >> xl) != 0 {
                            let column = ((vx + xl) as u32);
                            let row = ((vy + yl) as u32);

                            // Clipping ON, to turn off clipping just do column % 64 and row % 32
                            if column > 63 || row > 31 {
                                continue;
                            }

                            let pixel_index = (column + (row * 64)) as usize;
                            if self.screen[pixel_index] == 1 {
                                self.v_regs[0xF] = 1;
                            }
                            self.screen[pixel_index] ^= 1;
                        }
                    }
                }
                self.graphics_update = true;
                self.pc += 2;
            }
            0xE => {
                let x = ((current_instruction & 0x0F00) >> 8) as usize;
                match current_instruction & 0x00FF {
                    0x9E => {
                        if self.keys[self.v_regs[x] as usize] {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    }
                    0xA1 => {
                        if !self.keys[self.v_regs[x] as usize] {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    }
                    _ => {}
                }
            }
            0xF => {
                let x = ((current_instruction & 0x0F00) >> 8) as usize;
                match current_instruction & 0x00FF {
                    7 => {
                        self.v_regs[x] = self.delay_timer;
                        self.pc += 2;
                    }
                    0xA => {
                        let mut pressed = false;
                        for i in 0..16 {
                            if self.keys[i] {
                                self.v_regs[x] = i as u8;
                                pressed = true;
                            }
                        }
                        if !pressed {
                            return;
                        }
                        self.pc += 2;
                    }
                    0x15 => {
                        self.delay_timer = self.v_regs[x];
                        self.pc += 2;
                    }
                    0x18 => {
                        self.sound_timer = self.v_regs[x];
                        self.pc += 2;
                    }
                    0x1E => {
                        self.i_reg += self.v_regs[x] as u16;
                        self.pc += 2;
                    }
                    0x29 => {
                        self.i_reg = (self.v_regs[x] * 5) as u16;
                        self.pc += 2;
                    }
                    0x33 => {
                        self.ram[self.i_reg as usize] = (self.v_regs[x] as f32 / 100.0).floor() as u8;
                        self.ram[self.i_reg as usize + 1] = ((self.v_regs[x] as f32 / 10.0).floor() as u8) % 10;
                        self.ram[self.i_reg as usize + 2] = self.v_regs[x] % 10;
                        self.pc += 2;
                    }
                    0x55 => {
                        for i in 0..=x {
                            self.ram[self.i_reg as usize + i] = self.v_regs[i];
                        }
                        self.i_reg += x as u16 + 1;
                        self.pc += 2;
                    }
                    0x65 => {
                        for i in 0..=x {
                            self.v_regs[i] = self.ram[self.i_reg as usize + i];
                        }
                        self.i_reg += x as u16 + 1;
                        self.pc += 2;
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        if self.cycle % 8 == 0 {
            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }

            if self.sound_timer > 0 {
                self.sound_timer -= 1;
            }
        }
    }

    pub fn draw(&mut self, renderer: &Rc<Context>, texture: NativeTexture) {
        if !self.graphics_update {return}

        for yl in 0..32 {
            for xl in 0..64 {
                let pixel_index = (xl + (yl * 64)) as usize;
                if self.screen[pixel_index] != 0 {
                    for i in 0..3 {
                        self.display_screen[pixel_index * 3 + i] = 255;
                    }
                } else {
                    for i in 0..3 {
                        self.display_screen[pixel_index * 3 + i] = 0;
                    }
                }
            }
        }

        unsafe {
            renderer.bind_texture(TEXTURE_2D, Some(texture));
            renderer.tex_image_2d(TEXTURE_2D, 0, glow::RGB as i32, 64, 32, 0, glow::RGB, glow::UNSIGNED_BYTE, Some(&self.display_screen));
            renderer.bind_texture(TEXTURE_2D, None);
        }
    }
}