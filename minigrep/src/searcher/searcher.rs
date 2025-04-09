pub trait Searcher {
    //搜索结构，输出待查找的字符串，输出查找的结果（所有符合的下标）
    fn search(&self, search: &str) -> Vec<usize>;
}

