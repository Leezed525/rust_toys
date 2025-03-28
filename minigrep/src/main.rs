use std::env;
use clap::Parser;
use minigrep::parser;


#[derive(Debug)]
struct ArgParameter {
    name: String,
    value: String,
}

#[derive(Debug)]
struct Config {
    project_name: String,
    parameters: Vec<ArgParameter>,
}

impl Config {
    fn new(project_name: String) -> Config {
        Config {
            project_name,
            parameters: Vec::new(),
        }
    }
    fn set_parameter(&mut self, name: String, value: String) {
        self.parameters.push(ArgParameter { name, value });
    }

    fn mini_grep_config(args: &Vec<String>) -> Config {
        let mut config = Config::new("mini_grep".to_string());
        let need_find_key = vec!["--filename", "--find"];
        let mut find_key_value_count = 0;
        for i in 0..args.len() {
            if args[i].starts_with("-") {
                let key = args[i].clone();
                if need_find_key.iter().find(|&&x| x == key).is_none() {
                    //报错
                    panic!("{} is not a valid parameter", key);
                }
                //判断是否有下一个参数
                if i + 1 < args.len() && !args[i + 1].starts_with("-") {
                    let value = args[i + 1].clone();
                    config.set_parameter(key, value);
                    find_key_value_count += 1;
                } else {
                    config.set_parameter(key, "".to_string());
                }
            }
        }

        if find_key_value_count != need_find_key.len() {
            panic!("--filename and --find must be set");
        }
        config
    }
}

fn main() {
    let args = parser::MiniGrepConfig::parse();

    dbg!(args);
    // let args: Vec<String> = env::args().collect();
    // let config = Config::mini_grep_config(&args);
    //
    // dbg!(config);
    // println!("Hello, world!");
}
