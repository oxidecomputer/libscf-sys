#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Transitive dependencies use u128 arguments, which are not FFI-safe.
// Care should be taken to not expose these arguments.
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_creation() {
        unsafe {
            let handle = scf_handle_create(SCF_VERSION.into());
            assert!(!handle.is_null());
            scf_handle_destroy(handle);
        }
    }
}
