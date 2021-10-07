use crate::components::Component;
use crate::utils::{encoding_to_wiring, WireSize, WiringSize};

#[derive(Debug)]
pub struct Reflector {
    wiring: WiringSize,
}

impl Reflector {
    pub fn new(reflector_type: &str) -> Reflector {
        match reflector_type.to_lowercase().as_str() {
            "a" => Reflector::from_encoding("ejmzalyxvbwfcrquontspikhgd"),
            "b" => Reflector::from_encoding("yruhqsldpxngokmiebfzcwvjat"),
            "c" => Reflector::from_encoding("fvpjiaoyedrzxwgctkuqsbnmhl"),
            "i" => Reflector::from_encoding("abcdefghijklmnopqrstuvwxyz"),
            _ => panic!("Invalid reflector type, {}", reflector_type),
        }
    }

    pub fn from_encoding(encoding: &str) -> Reflector {
        Reflector {
            wiring: encoding_to_wiring(encoding),
        }
    }
}

impl Component for Reflector {
    fn forward(&self, letter: WireSize) -> WireSize {
        self.wiring[letter as usize]
    }
    fn backward(&self, letter: WireSize) -> WireSize {
        return letter;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::wiring_to_encoding;

    #[test]
    fn reflector_identity() {
        let encoding = "abcdefghijklmnopqrstuvwxyz";
        let r = Reflector::from_encoding(encoding);

        assert_eq!(wiring_to_encoding(r.wiring), encoding);
    }
    #[test]
    fn reflector_other() {
        let encoding = "yruhqsldpxngokmiebfzcwvjat";
        let r = Reflector::from_encoding(encoding);

        assert_eq!(wiring_to_encoding(r.wiring), encoding);
    }
}
