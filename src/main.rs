use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = minigrep::build(&args).unwrap_or_else(|err| {
        println!("[ERROR] {err}");
        process::exit(1);
    });
    println!("Searching for '{}' in '{}'...", config.query, config.path);
    if let Err(e) = minigrep::run(&config) {
        println!("[ERROR] {e}");
        process::exit(1);
    }
}
