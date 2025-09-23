pub trait ThreadingModel: sealed::Sealed {
    const IS_EXCLUSIVE: bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Concurrent;
impl ThreadingModel for Concurrent {
    const IS_EXCLUSIVE: bool = false;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Exclusive;
impl ThreadingModel for Exclusive {
    const IS_EXCLUSIVE: bool = true;
}

mod sealed {
    pub trait Sealed {}
    impl Sealed for super::Concurrent {}
    impl Sealed for super::Exclusive {}
}
