mod components;
mod machine;

pub use machine::Machine;

pub const ALPHABET_SIZE: usize = 26;

pub(crate) fn identity_wiring() -> [u8; ALPHABET_SIZE] {
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25,
    ]
}

pub(crate) fn char_to_wire(c: char) -> u8 {
    debug_assert!(c.is_alphabetic());
    (c.to_ascii_uppercase() as u8) - 65
}

pub(crate) fn encoding_to_wiring(encoding: impl Into<String>) -> [u8; ALPHABET_SIZE] {
    let encoding = encoding.into();
    debug_assert_eq!(encoding.len(), ALPHABET_SIZE);

    let mut wiring = identity_wiring();
    for (c, w) in encoding.chars().into_iter().zip(wiring.iter_mut()) {
        *w = char_to_wire(c)
    }

    wiring
}
