export module cxx_auto:ffi.elaboration;
import :derive;

import cxx_auto.std;

namespace cxx_auto {
export struct alignas(32) TypeElabFFI final
{
  std::size_t cxx_abi_align;
  std::size_t cxx_abi_size;
  bool rust_should_impl_cxx_extern_type_trivial;
  bool rust_should_impl_unpin;
  bool rust_should_impl_send;
  bool rust_should_impl_sync;
  bool rust_should_impl_drop;
  bool rust_should_impl_copy;
  bool rust_should_impl_default;
  bool rust_should_impl_moveref_copy_new;
  bool rust_should_impl_moveref_move_new;
  bool rust_should_impl_eq;
  bool rust_should_impl_partial_eq;
  bool rust_should_impl_partial_ord;
  bool rust_should_impl_ord;
  bool rust_should_impl_hash;
  bool rust_should_impl_debug;
  bool rust_should_impl_display;

  template<typename Self>
  static inline consteval auto elab() -> TypeElabFFI
  {
    return {
      .cxx_abi_align = alignof(Self),
      .cxx_abi_size = sizeof(Self),
      .rust_should_impl_cxx_extern_type_trivial = derive::rust_should_impl_cxx_extern_type_trivial<Self>,
      .rust_should_impl_unpin = derive::rust_should_impl_unpin<Self>,
      .rust_should_impl_send = derive::rust_should_impl_send<Self>,
      .rust_should_impl_sync = derive::rust_should_impl_sync<Self>,
      .rust_should_impl_drop = derive::rust_should_impl_drop<Self>,
      .rust_should_impl_copy = derive::rust_should_impl_copy<Self>,
      .rust_should_impl_default = derive::rust_should_impl_default<Self>,
      .rust_should_impl_moveref_copy_new = derive::rust_should_impl_moveref_copy_new<Self>,
      .rust_should_impl_moveref_move_new = derive::rust_should_impl_moveref_move_new<Self>,
      .rust_should_impl_eq = derive::rust_should_impl_eq<Self>,
      .rust_should_impl_partial_eq = derive::rust_should_impl_partial_eq<Self>,
      .rust_should_impl_partial_ord = derive::rust_should_impl_partial_ord<Self>,
      .rust_should_impl_ord = derive::rust_should_impl_ord<Self>,
      .rust_should_impl_hash = derive::rust_should_impl_hash<Self>,
      .rust_should_impl_debug = derive::rust_should_impl_debug<Self>,
      .rust_should_impl_display = derive::rust_should_impl_display<Self>,
    };
  }
};
} // namespace cxx_auto
