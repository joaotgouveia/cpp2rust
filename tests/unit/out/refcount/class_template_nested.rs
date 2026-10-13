extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type Traits_char__Kind = u32;
pub const Traits_char__Kind_Small: Traits_char__Kind = 0;
pub const Traits_char__Kind_Large: Traits_char__Kind = 1;
pub type anon_1 = u32;
pub const anon_1_Size: anon_1 = 1;
thread_local!(
    pub static Traits_char__Width_2: Value<i32> = Rc::new(RefCell::new(
        (((1usize as usize).wrapping_mul(8_usize)) as i32),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Traits_char__Box {
    #[offset(0)]
    pub value: i8,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Traits_0 {}
impl Traits_0 {
    pub fn kind() -> Traits_char__Kind {
        return if (1usize > 2_usize) {
            Traits_char__Kind_Large
        } else {
            Traits_char__Kind_Small
        };
    }
}
pub type Traits_int__Kind = u32;
pub const Traits_int__Kind_Small: Traits_int__Kind = 0;
pub const Traits_int__Kind_Large: Traits_int__Kind = 1;
pub type anon_4 = u32;
pub const anon_4_Size: anon_4 = 4;
thread_local!(
    pub static Traits_int__Width_5: Value<i32> = Rc::new(RefCell::new(
        (((4usize as usize).wrapping_mul(8_usize)) as i32),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Traits_int__Box {
    #[offset(0)]
    pub value: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Traits_3 {}
impl Traits_3 {
    pub fn kind() -> Traits_int__Kind {
        return if (4usize > 2_usize) {
            Traits_int__Kind_Large
        } else {
            Traits_int__Kind_Small
        };
    }
}
pub type anon_7 = u32;
pub const anon_7_value: anon_7 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct IsSame_6 {}
pub type anon_9 = u32;
pub const anon_9_value: anon_9 = 0;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct IsSame_8 {}
pub type anon_11 = u32;
pub const anon_11_value: anon_11 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct IsSame_10 {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    (if ((anon_1_Size as i32) == 1) {
        &(0);
    } else {
        ({
            __assert_fail_13(
                Ptr::<i8>::from_string_literal(b"Traits<char>::Size == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((anon_4_Size as i32) == 4) {
        &(0);
    } else {
        ({
            __assert_fail_13(
                Ptr::<i8>::from_string_literal(b"Traits<int>::Size == 4"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (Traits_char__Width_2.with(|rc| *rc.borrow()) == 8) {
        &(0);
    } else {
        ({
            __assert_fail_13(
                Ptr::<i8>::from_string_literal(b"Traits<char>::Width == 8"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (Traits_int__Width_5.with(|rc| *rc.borrow()) == 32) {
        &(0);
    } else {
        ({
            __assert_fail_13(
                Ptr::<i8>::from_string_literal(b"Traits<int>::Width == 32"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((({ Traits_0::kind() }) as i32) == (Traits_char__Kind_Small as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_13(
                Ptr::<i8>::from_string_literal(b"Traits<char>::kind() == Traits<char>::Small"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((({ Traits_3::kind() }) as i32) == (Traits_int__Kind_Large as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_13(
                Ptr::<i8>::from_string_literal(b"Traits<int>::kind() == Traits<int>::Large"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let tc: Value<Traits_0> = Rc::new(RefCell::new(<Traits_0>::default()));
    let ti: Value<Traits_3> = Rc::new(RefCell::new(<Traits_3>::default()));
    let mut bc: Traits_char__Box = ({ Traits_0Impl::box_(&tc.as_pointer(), ('a' as i8)) });
    let mut bi: Traits_int__Box = ({ Traits_3Impl::box_(&ti.as_pointer(), 7) });
    (if ((bc.value as i32) == (('a' as i8) as i32)) {
        &(0);
    } else {
        ({
            __assert_fail_13(
                Ptr::<i8>::from_string_literal(b"bc.value == \'a\'"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (bi.value == 7) {
        &(0);
    } else {
        ({
            __assert_fail_13(
                Ptr::<i8>::from_string_literal(b"bi.value == 7"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((anon_7_value as i32) == 1) {
        &(0);
    } else {
        ({
            __assert_fail_13(
                Ptr::<i8>::from_string_literal(b"(IsSame<int, int>::value == 1)"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((anon_9_value as i32) == 0) {
        &(0);
    } else {
        ({
            __assert_fail_13(
                Ptr::<i8>::from_string_literal(b"(IsSame<int, char>::value == 0)"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((anon_11_value as i32) == 1) {
        &(0);
    } else {
        ({
            __assert_fail_13(
                Ptr::<i8>::from_string_literal(b"(IsSame<char, char>::value == 1)"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_13(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub trait Traits_0Impl {
    fn box_(&self, v: i8) -> Traits_char__Box;
}
impl Traits_0Impl for Ptr<Traits_0> {
    fn box_(&self, mut v: i8) -> Traits_char__Box {
        return Traits_char__Box { value: v };
    }
}
pub trait Traits_3Impl {
    fn box_(&self, v: i32) -> Traits_int__Box;
}
impl Traits_3Impl for Ptr<Traits_3> {
    fn box_(&self, mut v: i32) -> Traits_int__Box {
        return Traits_int__Box { value: v };
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = Traits_char__Width_2.with(|_| ());
    let _ = Traits_int__Width_5.with(|_| ());
}
