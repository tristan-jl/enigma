pub mod plugboard;
pub mod reflector;
pub mod rotor;
use crate::utils::WireSize;

pub trait Component {
    fn forward(&self, letter: WireSize) -> WireSize;
    fn backward(&self, letter: WireSize) -> WireSize;
}
