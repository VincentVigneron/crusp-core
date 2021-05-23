
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VariableId(usize);

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConstraintId(usize);

impl From<VariableId> for usize {
    fn from(idx: VariableId) -> Self {
        idx.0
    }
}

impl From<ConstraintId> for usize {
    fn from(idx: ConstraintId) -> Self {
        idx.0
    }
}

pub trait Subsumed {
    fn is_subsumed_under(&self, val: &Self) -> bool;
}
pub trait Nullable {
    fn is_null(&self) -> bool;
    fn null() -> Self;
    fn nullify(&mut self) -> Self; // return previous value
}
pub trait Mergeable: Copy {
    fn merge(&self, rhs: Self) -> Self;
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
