use crate::components::Component;
use crate::utils::encoding_to_wiring;
use crate::utils::ClockInt;
use crate::utils::WireSize;
use crate::utils::WiringSize;
use std::vec;

#[derive(Debug)]
pub struct Rotor {
    position: ClockInt,
    ring_setting: ClockInt,
    wiring: WiringSize,
    inverse_wiring: WiringSize,
    notch_positions: Vec<WireSize>,
}

impl Rotor {
    pub fn new(name: &str, raw_position: WireSize, raw_ring_setting: WireSize) -> Rotor {
        let helper = move |encoding: &str, notch_positions: Vec<WireSize>| {
            let wiring = encoding_to_wiring(encoding.to_lowercase().as_str());
            Rotor {
                position: ClockInt::from_u8(raw_position),
                ring_setting: ClockInt::from_u8(raw_ring_setting),
                wiring,
                inverse_wiring: invert_wiring(wiring),
                notch_positions,
            }
        };

        match name {
            "I" => helper("EKMFLGDQVZNTOWYHXUSPAIBRCJ", vec![16]),
            "II" => helper("AJDKSIRUXBLHWTMCQGZNPYFVOE", vec![4]),
            "III" => helper("BDFHJLCPRTXVZNYEIWGAKMUSQO", vec![21]),
            "IV" => helper("ESOVPZJAYQUIRHXLNFTGKDCMWB", vec![21]),
            "V" => helper("VZBRGITYUPSDNHLXAWMJQOFECK", vec![21]),
            "VI" => helper("JPGVOUMFYQBENHZRDKASXLICTW", vec![21]),
            "VII" => helper("NZJHGRCXMYSWBOUFAIVLPEKQDT", vec![21]),
            "VIII" => helper("FKQHTLXOCBJSPDZRAMEWNIUYGV", vec![21]),
            _ => helper("ABCDEFGHIJKLMNOPQRSTUVWXYZ", vec![0]),
        }
    }

    pub fn at_notch(&self) -> bool {
        self.notch_positions
            .iter()
            .any(|&x| x == self.position.value())
    }
    pub fn turnover(&mut self) {
        self.position += 1
    }
}

impl Component for Rotor {
    fn forward(&self, letter: WireSize) -> WireSize {
        let offset = self.position - self.ring_setting;
        (ClockInt::from_u8(self.wiring[(offset + letter).value() as usize]) - offset).value()
    }
    fn backward(&self, letter: WireSize) -> WireSize {
        let offset = self.position - self.ring_setting;
        (ClockInt::from_u8(self.inverse_wiring[(offset + letter).value() as usize]) - offset)
            .value()
    }
}

fn invert_wiring(wiring: WiringSize) -> WiringSize {
    let mut inverse_wiring = [0; 26];
    wiring
        .iter()
        .enumerate()
        .for_each(|(n, &w)| inverse_wiring[w as usize] = n as WireSize);

    inverse_wiring
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotor_constructs() {
        let r = Rotor::new("I", 37, 45);

        assert_eq!(r.position.value(), 11);
        assert_eq!(r.ring_setting.value(), 19);
    }
    #[test]
    fn rotor_notches() {
        let mut r = Rotor {
            position: ClockInt::from_u8(5),
            ring_setting: ClockInt::from_u8(0),
            wiring: [0; 26],
            inverse_wiring: [0; 26],
            notch_positions: vec![5, 10],
        };

        assert!(r.at_notch());
        r.position += 1;
        assert!(!r.at_notch());
        r.position += 4;
        assert!(r.at_notch());
    }
    #[test]
    fn rotor_turnover() {
        let mut r = Rotor {
            position: ClockInt::from_u8(5),
            ring_setting: ClockInt::from_u8(0),
            wiring: [0; 26],
            inverse_wiring: [0; 26],
            notch_positions: vec![5, 10],
        };

        assert_eq!(r.position.value(), 5);
        r.turnover();
        assert_eq!(r.position.value(), 6);
    }
    #[test]
    fn invert_wiring_works() {
        let wiring: WiringSize = encoding_to_wiring("cabdefghijklmnopqrstuvwxyz");
        let expected: WiringSize = encoding_to_wiring("bcadefghijklmnopqrstuvwxyz");

        let res = invert_wiring(wiring);

        assert_eq!(res.len(), expected.len());
        assert!(
            res.iter().zip(expected.iter()).all(|(a, b)| a == b),
            "Arrays are not equal"
        );
    }
}
