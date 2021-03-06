use crate::components::Component;
use crate::utils::{WireSize, WiringSize};
use crate::InvalidArgsError;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Plugboard {
    wiring: WiringSize,
}

impl Plugboard {
    pub fn new(connections: Vec<&str>) -> Result<Plugboard, InvalidArgsError> {
        let mut wiring: WiringSize = [0; 26];
        for (i, elem) in wiring.iter_mut().enumerate() {
            *elem = i as WireSize;
        }

        if !connections.is_empty() {
            let mut seen = HashSet::new();

            for &char_pair in connections.iter() {
                if char_pair.len() != 2 {
                    return Err(InvalidArgsError::from(
                        format!("plugboard connections {} not a pair", char_pair).as_str(),
                    ));
                }

                let byte_slice = char_pair.as_bytes();
                let c1 = byte_slice[0] - 97;
                let c2 = byte_slice[1] - 97;

                if seen.contains(&c1) || seen.contains(&c2) {
                    return Err(InvalidArgsError::from(
                        format!("Invalid connections. {} or {} is duplicated.", c1, c2).as_str(),
                    ));
                }

                seen.insert(c1);
                seen.insert(c2);

                wiring[c1 as usize] = c2;
                wiring[c2 as usize] = c1;
            }
        }

        Ok(Plugboard { wiring })
    }
}

impl Component for Plugboard {
    fn forward(&self, letter: WireSize) -> WireSize {
        self.wiring[letter as usize]
    }

    fn backward(&self, letter: WireSize) -> WireSize {
        self.wiring[letter as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::wiring_to_encoding;

    #[test]
    fn plugboard_no_cons() {
        let p = Plugboard::new(vec![]).unwrap();
        let expected = "abcdefghijklmnopqrstuvwxyz";

        assert_eq!(wiring_to_encoding(p.wiring), expected);
    }

    #[test]
    fn plugboard_cons() {
        let p = Plugboard::new(vec!["az", "by", "cx", "dw", "ev"]).unwrap();
        let expected = "zyxwvfghijklmnopqrstuedcba";

        assert_eq!(wiring_to_encoding(p.wiring), expected);
    }
}
