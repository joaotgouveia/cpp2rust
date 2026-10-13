extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(232)]
pub struct __locale_struct {
    #[offset(0)]
    #[byte_size(104)]
    pub __locales: Value<Box<[Ptr<__locale_data>]>>,
    #[offset(104)]
    #[byte_size(8)]
    pub __ctype_b: Ptr<u16>,
    #[offset(112)]
    #[byte_size(8)]
    pub __ctype_tolower: Ptr<i32>,
    #[offset(120)]
    #[byte_size(8)]
    pub __ctype_toupper: Ptr<i32>,
    #[offset(128)]
    #[byte_size(104)]
    pub __names: Value<Box<[Ptr<i8>]>>,
}
impl Clone for __locale_struct {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __locale_struct {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Inner {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: i32,
}
pub fn sum_inner_0(mut i: Ptr<Inner>) -> i32 {
    return ({ i.with(|__s| __s.a) } + { i.with(|__s| __s.b) });
}
#[derive(VaArg, FnPtrArg, ByteRepr, DeepClone)]
#[byte_size(16)]
pub struct anon_1 {
    #[offset(0)]
    #[byte_size(16)]
    __bytes: Value<Box<[u8]>>,
}
impl anon_1 {
    pub fn inner(this: Ptr<Self>) -> Ptr<Inner> {
        this.reinterpret_cast()
    }
    pub fn raw_(this: Ptr<Self>) -> Ptr<i8> {
        this.reinterpret_cast()
    }
}
impl Default for anon_1 {
    fn default() -> Self {
        anon_1 {
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 16]))),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(16)]
    pub u: anon_1,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let standalone: Value<Inner> = <Value<Inner>>::default();
    (*standalone.borrow_mut()).a = 3;
    (*standalone.borrow_mut()).b = 4;
    {
        &(4usize);
        {
            if (((({ sum_inner_0((standalone.as_pointer())) }) == 7) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"sum_inner(&standalone) == 7"),
                        Ptr::<i8>::from_string_literal(b"union_struct_dual_use.c"),
                        22_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    let outer: Value<Outer> = <Value<Outer>>::default();
    ({
        let ___s: AnyPtr = ((outer.as_pointer()) as Ptr<Outer>).to_any();
        let ___n: usize = 16usize;
        memset_3(___s, 0, ___n)
    });
    field!(anon_1::inner(field_ptr!(outer.as_pointer(), u)), a).write(3);
    field!(anon_1::inner(field_ptr!(outer.as_pointer(), u)), b).write(4);
    {
        &(4usize);
        {
            if (((({ sum_inner_0((anon_1::inner(field_ptr!(outer.as_pointer(), u))).clone()) })
                == 7) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"sum_inner(&outer.u.inner) == 7"),
                        Ptr::<i8>::from_string_literal(b"union_struct_dual_use.c"),
                        28_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((((((elem!(
                (anon_1::raw_(field_ptr!(outer.as_pointer(), u)).reinterpret_cast::<i8>()
                    as Ptr::<i8>),
                0
            )
            .read()) as u8) as i32)
                == 3) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"(unsigned char)outer.u.raw[0] == 3"),
                        Ptr::<i8>::from_string_literal(b"union_struct_dual_use.c"),
                        30_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((((((elem!(
                (anon_1::raw_(field_ptr!(outer.as_pointer(), u)).reinterpret_cast::<i8>()
                    as Ptr::<i8>),
                4
            )
            .read()) as u8) as i32)
                == 4) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"(unsigned char)outer.u.raw[4] == 4"),
                        Ptr::<i8>::from_string_literal(b"union_struct_dual_use.c"),
                        31_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct __locale_data;
pub fn __assert_fail_2(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn memset_3(mut __s: AnyPtr, mut __c: i32, mut __n: usize) -> AnyPtr {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
