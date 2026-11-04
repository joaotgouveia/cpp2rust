extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type Flag = bool;
pub const Flag_FLAG_OFF: Flag = false;
pub const Flag_FLAG_ON: Flag = true;
pub type Small = u8;
pub const Small_SMALL_ZERO: Small = 0;
pub const Small_SMALL_MAX: Small = 255;
pub type Signed = i8;
pub const Signed_SIGNED_MIN: Signed = -128;
pub const Signed_SIGNED_ONE: Signed = 1;
pub type Wide = i64;
pub const Wide_WIDE_NEG: Wide = -1;
pub const Wide_WIDE_BIG: Wide = 1099511627776;
pub type Sized = u16;
pub const Sized_SIZED_A: Sized = 1;
pub const Sized_SIZED_B: Sized = 65535;
pub type Scoped = i8;
pub const Scoped_NEG: Scoped = -2;
pub const Scoped_POS: Scoped = 2;
pub type Empty = bool;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct HoldsEmpty {
    #[offset(0)]
    pub e: Empty,
}
impl CppMove for HoldsEmpty {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for HoldsEmpty {
    fn default() -> Self {
        {
            HoldsEmpty {
                e: <Empty>::default(),
            }
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut flag: Flag = Flag_FLAG_ON;
    ({ cpp2rust_assert_fail_0(flag) });
    ({ cpp2rust_assert_fail_0(!(Flag_FLAG_OFF)) });
    flag = Flag_FLAG_OFF;
    ({ cpp2rust_assert_fail_0(((flag as i32) == (Flag_FLAG_OFF as i32))) });
    let mut b: bool = flag;
    ({ cpp2rust_assert_fail_0(((b as i32) == (false as i32))) });
    let mut s: Small = Small_SMALL_MAX;
    ({ cpp2rust_assert_fail_0(((s as i32) == 255)) });
    ({ cpp2rust_assert_fail_0((::std::mem::size_of::<Small>() == 1_u64)) });
    s = Small_SMALL_ZERO;
    ({ cpp2rust_assert_fail_0((((s as i32) + 1) == 1)) });
    let mut sg: Signed = Signed_SIGNED_MIN;
    ({ cpp2rust_assert_fail_0(((sg as i32) == -128_i32)) });
    ({ cpp2rust_assert_fail_0(((sg as i32) < (Signed_SIGNED_ONE as i32))) });
    let mut w: Wide = Wide_WIDE_BIG;
    ({ cpp2rust_assert_fail_0(((w as i64) == (1_i64 << 40))) });
    ({ cpp2rust_assert_fail_0((::std::mem::size_of::<Wide>() == 8_u64)) });
    ({ cpp2rust_assert_fail_0(((Wide_WIDE_NEG as i64) < 0_i64)) });
    let mut z: Sized = Sized_SIZED_B;
    ({ cpp2rust_assert_fail_0(((z as i32) == 65535)) });
    ({ cpp2rust_assert_fail_0((::std::mem::size_of::<Sized>() == 2_u64)) });
    ({ cpp2rust_assert_fail_0((((Sized_SIZED_A as i32) + 1) == 2)) });
    let mut sc: Scoped = Scoped_NEG;
    ({ cpp2rust_assert_fail_0(((sc as i32) == -2_i32)) });
    ({ cpp2rust_assert_fail_0((((Scoped_POS as i8) as i32) == 2)) });
    let mut he: HoldsEmpty = <HoldsEmpty>::default();
    ({ cpp2rust_assert_fail_0(!(he.e)) });
    let mut ev: Empty = <Empty>::default();
    ({ cpp2rust_assert_fail_0(((ev as i32) == (he.e as i32))) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
