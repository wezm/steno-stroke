use error::Error;
use hotkey::KeyPress;
use radix_trie::TrieKey;
use std::str::FromStr;

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

impl Stroke {
    pub fn set_keypress(&mut self, keysym: &KeyPress) {
        use self::KeyPress::*;

        match keysym {
            KeyEsc => (),
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

    pub fn raw_steno(self) -> String {
        let mut raw = String::new();

        if self & Stroke::HASH == Stroke::HASH {
            raw.push('#')
        }
        if self & Stroke::S == Stroke::S {
            raw.push('S')
        }
        if self & Stroke::T == Stroke::T {
            raw.push('T')
        }
        if self & Stroke::K == Stroke::K {
            raw.push('K')
        }
        if self & Stroke::P == Stroke::P {
            raw.push('P')
        }
        if self & Stroke::W == Stroke::W {
            raw.push('W')
        }
        if self & Stroke::H == Stroke::H {
            raw.push('H')
        }
        if self & Stroke::R == Stroke::R {
            raw.push('R')
        }
        if self & Stroke::A == Stroke::A {
            raw.push('A')
        }
        if self & Stroke::O == Stroke::O {
            raw.push('O')
        }
        if self & Stroke::STAR == Stroke::STAR {
            raw.push('*')
        }
        if self & Stroke::E == Stroke::E {
            raw.push('E')
        }
        if self & Stroke::U == Stroke::U {
            raw.push('U')
        }
        if self & Stroke::F == Stroke::F {
            raw.push('F')
        }
        if self & Stroke::RR == Stroke::RR {
            raw.push('R')
        }
        if self & Stroke::RP == Stroke::RP {
            raw.push('P')
        }
        if self & Stroke::B == Stroke::B {
            raw.push('B')
        }
        if self & Stroke::L == Stroke::L {
            raw.push('L')
        }
        if self & Stroke::G == Stroke::G {
            raw.push('G')
        }
        if self & Stroke::RT == Stroke::RT {
            raw.push('T')
        }
        if self & Stroke::RS == Stroke::RS {
            raw.push('S')
        }
        if self & Stroke::D == Stroke::D {
            raw.push('D')
        }
        if self & Stroke::Z == Stroke::Z {
            raw.push('Z')
        }

        raw
    }

    pub fn is_star(self) -> bool {
        self == Stroke::STAR
    }
}

impl FromStr for Stroke {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stroke = Stroke::empty();
        let mut right = false;

        fn maybe_set_right(stroke: &mut Stroke, left: Stroke, right: Stroke, is_right: bool) {
            // stroke > left will set the right bit if any bit after left is set
            if is_right || *stroke >= left {
                stroke.set(right, true);
            } else {
                stroke.set(left, true);
            }
        }

        for c in s.chars() {
            match c {
                '-' => right = true,
                '#' => stroke.set(Stroke::HASH, true),
                'S' => maybe_set_right(&mut stroke, Stroke::S, Stroke::RS, right),
                'T' => maybe_set_right(&mut stroke, Stroke::T, Stroke::RT, right),
                'K' => stroke.set(Stroke::K, true),
                'P' => maybe_set_right(&mut stroke, Stroke::P, Stroke::RP, right),
                'W' => stroke.set(Stroke::W, true),
                'H' => stroke.set(Stroke::H, true),
                'R' => maybe_set_right(&mut stroke, Stroke::R, Stroke::RR, right),
                'A' => stroke.set(Stroke::A, true),
                'O' => stroke.set(Stroke::O, true),
                '*' => stroke.set(Stroke::STAR, true),
                'E' => stroke.set(Stroke::E, true),
                'U' => stroke.set(Stroke::U, true),
                'F' => stroke.set(Stroke::F, true),
                // 'RR' => stroke.set(Stroke::, true),
                // 'RP' => stroke.set(Stroke::, true),
                'B' => stroke.set(Stroke::B, true),
                'L' => stroke.set(Stroke::L, true),
                'G' => stroke.set(Stroke::G, true),
                // 'RT' => stroke.set(Stroke::, true),
                // 'RS' => stroke.set(Stroke::, true),
                'D' => stroke.set(Stroke::D, true),
                'Z' => stroke.set(Stroke::Z, true),
                _ => (),
            }
        }

        Ok(stroke)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Outline(Vec<Stroke>);

impl Outline {
    pub fn new() -> Self {
        Outline(Vec::new())
    }

    pub fn push(&mut self, stroke: Stroke) {
        self.0.push(stroke);
    }

    pub fn pop(&mut self) -> Option<Stroke> {
        self.0.pop()
    }

    pub fn split(&mut self) -> Outline {
        let split_at = self.0.len() - 1;
        let new = self.0.split_off(split_at);
        Outline::from(new)
    }

    pub fn is_multistroke(&self) -> bool {
        self.0.len() > 1
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn raw_steno(&self) -> String {
        self.0
            .iter()
            .map(|stroke| stroke.raw_steno())
            .collect::<Vec<_>>()
            .join("/")
    }

    pub fn strokes(&self) -> &[Stroke] {
        &self.0
    }
}

impl From<Vec<Stroke>> for Outline {
    fn from(strokes: Vec<Stroke>) -> Self {
        Outline(strokes)
    }
}

impl FromStr for Outline {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let strokes = s
            .split("/")
            .map(Stroke::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Outline::from(strokes))
    }
}

impl Default for Outline {
    fn default() -> Self {
        Outline(Default::default())
    }
}

impl TrieKey for Outline {
    fn encode_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for stroke in self.0.iter() {
            bytes.append(&mut stroke.bits().encode_bytes());
        }

        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stroke_raw_steno() {
        assert_eq!(Stroke::all().raw_steno(), "#STKPWHRAO*EUFRPBLGTSDZ");
    }

    #[test]
    fn test_outline_raw_steno() {
        let mut outline = Outline::new();
        let stroke = Stroke::T | Stroke::E | Stroke::F | Stroke::RT;
        outline.push(stroke);
        outline.push(stroke);

        assert_eq!(outline.raw_steno(), "TEFT/TEFT");
    }

    #[test]
    fn test_stroke_from_str() {
        let stroke = Stroke::T | Stroke::E | Stroke::F | Stroke::RT;
        assert_eq!(Stroke::from_str("TEFT").expect("parse error"), stroke);
    }

    #[test]
    fn test_stroke_from_str_right_t() {
        let stroke = Stroke::K | Stroke::A | Stroke::RT;
        assert_eq!(Stroke::from_str("KAT").expect("parse error"), stroke);
    }

    #[test]
    fn test_stroke_from_str_dash() {
        let stroke = Stroke::S | Stroke::RP | Stroke::RT;
        assert_eq!(Stroke::from_str("S-PT").expect("parse error"), stroke);
    }

    #[test]
    fn test_stroke_from_str_star() {
        let stroke = Stroke::W | Stroke::STAR | Stroke::RR;
        assert_eq!(Stroke::from_str("W*R").expect("parse error"), stroke);
    }

    #[test]
    fn test_outline_from_str() {
        let mut outline = Outline::new();
        let stroke = Stroke::T | Stroke::E | Stroke::F | Stroke::RT;
        outline.push(stroke);
        outline.push(stroke);

        assert_eq!(
            Outline::from_str("TEFT/TEFT").expect("parse error"),
            outline
        );
    }
}
