use std::io::BufRead;
use super::Searcher;
use super::KMP;


// 这个结构体主要做
// 1. 读取文件
// 2. 读取文件的每一行
// 3. 将读到的每一行传递给一开始制定的searcher进行搜索
// 4. 如果搜索到了，就打印出来
pub struct Engine {
    searcher: Box<dyn Searcher>,
    filename: String,
}


impl Engine {
    pub fn new(search_mode: &str, pattern: String, filename: String) -> Self {
        let searcher: Box<dyn Searcher> = match search_mode {
            "kmp" => Box::new(KMP::new(pattern)),
            _ => panic!("Unsupported search mode"),
        };
        Self {
            searcher,
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
            // 比如输出搜索到的下标
            if search_result.len() > 0 {
                
                println!("Found at: {:?}", search_result);
            }
            // if self.searcher.search(&line) {
            //     println!("{}", line);
            // }
        }
    }
}