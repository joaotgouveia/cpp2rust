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
        let p: Value<std_shared_ptr_intarr8arr_> =
            Rc::new(RefCell::new(std_shared_ptr_intarr8arr_::new_1({
                Ptr::alloc_array((0..8_u64).map(|_| 0_i32).collect::<Box<[i32]>>())
            })));
        let mut i: i32 = 0;
        'loop_: while (i < 8) {
            let __rhs = i;
            ((*p.borrow_mut())
                as std___shared_ptr_access_intarr8arr____gnu_cxx__S_atomic__true__false_)
                [(i as i64)] = __rhs;
            i.prefix_inc();
        }
        let mut i: i32 = 0;
        'loop_: while (i < 8) {
            ({
                cpp2rust_assert_fail_0(
                    ({
                        ( (*p.borrow()) as std___shared_ptr_access_intarr8arr____gnu_cxx__S_atomic__true__false_ ) [( i as i64 ) ]
                    } == i),
                )
            });
            i.prefix_inc();
        }
    }
    {
        let mut iptr: Ptr<i32> =
            Ptr::alloc_array((0..8_u64).map(|_| 0_i32).collect::<Box<[i32]>>());
        let mut i: i32 = 0;
        'loop_: while (i < 8) {
            elem!(iptr, i).write({ i });
            i.prefix_inc();
        }
        let p: Value<std_shared_ptr_intarr8arr_> =
            Rc::new(RefCell::new(std_shared_ptr_intarr8arr_::new_1({
                (iptr).clone()
            })));
        let mut i: i32 = 0;
        'loop_: while (i < 8) {
            ({
                cpp2rust_assert_fail_0(
                    ({
                        ( (*p.borrow()) as std___shared_ptr_access_intarr8arr____gnu_cxx__S_atomic__true__false_ ) [( i as i64 ) ]
                    } == i),
                )
            });
            i.prefix_inc();
        }
    }
    {
        let p: Value<std_shared_ptr_intarrarr_> =
            Rc::new(RefCell::new(std_shared_ptr_intarrarr_::new_1({
                Ptr::alloc_array((0..8_u64).map(|_| 0_i32).collect::<Box<[i32]>>())
            })));
        let mut i: i32 = 0;
        'loop_: while (i < 8) {
            let __rhs = i;
            ((*p.borrow_mut())
                as std___shared_ptr_access_intarrarr____gnu_cxx__S_atomic__true__false_)
                [(i as i64)] = __rhs;
            i.prefix_inc();
        }
        let mut i: i32 = 0;
        'loop_: while (i < 8) {
            ({
                cpp2rust_assert_fail_0(
                    ({
                        ((*p.borrow())
                            as std___shared_ptr_access_intarrarr____gnu_cxx__S_atomic__true__false_)
                            [(i as i64)]
                    } == i),
                )
            });
            i.prefix_inc();
        }
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
