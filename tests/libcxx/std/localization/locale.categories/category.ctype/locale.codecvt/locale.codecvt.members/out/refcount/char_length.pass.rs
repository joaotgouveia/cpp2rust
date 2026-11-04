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
    let l: Value<std_locale> = Rc::new(RefCell::new(
        (*({ std_locale::classic() }).upgrade().deref()).clone(),
    ));
    let f: Ptr<std_codecvt_char__char____mbstate_t_> = ({ use_facet_0(l.as_pointer()) });
    let mbs: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
    let from: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
    ])));
    assert!(
        ({
            (({
                let ___from: Ptr<i8> = (from.as_pointer() as Ptr<i8>);
                let ___end: Ptr<i8> = (from.as_pointer() as Ptr<i8>).offset((10) as isize);
                ((*f.upgrade().deref()) as std___codecvt_abstract_base_char__char____mbstate_t_)
                    .length(mbs.as_pointer(), ___from, ___end, 0_usize)
            }) == 0)
        })
    );
    assert!(
        ({
            (({
                let ___from: Ptr<i8> = (from.as_pointer() as Ptr<i8>);
                let ___end: Ptr<i8> = (from.as_pointer() as Ptr<i8>).offset((10) as isize);
                ((*f.upgrade().deref()) as std___codecvt_abstract_base_char__char____mbstate_t_)
                    .length(mbs.as_pointer(), ___from, ___end, 9_usize)
            }) == 9)
        })
    );
    assert!(
        ({
            (({
                let ___from: Ptr<i8> = (from.as_pointer() as Ptr<i8>);
                let ___end: Ptr<i8> = (from.as_pointer() as Ptr<i8>).offset((10) as isize);
                ((*f.upgrade().deref()) as std___codecvt_abstract_base_char__char____mbstate_t_)
                    .length(mbs.as_pointer(), ___from, ___end, 10_usize)
            }) == 10)
        })
    );
    assert!(
        ({
            (({
                let ___from: Ptr<i8> = (from.as_pointer() as Ptr<i8>);
                let ___end: Ptr<i8> = (from.as_pointer() as Ptr<i8>).offset((10) as isize);
                ((*f.upgrade().deref()) as std___codecvt_abstract_base_char__char____mbstate_t_)
                    .length(mbs.as_pointer(), ___from, ___end, 11_usize)
            }) == 10)
        })
    );
    assert!(
        ({
            (({
                let ___from: Ptr<i8> = (from.as_pointer() as Ptr<i8>);
                let ___end: Ptr<i8> = (from.as_pointer() as Ptr<i8>).offset((10) as isize);
                ((*f.upgrade().deref()) as std___codecvt_abstract_base_char__char____mbstate_t_)
                    .length(mbs.as_pointer(), ___from, ___end, 100_usize)
            }) == 10)
        })
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
