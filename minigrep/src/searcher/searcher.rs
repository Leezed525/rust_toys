pub trait Searcher {
    fn search(&self, search: &str) -> bool;
}

