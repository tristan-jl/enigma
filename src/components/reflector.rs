use crate::components::Component;
use crate::utils::{encoding_to_wiring, WireSize, WiringSize};
use crate::InvalidArgsError;

#[derive(Debug)]
pub struct Reflector {
    wiring: WiringSize,
}

impl Reflector {
    pub fn new(reflector_type: &str) -> Result<Reflector, InvalidArgsError> {
        match reflector_type.to_lowercase().as_str() {
            "a" => Ok(Reflector::from_encoding("ejmzalyxvbwfcrquontspikhgd")),
            "b" => Ok(Reflector::from_encoding("yruhqsldpxngokmiebfzcwvjat")),
            "c" => Ok(Reflector::from_encoding("fvpjiaoyedrzxwgctkuqsbnmhl")),
            "i" => Ok(Reflector::from_encoding("abcdefghijklmnopqrstuvwxyz")),
            _ => Err(InvalidArgsError::from(
                format!("Invalid reflector type, {}", reflector_type).as_str(),
            )),
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
        letter
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
    #[test]
    fn reflector_construct() {
        let r = Reflector::new("a").unwrap();
        assert_eq!(r.forward(7), 23);
    }
}
