mod pattern;

use mini_grep::Config;

use std::env;
use std::process;

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    if !config.match_pattern() {
        println!("Pattern not matched!");
        process::exit(1);
    }

    println!("Pattern matched")
}
