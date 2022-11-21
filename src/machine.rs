use crate::components::{Plugboard, Reflector, Rotor};

pub struct Machine {
    left_rotor: Rotor,
    middle_rotor: Rotor,
    right_rotor: Rotor,
    reflector: Reflector,
    plugboard: Plugboard,
}

impl Machine {
    fn new(
        left_rotor: Rotor,
        middle_rotor: Rotor,
        right_rotor: Rotor,
        reflector: Reflector,
        plugboard: Plugboard,
    ) -> Self {
        Self {
            left_rotor,
            middle_rotor,
            right_rotor,
            reflector,
            plugboard,
        }
    }
}
