fn main() { std::process::exit(std::process::Command::new("bun").args(std::env::args().skip(1)).status().unwrap().code().unwrap_or(1)); }
