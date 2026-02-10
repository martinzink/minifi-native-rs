pub use minifi_macros::IdentifyComponent;

pub trait ComponentIdentifier {
    const CLASS_NAME: &'static str;
    const GROUP_NAME: &'static str;
    const VERSION: &'static str;
}
