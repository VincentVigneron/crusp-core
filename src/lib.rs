pub trait Subsumed {
    fn is_subsumed_under(&self, val: &Self) -> bool;
}

#[macro_export]
macro_rules! unwrap_first {
    ($vec:ident) => {
        *$vec.first().unwrap()
    };
}

#[macro_export]
macro_rules! unwrap_last {
    ($vec:ident) => {
        *$vec.last().unwrap()
    };
}