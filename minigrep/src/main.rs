use clap::Parser;
use minigrep::parser;
use minigrep::searcher::Engine;


fn main() {
    let args = parser::MiniGrepConfig::parse();
    let engine = Engine::new(&args.search_mode, args.pattern.clone(), args.file_paths[0].clone());
    engine.run();
    // dbg!(args);
}
