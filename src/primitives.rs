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

pub(crate) const fn static_minifi_string_view(str: &'static str) -> MinifiStringView {
    MinifiStringView {
        data: str.as_ptr() as *const i8,
        length: str.len() as u32,
    }
}

pub(crate) const fn create_bool(value: bool) -> MinifiBool {
    if value {
        MINIFI_TRUE
    } else {
        MINIFI_FALSE
    }
}
