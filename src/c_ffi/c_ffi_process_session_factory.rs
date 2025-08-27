use minifi_native_sys::MinifiProcessSessionFactory;
use crate::api::ProcessSessionFactory;

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