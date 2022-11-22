use crate::components::{Plugboard, Reflector, Rotor};
use crate::{char_to_wire, wire_to_char};

pub struct Machine {
    left_rotor: Rotor,
    middle_rotor: Rotor,
    right_rotor: Rotor,
    reflector: Reflector,
    plugboard: Plugboard,
}

impl Machine {
    pub fn new(
        rotors: (&str, &str, &str),
        ring_settings: (usize, usize, usize),
        rotor_positions: (usize, usize, usize),
        reflector_type: &str,
        plugboard_connections: &str,
    ) -> Self {
        Self {
            left_rotor: Rotor::from_name(rotors.0, ring_settings.0, rotor_positions.0),
            middle_rotor: Rotor::from_name(rotors.1, ring_settings.1, rotor_positions.1),
            right_rotor: Rotor::from_name(rotors.2, ring_settings.2, rotor_positions.2),
            reflector: Reflector::from_name(reflector_type),
            plugboard: Plugboard::from_connections(plugboard_connections),
        }
    }

    fn rotate(&mut self) {
        if self.middle_rotor.at_notch() {
            self.middle_rotor.turnover();
            self.left_rotor.turnover();
        } else if self.right_rotor.at_notch() {
            self.middle_rotor.turnover();
        }

        self.right_rotor.turnover();
    }

    pub fn encrypt(&mut self, message: &str) -> String {
        message
            .chars()
            .flat_map(|c| {
                if !c.is_ascii_alphabetic() {
                    eprintln!("Skipping char: {}", c);
                    return None;
                }

                self.rotate();

                let l = char_to_wire(c);
                let l = self.plugboard.forward(l);
                let l = self.right_rotor.forward(l);
                let l = self.middle_rotor.forward(l);
                let l = self.left_rotor.forward(l);
                let l = self.reflector.forward(l);
                let l = self.left_rotor.backward(l);
                let l = self.middle_rotor.backward(l);
                let l = self.right_rotor.backward(l);
                let l = self.plugboard.forward(l);
                Some(wire_to_char(l))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_encryption {
        ($input:literal, $expect:literal) => {
            let mut machine = Machine::new(("I", "II", "III"), (1, 1, 1), (0, 0, 0), "B", "");
            assert_eq!(machine.encrypt($input), $expect);
        };
    }

    #[test]
    fn test_encryption() {
        test_encryption!("AAAAA", "EWTYX");
        test_encryption!("HELLOXWORLD", "LOFUHZZLZOM");
        test_encryption!("toxcaps", "PESEXKY");
        test_encryption!("", "");
        test_encryption!(
            "TOMORROWANDTOMORROWANDTOMORROWCREEPSINTHISPETTYPACEFROMDAYTODAYTOTHELASTSYLLABLEOFRECORDEDTIMEANDALLOURYESTERDAYSHAVELIGHTEDFOOLSTHEWAYTODUSTYDEATHOUTOUTBRIEFCANDLELIFESBUTAWALKINGSHADOWAPOORPLAYERTHATSTRUTSANDFRETSHISHOURUPONTHESTAGEANDTHENISHEARDNOMOREITISATALETOLDBYANIDIOTFULLOFSOUNDANDFURYSIGNIFYINGNOTHING",
            "PEKGUOMYWIMRREKEVQUTKUYHPEUNARUKIAHIMFOKUTWCWYDITIKPPTQKWDJIGHRYLWDSCIPXOGYXVJPSZOAJRAWTRRFXCLHSKYHSNVLVMTNVBSZEBOHUWSQJDEOFBNKKISVBYKQJSZZRYDGCJHVNPDGNRPBDRKUQBLPWZNVCMGFBUCFTNYGROTUVPJUDECYMJKEHWNCKULMLNEFEBXAAZABEGLTDJFMJFSKXTLIOWWZOMZONONVXVIISACDUACYVQRWUDKKGMSYEKBOGCDBUOSJBCJWKNKFETOIPYDVKWLDIXLLWQDPBTSY"
        );
    }
}
