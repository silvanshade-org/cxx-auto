// An empty bridge for us to compile to ensure we produce a library (even if ctypes are disabled).
#[cxx::bridge]
mod ffi {}
