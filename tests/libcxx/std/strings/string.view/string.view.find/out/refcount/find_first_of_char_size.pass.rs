extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct constexpr_char_traits_char_ {}
impl constexpr_char_traits_char_ {
    pub fn eq(mut c1: (), mut c2: ()) -> () {
        return (c1 == c2);
    }
    pub fn find(s: Ptr<()>, mut n: (), a: Ptr<()>) -> Ptr<()> {
        let s: Value<Ptr<()>> = Rc::new(RefCell::new(s));
        'loop_: while (n != 0) {
            if ({
                let _c1: () = ((*s.borrow()).read());
                let _c2: () = (a.read());
                constexpr_char_traits_char_::eq(_c1, _c2)
            }) {
                return (*s.borrow()).clone();
            }
            (*s.borrow_mut()).prefix_inc();
            n.prefix_dec();
        }
        return Ptr::<()>::null();
    }
}
pub fn test_0(s: Ptr<()>, mut c: (), mut pos: (), mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if (x != npos_1.with(|rc| *rc.borrow())) {
        unimplemented!();
    }
}
pub fn test_2(s: Ptr<()>, mut c: (), mut x: ()) {
    unimplemented!();
    if (x != npos_1.with(|rc| *rc.borrow())) {
        unimplemented!();
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    {
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(
                _s.as_pointer(),
                ('e' as ()),
                0_(),
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(
                _s.as_pointer(),
                ('e' as ()),
                1_(),
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(
                _s.as_pointer(),
                ('e' as ()),
                0_(),
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(
                _s.as_pointer(),
                ('e' as ()),
                1_(),
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(
                _s.as_pointer(),
                ('e' as ()),
                2_(),
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(
                _s.as_pointer(),
                ('e' as ()),
                4_(),
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(
                _s.as_pointer(),
                ('e' as ()),
                5_(),
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(
                _s.as_pointer(),
                ('e' as ()),
                6_(),
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(
                _s.as_pointer(),
                ('e' as ()),
                0_(),
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), ('e' as ()), 1_(), 4_())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), ('e' as ()), 5_(), 6_())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(
                _s.as_pointer(),
                ('e' as ()),
                9_(),
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(
                _s.as_pointer(),
                ('e' as ()),
                10_(),
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(
                _s.as_pointer(),
                ('e' as ()),
                11_(),
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), ('e' as ()), 0_(), 7_())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), ('e' as ()), 1_(), 4_())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(
                _s.as_pointer(),
                ('e' as ()),
                10_(),
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(
                _s.as_pointer(),
                ('e' as ()),
                19_(),
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(
                _s.as_pointer(),
                ('e' as ()),
                20_(),
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(
                _s.as_pointer(),
                ('e' as ()),
                21_(),
                npos_1.with(|rc| *rc.borrow()),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_2(_s.as_pointer(), ('e' as ()), npos_1.with(|rc| *rc.borrow()))
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_2(_s.as_pointer(), ('e' as ()), 4_())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_2(_s.as_pointer(), ('e' as ()), npos_1.with(|rc| *rc.borrow()))
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_2(_s.as_pointer(), ('e' as ()), 2_())
        });
    }
    {
        let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let sv2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "sv1.find_first_of(\'e\', 0) == SV::npos");;
        const _: () = assert!(true, "sv1.find_first_of(\'e\', 1) == SV::npos");;
        const _: () = assert!(true, "sv2.find_first_of(\'q\', 0) == SV::npos");;
        const _: () = assert!(true, "sv2.find_first_of(\'e\', 1) == 4");;
        const _: () = assert!(true, "sv2.find_first_of(\'e\', 5) == SV::npos");;
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
