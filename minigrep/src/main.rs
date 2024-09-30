use std::env;
use std::process;

use minigrep_lazy::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprint!("{}", err);
        process::exit(1);
    });
    

    if let Err(e) = minigrep_lazy::run(config) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
