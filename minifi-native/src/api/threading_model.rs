pub trait ThreadingModel: sealed::Sealed {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Concurrent;
impl ThreadingModel for Concurrent {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Exclusive;
impl ThreadingModel for Exclusive {}

mod sealed {
    pub trait Sealed {}
    impl Sealed for super::Concurrent {}
    impl Sealed for super::Exclusive {}
}
