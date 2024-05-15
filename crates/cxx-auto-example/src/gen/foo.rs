#[cxx::bridge]
mod bridge {
    #[namespace = "example"]
    extern "C++" {
        include!("cxx-auto-example/cxx/include/foo.hh");

        pub type Foo;
    }
}
pub use bridge::*;
