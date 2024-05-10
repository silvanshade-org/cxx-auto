#[cfg(feature = "ctypes")]
#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-auto/cxx/include/cxx-auto.hxx");

        #[cxx_name = "c_char"]
        type _c_char = crate::ffi::ctypes::c_char;
    }

    impl CxxVector<_c_char> {
    }
    impl UniquePtr<_c_char> {
    }
}
