extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_val_ctor_0() {
    {
        let v: Value<std_bitset_0_> = Rc::new(RefCell::new(std_bitset_0_::new_1({
            12297829382473034410_u64
        })));
        assert!(({ (({ (*v.borrow()).size() }) == (0_u64 as usize)) }));
        let mut M: usize = ({
            let __tmp_1: Value<u64> = Rc::new(RefCell::new(64_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_0,) => {
                    if CppLess::less(&__tmp_1.as_pointer(), &(__tmp_0).clone()) {
                        __tmp_1.as_pointer()
                    } else {
                        (__tmp_0).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(({ (({ (*v.borrow())[i] } as i32) == (((i & 1_usize) != 0_usize) as i32)) }));
            i.prefix_inc();
        }
        let mut i: usize = M;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(({ (({ (*v.borrow())[i] } as i32) == (false as i32)) }));
            i.prefix_inc();
        }
    }
    {
        let v: Value<std_bitset_0_> = Rc::new(RefCell::new(std_bitset_0_::new_1({
            12297829382473034410_u64
        })));
        const _: () = assert!(true, "v.size() == N");;
    }
}
pub fn test_val_ctor_1() {
    {
        let v: Value<std_bitset_1_> = Rc::new(RefCell::new(std_bitset_1_::new_1({
            12297829382473034410_u64
        })));
        assert!(({ (({ (*v.borrow()).size() }) == (1_u64 as usize)) }));
        let mut M: usize = ({
            let __tmp_3: Value<u64> = Rc::new(RefCell::new(64_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_2,) => {
                    if CppLess::less(&__tmp_3.as_pointer(), &(__tmp_2).clone()) {
                        __tmp_3.as_pointer()
                    } else {
                        (__tmp_2).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(({ (({ (*v.borrow())[i] } as i32) == (((i & 1_usize) != 0_usize) as i32)) }));
            i.prefix_inc();
        }
        let mut i: usize = M;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(({ (({ (*v.borrow())[i] } as i32) == (false as i32)) }));
            i.prefix_inc();
        }
    }
    {
        let v: Value<std_bitset_1_> = Rc::new(RefCell::new(std_bitset_1_::new_1({
            12297829382473034410_u64
        })));
        const _: () = assert!(true, "v.size() == N");;
    }
}
pub fn test_val_ctor_2() {
    {
        let v: Value<std_bitset_31_> = Rc::new(RefCell::new(std_bitset_31_::new_1({
            12297829382473034410_u64
        })));
        assert!(({ (({ (*v.borrow()).size() }) == (31_u64 as usize)) }));
        let mut M: usize = ({
            let __tmp_5: Value<u64> = Rc::new(RefCell::new(64_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_4,) => {
                    if CppLess::less(&__tmp_5.as_pointer(), &(__tmp_4).clone()) {
                        __tmp_5.as_pointer()
                    } else {
                        (__tmp_4).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(({ (({ (*v.borrow())[i] } as i32) == (((i & 1_usize) != 0_usize) as i32)) }));
            i.prefix_inc();
        }
        let mut i: usize = M;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(({ (({ (*v.borrow())[i] } as i32) == (false as i32)) }));
            i.prefix_inc();
        }
    }
    {
        let v: Value<std_bitset_31_> = Rc::new(RefCell::new(std_bitset_31_::new_1({
            12297829382473034410_u64
        })));
        const _: () = assert!(true, "v.size() == N");;
    }
}
pub fn test_val_ctor_3() {
    {
        let v: Value<std_bitset_32_> = Rc::new(RefCell::new(std_bitset_32_::new_1({
            12297829382473034410_u64
        })));
        assert!(({ (({ (*v.borrow()).size() }) == (32_u64 as usize)) }));
        let mut M: usize = ({
            let __tmp_7: Value<u64> = Rc::new(RefCell::new(64_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_6,) => {
                    if CppLess::less(&__tmp_7.as_pointer(), &(__tmp_6).clone()) {
                        __tmp_7.as_pointer()
                    } else {
                        (__tmp_6).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(({ (({ (*v.borrow())[i] } as i32) == (((i & 1_usize) != 0_usize) as i32)) }));
            i.prefix_inc();
        }
        let mut i: usize = M;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(({ (({ (*v.borrow())[i] } as i32) == (false as i32)) }));
            i.prefix_inc();
        }
    }
    {
        let v: Value<std_bitset_32_> = Rc::new(RefCell::new(std_bitset_32_::new_1({
            12297829382473034410_u64
        })));
        const _: () = assert!(true, "v.size() == N");;
    }
}
pub fn test_val_ctor_4() {
    {
        let v: Value<std_bitset_33_> = Rc::new(RefCell::new(std_bitset_33_::new_1({
            12297829382473034410_u64
        })));
        assert!(({ (({ (*v.borrow()).size() }) == (33_u64 as usize)) }));
        let mut M: usize = ({
            let __tmp_9: Value<u64> = Rc::new(RefCell::new(64_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_8,) => {
                    if CppLess::less(&__tmp_9.as_pointer(), &(__tmp_8).clone()) {
                        __tmp_9.as_pointer()
                    } else {
                        (__tmp_8).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(({ (({ (*v.borrow())[i] } as i32) == (((i & 1_usize) != 0_usize) as i32)) }));
            i.prefix_inc();
        }
        let mut i: usize = M;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(({ (({ (*v.borrow())[i] } as i32) == (false as i32)) }));
            i.prefix_inc();
        }
    }
    {
        let v: Value<std_bitset_33_> = Rc::new(RefCell::new(std_bitset_33_::new_1({
            12297829382473034410_u64
        })));
        const _: () = assert!(true, "v.size() == N");;
    }
}
pub fn test_val_ctor_5() {
    {
        let v: Value<std_bitset_63_> = Rc::new(RefCell::new(std_bitset_63_::new_1({
            12297829382473034410_u64
        })));
        assert!(({ (({ (*v.borrow()).size() }) == (63_u64 as usize)) }));
        let mut M: usize = ({
            let __tmp_11: Value<u64> = Rc::new(RefCell::new(64_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_10,) => {
                    if CppLess::less(&__tmp_11.as_pointer(), &(__tmp_10).clone()) {
                        __tmp_11.as_pointer()
                    } else {
                        (__tmp_10).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(({ (({ (*v.borrow())[i] } as i32) == (((i & 1_usize) != 0_usize) as i32)) }));
            i.prefix_inc();
        }
        let mut i: usize = M;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(({ (({ (*v.borrow())[i] } as i32) == (false as i32)) }));
            i.prefix_inc();
        }
    }
    {
        let v: Value<std_bitset_63_> = Rc::new(RefCell::new(std_bitset_63_::new_1({
            12297829382473034410_u64
        })));
        const _: () = assert!(true, "v.size() == N");;
    }
}
pub fn test_val_ctor_6() {
    {
        let v: Value<std_bitset_64_> = Rc::new(RefCell::new(std_bitset_64_::new_1({
            12297829382473034410_u64
        })));
        assert!(({ (({ (*v.borrow()).size() }) == (64_u64 as usize)) }));
        let mut M: usize = ({
            let __tmp_13: Value<u64> = Rc::new(RefCell::new(64_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_12,) => {
                    if CppLess::less(&__tmp_13.as_pointer(), &(__tmp_12).clone()) {
                        __tmp_13.as_pointer()
                    } else {
                        (__tmp_12).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(({ (({ (*v.borrow())[i] } as i32) == (((i & 1_usize) != 0_usize) as i32)) }));
            i.prefix_inc();
        }
        let mut i: usize = M;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(({ (({ (*v.borrow())[i] } as i32) == (false as i32)) }));
            i.prefix_inc();
        }
    }
    {
        let v: Value<std_bitset_64_> = Rc::new(RefCell::new(std_bitset_64_::new_1({
            12297829382473034410_u64
        })));
        const _: () = assert!(true, "v.size() == N");;
    }
}
pub fn test_val_ctor_7() {
    {
        let v: Value<std_bitset_65_> = Rc::new(RefCell::new(std_bitset_65_::new_1({
            12297829382473034410_u64
        })));
        assert!(({ (({ (*v.borrow()).size() }) == (65_u64 as usize)) }));
        let mut M: usize = ({
            let __tmp_15: Value<u64> = Rc::new(RefCell::new(64_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_14,) => {
                    if CppLess::less(&__tmp_15.as_pointer(), &(__tmp_14).clone()) {
                        __tmp_15.as_pointer()
                    } else {
                        (__tmp_14).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(({ (({ (*v.borrow())[i] } as i32) == (((i & 1_usize) != 0_usize) as i32)) }));
            i.prefix_inc();
        }
        let mut i: usize = M;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(({ (({ (*v.borrow())[i] } as i32) == (false as i32)) }));
            i.prefix_inc();
        }
    }
    {
        let v: Value<std_bitset_65_> = Rc::new(RefCell::new(std_bitset_65_::new_1({
            12297829382473034410_u64
        })));
        const _: () = assert!(true, "v.size() == N");;
    }
}
pub fn test_val_ctor_8() {
    {
        let v: Value<std_bitset_1000_> = Rc::new(RefCell::new(std_bitset_1000_::new_1({
            12297829382473034410_u64
        })));
        assert!(({ (({ (*v.borrow()).size() }) == (1000_u64 as usize)) }));
        let mut M: usize = ({
            let __tmp_17: Value<u64> = Rc::new(RefCell::new(64_u64));
            (match (Rc::new(RefCell::new((({ (*v.borrow()).size() }) as u64))).as_pointer(),) {
                (__tmp_16,) => {
                    if CppLess::less(&__tmp_17.as_pointer(), &(__tmp_16).clone()) {
                        __tmp_17.as_pointer()
                    } else {
                        (__tmp_16).clone()
                    }
                }
            }
            .read())
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while (i < M) {
            assert!(({ (({ (*v.borrow())[i] } as i32) == (((i & 1_usize) != 0_usize) as i32)) }));
            i.prefix_inc();
        }
        let mut i: usize = M;
        'loop_: while (i < ({ (*v.borrow()).size() })) {
            assert!(({ (({ (*v.borrow())[i] } as i32) == (false as i32)) }));
            i.prefix_inc();
        }
    }
    {
        let v: Value<std_bitset_1000_> = Rc::new(RefCell::new(std_bitset_1000_::new_1({
            12297829382473034410_u64
        })));
        const _: () = assert!(true, "v.size() == N");;
    }
}
pub fn test_9() -> bool {
    ({ test_val_ctor_0() });
    ({ test_val_ctor_1() });
    ({ test_val_ctor_2() });
    ({ test_val_ctor_3() });
    ({ test_val_ctor_4() });
    ({ test_val_ctor_5() });
    ({ test_val_ctor_6() });
    ({ test_val_ctor_7() });
    ({ test_val_ctor_8() });
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
