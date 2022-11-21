use crate::{char_to_wire, encoding_to_wiring, identity_wiring, ALPHABET_SIZE};
use std::collections::hash_set::HashSet;

pub(crate) struct Rotor {
    forward_wiring: [u8; ALPHABET_SIZE],
    backward_wiring: [u8; ALPHABET_SIZE],
    position: usize,
    ring_setting: usize,
    notch_position: Vec<usize>,
}

macro_rules! rotor_cons {
    ($t:ty, $name:tt, $encoding:literal, $notches: expr) => {
        pub fn $name(ring_setting: usize, position: usize) -> $t {
            Self::new($encoding, position, ring_setting, $notches)
        }
    };
}

impl Rotor {
    pub fn new(
        encoding: impl Into<String>,
        position: usize,
        ring_setting: usize,
        notch_position: Vec<usize>,
    ) -> Self {
        let forward_wiring = encoding_to_wiring(encoding);

        let mut backward_wiring = [0; ALPHABET_SIZE];
        for i in 0..backward_wiring.len() {
            backward_wiring[forward_wiring[i] as usize] = i as u8;
        }

        Self {
            forward_wiring,
            backward_wiring,
            position,
            ring_setting,
            notch_position,
        }
    }

    rotor_cons!(Self, i, "EKMFLGDQVZNTOWYHXUSPAIBRCJ", vec![16]);
    rotor_cons!(Self, ii, "AJDKSIRUXBLHWTMCQGZNPYFVOE", vec![4]);
    rotor_cons!(Self, iii, "BDFHJLCPRTXVZNYEIWGAKMUSQO", vec![21]);
    rotor_cons!(Self, iv, "ESOVPZJAYQUIRHXLNFTGKDCMWB", vec![9]);
    rotor_cons!(Self, v, "VZBRGITYUPSDNHLXAWMJQOFECK", vec![25]);
    rotor_cons!(Self, vi, "JPGVOUMFYQBENHZRDKASXLICTW", vec![12, 25]);
    rotor_cons!(Self, vii, "NZJHGRCXMYSWBOUFAIVLPEKQDT", vec![12, 25]);
    rotor_cons!(Self, viii, "FKQHTLXOCBJSPDZRAMEWNIUYGV", vec![12, 25]);

    pub fn at_notch(&self) -> bool {
        self.notch_position.iter().any(|&n| self.position == n)
    }

    pub fn turnover(&mut self) {
        self.position = (self.position + 1) % ALPHABET_SIZE;
    }

    fn encipher(&self, wiring: [u8; ALPHABET_SIZE], letter: u8) -> u8 {
        let shift = self.position as isize - self.ring_setting as isize;
        ((wiring[((letter as isize + shift + 26) % 26) as usize] as isize - shift + 26) % 26) as u8
    }

    fn forward(&self, letter: u8) -> u8 {
        debug_assert!((letter as usize) < ALPHABET_SIZE);
        self.encipher(self.forward_wiring, letter)
    }

    fn backward(&self, letter: u8) -> u8 {
        debug_assert!((letter as usize) < ALPHABET_SIZE);
        self.encipher(self.backward_wiring, letter)
    }
}

impl Default for Rotor {
    fn default() -> Self {
        Self {
            forward_wiring: identity_wiring(),
            backward_wiring: identity_wiring(),
            position: 0,
            ring_setting: 0,
            notch_position: vec![0],
        }
    }
}

pub(crate) struct Plugboard {
    wiring: [u8; ALPHABET_SIZE],
}

impl Plugboard {
    pub fn from_connections(connections: impl Into<String>) -> Self {
        let mut wiring = identity_wiring();
        let connections: String = connections.into();

        let mut seen: HashSet<u8> = HashSet::new();

        for char_pair in connections.split_whitespace() {
            let mut char_pair = char_pair.chars();
            let char1 = char_pair.next();
            let char2 = char_pair.next();

            if char1.is_none() || char2.is_none() {
                continue;
            }

            let char1 = char_to_wire(char1.unwrap());
            let char2 = char_to_wire(char2.unwrap());

            if !seen.insert(char1) || !seen.insert(char2) {
                panic!("Invalid connections")
            }

            wiring[char1 as usize] = char2;
            wiring[char2 as usize] = char1;
        }

        Self { wiring }
    }

    fn forward(&self, letter: u8) -> u8 {
        debug_assert!((letter as usize) < ALPHABET_SIZE);
        self.wiring[letter as usize]
    }

    fn backward(&self, letter: u8) -> u8 {
        self.forward(letter)
    }
}

impl Default for Plugboard {
    fn default() -> Self {
        Self {
            wiring: identity_wiring(),
        }
    }
}

pub(crate) struct Reflector {
    wiring: [u8; ALPHABET_SIZE],
}

impl Reflector {
    pub fn from_encoding(encoding: impl Into<String>) -> Self {
        Self {
            wiring: encoding_to_wiring(encoding),
        }
    }

    pub fn a() -> Self {
        Self::from_encoding("EJMZALYXVBWFCRQUONTSPIKHGD")
    }

    pub fn b() -> Self {
        Self::from_encoding("YRUHQSLDPXNGOKMIEBFZCWVJAT")
    }

    pub fn c() -> Self {
        Self::from_encoding("FVPJIAOYEDRZXWGCTKUQSBNMHL")
    }

    fn forward(&self, letter: u8) -> u8 {
        debug_assert!((letter as usize) < ALPHABET_SIZE);
        self.wiring[letter as usize]
    }
}

impl Default for Reflector {
    fn default() -> Self {
        Self {
            wiring: identity_wiring(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotor_wiring() {
        let plug = Plugboard::from_connections("");
        let expected = identity_wiring();
        assert_eq!(plug.wiring, expected);
    }

    #[test]
    fn test_plugboard_build_ident() {
        let plug = Plugboard::from_connections("");
        let expected = identity_wiring();
        assert_eq!(plug.wiring, expected);
    }

    #[test]
    fn test_plugboard_build() {
        let plug = Plugboard::from_connections("AB DE");
        let mut expected = identity_wiring();
        expected[0] = 1;
        expected[1] = 0;
        expected[3] = 4;
        expected[4] = 3;

        assert_eq!(plug.wiring, expected);
    }

    #[test]
    fn test_reflector_build() {
        let refl = Reflector::from_encoding("BACDEFGHIJKLMNOPQRSTUVWXYZ");
        let mut expected = identity_wiring();
        expected[0] = 1;
        expected[1] = 0;

        assert_eq!(refl.wiring, expected);
    }

    #[test]
    fn test_reflector_build2() {
        let refl = Reflector::from_encoding("ABCDEFGHIJKLMNOPQRSTUVWXZY");
        let mut expected = identity_wiring();
        expected[24] = 25;
        expected[25] = 24;

        assert_eq!(refl.wiring, expected);
    }
}
