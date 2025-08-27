use minifi_native_sys::{MinifiBool, MinifiStringView, MINIFI_FALSE, MINIFI_TRUE};

#[derive(Debug)]
pub(crate) struct StringView<'a> {
    inner: MinifiStringView,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> StringView<'a> {
    pub(crate) fn new(str: &'a str) -> Self {
        Self {
            inner: MinifiStringView {
                data: str.as_ptr() as *const i8,
                length: str.len() as u32,
            },
            _marker: std::marker::PhantomData,
        }
    }

    pub unsafe fn as_raw(&self) -> MinifiStringView {
        self.inner
    }
}

pub trait StaticStrAsMinifiCStr {
    fn as_minifi_c_type(&self) -> MinifiStringView;
}

impl StaticStrAsMinifiCStr for &'static str{
    fn as_minifi_c_type(&self) -> MinifiStringView {
        MinifiStringView {
            data: self.as_ptr() as *const i8,
            length: self.len() as u32,
        }
    }
}

pub trait BoolAsMinifiCBool {
    fn as_minifi_c_type(&self) -> MinifiBool;
}

impl BoolAsMinifiCBool for bool {
    fn as_minifi_c_type(&self) -> MinifiBool {
        if *self {
            MINIFI_TRUE
        } else {
            MINIFI_FALSE
        }
    }
}
