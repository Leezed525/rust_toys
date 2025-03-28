use clap::Parser;
use minigrep::parser;


fn main() {
    let args = parser::MiniGrepConfig::parse();

    dbg!(args);
}
