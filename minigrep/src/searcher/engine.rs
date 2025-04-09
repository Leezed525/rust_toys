use std::io::BufRead;
use super::Searcher;
use super::KMP;
use colored::Colorize;


// 这个结构体主要做
// 1. 读取文件
// 2. 读取文件的每一行
// 3. 将读到的每一行传递给一开始制定的searcher进行搜索
// 4. 如果搜索到了，就打印出来
pub struct Engine {
    searcher: Box<dyn Searcher>,
    pattern: String,
    filename: String,
}


impl Engine {
    pub fn new(search_mode: &str, pattern: String, filename: String) -> Self {
        let searcher: Box<dyn Searcher> = match search_mode {
            "kmp" => Box::new(KMP::new(pattern.clone())),
            _ => panic!("Unsupported search mode"),
        };
        Self {
            searcher,
            pattern,
            filename,
        }
    }

    pub fn run(&self) {
        let file = std::fs::File::open(&self.filename).expect("Failed to open file");
        let reader = std::io::BufReader::new(file);
        for line in reader.lines() {
            let line = line.unwrap();
            let search_result = self.searcher.search(&line);
            // 这里可以根据需要输出搜索结果
            if search_result.len() > 0 {
                // println!("Found at: {:?}", search_result);
                self.beautify(&line, &search_result);
            }
        }
    }

    fn beautify(&self, line: &str, search_result: &Vec<usize>) {
        let mut result = String::new();
        let mut last_index = 0;
        for &index in search_result {
            if index > last_index {
                result.push_str(&line[last_index..index]);
                result.push_str(&line[index..index + self.pattern.len()].red().to_string());
                last_index = index + self.pattern.len();
            }
        }
        result.push_str(&line[last_index..]);
        println!("{}", result);
    }
}