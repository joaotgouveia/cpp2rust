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
}
pub fn test_0(s: Ptr<()>, mut c: (), mut pos: (), mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if (x != unimplemented!()) {
        unimplemented!();
    }
}
pub fn test_1(s: Ptr<()>, mut c: (), mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if (x != unimplemented!()) {
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
            test_0(_s.as_pointer(), ('b' as ()), 0_(), unimplemented!())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), ('b' as ()), 1_(), unimplemented!())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), ('b' as ()), 0_(), unimplemented!())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), ('b' as ()), 1_(), 1_())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), ('b' as ()), 2_(), 1_())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), ('b' as ()), 4_(), 1_())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), ('b' as ()), 5_(), 1_())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), ('b' as ()), 6_(), 1_())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), ('b' as ()), 0_(), unimplemented!())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), ('b' as ()), 1_(), 1_())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), ('b' as ()), 5_(), 1_())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), ('b' as ()), 9_(), 6_())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), ('b' as ()), 10_(), 6_())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), ('b' as ()), 11_(), 6_())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), ('b' as ()), 0_(), unimplemented!())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), ('b' as ()), 1_(), 1_())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), ('b' as ()), 10_(), 6_())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), ('b' as ()), 19_(), 16_())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), ('b' as ()), 20_(), 16_())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_0(_s.as_pointer(), ('b' as ()), 21_(), 16_())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_1(_s.as_pointer(), ('b' as ()), unimplemented!())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_1(_s.as_pointer(), ('b' as ()), 1_())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_1(_s.as_pointer(), ('b' as ()), 6_())
        });
        ({
            let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_1(_s.as_pointer(), ('b' as ()), 16_())
        });
    }
    {
        let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let sv2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "sv1.rfind(\'b\', 0) == SV::npos");;
        const _: () = assert!(true, "sv1.rfind(\'b\', 1) == SV::npos");;
        const _: () = assert!(true, "sv2.rfind(\'b\', 0) == SV::npos");;
        const _: () = assert!(true, "sv2.rfind(\'b\', 1) == 1");;
        const _: () = assert!(true, "sv2.rfind(\'b\', 2) == 1");;
        const _: () = assert!(true, "sv2.rfind(\'b\', 3) == 1");;
        const _: () = assert!(true, "sv2.rfind(\'b\', 4) == 1");;
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
