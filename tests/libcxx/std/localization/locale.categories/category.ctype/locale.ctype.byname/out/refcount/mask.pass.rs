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
    let l: Value<std_locale> = Rc::new(RefCell::new(std_locale::new_1({
        Ptr::<i8>::from_string_literal(b"C")
    })));
    {
        let cf: Ptr<std_ctype_char_> = ({ use_facet_0(l.as_pointer()) });
        assert!(
            ({
                ({
                    let ___m: u16 = upper_1.with(|rc| *rc.borrow());
                    (*cf.upgrade().deref()).is_3(___m, ('A' as i8))
                })
            })
        );
        assert!(
            ({
                !({
                    let ___m: u16 = lower_2.with(|rc| *rc.borrow());
                    (*cf.upgrade().deref()).is_3(___m, ('A' as i8))
                })
            })
        );
        assert!(
            ({
                ({
                    let ___m: u16 = alpha_3.with(|rc| *rc.borrow());
                    (*cf.upgrade().deref()).is_3(___m, ('A' as i8))
                })
            })
        );
        assert!(
            ({
                !({
                    let ___m: u16 = upper_1.with(|rc| *rc.borrow());
                    (*cf.upgrade().deref()).is_3(___m, ('a' as i8))
                })
            })
        );
        assert!(
            ({
                ({
                    let ___m: u16 = lower_2.with(|rc| *rc.borrow());
                    (*cf.upgrade().deref()).is_3(___m, ('a' as i8))
                })
            })
        );
        assert!(
            ({
                ({
                    let ___m: u16 = alpha_3.with(|rc| *rc.borrow());
                    (*cf.upgrade().deref()).is_3(___m, ('a' as i8))
                })
            })
        );
    }
    {
        let wf: Ptr<std_ctype_wchar_t_> = ({ use_facet_4(l.as_pointer()) });
        assert!(
            ({
                ({
                    ((*wf.upgrade().deref()) as std___ctype_abstract_base_wchar_t_)
                        .is_1(upper_1.with(|rc| *rc.borrow()), (65 as i32))
                })
            })
        );
        assert!(
            ({
                !({
                    ((*wf.upgrade().deref()) as std___ctype_abstract_base_wchar_t_)
                        .is_1(lower_2.with(|rc| *rc.borrow()), (65 as i32))
                })
            })
        );
        assert!(
            ({
                ({
                    ((*wf.upgrade().deref()) as std___ctype_abstract_base_wchar_t_)
                        .is_1(alpha_3.with(|rc| *rc.borrow()), (65 as i32))
                })
            })
        );
        assert!(
            ({
                !({
                    ((*wf.upgrade().deref()) as std___ctype_abstract_base_wchar_t_)
                        .is_1(upper_1.with(|rc| *rc.borrow()), (97 as i32))
                })
            })
        );
        assert!(
            ({
                ({
                    ((*wf.upgrade().deref()) as std___ctype_abstract_base_wchar_t_)
                        .is_1(lower_2.with(|rc| *rc.borrow()), (97 as i32))
                })
            })
        );
        assert!(
            ({
                ({
                    ((*wf.upgrade().deref()) as std___ctype_abstract_base_wchar_t_)
                        .is_1(alpha_3.with(|rc| *rc.borrow()), (97 as i32))
                })
            })
        );
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
