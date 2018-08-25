extern crate x11;

use self::x11::xlib;
use keysym::*;

// Steno order: STKPWHRAO*EUFRPBLGTSDZ
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

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Outline(Vec<Stroke>);

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

    pub fn raw_steno(&self) -> String {
        let mut raw = String::new();

        if *self & Stroke::HASH == Stroke::HASH { raw.push('#') }
        if *self & Stroke::S    == Stroke::S    { raw.push('S') }
        if *self & Stroke::T    == Stroke::T    { raw.push('T') }
        if *self & Stroke::K    == Stroke::K    { raw.push('K') }
        if *self & Stroke::P    == Stroke::P    { raw.push('P') }
        if *self & Stroke::W    == Stroke::W    { raw.push('W') }
        if *self & Stroke::H    == Stroke::H    { raw.push('H') }
        if *self & Stroke::R    == Stroke::R    { raw.push('R') }
        if *self & Stroke::A    == Stroke::A    { raw.push('A') }
        if *self & Stroke::O    == Stroke::O    { raw.push('O') }
        if *self & Stroke::STAR == Stroke::STAR { raw.push('*') }
        if *self & Stroke::E    == Stroke::E    { raw.push('E') }
        if *self & Stroke::U    == Stroke::U    { raw.push('U') }
        if *self & Stroke::F    == Stroke::F    { raw.push('F') }
        if *self & Stroke::RR   == Stroke::RR   { raw.push('R') }
        if *self & Stroke::RP   == Stroke::RP   { raw.push('P') }
        if *self & Stroke::B    == Stroke::B    { raw.push('B') }
        if *self & Stroke::L    == Stroke::L    { raw.push('L') }
        if *self & Stroke::G    == Stroke::G    { raw.push('G') }
        if *self & Stroke::RT   == Stroke::RT   { raw.push('T') }
        if *self & Stroke::RS   == Stroke::RS   { raw.push('S') }
        if *self & Stroke::D    == Stroke::D    { raw.push('D') }
        if *self & Stroke::Z    == Stroke::Z    { raw.push('Z') }

        raw
    }
}

impl Outline {
    pub fn new() -> Self {
        Outline(Vec::new())
    }

    pub fn push(&mut self, stroke: Stroke) {
        self.0.push(stroke);
    }

    pub fn compact(&mut self) {
        self.0 = vec![self.0.pop().expect("cannot compact empty outline")];
    }

    pub fn is_multistroke(&self) -> bool {
        self.0.len() > 1
    }

    pub fn raw_steno(&self) -> String {
        self.0.iter().map(|stroke| stroke.raw_steno()).collect::<Vec<_>>().join("/")
    }
}

impl From<Vec<Stroke>> for Outline {
    fn from(strokes: Vec<Stroke>) -> Self {
        Outline(strokes)
    }
}
