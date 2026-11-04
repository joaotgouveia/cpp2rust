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
        let v: Value<std_valarray_int_> = Rc::new(RefCell::new(std_valarray_int_::new_1({
            Vec::from([1, 2, 3, 4, 5])
        })));
        const _: () = assert!(true, "");;
    }
    {
        let a: Value<Box<[i64]>> =
            Rc::new(RefCell::new(Box::new([1_i64, 2_i64, 3_i64, 4_i64, 5_i64])));
        let v: Value<std_valarray_long_> = Rc::new(RefCell::new(std_valarray_long_::new_1(
            { (a.as_pointer() as Ptr<i64>) },
            { 5_usize },
        )));
        const _: () = assert!(true, "");;
    }
    {
        let a: Value<Box<[i64]>> =
            Rc::new(RefCell::new(Box::new([1_i64, 2_i64, 3_i64, 4_i64, 5_i64])));
        let v: Value<std_valarray_long_ptr_> = Rc::new(RefCell::new({
            let __tmp_0: Value<Ptr<i64>> =
                Rc::new(RefCell::new(((a.as_pointer() as Ptr<i64>).offset(0))));
            std_valarray_long_ptr_::new_1({ __tmp_0.as_pointer() }, { 5_usize })
        }));
        const _: () = assert!(true, "");;
    }
    {
        let v: Value<std_valarray_long_> = Rc::new(RefCell::new(std_valarray_long_::new_2({
            Vec::from([1_i64, 2_i64, 3_i64, 4_i64, 5_i64])
        })));
        let v2: Value<std_valarray_long_> = Rc::new(RefCell::new({
            let __tmp_1: Value<std_slice_array_long_> = Rc::new(RefCell::new(
                elem!(
                    v.as_pointer(),
                    std_slice::new_1({ 2_usize }, { 3_usize }, { 1_usize },)
                )
                .read(),
            ));
            std_valarray_long_::new_3({ __tmp_1.as_pointer() })
        }));
        const _: () = assert!(true, "std::is_same_v<decltype(v2), std::valarray<long>>");;
    }
    {
        let v: Value<std_valarray_long_> = Rc::new(RefCell::new(std_valarray_long_::new_2({
            Vec::from([1_i64, 2_i64, 3_i64, 4_i64, 5_i64])
        })));
        let v2: Value<std_valarray_long_> = Rc::new(RefCell::new({
            let __tmp_2: Value<std_gslice_array_long_> = Rc::new(RefCell::new(
                elem!(v.as_pointer(), {
                    let __tmp_3: Value<std_valarray_unsigned_long_> =
                        Rc::new(RefCell::new(std_valarray_unsigned_long_::new_1({
                            Vec::from([5_u64])
                        })));
                    let __tmp_4: Value<std_valarray_unsigned_long_> =
                        Rc::new(RefCell::new(std_valarray_unsigned_long_::new_1({
                            Vec::from([1_u64])
                        })));
                    std_gslice::new_1({ 0_usize }, { __tmp_3.as_pointer() }, {
                        __tmp_4.as_pointer()
                    })
                })
                .read(),
            ));
            std_valarray_long_::new_4({ __tmp_2.as_pointer() })
        }));
        const _: () = assert!(true, "std::is_same_v<decltype(v2), std::valarray<long>>");;
    }
    {
        let v: Value<std_valarray_long_> = Rc::new(RefCell::new(std_valarray_long_::new_2({
            Vec::from([1_i64, 2_i64, 3_i64, 4_i64, 5_i64])
        })));
        let m: Value<std_valarray_bool_> = Rc::new(RefCell::new(std_valarray_bool_::new_1({
            Vec::from([true, false, true, false, true])
        })));
        let v2: Value<std_valarray_long_> = Rc::new(RefCell::new({
            let __tmp_5: Value<std_mask_array_long_> =
                Rc::new(RefCell::new(((*v.borrow())[(*m.borrow())]).clone()));
            std_valarray_long_::new_5({ __tmp_5.as_pointer() })
        }));
        const _: () = assert!(true, "std::is_same_v<decltype(v2), std::valarray<long>>");;
    }
    {
        let v: Value<std_valarray_long_> = Rc::new(RefCell::new(std_valarray_long_::new_2({
            Vec::from([1_i64, 2_i64, 3_i64, 4_i64, 5_i64])
        })));
        let i: Value<std_valarray_unsigned_long_> =
            Rc::new(RefCell::new(std_valarray_unsigned_long_::new_1({
                Vec::from([1_u64, 2_u64, 3_u64])
            })));
        let v2: Value<std_valarray_long_> = Rc::new(RefCell::new({
            let __tmp_6: Value<std_indirect_array_long_> =
                Rc::new(RefCell::new(((*v.borrow())[(*i.borrow())]).clone()));
            std_valarray_long_::new_6({ __tmp_6.as_pointer() })
        }));
        const _: () = assert!(true, "std::is_same_v<decltype(v2), std::valarray<long>>");;
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
