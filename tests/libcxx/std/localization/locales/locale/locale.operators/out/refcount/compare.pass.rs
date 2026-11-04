extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};

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
        let l: Value<std_locale> = Rc::new(RefCell::new(<std_locale>::default()));
        {
            let s2: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"aaaaaaA").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            let s3: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"BaaaaaA").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            assert!(
                ({
                    ({
                        (*l.borrow_mut())(
                            (s3.as_pointer()).reinterpret_cast::<Vec<i8>>(),
                            (s2.as_pointer()).reinterpret_cast::<Vec<i8>>(),
                        )
                    })
                })
            );
        }
        {
            let s2: Value<Vec<i32>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i32>::from_string_literal(&[
                    97 as i32, 97 as i32, 97 as i32, 97 as i32, 97 as i32, 97 as i32, 65 as i32,
                    0 as i32,
                ])
                .to_c_bytes();
                __bytes.push(0_i32);
                __bytes
            }));
            let s3: Value<Vec<i32>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i32>::from_string_literal(&[
                    66 as i32, 97 as i32, 97 as i32, 97 as i32, 97 as i32, 97 as i32, 65 as i32,
                    0 as i32,
                ])
                .to_c_bytes();
                __bytes.push(0_i32);
                __bytes
            }));
            assert!(
                ({
                    ({
                        (*l.borrow_mut())(
                            (s3.as_pointer()).reinterpret_cast::<Vec<i32>>(),
                            (s2.as_pointer()).reinterpret_cast::<Vec<i32>>(),
                        )
                    })
                })
            );
        }
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
