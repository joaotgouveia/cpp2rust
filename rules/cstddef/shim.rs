// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

/// `std::to_integer<T>(std::byte)`: the byte's value as the integer type.
pub trait FromByte {
    fn cpp_to_integer(b: u8) -> Self;
}

macro_rules! impl_from_byte {
    ($($t:ty),* $(,)?) => {
        $(
            impl FromByte for $t {
                fn cpp_to_integer(b: u8) -> Self {
                    b as $t
                }
            }
        )*
    };
}

impl_from_byte!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);

impl FromByte for bool {
    fn cpp_to_integer(b: u8) -> Self {
        b != 0
    }
}
