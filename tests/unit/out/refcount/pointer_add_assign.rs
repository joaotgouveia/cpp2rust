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
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let arr: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([1_i8, 2_i8, 3_i8])));
    {
        let mut p: Ptr<i8> = ((arr.as_pointer() as Ptr<i8>).offset(0));
        p += (1_u8 as i32);
        (if (((p.read()) as i32) == 2) {
            &(0);
        } else {
            ({
                __assert_fail_0(
                    Ptr::<i8>::from_string_literal(b"*p == 2"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
    {
        let mut p: Ptr<i8> = ((arr.as_pointer() as Ptr<i8>).offset(0));
        p += (1_i8 as i32);
        (if (((p.read()) as i32) == 2) {
            &(0);
        } else {
            ({
                __assert_fail_0(
                    Ptr::<i8>::from_string_literal(b"*p == 2"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
    {
        let mut p: Ptr<i8> = ((arr.as_pointer() as Ptr<i8>).offset(0));
        p += (1_u16 as i32);
        (if (((p.read()) as i32) == 2) {
            &(0);
        } else {
            ({
                __assert_fail_0(
                    Ptr::<i8>::from_string_literal(b"*p == 2"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
    {
        let mut p: Ptr<i8> = ((arr.as_pointer() as Ptr<i8>).offset(0));
        p += (1_i16 as i32);
        (if (((p.read()) as i32) == 2) {
            &(0);
        } else {
            ({
                __assert_fail_0(
                    Ptr::<i8>::from_string_literal(b"*p == 2"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
    {
        let mut p: Ptr<i8> = ((arr.as_pointer() as Ptr<i8>).offset(0));
        p += 1_u32;
        (if (((p.read()) as i32) == 2) {
            &(0);
        } else {
            ({
                __assert_fail_0(
                    Ptr::<i8>::from_string_literal(b"*p == 2"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
    {
        let mut p: Ptr<i8> = ((arr.as_pointer() as Ptr<i8>).offset(0));
        p += (1 as i32);
        (if (((p.read()) as i32) == 2) {
            &(0);
        } else {
            ({
                __assert_fail_0(
                    Ptr::<i8>::from_string_literal(b"*p == 2"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
    {
        let mut p: Ptr<i8> = ((arr.as_pointer() as Ptr<i8>).offset(0));
        p += 1_u64;
        (if (((p.read()) as i32) == 2) {
            &(0);
        } else {
            ({
                __assert_fail_0(
                    Ptr::<i8>::from_string_literal(b"*p == 2"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
    {
        let mut p: Ptr<i8> = ((arr.as_pointer() as Ptr<i8>).offset(0));
        p += 1_i64;
        (if (((p.read()) as i32) == 2) {
            &(0);
        } else {
            ({
                __assert_fail_0(
                    Ptr::<i8>::from_string_literal(b"*p == 2"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
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
