
use clap::Parser;


#[derive(Parser,Debug)]
#[clap(version = "1.0", author = "Leezed <leezed525@qq.com>", about = "mini grep config")]
pub struct MiniGrepConfig {
    #[arg(index = 1)]
    pub pattern: String,

    // #[arg(index = 2, action = ArgAction::Append)]
    #[arg(index = 2, num_args = 1..)]
    pub file_paths: Vec<String>,


}