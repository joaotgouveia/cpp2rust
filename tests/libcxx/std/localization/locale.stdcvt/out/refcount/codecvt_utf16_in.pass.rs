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
        let c: Value<std_codecvt_utf16_char32_t__1114111__0_> = Rc::new(RefCell::new(
            <std_codecvt_utf16_char32_t__1114111__0_>::default(),
        ));
        let w: Value<u32> = Rc::new(RefCell::new(0_u32));
        let n: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([-40_i8, -64_i8, -36_i8, 3_i8])));
        let wp: Value<Ptr<u32>> = Rc::new(RefCell::new(Ptr::<u32>::null()));
        let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let np: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
        let mut r: std_codecvt_base_result = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((4) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 262147_u32) }));
        (*n.borrow_mut())[(0) as usize] = 16_i8;
        (*n.borrow_mut())[(1) as usize] = 5_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 4101_u32) }));
        (*n.borrow_mut())[(0) as usize] = 4_i8;
        (*n.borrow_mut())[(1) as usize] = 83_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 1107_u32) }));
        (*w.borrow_mut()) = 86_u32;
        (*n.borrow_mut())[(0) as usize] = 0_i8;
        (*n.borrow_mut())[(1) as usize] = 86_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 86_u32) }));
    }
    {
        let c: Value<std_codecvt_utf16_char32_t__4096__0_> = Rc::new(RefCell::new(
            <std_codecvt_utf16_char32_t__4096__0_>::default(),
        ));
        let w: Value<u32> = Rc::new(RefCell::new(0_u32));
        let n: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([-40_i8, -64_i8, -36_i8, 3_i8])));
        let wp: Value<Ptr<u32>> = Rc::new(RefCell::new(Ptr::<u32>::null()));
        let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let np: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
        let mut r: std_codecvt_base_result = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_error as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()) }) }));
        assert!(({ ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr::<i8>) }) }));
        assert!(({ ((*w.borrow()) == 0_u32) }));
        (*n.borrow_mut())[(0) as usize] = 16_i8;
        (*n.borrow_mut())[(1) as usize] = 5_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_error as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()) }) }));
        assert!(({ ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr::<i8>) }) }));
        assert!(({ ((*w.borrow()) == 0_u32) }));
        (*n.borrow_mut())[(0) as usize] = 4_i8;
        (*n.borrow_mut())[(1) as usize] = 83_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 1107_u32) }));
        (*w.borrow_mut()) = 86_u32;
        (*n.borrow_mut())[(0) as usize] = 0_i8;
        (*n.borrow_mut())[(1) as usize] = 86_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 86_u32) }));
    }
    {
        let c: Value<std_codecvt_utf16_char32_t__1114111__std_consume_header_> = Rc::new(
            RefCell::new(<std_codecvt_utf16_char32_t__1114111__std_consume_header_>::default()),
        );
        let w: Value<u32> = Rc::new(RefCell::new(0_u32));
        let n: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            -2_i8, -1_i8, -40_i8, -64_i8, -36_i8, 3_i8,
        ])));
        let wp: Value<Ptr<u32>> = Rc::new(RefCell::new(Ptr::<u32>::null()));
        let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let np: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
        let mut r: std_codecvt_base_result = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((6) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((6) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 262147_u32) }));
        (*n.borrow_mut())[(0) as usize] = 16_i8;
        (*n.borrow_mut())[(1) as usize] = 5_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 4101_u32) }));
        (*n.borrow_mut())[(0) as usize] = 4_i8;
        (*n.borrow_mut())[(1) as usize] = 83_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 1107_u32) }));
        (*w.borrow_mut()) = 86_u32;
        (*n.borrow_mut())[(0) as usize] = 0_i8;
        (*n.borrow_mut())[(1) as usize] = 86_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 86_u32) }));
    }
    {
        let c: Value<std_codecvt_utf16_char32_t__1114111__std_little_endian_> = Rc::new(
            RefCell::new(<std_codecvt_utf16_char32_t__1114111__std_little_endian_>::default()),
        );
        let w: Value<u32> = Rc::new(RefCell::new(0_u32));
        let n: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([-64_i8, -40_i8, 3_i8, -36_i8])));
        let wp: Value<Ptr<u32>> = Rc::new(RefCell::new(Ptr::<u32>::null()));
        let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let np: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
        let mut r: std_codecvt_base_result = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((4) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 262147_u32) }));
        (*n.borrow_mut())[(1) as usize] = 16_i8;
        (*n.borrow_mut())[(0) as usize] = 5_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 4101_u32) }));
        (*n.borrow_mut())[(1) as usize] = 4_i8;
        (*n.borrow_mut())[(0) as usize] = 83_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 1107_u32) }));
        (*w.borrow_mut()) = 86_u32;
        (*n.borrow_mut())[(1) as usize] = 0_i8;
        (*n.borrow_mut())[(0) as usize] = 86_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 86_u32) }));
    }
    {
        let c: Value<std_codecvt_utf16_char32_t__4096__std_little_endian_> = Rc::new(RefCell::new(
            <std_codecvt_utf16_char32_t__4096__std_little_endian_>::default(),
        ));
        let w: Value<u32> = Rc::new(RefCell::new(0_u32));
        let n: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([-64_i8, -40_i8, 3_i8, -36_i8])));
        let wp: Value<Ptr<u32>> = Rc::new(RefCell::new(Ptr::<u32>::null()));
        let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let np: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
        let mut r: std_codecvt_base_result = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_error as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()) }) }));
        assert!(({ ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr::<i8>) }) }));
        assert!(({ ((*w.borrow()) == 0_u32) }));
        (*n.borrow_mut())[(1) as usize] = 16_i8;
        (*n.borrow_mut())[(0) as usize] = 5_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_error as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()) }) }));
        assert!(({ ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr::<i8>) }) }));
        assert!(({ ((*w.borrow()) == 0_u32) }));
        (*n.borrow_mut())[(1) as usize] = 4_i8;
        (*n.borrow_mut())[(0) as usize] = 83_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 1107_u32) }));
        (*w.borrow_mut()) = 86_u32;
        (*n.borrow_mut())[(1) as usize] = 0_i8;
        (*n.borrow_mut())[(0) as usize] = 86_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 86_u32) }));
    }
    {
        let c: Value<std_codecvt_utf16_char32_t__1114111__5_> = Rc::new(RefCell::new(
            <std_codecvt_utf16_char32_t__1114111__5_>::default(),
        ));
        let w: Value<u32> = Rc::new(RefCell::new(0_u32));
        let n: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            -1_i8, -2_i8, -64_i8, -40_i8, 3_i8, -36_i8,
        ])));
        let wp: Value<Ptr<u32>> = Rc::new(RefCell::new(Ptr::<u32>::null()));
        let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let np: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
        let mut r: std_codecvt_base_result = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((6) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((6) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 262147_u32) }));
        (*n.borrow_mut())[(1) as usize] = 16_i8;
        (*n.borrow_mut())[(0) as usize] = 5_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 4101_u32) }));
        (*n.borrow_mut())[(1) as usize] = 4_i8;
        (*n.borrow_mut())[(0) as usize] = 83_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 1107_u32) }));
        (*w.borrow_mut()) = 86_u32;
        (*n.borrow_mut())[(1) as usize] = 0_i8;
        (*n.borrow_mut())[(0) as usize] = 86_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 86_u32) }));
    }
    {
        let c: Value<std_codecvt_utf16_char32_t__1114111__0_> = Rc::new(RefCell::new(
            <std_codecvt_utf16_char32_t__1114111__0_>::default(),
        ));
        let w: Value<u32> = Rc::new(RefCell::new(0_u32));
        let n: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([-40_i8, -64_i8, -36_i8, 3_i8])));
        let wp: Value<Ptr<u32>> = Rc::new(RefCell::new(Ptr::<u32>::null()));
        let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let np: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
        let mut r: std_codecvt_base_result = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((4) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 262147_u32) }));
        (*n.borrow_mut())[(0) as usize] = 16_i8;
        (*n.borrow_mut())[(1) as usize] = 5_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 4101_u32) }));
        (*n.borrow_mut())[(0) as usize] = 4_i8;
        (*n.borrow_mut())[(1) as usize] = 83_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 1107_u32) }));
        (*w.borrow_mut()) = 86_u32;
        (*n.borrow_mut())[(0) as usize] = 0_i8;
        (*n.borrow_mut())[(1) as usize] = 86_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 86_u32) }));
    }
    {
        let c: Value<std_codecvt_utf16_char32_t__4096__0_> = Rc::new(RefCell::new(
            <std_codecvt_utf16_char32_t__4096__0_>::default(),
        ));
        let w: Value<u32> = Rc::new(RefCell::new(0_u32));
        let n: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([-40_i8, -64_i8, -36_i8, 3_i8])));
        let wp: Value<Ptr<u32>> = Rc::new(RefCell::new(Ptr::<u32>::null()));
        let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let np: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
        let mut r: std_codecvt_base_result = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_error as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()) }) }));
        assert!(({ ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr::<i8>) }) }));
        assert!(({ ((*w.borrow()) == 0_u32) }));
        (*n.borrow_mut())[(0) as usize] = 16_i8;
        (*n.borrow_mut())[(1) as usize] = 5_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_error as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()) }) }));
        assert!(({ ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr::<i8>) }) }));
        assert!(({ ((*w.borrow()) == 0_u32) }));
        (*n.borrow_mut())[(0) as usize] = 4_i8;
        (*n.borrow_mut())[(1) as usize] = 83_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 1107_u32) }));
        (*w.borrow_mut()) = 86_u32;
        (*n.borrow_mut())[(0) as usize] = 0_i8;
        (*n.borrow_mut())[(1) as usize] = 86_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 86_u32) }));
    }
    {
        let c: Value<std_codecvt_utf16_char32_t__1114111__std_consume_header_> = Rc::new(
            RefCell::new(<std_codecvt_utf16_char32_t__1114111__std_consume_header_>::default()),
        );
        let w: Value<u32> = Rc::new(RefCell::new(0_u32));
        let n: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            -2_i8, -1_i8, -40_i8, -64_i8, -36_i8, 3_i8,
        ])));
        let wp: Value<Ptr<u32>> = Rc::new(RefCell::new(Ptr::<u32>::null()));
        let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let np: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
        let mut r: std_codecvt_base_result = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((6) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((6) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 262147_u32) }));
        (*n.borrow_mut())[(0) as usize] = 16_i8;
        (*n.borrow_mut())[(1) as usize] = 5_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 4101_u32) }));
        (*n.borrow_mut())[(0) as usize] = 4_i8;
        (*n.borrow_mut())[(1) as usize] = 83_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 1107_u32) }));
        (*w.borrow_mut()) = 86_u32;
        (*n.borrow_mut())[(0) as usize] = 0_i8;
        (*n.borrow_mut())[(1) as usize] = 86_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 86_u32) }));
    }
    {
        let c: Value<std_codecvt_utf16_char32_t__1114111__std_little_endian_> = Rc::new(
            RefCell::new(<std_codecvt_utf16_char32_t__1114111__std_little_endian_>::default()),
        );
        let w: Value<u32> = Rc::new(RefCell::new(0_u32));
        let n: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([-64_i8, -40_i8, 3_i8, -36_i8])));
        let wp: Value<Ptr<u32>> = Rc::new(RefCell::new(Ptr::<u32>::null()));
        let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let np: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
        let mut r: std_codecvt_base_result = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((4) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 262147_u32) }));
        (*n.borrow_mut())[(1) as usize] = 16_i8;
        (*n.borrow_mut())[(0) as usize] = 5_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 4101_u32) }));
        (*n.borrow_mut())[(1) as usize] = 4_i8;
        (*n.borrow_mut())[(0) as usize] = 83_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 1107_u32) }));
        (*w.borrow_mut()) = 86_u32;
        (*n.borrow_mut())[(1) as usize] = 0_i8;
        (*n.borrow_mut())[(0) as usize] = 86_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 86_u32) }));
    }
    {
        let c: Value<std_codecvt_utf16_char32_t__4096__std_little_endian_> = Rc::new(RefCell::new(
            <std_codecvt_utf16_char32_t__4096__std_little_endian_>::default(),
        ));
        let w: Value<u32> = Rc::new(RefCell::new(0_u32));
        let n: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([-64_i8, -40_i8, 3_i8, -36_i8])));
        let wp: Value<Ptr<u32>> = Rc::new(RefCell::new(Ptr::<u32>::null()));
        let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let np: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
        let mut r: std_codecvt_base_result = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_error as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()) }) }));
        assert!(({ ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr::<i8>) }) }));
        assert!(({ ((*w.borrow()) == 0_u32) }));
        (*n.borrow_mut())[(1) as usize] = 16_i8;
        (*n.borrow_mut())[(0) as usize] = 5_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_error as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()) }) }));
        assert!(({ ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr::<i8>) }) }));
        assert!(({ ((*w.borrow()) == 0_u32) }));
        (*n.borrow_mut())[(1) as usize] = 4_i8;
        (*n.borrow_mut())[(0) as usize] = 83_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 1107_u32) }));
        (*w.borrow_mut()) = 86_u32;
        (*n.borrow_mut())[(1) as usize] = 0_i8;
        (*n.borrow_mut())[(0) as usize] = 86_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 86_u32) }));
    }
    {
        let c: Value<std_codecvt_utf16_char32_t__1114111__5_> = Rc::new(RefCell::new(
            <std_codecvt_utf16_char32_t__1114111__5_>::default(),
        ));
        let w: Value<u32> = Rc::new(RefCell::new(0_u32));
        let n: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            -1_i8, -2_i8, -64_i8, -40_i8, 3_i8, -36_i8,
        ])));
        let wp: Value<Ptr<u32>> = Rc::new(RefCell::new(Ptr::<u32>::null()));
        let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let np: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
        let mut r: std_codecvt_base_result = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((6) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((6) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 262147_u32) }));
        (*n.borrow_mut())[(1) as usize] = 16_i8;
        (*n.borrow_mut())[(0) as usize] = 5_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 4101_u32) }));
        (*n.borrow_mut())[(1) as usize] = 4_i8;
        (*n.borrow_mut())[(0) as usize] = 83_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 1107_u32) }));
        (*w.borrow_mut()) = 86_u32;
        (*n.borrow_mut())[(1) as usize] = 0_i8;
        (*n.borrow_mut())[(0) as usize] = 86_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u32> = (w.as_pointer());
            let ___to_end: Ptr<u32> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char32_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ ((*w.borrow()) == 86_u32) }));
    }
    {
        let c: Value<std_codecvt_utf16_char16_t__1114111__0_> = Rc::new(RefCell::new(
            <std_codecvt_utf16_char16_t__1114111__0_>::default(),
        ));
        let w: Value<u16> = Rc::new(RefCell::new(0_u16));
        let n: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([-40_i8, -64_i8, -36_i8, 3_i8])));
        let wp: Value<Ptr<u16>> = Rc::new(RefCell::new(Ptr::<u16>::null()));
        let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let np: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
        let mut r: std_codecvt_base_result = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
            let ___to: Ptr<u16> = (w.as_pointer());
            let ___to_end: Ptr<u16> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char16_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_error as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()) }) }));
        assert!(({ ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr::<i8>) }) }));
        assert!(({ (((*w.borrow()) as i32) == 0) }));
        (*n.borrow_mut())[(0) as usize] = 16_i8;
        (*n.borrow_mut())[(1) as usize] = 5_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u16> = (w.as_pointer());
            let ___to_end: Ptr<u16> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char16_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ (((*w.borrow()) as i32) == 4101) }));
        (*n.borrow_mut())[(0) as usize] = 4_i8;
        (*n.borrow_mut())[(1) as usize] = 83_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u16> = (w.as_pointer());
            let ___to_end: Ptr<u16> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char16_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ (((*w.borrow()) as i32) == 1107) }));
        (*w.borrow_mut()) = 86_u16;
        (*n.borrow_mut())[(0) as usize] = 0_i8;
        (*n.borrow_mut())[(1) as usize] = 86_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u16> = (w.as_pointer());
            let ___to_end: Ptr<u16> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char16_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ (((*w.borrow()) as i32) == 86) }));
    }
    {
        let c: Value<std_codecvt_utf16_char16_t__4096__0_> = Rc::new(RefCell::new(
            <std_codecvt_utf16_char16_t__4096__0_>::default(),
        ));
        let w: Value<u16> = Rc::new(RefCell::new(0_u16));
        let n: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([-40_i8, -64_i8, -36_i8, 3_i8])));
        let wp: Value<Ptr<u16>> = Rc::new(RefCell::new(Ptr::<u16>::null()));
        let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let np: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
        let mut r: std_codecvt_base_result = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
            let ___to: Ptr<u16> = (w.as_pointer());
            let ___to_end: Ptr<u16> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char16_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_error as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()) }) }));
        assert!(({ ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr::<i8>) }) }));
        assert!(({ (((*w.borrow()) as i32) == 0) }));
        (*n.borrow_mut())[(0) as usize] = 16_i8;
        (*n.borrow_mut())[(1) as usize] = 5_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u16> = (w.as_pointer());
            let ___to_end: Ptr<u16> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char16_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_error as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()) }) }));
        assert!(({ ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr::<i8>) }) }));
        assert!(({ (((*w.borrow()) as i32) == 0) }));
        (*n.borrow_mut())[(0) as usize] = 4_i8;
        (*n.borrow_mut())[(1) as usize] = 83_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u16> = (w.as_pointer());
            let ___to_end: Ptr<u16> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char16_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ (((*w.borrow()) as i32) == 1107) }));
        (*w.borrow_mut()) = 86_u16;
        (*n.borrow_mut())[(0) as usize] = 0_i8;
        (*n.borrow_mut())[(1) as usize] = 86_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u16> = (w.as_pointer());
            let ___to_end: Ptr<u16> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char16_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ (((*w.borrow()) as i32) == 86) }));
    }
    {
        let c: Value<std_codecvt_utf16_char16_t__1114111__std_consume_header_> = Rc::new(
            RefCell::new(<std_codecvt_utf16_char16_t__1114111__std_consume_header_>::default()),
        );
        let w: Value<u16> = Rc::new(RefCell::new(0_u16));
        let n: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            -2_i8, -1_i8, -40_i8, -64_i8, -36_i8, 3_i8,
        ])));
        let wp: Value<Ptr<u16>> = Rc::new(RefCell::new(Ptr::<u16>::null()));
        let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let np: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
        let mut r: std_codecvt_base_result = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((6) as isize);
            let ___to: Ptr<u16> = (w.as_pointer());
            let ___to_end: Ptr<u16> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char16_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_error as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ (((*w.borrow()) as i32) == 0) }));
        (*n.borrow_mut())[(0) as usize] = 16_i8;
        (*n.borrow_mut())[(1) as usize] = 5_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u16> = (w.as_pointer());
            let ___to_end: Ptr<u16> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char16_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ (((*w.borrow()) as i32) == 4101) }));
        (*n.borrow_mut())[(0) as usize] = 4_i8;
        (*n.borrow_mut())[(1) as usize] = 83_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u16> = (w.as_pointer());
            let ___to_end: Ptr<u16> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char16_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ (((*w.borrow()) as i32) == 1107) }));
        (*w.borrow_mut()) = 86_u16;
        (*n.borrow_mut())[(0) as usize] = 0_i8;
        (*n.borrow_mut())[(1) as usize] = 86_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u16> = (w.as_pointer());
            let ___to_end: Ptr<u16> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char16_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ (((*w.borrow()) as i32) == 86) }));
    }
    {
        let c: Value<std_codecvt_utf16_char16_t__1114111__std_little_endian_> = Rc::new(
            RefCell::new(<std_codecvt_utf16_char16_t__1114111__std_little_endian_>::default()),
        );
        let w: Value<u16> = Rc::new(RefCell::new(0_u16));
        let n: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([-64_i8, -40_i8, 3_i8, -36_i8])));
        let wp: Value<Ptr<u16>> = Rc::new(RefCell::new(Ptr::<u16>::null()));
        let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let np: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
        let mut r: std_codecvt_base_result = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
            let ___to: Ptr<u16> = (w.as_pointer());
            let ___to_end: Ptr<u16> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char16_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_error as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()) }) }));
        assert!(({ ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr::<i8>) }) }));
        assert!(({ (((*w.borrow()) as i32) == 0) }));
        (*n.borrow_mut())[(1) as usize] = 16_i8;
        (*n.borrow_mut())[(0) as usize] = 5_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u16> = (w.as_pointer());
            let ___to_end: Ptr<u16> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char16_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ (((*w.borrow()) as i32) == 4101) }));
        (*n.borrow_mut())[(1) as usize] = 4_i8;
        (*n.borrow_mut())[(0) as usize] = 83_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u16> = (w.as_pointer());
            let ___to_end: Ptr<u16> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char16_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ (((*w.borrow()) as i32) == 1107) }));
        (*w.borrow_mut()) = 86_u16;
        (*n.borrow_mut())[(1) as usize] = 0_i8;
        (*n.borrow_mut())[(0) as usize] = 86_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u16> = (w.as_pointer());
            let ___to_end: Ptr<u16> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char16_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ (((*w.borrow()) as i32) == 86) }));
    }
    {
        let c: Value<std_codecvt_utf16_char16_t__4096__std_little_endian_> = Rc::new(RefCell::new(
            <std_codecvt_utf16_char16_t__4096__std_little_endian_>::default(),
        ));
        let w: Value<u16> = Rc::new(RefCell::new(0_u16));
        let n: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([-64_i8, -40_i8, 3_i8, -36_i8])));
        let wp: Value<Ptr<u16>> = Rc::new(RefCell::new(Ptr::<u16>::null()));
        let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let np: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
        let mut r: std_codecvt_base_result = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((4) as isize);
            let ___to: Ptr<u16> = (w.as_pointer());
            let ___to_end: Ptr<u16> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char16_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_error as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()) }) }));
        assert!(({ ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr::<i8>) }) }));
        assert!(({ (((*w.borrow()) as i32) == 0) }));
        (*n.borrow_mut())[(1) as usize] = 16_i8;
        (*n.borrow_mut())[(0) as usize] = 5_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u16> = (w.as_pointer());
            let ___to_end: Ptr<u16> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char16_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_error as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()) }) }));
        assert!(({ ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr::<i8>) }) }));
        assert!(({ (((*w.borrow()) as i32) == 0) }));
        (*n.borrow_mut())[(1) as usize] = 4_i8;
        (*n.borrow_mut())[(0) as usize] = 83_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u16> = (w.as_pointer());
            let ___to_end: Ptr<u16> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char16_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ (((*w.borrow()) as i32) == 1107) }));
        (*w.borrow_mut()) = 86_u16;
        (*n.borrow_mut())[(1) as usize] = 0_i8;
        (*n.borrow_mut())[(0) as usize] = 86_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u16> = (w.as_pointer());
            let ___to_end: Ptr<u16> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char16_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ (((*w.borrow()) as i32) == 86) }));
    }
    {
        let c: Value<std_codecvt_utf16_char16_t__1114111__5_> = Rc::new(RefCell::new(
            <std_codecvt_utf16_char16_t__1114111__5_>::default(),
        ));
        let w: Value<u16> = Rc::new(RefCell::new(0_u16));
        let n: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            -1_i8, -2_i8, -64_i8, -40_i8, 3_i8, -36_i8,
        ])));
        let wp: Value<Ptr<u16>> = Rc::new(RefCell::new(Ptr::<u16>::null()));
        let m: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
        let np: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
        let mut r: std_codecvt_base_result = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((6) as isize);
            let ___to: Ptr<u16> = (w.as_pointer());
            let ___to_end: Ptr<u16> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char16_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_error as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ (((*w.borrow()) as i32) == 0) }));
        (*n.borrow_mut())[(1) as usize] = 16_i8;
        (*n.borrow_mut())[(0) as usize] = 5_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u16> = (w.as_pointer());
            let ___to_end: Ptr<u16> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char16_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ (((*w.borrow()) as i32) == 4101) }));
        (*n.borrow_mut())[(1) as usize] = 4_i8;
        (*n.borrow_mut())[(0) as usize] = 83_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u16> = (w.as_pointer());
            let ___to_end: Ptr<u16> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char16_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ (((*w.borrow()) as i32) == 1107) }));
        (*w.borrow_mut()) = 86_u16;
        (*n.borrow_mut())[(1) as usize] = 0_i8;
        (*n.borrow_mut())[(0) as usize] = 86_i8;
        r = ({
            let ___from: Ptr<i8> = (n.as_pointer() as Ptr<i8>);
            let ___from_end: Ptr<i8> = (n.as_pointer() as Ptr<i8>).offset((2) as isize);
            let ___to: Ptr<u16> = (w.as_pointer());
            let ___to_end: Ptr<u16> = (w.as_pointer()).offset((1) as isize);
            ((*c.borrow()) as std___codecvt_abstract_base_char16_t__char____mbstate_t_).in_(
                m.as_pointer(),
                ___from,
                ___from_end,
                np.as_pointer(),
                ___to,
                ___to_end,
                wp.as_pointer(),
            )
        });
        assert!(((r as i32) == (std_codecvt_base_result_ok as i32)));
        assert!(({ ({ (*wp.borrow()).clone() } == { (w.as_pointer()).offset((1) as isize) }) }));
        assert!(
            ({
                ({ (*np.borrow()).clone() } == { (n.as_pointer() as Ptr<i8>).offset((2) as isize) })
            })
        );
        assert!(({ (((*w.borrow()) as i32) == 86) }));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
