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
        let c: Value<std_codecvt_utf16_wchar_t__1114111__0_> = Rc::new(RefCell::new(
            <std_codecvt_utf16_wchar_t__1114111__0_>::default(),
        ));
        let n: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([0_i8, 0_i8, 0_i8, 0_i8])));
        let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let np: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
        let mut r: std_codecvt_base_result = ({
            let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).unshift(
                m.as_pointer(),
                ___to,
                ___to_end,
                np.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_noconv as i32)));
    }
    {
        let c: Value<std_codecvt_utf16_char16_t__1114111__0_> = Rc::new(RefCell::new(
            <std_codecvt_utf16_char16_t__1114111__0_>::default(),
        ));
        let n: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([0_i8, 0_i8, 0_i8, 0_i8])));
        let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let np: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
        let mut r: std_codecvt_base_result = ({
            let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char16_t__char____mbstate_t_).unshift(
                m.as_pointer(),
                ___to,
                ___to_end,
                np.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_noconv as i32)));
    }
    {
        let c: Value<std_codecvt_utf16_char32_t__1114111__0_> = Rc::new(RefCell::new(
            <std_codecvt_utf16_char32_t__1114111__0_>::default(),
        ));
        let n: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([0_i8, 0_i8, 0_i8, 0_i8])));
        let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let np: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
        let mut r: std_codecvt_base_result = ({
            let ___to: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___to_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).unshift(
                m.as_pointer(),
                ___to,
                ___to_end,
                np.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_noconv as i32)));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
