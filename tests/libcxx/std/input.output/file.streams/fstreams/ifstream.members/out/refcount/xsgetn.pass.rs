extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn check_unget_0(mut fb: Ptr<()>, mut expected: i8) {
    let mut c: () = unimplemented!();
    if (c != (-1_i32)) {
        assert!((c == (expected as i32)));
        assert!(({ unimplemented!() } == { (expected as i32) }));
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
    let stream_buffer: Value<Vec<i8>> = Rc::new(RefCell::new(
        (0..(10_usize) as usize).map(|_| 0_i8).collect::<Vec<_>>(),
    ));
    let fs: Value<::std::fs::File> = Rc::new(RefCell::new(unimplemented!()));
    let mut fb: Ptr<()> = (*fs.borrow()).clone();
    unimplemented!();
    assert!((unimplemented!() == (('t' as i8) as i32)));
    let test_buffer: Value<Vec<i8>> = Rc::new(RefCell::new(
        (0..(5_usize) as usize).map(|_| 0_i8).collect::<Vec<_>>(),
    ));
    test_buffer
        .decay()
        .offset(0_usize as isize)
        .write(('\0' as i8));
    {
        assert!((unimplemented!() == 5_()));
        assert!(
            Ptr::<i8>::from_string_literal(b"this ").with_c_str(|__s| {
                let mut __v = Vec::with_capacity(5_usize as usize + 1);
                test_buffer
                    .decay()
                    .with_slice(5_usize as usize, |__s| __v.extend_from_slice(__s));
                __v.push(0_i8);
                __v
            }[..{
                let mut __v = Vec::with_capacity(5_usize as usize + 1);
                test_buffer
                    .decay()
                    .with_slice(5_usize as usize, |__s| __v.extend_from_slice(__s));
                __v.push(0_i8);
                __v
            }
            .len()
            .saturating_sub(1)]
                == *__s)
        );
        ({ check_unget_0((fb).clone(), (' ' as i8)) });
    }
    {
        assert!((unimplemented!() == 5_()));
        assert!(
            Ptr::<i8>::from_string_literal(b"is so").with_c_str(|__s| {
                let mut __v = Vec::with_capacity(5_usize as usize + 1);
                test_buffer
                    .decay()
                    .with_slice(5_usize as usize, |__s| __v.extend_from_slice(__s));
                __v.push(0_i8);
                __v
            }[..{
                let mut __v = Vec::with_capacity(5_usize as usize + 1);
                test_buffer
                    .decay()
                    .with_slice(5_usize as usize, |__s| __v.extend_from_slice(__s));
                __v.push(0_i8);
                __v
            }
            .len()
            .saturating_sub(1)]
                == *__s)
        );
        ({ check_unget_0((fb).clone(), ('o' as i8)) });
    }
    {
        {
            let __a0 = 12_usize as usize;
            (*test_buffer.borrow_mut()).resize_with(__a0, || 0_i8)
        };
        assert!((unimplemented!() == 12_()));
        assert!(
            Ptr::<i8>::from_string_literal(b"me random da").with_c_str(|__s| {
                let mut __v = Vec::with_capacity(12_usize as usize + 1);
                test_buffer
                    .decay()
                    .with_slice(12_usize as usize, |__s| __v.extend_from_slice(__s));
                __v.push(0_i8);
                __v
            }[..{
                let mut __v = Vec::with_capacity(12_usize as usize + 1);
                test_buffer
                    .decay()
                    .with_slice(12_usize as usize, |__s| __v.extend_from_slice(__s));
                __v.push(0_i8);
                __v
            }
            .len()
            .saturating_sub(1)]
                == *__s)
        );
        ({ check_unget_0((fb).clone(), ('a' as i8)) });
    }
    {
        {
            let __a0 = 2_usize as usize;
            (*test_buffer.borrow_mut()).resize_with(__a0, || 0_i8)
        };
        assert!((unimplemented!() == 2_()));
        assert!(
            Ptr::<i8>::from_string_literal(b"ta").with_c_str(|__s| {
                let mut __v = Vec::with_capacity(2_usize as usize + 1);
                test_buffer
                    .decay()
                    .with_slice(2_usize as usize, |__s| __v.extend_from_slice(__s));
                __v.push(0_i8);
                __v
            }[..{
                let mut __v = Vec::with_capacity(2_usize as usize + 1);
                test_buffer
                    .decay()
                    .with_slice(2_usize as usize, |__s| __v.extend_from_slice(__s));
                __v.push(0_i8);
                __v
            }
            .len()
            .saturating_sub(1)]
                == *__s)
        );
        ({ check_unget_0((fb).clone(), ('a' as i8)) });
        {
            let __a0 = 12_usize as usize;
            (*test_buffer.borrow_mut()).resize_with(__a0, || 0_i8)
        };
        assert!((unimplemented!() == 12_()));
        assert!(
            Ptr::<i8>::from_string_literal(b" to be able ").with_c_str(|__s| {
                let mut __v = Vec::with_capacity(12_usize as usize + 1);
                test_buffer
                    .decay()
                    .with_slice(12_usize as usize, |__s| __v.extend_from_slice(__s));
                __v.push(0_i8);
                __v
            }[..{
                let mut __v = Vec::with_capacity(12_usize as usize + 1);
                test_buffer
                    .decay()
                    .with_slice(12_usize as usize, |__s| __v.extend_from_slice(__s));
                __v.push(0_i8);
                __v
            }
            .len()
            .saturating_sub(1)]
                == *__s)
        );
        ({ check_unget_0((fb).clone(), (' ' as i8)) });
    }
    {
        {
            let __a0 = 30_usize as usize;
            (*test_buffer.borrow_mut()).resize_with(__a0, || 0_i8)
        };
        assert!((unimplemented!() == 24_()));
        {
            let __a0 = 24_usize as usize;
            (*test_buffer.borrow_mut()).resize_with(__a0, || 0_i8)
        };
        assert!(
            Ptr::<i8>::from_string_literal(b"to test buffer behaviour").with_c_str(|__s| {
                let mut __v = Vec::with_capacity(24_usize as usize + 1);
                test_buffer
                    .decay()
                    .with_slice(24_usize as usize, |__s| __v.extend_from_slice(__s));
                __v.push(0_i8);
                __v
            }[..{
                let mut __v = Vec::with_capacity(24_usize as usize + 1);
                test_buffer
                    .decay()
                    .with_slice(24_usize as usize, |__s| __v.extend_from_slice(__s));
                __v.push(0_i8);
                __v
            }
            .len()
            .saturating_sub(1)]
                == *__s)
        );
        ({ check_unget_0((fb).clone(), ('r' as i8)) });
    }
    {
        let buf: Value<Box<[i8]>> =
            Rc::new(RefCell::new((0..10).map(|_| 0_i8).collect::<Box<[i8]>>()));
        let ifs: Value<::std::fs::File> = Rc::new(RefCell::new(unimplemented!()));
        let mut filebuf: Ptr<()> = (*ifs.borrow()).clone();
        assert!(!(unimplemented!()));
        assert!((unimplemented!() == 0_()));
        assert!(!(unimplemented!()));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
