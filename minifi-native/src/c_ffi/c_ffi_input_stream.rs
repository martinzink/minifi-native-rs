use std::io::{Read, Error, ErrorKind};
use minifi_native_sys::{MinifiInputStream, MinifiInputStreamRead};

/// A transient wrapper around the C++ MinifiInputStream.
/// This struct is only valid during the `MinifiProcessSessionRead` callback.
pub struct CffiInputStream<'a> {
    pub(crate) ptr: *mut MinifiInputStream,
    // Prevents this struct from outliving the C callback scope
    pub(crate) _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> Read for CffiInputStream<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        unsafe {
            let ret = MinifiInputStreamRead(
                self.ptr,
                buf.as_mut_ptr() as *mut std::ffi::c_char,
                buf.len()
            );

            if ret < 0 {
                return Err(Error::new(ErrorKind::Other, "MinifiInputStreamRead failed"));
            }

            Ok(ret as usize)
        }
    }
}
