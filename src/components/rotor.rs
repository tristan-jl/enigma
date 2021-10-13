use crate::components::Component;
use crate::utils::encoding_to_wiring;
use crate::utils::ClockInt;
use crate::utils::WireSize;
use crate::utils::WiringSize;
use crate::InvalidArgsError;
use std::convert::TryFrom;
use std::vec;

#[derive(Debug, Clone, Copy)]
enum RotorName {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl TryFrom<&str> for RotorName {
    type Error = InvalidArgsError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        match input {
            "I" => Ok(RotorName::One),
            "II" => Ok(RotorName::Two),
            "III" => Ok(RotorName::Three),
            "IV" => Ok(RotorName::Four),
            "V" => Ok(RotorName::Five),
            "VI" => Ok(RotorName::Six),
            "VII" => Ok(RotorName::Seven),
            "VIII" => Ok(RotorName::Eight),
            _ => Err(InvalidArgsError::from(
                "RotorName must be a roman numeral from 1 to 8.",
            )),
        }
    }
}

#[derive(Debug)]
pub struct Rotor {
    name: RotorName,
    position: ClockInt,
    ring_setting: ClockInt,
    wiring: WiringSize,
    inverse_wiring: WiringSize,
    notch_positions: Vec<WireSize>,
}

impl Rotor {
    pub fn new(
        name: &str,
        raw_position: WireSize,
        raw_ring_setting: WireSize,
    ) -> Result<Rotor, InvalidArgsError> {
        let rotor_name = RotorName::try_from(name)?;

        let helper = move |encoding: &str, notch_positions: Vec<WireSize>| {
            let wiring = encoding_to_wiring(encoding.to_lowercase().as_str());
            Rotor {
                name: rotor_name,
                position: ClockInt::from(raw_position),
                ring_setting: ClockInt::from(raw_ring_setting),
                wiring,
                inverse_wiring: invert_wiring(wiring),
                notch_positions,
            }
        };

        match &rotor_name {
            RotorName::One => Ok(helper("EKMFLGDQVZNTOWYHXUSPAIBRCJ", vec![16])),
            RotorName::Two => Ok(helper("AJDKSIRUXBLHWTMCQGZNPYFVOE", vec![4])),
            RotorName::Three => Ok(helper("BDFHJLCPRTXVZNYEIWGAKMUSQO", vec![21])),
            RotorName::Four => Ok(helper("ESOVPZJAYQUIRHXLNFTGKDCMWB", vec![9])),
            RotorName::Five => Ok(helper("VZBRGITYUPSDNHLXAWMJQOFECK", vec![25])),
            RotorName::Six => Ok(helper("JPGVOUMFYQBENHZRDKASXLICTW", vec![25, 12])),
            RotorName::Seven => Ok(helper("NZJHGRCXMYSWBOUFAIVLPEKQDT", vec![25, 12])),
            RotorName::Eight => Ok(helper("FKQHTLXOCBJSPDZRAMEWNIUYGV", vec![25, 12])),
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
        (ClockInt::from(self.wiring[(offset + letter).value() as usize]) - offset).value()
    }
    fn backward(&self, letter: WireSize) -> WireSize {
        let offset = self.position - self.ring_setting;
        (ClockInt::from(self.inverse_wiring[(offset + letter).value() as usize]) - offset).value()
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
        let r = Rotor::new("I", 37, 45).unwrap();

        assert_eq!(r.position.value(), 11);
        assert_eq!(r.ring_setting.value(), 19);
    }
    #[test]
    fn rotor_notches() {
        let mut r = Rotor {
            name: RotorName::Eight,
            position: ClockInt::from(5),
            ring_setting: ClockInt::from(0),
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
            name: RotorName::Eight,
            position: ClockInt::from(5),
            ring_setting: ClockInt::from(0),
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
