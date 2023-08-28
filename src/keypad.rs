use sdl2::keyboard::Keycode;
use crate::chip8::Chip8;

impl Chip8 {
    pub fn keypad_binds(&mut self, key: Keycode, toggled: bool) {
        match key {
            Keycode::Num1 => {
                self.keys[1] = toggled;
            },
            Keycode::Num2 => {
                self.keys[2] = toggled;
            },
            Keycode::Num3 => {
                self.keys[3] = toggled;
            },
            Keycode::Num4 => {
                self.keys[12] = toggled;
            },
            Keycode::Q => {
                self.keys[4] = toggled;
            },
            Keycode::W => {
                self.keys[5] = toggled;
            },
            Keycode::E => {
                self.keys[6] = toggled;
            },
            Keycode::R => {
                self.keys[13] = toggled;
            },
            Keycode::A => {
                self.keys[7] = toggled;
            },
            Keycode::S => {
                self.keys[8] = toggled;
            },
            Keycode::D => {
                self.keys[9] = toggled;
            },
            Keycode::F => {
                self.keys[14] = toggled;
            },
            Keycode::Z => {
                self.keys[10] = toggled;
            },
            Keycode::X => {
                self.keys[0] = toggled;
            },
            Keycode::C => {
                self.keys[11] = toggled;
            },
            Keycode::V => {
                self.keys[15] = toggled;
            },
            _ => {}
        }
    }
}