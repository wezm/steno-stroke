extern crate x11;

use self::x11::xlib;
use keysym::*;

// STKPWHRAO*EUFRPBLGTSDZ
bitflags! {
    pub struct Stroke: u32 {
        const HASH = 0b00000000000000000000001;
        const S    = 0b00000000000000000000010;
        const T    = 0b00000000000000000000100;
        const K    = 0b00000000000000000001000;
        const P    = 0b00000000000000000010000;
        const W    = 0b00000000000000000100000;
        const H    = 0b00000000000000001000000;
        const R    = 0b00000000000000010000000;
        const A    = 0b00000000000000100000000;
        const O    = 0b00000000000001000000000;
        const STAR = 0b00000000000010000000000;
        const E    = 0b00000000000100000000000;
        const U    = 0b00000000001000000000000;
        const F    = 0b00000000010000000000000;
        const RR   = 0b00000000100000000000000;
        const RP   = 0b00000001000000000000000;
        const B    = 0b00000010000000000000000;
        const L    = 0b00000100000000000000000;
        const G    = 0b00001000000000000000000;
        const RT   = 0b00010000000000000000000;
        const RS   = 0b00100000000000000000000;
        const D    = 0b01000000000000000000000;
        const Z    = 0b10000000000000000000000;
    }
}

impl Stroke {
    // TODO: Replace with KeyPress enum so match can be ensured exhaustive
    pub fn set_keysym(&mut self, keysym: xlib::KeySym) {
        match keysym {
            XK_A => self.set(Stroke::S, true),
            XK_B => (),
            XK_C => self.set(Stroke::A, true),
            XK_D => self.set(Stroke::W, true),
            XK_E => self.set(Stroke::P, true),
            XK_F => self.set(Stroke::R, true),
            XK_G => self.set(Stroke::STAR, true),
            XK_H => self.set(Stroke::STAR, true),
            XK_I => self.set(Stroke::RP, true),
            XK_J => self.set(Stroke::RR, true),
            XK_K => self.set(Stroke::B, true),
            XK_L => self.set(Stroke::G, true),
            XK_M => self.set(Stroke::U, true),
            XK_N => self.set(Stroke::E, true),
            XK_O => self.set(Stroke::L, true),
            XK_P => self.set(Stroke::RT, true),
            XK_Q => self.set(Stroke::S, true),
            XK_R => self.set(Stroke::H, true),
            XK_S => self.set(Stroke::K, true),
            XK_T => self.set(Stroke::STAR, true),
            XK_U => self.set(Stroke::F, true),
            XK_V => self.set(Stroke::O, true),
            XK_W => self.set(Stroke::T, true),
            XK_X => (),
            XK_Y => self.set(Stroke::STAR, true),
            XK_Z => (),
            XK_1 => self.set(Stroke::HASH, true),
            XK_2 => self.set(Stroke::HASH, true),
            XK_3 => self.set(Stroke::HASH, true),
            XK_4 => self.set(Stroke::HASH, true),
            XK_5 => self.set(Stroke::HASH, true),
            XK_6 => self.set(Stroke::HASH, true),
            XK_7 => self.set(Stroke::HASH, true),
            XK_8 => self.set(Stroke::HASH, true),
            XK_9 => self.set(Stroke::HASH, true),
            XK_0 => self.set(Stroke::HASH, true),
            XK_SEMICOLON => self.set(Stroke::RS, true),
            XK_BRACKET_LEFT => self.set(Stroke::D, true),
            XK_BACKSLASH => self.set(Stroke::Z, true),
            _ => (),
        }
    }
}
