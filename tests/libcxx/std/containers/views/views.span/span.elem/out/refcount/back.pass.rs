extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static iArr1_0: Value<Box<[i32]>> =
        Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9])));
);
thread_local!(
    pub static iArr2_1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    ])));
);

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
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 1))");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 2))");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 3))");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 4))");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int, 1>(iArr1, 1))");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int, 2>(iArr1, 2))");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int, 3>(iArr1, 3))");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int, 4>(iArr1, 4))");;
    ({
        testRuntimeSpan_2(std_span_int__18446744073709551615_::new_1(
            { (iArr2_1.with(|v| v.as_pointer()) as Ptr<i32>) },
            { 1_u64 },
        ))
    });
    ({
        testRuntimeSpan_2(std_span_int__18446744073709551615_::new_1(
            { (iArr2_1.with(|v| v.as_pointer()) as Ptr<i32>) },
            { 2_u64 },
        ))
    });
    ({
        testRuntimeSpan_2(std_span_int__18446744073709551615_::new_1(
            { (iArr2_1.with(|v| v.as_pointer()) as Ptr<i32>) },
            { 3_u64 },
        ))
    });
    ({
        testRuntimeSpan_2(std_span_int__18446744073709551615_::new_1(
            { (iArr2_1.with(|v| v.as_pointer()) as Ptr<i32>) },
            { 4_u64 },
        ))
    });
    ({
        testRuntimeSpan_3(std_span_int__1_::new_1(
            { (iArr2_1.with(|v| v.as_pointer()) as Ptr<i32>) },
            { 1_u64 },
        ))
    });
    ({
        testRuntimeSpan_4(std_span_int__2_::new_1(
            { (iArr2_1.with(|v| v.as_pointer()) as Ptr<i32>) },
            { 2_u64 },
        ))
    });
    ({
        testRuntimeSpan_5(std_span_int__3_::new_1(
            { (iArr2_1.with(|v| v.as_pointer()) as Ptr<i32>) },
            { 3_u64 },
        ))
    });
    ({
        testRuntimeSpan_6(std_span_int__4_::new_1(
            { (iArr2_1.with(|v| v.as_pointer()) as Ptr<i32>) },
            { 4_u64 },
        ))
    });
    let s: Value<Vec<i8>> = Rc::new(RefCell::new(vec![i8::default()]));
    ({
        testRuntimeSpan_7 ( std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ :: new_1 ( { (( s .as_pointer()  ) ).reinterpret_cast::<Vec<i8  >  >()  } , { 1_u64  } , )   , )
    });
    ({
        testRuntimeSpan_8(
            std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____1_::new_1(
                { (s.as_pointer()).reinterpret_cast::<Vec<i8>>() },
                { 1_u64 },
            ),
        )
    });
    let sp: Value<std_span_int__0_> = Rc::new(RefCell::new(<std_span_int__0_>::default()));
    ({ testEmptySpan_9((*sp.borrow()).clone()) });
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = iArr1_0.with(|_| ());
    let _ = iArr2_1.with(|_| ());
}
