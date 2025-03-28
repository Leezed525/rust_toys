use super::Searcher;


pub struct KMP {
    pattern: String,
    next: Vec<usize>,
}
impl KMP {
    //生成next数组
    fn compute_next(pattern: &str) -> Vec<usize> {
        let mut next = vec![0; pattern.len()];
        let mut j = 0;
        let s = pattern.chars().collect::<Vec<char>>();
        for i in 1..s.len() {
            while j > 0 && s[i] != s[j] {
                j = next[j - 1];
            }
            if s[i] == s[j] {
                j += 1;
            }
            next[i] = j;
        }
        next
    }
    pub fn new(pattern: String) -> Self {
        let next = Self::compute_next(&pattern);
        println!("{:?}", next);
        Self { pattern, next }
    }
}


impl Searcher for KMP {
    fn search(&self, search: &str) -> bool {
        let search = search.chars().collect::<Vec<char>>();
        let pattern = &self.pattern.chars().collect::<Vec<char>>();
        let mut j = 0;
        let next = &self.next;
        for i in 0..search.len() {
            while j > 0 && search[i] != pattern[j] {
                j = next[j - 1];
            }
            if search[i] == pattern[j] {
                j += 1;
            }
            if j == pattern.len() {
                return true;
            }
        }
        false
    }
}