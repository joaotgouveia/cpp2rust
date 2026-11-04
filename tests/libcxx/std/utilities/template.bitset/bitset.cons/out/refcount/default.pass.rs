extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_default_ctor_0() {
    {
        let v1: Value<std_bitset_0_> = Rc::new(RefCell::new(<std_bitset_0_>::default()));
        assert!(({ (({ (*v1.borrow()).size() }) == (0_u64 as usize)) }));
        let mut i: usize = 0_usize;
        'loop_: while (i < ({ (*v1.borrow()).size() })) {
            assert!(({ (({ (*v1.borrow())[i] } as i32) == (false as i32)) }));
            i.prefix_inc();
        }
    }
    {
        let v1: Value<std_bitset_0_> = Rc::new(RefCell::new(<std_bitset_0_>::default()));
        const _: () = assert!(true, "v1.size() == N");;
    }
}
pub fn test_default_ctor_1() {
    {
        let v1: Value<std_bitset_1_> = Rc::new(RefCell::new(<std_bitset_1_>::default()));
        assert!(({ (({ (*v1.borrow()).size() }) == (1_u64 as usize)) }));
        let mut i: usize = 0_usize;
        'loop_: while (i < ({ (*v1.borrow()).size() })) {
            assert!(({ (({ (*v1.borrow())[i] } as i32) == (false as i32)) }));
            i.prefix_inc();
        }
    }
    {
        let v1: Value<std_bitset_1_> = Rc::new(RefCell::new(<std_bitset_1_>::default()));
        const _: () = assert!(true, "v1.size() == N");;
    }
}
pub fn test_default_ctor_2() {
    {
        let v1: Value<std_bitset_31_> = Rc::new(RefCell::new(<std_bitset_31_>::default()));
        assert!(({ (({ (*v1.borrow()).size() }) == (31_u64 as usize)) }));
        let mut i: usize = 0_usize;
        'loop_: while (i < ({ (*v1.borrow()).size() })) {
            assert!(({ (({ (*v1.borrow())[i] } as i32) == (false as i32)) }));
            i.prefix_inc();
        }
    }
    {
        let v1: Value<std_bitset_31_> = Rc::new(RefCell::new(<std_bitset_31_>::default()));
        const _: () = assert!(true, "v1.size() == N");;
    }
}
pub fn test_default_ctor_3() {
    {
        let v1: Value<std_bitset_32_> = Rc::new(RefCell::new(<std_bitset_32_>::default()));
        assert!(({ (({ (*v1.borrow()).size() }) == (32_u64 as usize)) }));
        let mut i: usize = 0_usize;
        'loop_: while (i < ({ (*v1.borrow()).size() })) {
            assert!(({ (({ (*v1.borrow())[i] } as i32) == (false as i32)) }));
            i.prefix_inc();
        }
    }
    {
        let v1: Value<std_bitset_32_> = Rc::new(RefCell::new(<std_bitset_32_>::default()));
        const _: () = assert!(true, "v1.size() == N");;
    }
}
pub fn test_default_ctor_4() {
    {
        let v1: Value<std_bitset_33_> = Rc::new(RefCell::new(<std_bitset_33_>::default()));
        assert!(({ (({ (*v1.borrow()).size() }) == (33_u64 as usize)) }));
        let mut i: usize = 0_usize;
        'loop_: while (i < ({ (*v1.borrow()).size() })) {
            assert!(({ (({ (*v1.borrow())[i] } as i32) == (false as i32)) }));
            i.prefix_inc();
        }
    }
    {
        let v1: Value<std_bitset_33_> = Rc::new(RefCell::new(<std_bitset_33_>::default()));
        const _: () = assert!(true, "v1.size() == N");;
    }
}
pub fn test_default_ctor_5() {
    {
        let v1: Value<std_bitset_63_> = Rc::new(RefCell::new(<std_bitset_63_>::default()));
        assert!(({ (({ (*v1.borrow()).size() }) == (63_u64 as usize)) }));
        let mut i: usize = 0_usize;
        'loop_: while (i < ({ (*v1.borrow()).size() })) {
            assert!(({ (({ (*v1.borrow())[i] } as i32) == (false as i32)) }));
            i.prefix_inc();
        }
    }
    {
        let v1: Value<std_bitset_63_> = Rc::new(RefCell::new(<std_bitset_63_>::default()));
        const _: () = assert!(true, "v1.size() == N");;
    }
}
pub fn test_default_ctor_6() {
    {
        let v1: Value<std_bitset_64_> = Rc::new(RefCell::new(<std_bitset_64_>::default()));
        assert!(({ (({ (*v1.borrow()).size() }) == (64_u64 as usize)) }));
        let mut i: usize = 0_usize;
        'loop_: while (i < ({ (*v1.borrow()).size() })) {
            assert!(({ (({ (*v1.borrow())[i] } as i32) == (false as i32)) }));
            i.prefix_inc();
        }
    }
    {
        let v1: Value<std_bitset_64_> = Rc::new(RefCell::new(<std_bitset_64_>::default()));
        const _: () = assert!(true, "v1.size() == N");;
    }
}
pub fn test_default_ctor_7() {
    {
        let v1: Value<std_bitset_65_> = Rc::new(RefCell::new(<std_bitset_65_>::default()));
        assert!(({ (({ (*v1.borrow()).size() }) == (65_u64 as usize)) }));
        let mut i: usize = 0_usize;
        'loop_: while (i < ({ (*v1.borrow()).size() })) {
            assert!(({ (({ (*v1.borrow())[i] } as i32) == (false as i32)) }));
            i.prefix_inc();
        }
    }
    {
        let v1: Value<std_bitset_65_> = Rc::new(RefCell::new(<std_bitset_65_>::default()));
        const _: () = assert!(true, "v1.size() == N");;
    }
}
pub fn test_default_ctor_8() {
    {
        let v1: Value<std_bitset_1000_> = Rc::new(RefCell::new(<std_bitset_1000_>::default()));
        assert!(({ (({ (*v1.borrow()).size() }) == (1000_u64 as usize)) }));
        let mut i: usize = 0_usize;
        'loop_: while (i < ({ (*v1.borrow()).size() })) {
            assert!(({ (({ (*v1.borrow())[i] } as i32) == (false as i32)) }));
            i.prefix_inc();
        }
    }
    {
        let v1: Value<std_bitset_1000_> = Rc::new(RefCell::new(<std_bitset_1000_>::default()));
        const _: () = assert!(true, "v1.size() == N");;
    }
}
pub fn test_9() -> bool {
    ({ test_default_ctor_0() });
    ({ test_default_ctor_1() });
    ({ test_default_ctor_2() });
    ({ test_default_ctor_3() });
    ({ test_default_ctor_4() });
    ({ test_default_ctor_5() });
    ({ test_default_ctor_6() });
    ({ test_default_ctor_7() });
    ({ test_default_ctor_8() });
    return true;
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
    ({ test_9() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
