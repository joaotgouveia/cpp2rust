extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct __fsid_t {
    #[offset(0)]
    #[byte_size(8)]
    pub __val: Value<Box<[i32]>>,
}
impl Clone for __fsid_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __fsid_t {
    fn default() -> Self {
        unimplemented!()
    }
}
pub fn bump_0(mut arg: AnyPtr) -> i32 {
    let mut value: Ptr<i32> = arg.reinterpret_cast::<i32>();
    {
        value.with_mut(|__v| *__v = *__v + 1)
    };
    return (value.read());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let value: Value<i32> = Rc::new(RefCell::new(41));
    let mut opaque: AnyPtr = ((value.as_pointer()) as Ptr<i32>).to_any();
    let mut typed: Ptr<i32> = opaque.reinterpret_cast::<i32>();
    {
        &(4usize);
        {
            if (((({ bump_0((opaque).clone()) }) == 42) as i32) != 0) {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<i8>::from_string_literal(b"bump(opaque) == 42"),
                        Ptr::<i8>::from_string_literal(b"void_ptr_implicit_conversion.c"),
                        14_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((((typed.read()) == 42) as i32) != 0) {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<i8>::from_string_literal(b"*typed == 42"),
                        Ptr::<i8>::from_string_literal(b"void_ptr_implicit_conversion.c"),
                        15_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    typed.write(7);
    {
        &(4usize);
        {
            if ((((*value.borrow()) == 7) as i32) != 0) {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<i8>::from_string_literal(b"value == 7"),
                        Ptr::<i8>::from_string_literal(b"void_ptr_implicit_conversion.c"),
                        17_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    return 0;
}
pub fn __assert_fail_1(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
