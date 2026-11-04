extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn checkCV_0() {
    {
        let s1: Value<std_span_int__18446744073709551615_> = Rc::new(RefCell::new(
            <std_span_int__18446744073709551615_>::default(),
        ));
        let s2: Value<std_span_const_int__18446744073709551615_> = Rc::new(RefCell::new(
            <std_span_const_int__18446744073709551615_>::default(),
        ));
        let s3: Value<std_span_volatile_int__18446744073709551615_> = Rc::new(RefCell::new(
            <std_span_volatile_int__18446744073709551615_>::default(),
        ));
        let s4: Value<std_span_const_volatile_int__18446744073709551615_> = Rc::new(RefCell::new(
            <std_span_const_volatile_int__18446744073709551615_>::default(),
        ));
        assert!(
            ({
                (((({ (*s1.borrow()).size() }).wrapping_add(({ (*s2.borrow()).size() })))
                    .wrapping_add(({ (*s3.borrow()).size() })))
                .wrapping_add(({ (*s4.borrow()).size() }))
                    == 0_u64)
            })
        );
    }
    {
        let s1: Value<std_span_int__0_> = Rc::new(RefCell::new(<std_span_int__0_>::default()));
        let s2: Value<std_span_const_int__0_> =
            Rc::new(RefCell::new(<std_span_const_int__0_>::default()));
        let s3: Value<std_span_volatile_int__0_> =
            Rc::new(RefCell::new(<std_span_volatile_int__0_>::default()));
        let s4: Value<std_span_const_volatile_int__0_> =
            Rc::new(RefCell::new(<std_span_const_volatile_int__0_>::default()));
        assert!(
            ({
                ((((({ (*s1.borrow()).size() }) as u64)
                    .wrapping_add((({ (*s2.borrow()).size() }) as u64)) as u64)
                    .wrapping_add((({ (*s3.borrow()).size() }) as u64)) as u64)
                    .wrapping_add((({ (*s4.borrow()).size() }) as u64))
                    == 0_u64)
            })
        );
    }
}
pub fn testConstexprSpan_1() -> bool {
    let s1: Value<std_span_const_int__18446744073709551615_> = Rc::new(RefCell::new(
        <std_span_const_int__18446744073709551615_>::default(),
    ));
    let s2: Value<std_span_const_int__0_> =
        Rc::new(RefCell::new(<std_span_const_int__0_>::default()));
    return (((({ (*s1.borrow()).data() }).is_null()) && (({ (*s1.borrow()).size() }) == 0_u64))
        && (({ (*s2.borrow()).data() }).is_null()))
        && (({ (*s2.borrow()).size() }) == 0_u64);
}
pub fn testConstexprSpan_2() -> bool {
    let s1: Value<std_span_const_long__18446744073709551615_> = Rc::new(RefCell::new(
        <std_span_const_long__18446744073709551615_>::default(),
    ));
    let s2: Value<std_span_const_long__0_> =
        Rc::new(RefCell::new(<std_span_const_long__0_>::default()));
    return (((({ (*s1.borrow()).data() }).is_null()) && (({ (*s1.borrow()).size() }) == 0_u64))
        && (({ (*s2.borrow()).data() }).is_null()))
        && (({ (*s2.borrow()).size() }) == 0_u64);
}
pub fn testConstexprSpan_3() -> bool {
    let s1: Value<std_span_const_double__18446744073709551615_> = Rc::new(RefCell::new(
        <std_span_const_double__18446744073709551615_>::default(),
    ));
    let s2: Value<std_span_const_double__0_> =
        Rc::new(RefCell::new(<std_span_const_double__0_>::default()));
    return (((({ (*s1.borrow()).data() }).is_null()) && (({ (*s1.borrow()).size() }) == 0_u64))
        && (({ (*s2.borrow()).data() }).is_null()))
        && (({ (*s2.borrow()).size() }) == 0_u64);
}
pub fn testConstexprSpan_4() -> bool {
    let s1: Value<std_span_const_A__18446744073709551615_> = Rc::new(RefCell::new(
        <std_span_const_A__18446744073709551615_>::default(),
    ));
    let s2: Value<std_span_const_A__0_> = Rc::new(RefCell::new(<std_span_const_A__0_>::default()));
    return (((({ (*s1.borrow()).data() }).is_null()) && (({ (*s1.borrow()).size() }) == 0_u64))
        && (({ (*s2.borrow()).data() }).is_null()))
        && (({ (*s2.borrow()).size() }) == 0_u64);
}
pub fn testRuntimeSpan_5() {
    const _: () = assert!(true, "");;
    let s1: Value<std_span_const_int__18446744073709551615_> = Rc::new(RefCell::new(
        <std_span_const_int__18446744073709551615_>::default(),
    ));
    let s2: Value<std_span_const_int__0_> =
        Rc::new(RefCell::new(<std_span_const_int__0_>::default()));
    assert!(
        ({ (({ (*s1.borrow()).data() }).is_null()) && (({ (*s1.borrow()).size() }) == 0_u64) })
    );
    assert!(
        ({ (({ (*s2.borrow()).data() }).is_null()) && (({ (*s2.borrow()).size() }) == 0_u64) })
    );
}
pub fn testRuntimeSpan_6() {
    const _: () = assert!(true, "");;
    let s1: Value<std_span_const_long__18446744073709551615_> = Rc::new(RefCell::new(
        <std_span_const_long__18446744073709551615_>::default(),
    ));
    let s2: Value<std_span_const_long__0_> =
        Rc::new(RefCell::new(<std_span_const_long__0_>::default()));
    assert!(
        ({ (({ (*s1.borrow()).data() }).is_null()) && (({ (*s1.borrow()).size() }) == 0_u64) })
    );
    assert!(
        ({ (({ (*s2.borrow()).data() }).is_null()) && (({ (*s2.borrow()).size() }) == 0_u64) })
    );
}
pub fn testRuntimeSpan_7() {
    const _: () = assert!(true, "");;
    let s1: Value<std_span_const_double__18446744073709551615_> = Rc::new(RefCell::new(
        <std_span_const_double__18446744073709551615_>::default(),
    ));
    let s2: Value<std_span_const_double__0_> =
        Rc::new(RefCell::new(<std_span_const_double__0_>::default()));
    assert!(
        ({ (({ (*s1.borrow()).data() }).is_null()) && (({ (*s1.borrow()).size() }) == 0_u64) })
    );
    assert!(
        ({ (({ (*s2.borrow()).data() }).is_null()) && (({ (*s2.borrow()).size() }) == 0_u64) })
    );
}
pub fn testRuntimeSpan_8() {
    const _: () = assert!(true, "");;
    let s1 : Value<std_span_const_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ > = Rc::new(RefCell::new(<std_span_const_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_  >::default() )) ;
    let s2 : Value<std_span_const_std_basic_string_char__std_char_traits_char___std_allocator_char____0_ > = Rc::new(RefCell::new(<std_span_const_std_basic_string_char__std_char_traits_char___std_allocator_char____0_  >::default() )) ;
    assert!(
        ({ (({ (*s1.borrow()).data() }).is_null()) && (({ (*s1.borrow()).size() }) == 0_u64) })
    );
    assert!(
        ({ (({ (*s2.borrow()).data() }).is_null()) && (({ (*s2.borrow()).size() }) == 0_u64) })
    );
}
pub fn testRuntimeSpan_9() {
    const _: () = assert!(true, "");;
    let s1: Value<std_span_const_A__18446744073709551615_> = Rc::new(RefCell::new(
        <std_span_const_A__18446744073709551615_>::default(),
    ));
    let s2: Value<std_span_const_A__0_> = Rc::new(RefCell::new(<std_span_const_A__0_>::default()));
    assert!(
        ({ (({ (*s1.borrow()).data() }).is_null()) && (({ (*s1.borrow()).size() }) == 0_u64) })
    );
    assert!(
        ({ (({ (*s2.borrow()).data() }).is_null()) && (({ (*s2.borrow()).size() }) == 0_u64) })
    );
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A {}
impl CppMove for A {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
    const _: () = assert!(true, "testConstexprSpan<int>()");;
    const _: () = assert!(true, "testConstexprSpan<long>()");;
    const _: () = assert!(true, "testConstexprSpan<double>()");;
    const _: () = assert!(true, "testConstexprSpan<A>()");;
    ({ testRuntimeSpan_5() });
    ({ testRuntimeSpan_6() });
    ({ testRuntimeSpan_7() });
    ({ testRuntimeSpan_8() });
    ({ testRuntimeSpan_9() });
    ({ checkCV_0() });
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<std::span<int, std::dynamic_extent>>"
    );;
    const _: () = assert!(true, "std::is_default_constructible_v<std::span<int, 0>>");;
    const _: () = assert!(true, "!std::is_default_constructible_v<std::span<int, 2>>");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
