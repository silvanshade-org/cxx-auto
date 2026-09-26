/// Character conversion boundaries for the C ABI wrapper.
#[cfg(test)]
mod tests
{
    use cxx_auto::ctypes::CCharEncodingError;
    use cxx_auto::ctypes::c_char;

    #[test]
    fn character_conversion_preserves_or_rejects()
    {
        let maximum = c_char::try_from('\u{ff}').expect("Latin-1 maximum must fit");
        assert_eq!(::core::ffi::c_char::from(maximum).to_le_bytes(), [0xff]);
        assert_eq!(char::from(maximum), '\u{ff}');
        assert_eq!(
            c_char::try_from('\u{100}'),
            Err(CCharEncodingError::OutOfRange('\u{100}'))
        );
    }

    #[test]
    fn character_slice_views_preserve_high_bit_bytes()
    {
        let bytes = [0, 0x7f, 0x80, 0xff];
        let characters = c_char::from_bytes(&bytes);
        assert_eq!(c_char::into_bytes(characters), &bytes);
        assert_eq!(char::from(characters[3]), '\u{ff}');
    }
}
