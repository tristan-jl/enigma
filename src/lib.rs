pub mod components;
pub mod utils;

use crate::components::plugboard::Plugboard;
use crate::components::reflector::Reflector;
use crate::components::rotor::Rotor;
use crate::components::Component;
use crate::utils::WireSize;

#[derive(Debug)]
pub struct Enigma {
    rotors: Vec<Rotor>,
    reflector: Reflector,
    plugboard: Plugboard,
}

impl Enigma {
    pub fn new(
        rotor_names: Vec<&str>,
        ring_settings: Vec<WireSize>,
        rotor_positions: Vec<WireSize>,
        reflector_type: &str,
        plugboard_connections: Vec<&str>,
    ) -> Enigma {
        if (rotor_names.len() != ring_settings.len())
            || (rotor_names.len() != rotor_positions.len())
            || (rotor_names.len() <= 1)
        {
            panic!("Rotor names, ring setting and rotor positions must have the same length and their length must be greater than 1")
        }

        let mut rotors = Vec::new();
        for i in (0..rotor_names.len()).rev() {
            rotors.push(Rotor::new(
                rotor_names[i],
                rotor_positions[i],
                ring_settings[i],
            ))
        }

        Enigma {
            rotors,
            reflector: Reflector::new(reflector_type),
            plugboard: Plugboard::new(plugboard_connections),
        }
    }

    fn rotate(&mut self) {
        if self.rotors.len() > 1 {
            if self.rotors[1].at_notch() {
                self.rotors[1].turnover();
                self.rotors[2].turnover();
            } else if self.rotors[0].at_notch() {
                self.rotors[1].turnover();
            }
        }

        self.rotors[0].turnover()
    }

    fn encrypt_single(&mut self, letter: WireSize) -> WireSize {
        self.rotate();

        let mut res = letter;
        res = self.plugboard.forward(res);
        self.rotors.iter().for_each(|r| {
            res = r.forward(res);
        });
        res = self.reflector.forward(res);
        res = self.reflector.backward(res);
        self.rotors.iter().rev().for_each(|r| {
            res = r.backward(res);
        });
        res = self.plugboard.backward(res);

        res
    }

    pub fn encrypt(&mut self, message: &str) -> String {
        message
            .to_lowercase()
            .as_str()
            .as_bytes()
            .iter()
            .map(|b| match b {
                97..=122 => return (self.encrypt_single(b - 97) + 97) as char,
                _ => return *b as char,
            })
            .collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    macro_rules! enigma_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let mut enigma = Enigma::new(
                        vec!["I", "II", "III"],
                        vec![1, 1, 1],
                        vec![0, 0, 0],
                        "b",
                        vec![],
                    );
                    let (input, expected) = $value;
                    assert_eq!(enigma.encrypt(input), expected);
                }
            )*
        }
    }

    enigma_tests! {
        enigma_simple_message: ("aaaaa", "ewtyx"),
        enigma_hello_world: ("helloxworld", "lofuhzzlzom"),
        enigma_handles_empty_string: ("", ""),
        enigma_handles_spaces: ("words with spaces", "oepng hqvu ukokme"),
        enigma_handles_other_whitespace: ("a\nb\rb\n\n", "e\nz\ry\n\n"),
    }

    #[test]
    fn enigma_turnover_and_double_stepping() {
        let mut enigma = Enigma::new(
            vec!["I", "II", "III"],
            vec![1, 1, 1],
            vec![25, 25, 25],
            "b",
            vec![],
        );
        let input: &str = "tomorrow and tomorrow and tomorrow creeps in this petty \
        pace from day to day to the last syllable of recorded time and all our \
        yesterdays have lighted fools the way to dusty death out out brief candle \
        lifes but a walking shadow a poor player that struts and frets his hour upon \
        the stage and then is heard no more it is a tale told by an idiot full of \
        sound and fury signifying nothing";
        let exptd: &str = "mcddjuxx cjj jihhbblx zts kwssndhr amwkiy jp bidh bokyl \
        ngzw patp gia ei tcm mx nxj cghw egntojxb tl jdhinbrz hrnj hcv rba uhf \
        muilknumlo vxna przccnl qccca mbp cnj vq xwyvs kjrml ece qhe kguco jjhcjm \
        fepsx foa c lougdoo ezosig y ctmj mofpis wbtx nahxiz ruk zgkxi vsf bwku cvwh \
        nzk tkdmw whk mcfx jc pkcyi ij kwgp oh fc n pltf zria yh xt bjoha sovg wx \
        gszmz gsj akvz qtvwsapoyz vkbtrco";

        assert_eq!(enigma.encrypt(input), exptd);
    }
}
