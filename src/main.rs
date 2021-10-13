use enigma::Enigma;

#[derive(Debug)]
struct EnigmaArgs {
    rotor_names: Vec<String>,
    ring_settings: Vec<u8>,
    rotor_positions: Vec<u8>,
    reflector_type: String,
    plugboard_connections: Vec<String>,
}

fn main() {
    let (args, message) = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error parsing input arguments: {}.", e);
            std::process::exit(1);
        }
    };

    let mut enigma = match Enigma::new(
        args.rotor_names.iter().map(|s| s.as_str()).collect(),
        args.ring_settings,
        args.rotor_positions,
        args.reflector_type.as_str(),
        args.plugboard_connections
            .iter()
            .map(|s| s.as_str())
            .collect(),
    ) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e.message());
            std::process::exit(1);
        }
    };
    println!("{}", enigma.encrypt(&message));
}

fn parse_args() -> Result<(EnigmaArgs, String), pico_args::Error> {
    let mut p_args = pico_args::Arguments::from_env();

    if p_args.contains(["-h", "--help"]) {
        const HELP: &str = "\
Enigma

USAGE:
  enigma [OPTIONS] [MESSAGE]
FLAGS:
  -h, --help            Prints help information
OPTIONS:
  -n, --names NAMES              Comma separated list of rotor names, e.g. I,II,IV. Must be a Roman numeral between I and VIII inclusive.
  -s, --settings SETTINGS        Comma separated list of rotor ring settings, e.g. 0,17,22.
  -p, --positions POSITIONS      Comma separated list of initial rotor positions, e.g. 12,19,1.
  -r, --reflector REFLECTOR      Optional. Reflector type. Should be one of 'a', 'b', 'c', 'i'. [Default: 'i' - the identity reflector].
  -c, --connections CONNECTIONS  Optional. Plugboard connections. Should be a comma separated list of pairs of connections. [Default: no connections].
ARGS:
  <MESSAGE>
";
        print!("{}", HELP);
        std::process::exit(0);
    }

    let args = EnigmaArgs {
        rotor_names: p_args.value_from_fn(["-n", "--names"], parse_vec_string)?,
        ring_settings: p_args.value_from_fn(["-s", "--settings"], parse_vec_wiresize)?,
        rotor_positions: p_args.value_from_fn(["-p", "--positions"], parse_vec_wiresize)?,
        reflector_type: p_args
            .opt_value_from_str(["-r", "--reflector"])?
            .unwrap_or_else(|| String::from("i")),
        plugboard_connections: p_args
            .opt_value_from_fn(["-c", "--connections"], parse_vec_string)?
            .unwrap_or_default(),
    };

    let message = p_args
        .finish()
        .iter()
        .map(|s| {
            s.clone()
                .into_string()
                .expect("Message contains invalid Unicode characters.")
        })
        .fold(String::new(), |x, y| x + " " + &y)
        .trim()
        .to_owned();

    Ok((args, message))
}

fn parse_vec_string(s: &str) -> Result<Vec<String>, &'static str> {
    Ok(s.split(',').map(|s| s.trim().to_owned()).collect())
}
fn parse_vec_wiresize(s: &str) -> Result<Vec<u8>, &'static str> {
    Ok(s.split(',')
        .map(|s| s.parse().expect("Invalid number value"))
        .collect())
}
