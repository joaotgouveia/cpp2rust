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
pub struct A {}
impl CppMove for A {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_0() {
    let p: Value<std_shared_ptr_int_> = Rc::new(RefCell::new(<std_shared_ptr_int_>::default()));
    ({
        cpp2rust_assert_fail_1(
            (({ ((*p.borrow()) as std___shared_ptr_int____gnu_cxx__S_atomic_).use_count() })
                == 0_i64),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            ({ ((*p.borrow()) as std___shared_ptr_int____gnu_cxx__S_atomic_).get() }).is_null(),
        )
    });
}
pub fn test_2() {
    let p: Value<std_shared_ptr_const_int_> =
        Rc::new(RefCell::new(<std_shared_ptr_const_int_>::default()));
    ({
        cpp2rust_assert_fail_1(
            (({ ((*p.borrow()) as std___shared_ptr_const_int____gnu_cxx__S_atomic_).use_count() })
                == 0_i64),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            ({ ((*p.borrow()) as std___shared_ptr_const_int____gnu_cxx__S_atomic_).get() })
                .is_null(),
        )
    });
}
pub fn test_3() {
    let p: Value<std_shared_ptr_A_> = Rc::new(RefCell::new(<std_shared_ptr_A_>::default()));
    ({
        cpp2rust_assert_fail_1(
            (({ ((*p.borrow()) as std___shared_ptr_A____gnu_cxx__S_atomic_).use_count() })
                == 0_i64),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            ({ ((*p.borrow()) as std___shared_ptr_A____gnu_cxx__S_atomic_).get() }).is_null(),
        )
    });
}
pub fn test_4() {
    let p: Value<std_shared_ptr_const_A_> =
        Rc::new(RefCell::new(<std_shared_ptr_const_A_>::default()));
    ({
        cpp2rust_assert_fail_1(
            (({ ((*p.borrow()) as std___shared_ptr_const_A____gnu_cxx__S_atomic_).use_count() })
                == 0_i64),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            ({ ((*p.borrow()) as std___shared_ptr_const_A____gnu_cxx__S_atomic_).get() }).is_null(),
        )
    });
}
pub fn test_5() {
    let p: Value<std_shared_ptr_int_ptr_> =
        Rc::new(RefCell::new(<std_shared_ptr_int_ptr_>::default()));
    ({
        cpp2rust_assert_fail_1(
            (({ ((*p.borrow()) as std___shared_ptr_int_ptr____gnu_cxx__S_atomic_).use_count() })
                == 0_i64),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            ({ ((*p.borrow()) as std___shared_ptr_int_ptr____gnu_cxx__S_atomic_).get() }).is_null(),
        )
    });
}
pub fn test_6() {
    let p: Value<std_shared_ptr_const_int_ptr_> =
        Rc::new(RefCell::new(<std_shared_ptr_const_int_ptr_>::default()));
    ({
        cpp2rust_assert_fail_1(
            (({
                ((*p.borrow()) as std___shared_ptr_const_int_ptr____gnu_cxx__S_atomic_).use_count()
            }) == 0_i64),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            ({ ((*p.borrow()) as std___shared_ptr_const_int_ptr____gnu_cxx__S_atomic_).get() })
                .is_null(),
        )
    });
}
pub fn test_7() {
    let p: Value<std_shared_ptr_intarrarr_> =
        Rc::new(RefCell::new(<std_shared_ptr_intarrarr_>::default()));
    ({
        cpp2rust_assert_fail_1(
            (({ ((*p.borrow()) as std___shared_ptr_intarrarr____gnu_cxx__S_atomic_).use_count() })
                == 0_i64),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            ({ ((*p.borrow()) as std___shared_ptr_intarrarr____gnu_cxx__S_atomic_).get() })
                .is_null(),
        )
    });
}
pub fn test_8() {
    let p: Value<std_shared_ptr_intarr8arr_> =
        Rc::new(RefCell::new(<std_shared_ptr_intarr8arr_>::default()));
    ({
        cpp2rust_assert_fail_1(
            (({
                ((*p.borrow()) as std___shared_ptr_intarr8arr____gnu_cxx__S_atomic_).use_count()
            }) == 0_i64),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            ({ ((*p.borrow()) as std___shared_ptr_intarr8arr____gnu_cxx__S_atomic_).get() })
                .is_null(),
        )
    });
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
    ({ test_0() });
    ({ test_2() });
    ({ test_3() });
    ({ test_4() });
    ({ test_5() });
    ({ test_6() });
    ({ test_7() });
    ({ test_8() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
