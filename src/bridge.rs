//! Shared nominal results for every generated CXX bridge.

/// Whether a catching C++ operation completed or threw.
///
/// # Specification
/// - provides: the named states encoded as consecutive u8 values starting at
///   zero, with identical discriminants in cxx-auto-results.hxx.
/// - panics: none.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Completion
{
    /// Operation completed successfully.
    Completed = 0,
    /// Operation threw; the message accompanies the result.
    Threw = 1,
}

/// Bind Completion to the identically represented C++ enum.
///
/// # Safety
/// - unsafe invariants: both languages use the same u8 discriminants and
///   alignment; C++ returns only declared variants, and the enum is trivially
///   copyable. Cross-language discriminant agreement is a co-versioning
///   obligation, not validated by CXX or the C++ static assertions.
// SAFETY: the repr(u8) enum agrees with cxx-auto-results.hxx.
unsafe impl cxx::ExternType for Completion
{
    type Id = cxx::type_id!("cxx_auto::Completion");
    type Kind = cxx::kind::Trivial;
}

/// The C++ equality or inequality classification.
///
/// # Specification
/// - provides: the named states encoded as consecutive u8 values starting at
///   zero, with identical discriminants in cxx-auto-results.hxx.
/// - panics: none.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Equality
{
    /// The operands satisfy the selected equality relation.
    Equal = 0,
    /// The operands satisfy the selected inequality relation.
    NotEqual = 1,
}

/// Bind Equality to the identically represented C++ enum.
///
/// # Safety
/// - unsafe invariants: both languages use the same u8 discriminants and
///   alignment; C++ returns only declared variants, and the enum is trivially
///   copyable. Cross-language discriminant agreement is a co-versioning
///   obligation, not validated by CXX or the C++ static assertions.
// SAFETY: the repr(u8) enum agrees with cxx-auto-results.hxx.
unsafe impl cxx::ExternType for Equality
{
    type Id = cxx::type_id!("cxx_auto::Equality");
    type Kind = cxx::kind::Trivial;
}

/// The C++ three-way comparison classification.
///
/// # Specification
/// - provides: the named states encoded as consecutive u8 values starting at
///   zero, with identical discriminants in cxx-auto-results.hxx.
/// - panics: none.
#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
#[repr(u8)]
pub enum Comparison
{
    /// The left operand precedes the right operand.
    Less = 0,
    /// The operands are equivalent in the selected ordering.
    Equivalent = 1,
    /// The left operand follows the right operand.
    Greater = 2,
    /// Neither operand precedes or equals the other.
    Unordered = 3,
}

/// Bind Comparison to the identically represented C++ enum.
///
/// # Safety
/// - unsafe invariants: both languages use the same u8 discriminants and
///   alignment; C++ returns only declared variants, and the enum is trivially
///   copyable. Cross-language discriminant agreement is a co-versioning
///   obligation, not validated by CXX or the C++ static assertions.
// SAFETY: the repr(u8) enum agrees with cxx-auto-results.hxx.
unsafe impl cxx::ExternType for Comparison
{
    type Id = cxx::type_id!("cxx_auto::Comparison");
    type Kind = cxx::kind::Trivial;
}

/// The pointer-sized hash produced by the C++ type's selected hash function.
///
/// # Specification
/// - provides: a nominal `std::size_t` result with every bit pattern valid.
/// - panics: none.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct HashValue
{
    /// The word fed into a Rust Hasher by the generated Hash implementation.
    pub value: usize,
}

/// Bind `HashValue` to the single-word C++ representation.
///
/// # Safety
/// - unsafe invariants: Rust usize and C++ `std::size_t` have matching size and
///   alignment on the target; both representations contain one word and are
///   trivially copyable.
// SAFETY: repr(transparent) agrees with the C++ single-size_t struct.
unsafe impl cxx::ExternType for HashValue
{
    type Id = cxx::type_id!("cxx_auto::HashValue");
    type Kind = cxx::kind::Trivial;
}
