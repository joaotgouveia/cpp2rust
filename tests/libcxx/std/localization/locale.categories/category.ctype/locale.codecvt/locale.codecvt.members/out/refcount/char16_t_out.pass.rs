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
    let f: Ptr<std_codecvt_char16_t__char____mbstate_t_> = ({ use_facet_0(l.as_pointer()) });
    {
        let from: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::new([
            (('s' as i8) as u16),
            (('o' as i8) as u16),
            (('m' as i8) as u16),
            (('e' as i8) as u16),
            ((' ' as i8) as u16),
            (('t' as i8) as u16),
            (('e' as i8) as u16),
            (('x' as i8) as u16),
            (('t' as i8) as u16),
        ])));
        let to: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
        ])));
        let mbs: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let from_next: Value<Ptr<u16>> = Rc::new(RefCell::new(Ptr::<u16>::null()));
        let to_next: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
        let mut r: std_codecvt_base_result = ({
            let ___from: Ptr<u16> = (from.as_pointer() as Ptr<u16>);
            let ___from_end: Ptr<u16> = (from.as_pointer() as Ptr<u16>).offset((9) as isize);
            let ___to: Ptr<i8> = (to.as_pointer() as Ptr<i8>);
            let ___to_end: Ptr<i8> = (to.as_pointer() as Ptr<i8>).offset((9) as isize);
            ((*f.upgrade().deref()) as std___codecvt_abstract_base_char16_t__char____mbstate_t_)
                .out(
                    mbs.as_pointer(),
                    ___from,
                    ___from_end,
                    from_next.as_pointer(),
                    ___to,
                    ___to_end,
                    to_next.as_pointer(),
                )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(
            ({
                (((*from_next.borrow()).clone() - (from.as_pointer() as Ptr<u16>)) as i64 == 9_i64)
            })
        );
        assert!(
            ({ (((*to_next.borrow()).clone() - (to.as_pointer() as Ptr::<i8>)) as i64 == 9_i64) })
        );
        let mut i: u32 = 0_u32;
        'loop_: while (i < 9_u32) {
            assert!(
                ({
                    (((*to.borrow())[(i) as usize] as i32)
                        == ((*from.borrow())[(i) as usize] as i32))
                })
            );
            i.prefix_inc();
        }
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
