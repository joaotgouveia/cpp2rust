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
    assert!(!((unimplemented!()).is_null()));
    assert!((unimplemented!() >= 256_usize));
    let mut p: Ptr<()> = unimplemented!();
    let mut i: usize = 0_usize;
    'loop_: while (i < 128_usize) {
        let mut expect_cntrl: bool = ((i < 32_usize) || (126_usize < i));
        let mut expect_print: bool = ((32_usize <= i) && (i <= 126_usize));
        let mut expect_space: bool =
            ((9_usize <= i) && (i <= 13_usize)) || (i == ((' ' as i8) as usize));
        let mut expect_blank: bool =
            ((i == (('\t' as i8) as usize)) || (i == ((' ' as i8) as usize)));
        let mut expect_upper: bool =
            (((('A' as i8) as usize) <= i) && (i <= (('Z' as i8) as usize)));
        let mut expect_lower: bool =
            (((('a' as i8) as usize) <= i) && (i <= (('z' as i8) as usize)));
        let mut expect_alpha: bool = (expect_upper) || (expect_lower);
        let mut expect_digit: bool =
            (((('0' as i8) as usize) <= i) && (i <= (('9' as i8) as usize)));
        let mut expect_xdigit: bool = ((((('A' as i8) as usize) <= i)
            && (i <= (('F' as i8) as usize)))
            || (((('a' as i8) as usize) <= i) && (i <= (('f' as i8) as usize))))
            || (expect_digit);
        let mut expect_punct: bool = ((((33_usize <= i) && (i <= 47_usize))
            || ((58_usize <= i) && (i <= 64_usize)))
            || ((91_usize <= i) && (i <= 96_usize)))
            || ((123_usize <= i) && (i <= 126_usize));
        assert!(
            ({
                ((({ ((elem!(p, i).read()) as i32) } & { (unimplemented!() as i32) }) != 0) as i32)
            } == { (expect_cntrl as i32) })
        );
        assert!(
            ({
                ((({ ((elem!(p, i).read()) as i32) } & { (unimplemented!() as i32) }) != 0) as i32)
            } == { (expect_print as i32) })
        );
        assert!(
            ({
                ((({ ((elem!(p, i).read()) as i32) } & { (unimplemented!() as i32) }) != 0) as i32)
            } == { (expect_space as i32) })
        );
        assert!(
            ({
                ((({ ((elem!(p, i).read()) as i32) } & { (unimplemented!() as i32) }) != 0) as i32)
            } == { (expect_blank as i32) })
        );
        assert!(
            ({
                ((({ ((elem!(p, i).read()) as i32) } & { (unimplemented!() as i32) }) != 0) as i32)
            } == { (expect_lower as i32) })
        );
        assert!(
            ({
                ((({ ((elem!(p, i).read()) as i32) } & { (unimplemented!() as i32) }) != 0) as i32)
            } == { (expect_upper as i32) })
        );
        assert!(
            ({
                ((({ ((elem!(p, i).read()) as i32) } & { (unimplemented!() as i32) }) != 0) as i32)
            } == { (expect_alpha as i32) })
        );
        assert!(
            ({
                ((({ ((elem!(p, i).read()) as i32) } & { (unimplemented!() as i32) }) != 0) as i32)
            } == { (expect_digit as i32) })
        );
        assert!(
            ({
                ((({ ((elem!(p, i).read()) as i32) } & { (unimplemented!() as i32) }) != 0) as i32)
            } == { (expect_xdigit as i32) })
        );
        assert!(
            ({
                ((({ ((elem!(p, i).read()) as i32) } & { (unimplemented!() as i32) }) != 0) as i32)
            } == { (expect_punct as i32) })
        );
        i.prefix_inc();
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
