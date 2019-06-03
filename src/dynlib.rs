extern crate libloading as dll;

pub use dll::Library;
pub use dll::Symbol;

pub fn initialize_dll(filename: &str) -> dll::Library {
    match dll::Library::new(filename) {
        Ok(lib) => lib,
        Err(_) => panic!("DLL {} not loaded", filename)
    }
}

pub fn initialize_dll_function<T>(library: &'static dll::Library, fn_name: &[u8]) -> dll::Symbol<'static, T> {
    unsafe {
        match library.get(fn_name) {
            Ok(fun) => fun,
            Err(_) => panic!("Function {:?} not found!", fn_name)
        }
    }
}

