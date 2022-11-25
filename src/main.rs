use clap::Parser;
use enigma::{Machine, ALPHABET_SIZE};
use std::fmt::Display;

/// Encrypt/decrypt a message using a simulation of the Enigma machine.
#[derive(Parser, Debug)]
struct Cli {
    /// Comma-seperated list of length 3 of the names of which 3 rotors to use.
    #[arg(short, long)]
    #[clap(use_value_delimiter = true)]
    names: Vec<RotorNames>,

    /// Comma-seperated list of numbers of length 3 corresponding to the ring settings of the 3
    /// rotors.
    #[arg(short, long)]
    #[clap(use_value_delimiter = true)]
    settings: Vec<usize>,

    /// Reflector type.
    #[arg(short, long)]
    reflector: ReflectorNames,

    /// Plugboard connections as space-separated pairs of letters, e.g. 'AB CD' to swap the letters
    /// A and B, and the letters C and D.
    #[arg(short, long, default_value = "")]
    connections: String,

    /// Comma-seperated list of numbers of length 3 corresponding to the initial rotor positions.
    /// Each of these is taken modulo `crate::ALPHABET_SIZE`.
    #[arg(short, long)]
    #[clap(use_value_delimiter = true)]
    positions: Vec<usize>,

    /// Message to encrypt/decrypt. If not given reads from stdin.
    message: Option<String>,
}

#[derive(clap::ValueEnum, Clone, Debug)]
#[clap(rename_all = "UPPER")]
#[allow(clippy::upper_case_acronyms)]
enum RotorNames {
    I,
    II,
    III,
    IV,
    V,
    VI,
    VII,
    VIII,
}

#[derive(clap::ValueEnum, Clone, Debug)]
#[clap(rename_all = "UPPER")]
#[allow(clippy::upper_case_acronyms)]
enum ReflectorNames {
    A,
    B,
    C,
}

macro_rules! display_enums {
    ($t:ty) => {
        impl Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };
}

display_enums!(RotorNames);
display_enums!(ReflectorNames);

fn main() {
    let cli = Cli::parse();

    if cli.names.len() != 3 {
        eprintln!(
            "Error: 3 rotor names should be given, {} received",
            cli.names.len()
        );
        std::process::exit(1);
    }
    let rotor_names = (
        cli.names[0].to_string(),
        cli.names[1].to_string(),
        cli.names[2].to_string(),
    );

    if cli.settings.len() != 3 {
        eprintln!(
            "Error: 3 rotor settings should be given, {} received",
            cli.settings.len()
        );
        std::process::exit(1);
    }
    let settings = (cli.settings[0], cli.settings[1], cli.settings[2]);

    if cli.positions.len() != 3 {
        eprintln!(
            "Error: 3 rotor positions should be given, {} received",
            cli.positions.len()
        );
        std::process::exit(1);
    }
    let positions = (
        cli.positions[0] % ALPHABET_SIZE,
        cli.positions[1] % ALPHABET_SIZE,
        cli.positions[2] % ALPHABET_SIZE,
    );

    let mut machine = Machine::new(
        (
            rotor_names.0.as_str(),
            rotor_names.1.as_str(),
            rotor_names.2.as_str(),
        ),
        settings,
        positions,
        cli.reflector.to_string().as_str(),
        &cli.connections,
    );

    let mut buffer = String::new();
    let message = match &cli.message {
        Some(m) => m,
        None => {
            let stdin = std::io::stdin();

            for line in stdin.lines() {
                match line {
                    Ok(ref line) => {
                        buffer.push_str(line);
                    }
                    Err(err) => {
                        eprintln!("{}", err);
                        std::process::exit(1);
                    }
                }
            }
            &buffer
        }
    };

    println!("{}", machine.encrypt(message));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
