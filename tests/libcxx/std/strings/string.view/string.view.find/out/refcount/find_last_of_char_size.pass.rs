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
    pub fn eq(mut c1: i8, mut c2: i8) -> bool {
        return ((c1 as i32) == (c2 as i32));
    }
}
pub fn test_0(s: Ptr<()>, mut c: (), mut pos: (), mut x: ()) {
    const _: () = assert!(true, "");;
    assert!(({ unimplemented!() } == { x }));
    if (x != unimplemented!()) {
        assert!((x <= pos) && ({ x } < { unimplemented!() }));
    }
}
pub fn test_1(s: Ptr<()>, mut c: (), mut x: ()) {
    const _: () = assert!(true, "");;
    assert!(({ unimplemented!() } == { x }));
    if (x != unimplemented!()) {
        assert!(({ x } < { unimplemented!() }));
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
    {
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), (('m' as i8) as ()), 0_(), unimplemented!())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), (('m' as i8) as ()), 1_(), unimplemented!())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), (('m' as i8) as ()), 0_(), unimplemented!())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), (('m' as i8) as ()), 1_(), unimplemented!())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), (('m' as i8) as ()), 2_(), 2_())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), (('m' as i8) as ()), 4_(), unimplemented!())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), (('m' as i8) as ()), 5_(), unimplemented!())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), (('m' as i8) as ()), 6_(), unimplemented!())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), (('m' as i8) as ()), 0_(), unimplemented!())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), (('m' as i8) as ()), 1_(), unimplemented!())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), (('m' as i8) as ()), 5_(), unimplemented!())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), (('m' as i8) as ()), 9_(), unimplemented!())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(
                _s.as_pointer(),
                (('m' as i8) as ()),
                10_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(
                _s.as_pointer(),
                (('m' as i8) as ()),
                11_(),
                unimplemented!(),
            )
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), (('m' as i8) as ()), 0_(), unimplemented!())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), (('m' as i8) as ()), 1_(), unimplemented!())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), (('m' as i8) as ()), 10_(), 2_())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), (('m' as i8) as ()), 19_(), 12_())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), (('m' as i8) as ()), 20_(), 17_())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), (('m' as i8) as ()), 21_(), 18_())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_1(_s.as_pointer(), (('m' as i8) as ()), unimplemented!())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_1(_s.as_pointer(), (('m' as i8) as ()), unimplemented!())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_1(_s.as_pointer(), (('m' as i8) as ()), 3_())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_1(_s.as_pointer(), (('m' as i8) as ()), 15_())
        });
    }
    {
        let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let sv2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "sv1.find_last_of(\'i\', 0) == SV::npos");;
        const _: () = assert!(true, "sv1.find_last_of(\'i\', 1) == SV::npos");;
        const _: () = assert!(true, "sv2.find_last_of(\'a\', 0) == 0");;
        const _: () = assert!(true, "sv2.find_last_of(\'a\', 1) == 0");;
        const _: () = assert!(true, "sv2.find_last_of(\'e\', 5) == 4");;
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
