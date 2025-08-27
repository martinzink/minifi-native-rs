use crate::api::ProcessSessionFactory;
use minifi_native_sys::MinifiProcessSessionFactory;

pub struct CffiProcessSessionFactory<'a> {
    _ptr: MinifiProcessSessionFactory,
    _lifetime: std::marker::PhantomData<&'a ()>,
}

impl<'a> CffiProcessSessionFactory<'a> {
    pub fn new(ptr: MinifiProcessSessionFactory) -> Self {
        Self {
            _ptr: ptr,
            _lifetime: std::marker::PhantomData,
        }
    }
}

impl<'a> ProcessSessionFactory for CffiProcessSessionFactory<'a> {}
