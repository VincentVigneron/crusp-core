pub trait Subsumed {
    fn is_subsumed_under(&self, val: &Self) -> bool;
}