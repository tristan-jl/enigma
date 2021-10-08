pub(crate) mod components;
pub(crate) mod utils;

use crate::components::plugboard::Plugboard;
use crate::components::reflector::Reflector;
use crate::components::rotor::Rotor;
use crate::components::Component;
use crate::utils::WireSize;

/// An Enigma machine with rotors, a reflector and a plugboard.
#[derive(Debug)]
pub struct Enigma {
    rotors: Vec<Rotor>,
    reflector: Reflector,
    plugboard: Plugboard,
}

impl Enigma {
    /// Creates a new `Enigma` machine.
    ///
    /// Takes vectors of rotor names (should be in the form of Roman numerals from 'I'
    /// to 'VIII'), ring settings and initial rotor positions to set the rotors; a
    /// reflector type (should be one of 'a', 'b', 'c' or 'i' - the identity reflector);
    /// and plugboard connections (should be a vector of 2-length strings).
    ///
    /// # Examples
    ///
    /// Creating an `Enigma` machine with rotors 'I', 'II', 'III', ring settings of 5, 7, 9,
    /// ring positions of 15, 17, 19, the 'b' reflector and plugboard connections of 'A'
    /// to 'b', 'g' to 'k' and 'n' to 't'.
    ///
    /// ```
    /// use enigma::Enigma;
    ///
    /// let mut enigma = Enigma::new(
    ///     vec!["I", "II", "III"],
    ///     vec![5, 7, 9],
    ///     vec![15, 17, 19],
    ///     "b",
    ///     vec!["ab", "gk", "nt"],
    /// );
    /// ```
    ///
    /// # Panics
    ///
    /// If the rotor names, ring settings and rotor positions do not have the same
    /// length.
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
            panic!("Rotor names, ring settings and rotor positions must have the same length and their length must be greater than 1")
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

    /// Encrypts a message with the `Enigma` machine.
    ///
    /// Takes a message in the form of a string of Unicode characters. Message is
    /// converted to lowercase before encryption and non-alphanumeric characters are
    /// ignored (note: when ignored, the `Enigma` machine rotors are not rotated).
    ///
    /// # Examples
    ///
    /// Creating an `Enigma` machine (from `Enigma::new` example).
    ///
    /// ```
    /// use enigma::Enigma;
    ///
    /// let mut enigma = Enigma::new(
    ///     vec!["I", "II", "III"],
    ///     vec![5, 7, 9],
    ///     vec![15, 17, 19],
    ///     "b",
    ///     vec!["ab", "gk", "nt"],
    /// );
    /// let message = "hello world";
    /// assert_eq!(enigma.encrypt(message), "glgtn lmzul");
    /// ```
    pub fn encrypt(&mut self, message: &str) -> String {
        message
            .to_lowercase()
            .as_str()
            .as_bytes()
            .iter()
            .map(|b| match b {
                97..=122 => (self.encrypt_single(b - 97) + 97) as char,
                _ => *b as char,
            })
            .collect::<String>()
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
