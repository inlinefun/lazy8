#[cfg(not(debug_assertions))]
pub(crate) fn is_release_build() -> bool {
    true
}
#[cfg(debug_assertions)]
pub(crate) fn is_release_build() -> bool {
    false
}
