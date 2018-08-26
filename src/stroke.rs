extern crate x11;

use hotkey::KeyPress;

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
    pub fn set_keypress(&mut self, keysym: &KeyPress) {
        use self::KeyPress::*;

        match keysym {
            KeyA => self.set(Stroke::S, true),
            KeyB => (),
            KeyC => self.set(Stroke::A, true),
            KeyD => self.set(Stroke::W, true),
            KeyE => self.set(Stroke::P, true),
            KeyF => self.set(Stroke::R, true),
            KeyG => self.set(Stroke::STAR, true),
            KeyH => self.set(Stroke::STAR, true),
            KeyI => self.set(Stroke::RP, true),
            KeyJ => self.set(Stroke::RR, true),
            KeyK => self.set(Stroke::B, true),
            KeyL => self.set(Stroke::G, true),
            KeyM => self.set(Stroke::U, true),
            KeyN => self.set(Stroke::E, true),
            KeyO => self.set(Stroke::L, true),
            KeyP => self.set(Stroke::RT, true),
            KeyQ => self.set(Stroke::S, true),
            KeyR => self.set(Stroke::H, true),
            KeyS => self.set(Stroke::K, true),
            KeyT => self.set(Stroke::STAR, true),
            KeyU => self.set(Stroke::F, true),
            KeyV => self.set(Stroke::O, true),
            KeyW => self.set(Stroke::T, true),
            KeyX => (),
            KeyY => self.set(Stroke::STAR, true),
            KeyZ => (),
            Key1 => self.set(Stroke::HASH, true),
            Key2 => self.set(Stroke::HASH, true),
            Key3 => self.set(Stroke::HASH, true),
            Key4 => self.set(Stroke::HASH, true),
            Key5 => self.set(Stroke::HASH, true),
            Key6 => self.set(Stroke::HASH, true),
            Key7 => self.set(Stroke::HASH, true),
            Key8 => self.set(Stroke::HASH, true),
            Key9 => self.set(Stroke::HASH, true),
            Key0 => self.set(Stroke::HASH, true),
            KeySpace => (),
            KeyBracketLeft => self.set(Stroke::D, true),
            KeySemicolon => self.set(Stroke::RS, true),
            KeyBackslash => self.set(Stroke::Z, true),
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

#[test]
fn test_stroke_raw_steno() {
    assert_eq!(Stroke::all().raw_steno(), "#STKPWHRAO*EUFRPBLGTSDZ");
}

#[test]
fn test_outline_raw_steno() {
    let mut outline = Outline::new();
    let stroke = Stroke::T | Stroke::E | Stroke::F | Stroke::RT;
    outline.push(stroke);
    let stroke = Stroke::T | Stroke::E | Stroke::F | Stroke::RT;
    outline.push(stroke);

    assert_eq!(outline.raw_steno(), "TEFT/TEFT");
}
