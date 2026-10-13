extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(32)]
pub struct max_align_t {
    #[offset(0)]
    pub __clang_max_align_nonce1: i64,
    #[offset(16)]
    #[byte_size(16)]
    pub __clang_max_align_nonce2: f64,
}
impl Clone for max_align_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for max_align_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct header {
    #[offset(0)]
    pub tag: i32,
    #[offset(4)]
    pub size: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct view {
    #[offset(0)]
    pub tag: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let text: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"hi\0")));
    let mut cp: Ptr<i8> = (text.as_pointer() as Ptr<i8>);
    let mut u: Ptr<u8> = cp.reinterpret_cast::<u8>();
    {
        &(4usize);
        {
            if (((((elem!(u, 0).read()) as i32) == ('h' as i32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"u[0] == \'h\'"),
                        Ptr::<i8>::from_string_literal(b"ptr_cast_init.c"),
                        17_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((((elem!(u, 1).read()) as i32) == ('i' as i32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"u[1] == \'i\'"),
                        Ptr::<i8>::from_string_literal(b"ptr_cast_init.c"),
                        18_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    let h: Value<header> = Rc::new(RefCell::new(header { tag: 7, size: 32 }));
    let mut hp: Ptr<header> = (h.as_pointer());
    let mut v: Ptr<view> = hp.reinterpret_cast::<view>();
    {
        &(4usize);
        {
            if (((v.with(|__s| __s.tag) == 7) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"v->tag == 7"),
                        Ptr::<i8>::from_string_literal(b"ptr_cast_init.c"),
                        23_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    let data: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"hi\0")));
    let mut vp: AnyPtr = ((data.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any();
    let mut n: i32 = 2;
    let mut sel: Ptr<i8> = if (((n < 100) as i32) != 0) {
        (vp).clone()
    } else {
        (AnyPtr::default())
    }
    .reinterpret_cast::<i8>();
    {
        &(4usize);
        {
            if (((!((sel).is_null())) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"sel != 0"),
                        Ptr::<i8>::from_string_literal(b"ptr_cast_init.c"),
                        29_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((((elem!(sel, 0).read()) as i32) == ('h' as i32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"sel[0] == \'h\'"),
                        Ptr::<i8>::from_string_literal(b"ptr_cast_init.c"),
                        30_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    n = 200;
    sel = if (((n < 100) as i32) != 0) {
        (vp).clone()
    } else {
        (AnyPtr::default())
    }
    .reinterpret_cast::<i8>();
    {
        &(4usize);
        {
            if ((((sel).is_null()) as i32) != 0) {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"sel == 0"),
                        Ptr::<i8>::from_string_literal(b"ptr_cast_init.c"),
                        33_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    return 0;
}
pub fn __assert_fail_0(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
