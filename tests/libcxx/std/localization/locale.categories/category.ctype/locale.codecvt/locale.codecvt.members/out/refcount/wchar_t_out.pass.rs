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
    let f: Ptr<std_codecvt_wchar_t__char____mbstate_t_> = ({ use_facet_0(l.as_pointer()) });
    {
        let from: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[
                115 as i32, 111 as i32, 109 as i32, 101 as i32, 32 as i32, 116 as i32, 101 as i32,
                120 as i32, 116 as i32, 0 as i32,
            ])
            .to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        let to: Value<Vec<i8>> = Rc::new(RefCell::new(
            (0..({ ((*from.borrow()).len() - 1).wrapping_add(1_usize) }) as usize)
                .map(|_| 0_i8)
                .collect::<Vec<_>>(),
        ));
        let mbs: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let from_next: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::null()));
        let to_next: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
        let mut r: std_codecvt_base_result = ({
            let ___from: Ptr<i32> = from.decay();
            let ___from_end: Ptr<i32> = from.decay().offset(((*from.borrow()).len() - 1) as isize);
            let ___to: Ptr<i8> = to.decay();
            let ___to_end: Ptr<i8> = to.decay().offset(((*to.borrow()).len()) as isize);
            ((*f.upgrade().deref()) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).out(
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
                ({ ((((*from_next.borrow()).clone() - from.decay()) as i64) as usize) } == {
                    ((*from.borrow()).len() - 1)
                })
            })
        );
        assert!(
            ({
                ({ ((((*to_next.borrow()).clone() - to.decay()) as i64) as usize) } == {
                    ((*from.borrow()).len() - 1)
                })
            })
        );
        assert!(match (Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"some text").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }))
        .as_pointer(),)
        {
            (__tmp_0,) => {
                to.decay().with_c_str(|__s| {
                    (*__tmp_0.upgrade().deref())
                        [..(*__tmp_0.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        });
    }
    {
        let from: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[
                115 as i32, 111 as i32, 109 as i32, 101 as i32, 32 as i32, 116 as i32, 101 as i32,
                120 as i32, 116 as i32, 0 as i32,
            ])
            .to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        from.decay()
            .offset(4_usize as isize)
            .write((('\0' as i8) as i32));
        let to: Value<Vec<i8>> = Rc::new(RefCell::new(
            (0..({ ((*from.borrow()).len() - 1).wrapping_add(1_usize) }) as usize)
                .map(|_| 0_i8)
                .collect::<Vec<_>>(),
        ));
        let mbs: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let from_next: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::null()));
        let to_next: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
        let mut r: std_codecvt_base_result = ({
            let ___from: Ptr<i32> = from.decay();
            let ___from_end: Ptr<i32> = from.decay().offset(((*from.borrow()).len() - 1) as isize);
            let ___to: Ptr<i8> = to.decay();
            let ___to_end: Ptr<i8> = to.decay().offset(((*to.borrow()).len()) as isize);
            ((*f.upgrade().deref()) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).out(
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
                ({ ((((*from_next.borrow()).clone() - from.decay()) as i64) as usize) } == {
                    ((*from.borrow()).len() - 1)
                })
            })
        );
        assert!(
            ({
                ({ ((((*to_next.borrow()).clone() - to.decay()) as i64) as usize) } == {
                    ((*from.borrow()).len() - 1)
                })
            })
        );
        assert!(
            ((to.decay() as Ptr<i8>).to_any().memcmp(
                &Ptr::<i8>::from_string_literal(b"some\0text").to_any(),
                ({ ((*from.borrow()).len() - 1) })
            ) == 0)
        );
    }
    {
        let from: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[
                115 as i32, 111 as i32, 109 as i32, 101 as i32, 32 as i32, 116 as i32, 101 as i32,
                120 as i32, 116 as i32, 0 as i32,
            ])
            .to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        let to: Value<Vec<i8>> = Rc::new(RefCell::new(
            (0..({ ((*from.borrow()).len() - 1).wrapping_sub(1_usize) }) as usize)
                .map(|_| 0_i8)
                .collect::<Vec<_>>(),
        ));
        let mbs: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let from_next: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::null()));
        let to_next: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
        let mut r: std_codecvt_base_result = ({
            let ___from: Ptr<i32> = from.decay();
            let ___from_end: Ptr<i32> = from.decay().offset(((*from.borrow()).len() - 1) as isize);
            let ___to: Ptr<i8> = to.decay();
            let ___to_end: Ptr<i8> = to
                .decay()
                .offset(((*to.borrow()).len()) as isize)
                .offset(-((1) as isize));
            ((*f.upgrade().deref()) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).out(
                mbs.as_pointer(),
                ___from,
                ___from_end,
                from_next.as_pointer(),
                ___to,
                ___to_end,
                to_next.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_partial as i32)));
        assert!(
            ({
                ({ ((((*from_next.borrow()).clone() - from.decay()) as i64) as usize) } == {
                    ((*to.borrow()).len()).wrapping_sub(1_usize)
                })
            })
        );
        assert!(
            ({
                ({ ((((*to_next.borrow()).clone() - to.decay()) as i64) as usize) } == {
                    ((*to.borrow()).len()).wrapping_sub(1_usize)
                })
            })
        );
        assert!(match (Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"some te").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }))
        .as_pointer(),)
        {
            (__tmp_1,) => {
                to.decay().with_c_str(|__s| {
                    (*__tmp_1.upgrade().deref())
                        [..(*__tmp_1.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        });
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
