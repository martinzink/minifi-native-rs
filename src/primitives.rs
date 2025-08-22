use minifi_native_sys::{MinifiBool, MinifiStringView, MINIFI_TRUE, MINIFI_FALSE};

pub(crate) const fn create_string_view(str: &'static str) -> MinifiStringView {
    MinifiStringView {
        data: str.as_ptr() as *const i8,
        length: str.len() as u32,
    }
}

pub(crate) const fn create_bool(value: bool) -> MinifiBool {
    if value { MINIFI_TRUE } else { MINIFI_FALSE }
}