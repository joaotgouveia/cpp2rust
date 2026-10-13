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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(128)]
pub struct __sigset_t {
    #[offset(0)]
    #[byte_size(128)]
    pub __val: Value<Box<[u64]>>,
}
impl Clone for __sigset_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __sigset_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct timeval {
    #[offset(0)]
    pub tv_sec: i64,
    #[offset(8)]
    pub tv_usec: i64,
}
impl Clone for timeval {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for timeval {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct timespec {
    #[offset(0)]
    pub tv_sec: i64,
    #[offset(8)]
    pub tv_nsec: i64,
}
impl Clone for timespec {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for timespec {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(128)]
pub struct fd_set {
    #[offset(0)]
    #[byte_size(128)]
    pub __fds_bits: Value<Box<[i64]>>,
}
impl Clone for fd_set {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for fd_set {
    fn default() -> Self {
        unimplemented!()
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let set: Value<fd_set> = <Value<fd_set>>::default();
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut __i: u32 = 0_u32;
        let mut __arr: Ptr<fd_set> = (set.as_pointer());
        __i = 0_u32;
        'loop_: while ((((__i as usize) < (128usize as usize).wrapping_div((8usize as usize)))
            as i32)
            != 0)
        {
            elem!(((array_field_ptr!((__arr), __fds_bits)) as Ptr::<i64>), __i).write(0_i64);
            __i.prefix_inc();
        }
    }
    {
        &(4usize);
        {
            if !(((((elem!(
                ((array_field_ptr!((set.as_pointer()), __fds_bits)) as Ptr::<i64>),
                ((3) / (8 * (8usize as i32)))
            )
            .read())
                & ((1_u64 << ((3) % (8 * (8usize as i32)))) as i64))
                != 0_i64) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"!FD_ISSET(3, &set)"),
                        Ptr::<i8>::from_string_literal(b"fd_set_macros.c"),
                        7_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    ({
        {
            elem!(
                ((array_field_ptr!((set.as_pointer()), __fds_bits)) as Ptr::<i64>),
                ((3) / (8 * (8usize as i32)))
            )
            .with_mut(|__v| *__v = *__v | ((1_u64 << ((3) % (8 * (8usize as i32)))) as i64))
        }
    });
    {
        &(4usize);
        {
            if (((((elem!(
                ((array_field_ptr!((set.as_pointer()), __fds_bits)) as Ptr::<i64>),
                ((3) / (8 * (8usize as i32)))
            )
            .read())
                & ((1_u64 << ((3) % (8 * (8usize as i32)))) as i64))
                != 0_i64) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"FD_ISSET(3, &set)"),
                        Ptr::<i8>::from_string_literal(b"fd_set_macros.c"),
                        9_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    ({
        {
            elem!(
                ((array_field_ptr!((set.as_pointer()), __fds_bits)) as Ptr::<i64>),
                ((3) / (8 * (8usize as i32)))
            )
            .with_mut(|__v| *__v = *__v & !((1_u64 << ((3) % (8 * (8usize as i32)))) as i64))
        }
    });
    {
        &(4usize);
        {
            if !(((((elem!(
                ((array_field_ptr!((set.as_pointer()), __fds_bits)) as Ptr::<i64>),
                ((3) / (8 * (8usize as i32)))
            )
            .read())
                & ((1_u64 << ((3) % (8 * (8usize as i32)))) as i64))
                != 0_i64) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_0(
                        Ptr::<i8>::from_string_literal(b"!FD_ISSET(3, &set)"),
                        Ptr::<i8>::from_string_literal(b"fd_set_macros.c"),
                        11_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
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
