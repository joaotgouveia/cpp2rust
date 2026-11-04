extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct E {
    #[offset(0)]
    pub a: i32,
    #[offset(8)]
    pub b: f64,
    #[offset(16)]
    pub c: i8,
}
impl CppMove for E {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for E {
    fn default() -> Self {
        {
            E {
                a: 1,
                b: 1_f64,
                c: 1_i8,
            }
        }
    }
}
pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    {
        ({ test_0(14, 10, 11, 12, 13) });
        ({
            test_1(
                ('e' as i8),
                ('a' as i8),
                ('b' as i8),
                ('c' as i8),
                ('d' as i8),
            )
        });
    }
    {
        ({ test_2(14, 10, 11, 12, 13) });
        ({
            test_3(
                ('e' as i8),
                ('a' as i8),
                ('b' as i8),
                ('c' as i8),
                ('d' as i8),
            )
        });
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
