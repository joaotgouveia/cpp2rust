extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn checkCV_0() {
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
    let carr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([4, 5, 6])));
    let varr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([7, 8, 9])));
    let cvarr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 3, 5])));
    {
        let s1: Value<std_span_int__18446744073709551615_> =
            Rc::new(RefCell::new(std_span_int__18446744073709551615_::new_1({
                arr.as_pointer()
            })));
        let s2: Value<std_span_const_int__18446744073709551615_> = Rc::new(RefCell::new(
            std_span_const_int__18446744073709551615_::new_1({ carr.as_pointer() }),
        ));
        let s3: Value<std_span_volatile_int__18446744073709551615_> = Rc::new(RefCell::new(
            std_span_volatile_int__18446744073709551615_::new_1({ varr.as_pointer() }),
        ));
        let s4: Value<std_span_const_volatile_int__18446744073709551615_> = Rc::new(RefCell::new(
            std_span_const_volatile_int__18446744073709551615_::new_1({ cvarr.as_pointer() }),
        ));
        assert!(
            ({
                (((({ (*s1.borrow()).size() }).wrapping_add(({ (*s2.borrow()).size() })))
                    .wrapping_add(({ (*s3.borrow()).size() })))
                .wrapping_add(({ (*s4.borrow()).size() }))
                    == 12_u64)
            })
        );
    }
    {
        let s1: Value<std_span_int__3_> =
            Rc::new(RefCell::new(std_span_int__3_::new_1({ arr.as_pointer() })));
        let s2: Value<std_span_const_int__3_> =
            Rc::new(RefCell::new(std_span_const_int__3_::new_1({
                carr.as_pointer()
            })));
        let s3: Value<std_span_volatile_int__3_> =
            Rc::new(RefCell::new(std_span_volatile_int__3_::new_1({
                varr.as_pointer()
            })));
        let s4: Value<std_span_const_volatile_int__3_> =
            Rc::new(RefCell::new(std_span_const_volatile_int__3_::new_1({
                cvarr.as_pointer()
            })));
        assert!(
            ({
                ((((({ (*s1.borrow()).size() }) as u64)
                    .wrapping_add((({ (*s2.borrow()).size() }) as u64)) as u64)
                    .wrapping_add((({ (*s3.borrow()).size() }) as u64)) as u64)
                    .wrapping_add((({ (*s4.borrow()).size() }) as u64))
                    == 12_u64)
            })
        );
    }
    {
        let s1: Value<std_span_const_int__18446744073709551615_> = Rc::new(RefCell::new(
            std_span_const_int__18446744073709551615_::new_1({ arr.as_pointer() }),
        ));
        let s2: Value<std_span_volatile_int__18446744073709551615_> = Rc::new(RefCell::new(
            std_span_volatile_int__18446744073709551615_::new_1({ arr.as_pointer() }),
        ));
        let s3: Value<std_span_volatile_int__18446744073709551615_> = Rc::new(RefCell::new(
            std_span_volatile_int__18446744073709551615_::new_1({ arr.as_pointer() }),
        ));
        let s4: Value<std_span_const_volatile_int__18446744073709551615_> = Rc::new(RefCell::new(
            std_span_const_volatile_int__18446744073709551615_::new_1({ arr.as_pointer() }),
        ));
        let s5: Value<std_span_const_volatile_int__18446744073709551615_> = Rc::new(RefCell::new(
            std_span_const_volatile_int__18446744073709551615_::new_1({ carr.as_pointer() }),
        ));
        let s6: Value<std_span_const_volatile_int__18446744073709551615_> = Rc::new(RefCell::new(
            std_span_const_volatile_int__18446744073709551615_::new_1({ varr.as_pointer() }),
        ));
        assert!(
            ({
                (((((({ (*s1.borrow()).size() }).wrapping_add(({ (*s2.borrow()).size() })))
                    .wrapping_add(({ (*s3.borrow()).size() })))
                .wrapping_add(({ (*s4.borrow()).size() })))
                .wrapping_add(({ (*s5.borrow()).size() })))
                .wrapping_add(({ (*s6.borrow()).size() }))
                    == 18_u64)
            })
        );
    }
    {
        let s1: Value<std_span_const_int__3_> =
            Rc::new(RefCell::new(std_span_const_int__3_::new_1({
                arr.as_pointer()
            })));
        let s2: Value<std_span_volatile_int__3_> =
            Rc::new(RefCell::new(std_span_volatile_int__3_::new_1({
                arr.as_pointer()
            })));
        let s3: Value<std_span_volatile_int__3_> =
            Rc::new(RefCell::new(std_span_volatile_int__3_::new_1({
                arr.as_pointer()
            })));
        let s4: Value<std_span_const_volatile_int__3_> =
            Rc::new(RefCell::new(std_span_const_volatile_int__3_::new_1({
                arr.as_pointer()
            })));
        let s5: Value<std_span_const_volatile_int__3_> =
            Rc::new(RefCell::new(std_span_const_volatile_int__3_::new_1({
                carr.as_pointer()
            })));
        let s6: Value<std_span_const_volatile_int__3_> =
            Rc::new(RefCell::new(std_span_const_volatile_int__3_::new_1({
                varr.as_pointer()
            })));
        assert!(
            ({
                ((((((({ (*s1.borrow()).size() }) as u64)
                    .wrapping_add((({ (*s2.borrow()).size() }) as u64)) as u64)
                    .wrapping_add((({ (*s3.borrow()).size() }) as u64)) as u64)
                    .wrapping_add((({ (*s4.borrow()).size() }) as u64)) as u64)
                    .wrapping_add((({ (*s5.borrow()).size() }) as u64)) as u64)
                    .wrapping_add((({ (*s6.borrow()).size() }) as u64))
                    == 18_u64)
            })
        );
    }
}
pub fn testSpan_1() -> bool {
    let val: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0_i32, 0_i32])));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let s1: Value<std_span_int__18446744073709551615_> =
        Rc::new(RefCell::new(std_span_int__18446744073709551615_::new_2({
            val.as_pointer()
        })));
    let s2: Value<std_span_int__2_> =
        Rc::new(RefCell::new(std_span_int__2_::new_1({ val.as_pointer() })));
    let s3: Value<std_span_const_int__18446744073709551615_> = Rc::new(RefCell::new(
        std_span_const_int__18446744073709551615_::new_2({ val.as_pointer() }),
    ));
    let s4: Value<std_span_const_int__2_> = Rc::new(RefCell::new(std_span_const_int__2_::new_1({
        val.as_pointer()
    })));
    assert!(
        ({
            (({ (*s1.borrow()).data() }) == (val.as_pointer() as Ptr<i32>))
                && (({ (*s1.borrow()).size() }) == 2_u64)
        })
    );
    assert!(
        ({
            (({ (*s2.borrow()).data() }) == (val.as_pointer() as Ptr<i32>))
                && (({ (*s2.borrow()).size() }) == 2_u64)
        })
    );
    assert!(
        ({
            (({ (*s3.borrow()).data() }) == (val.as_pointer() as Ptr<i32>))
                && (({ (*s3.borrow()).size() }) == 2_u64)
        })
    );
    assert!(
        ({
            (({ (*s4.borrow()).data() }) == (val.as_pointer() as Ptr<i32>))
                && (({ (*s4.borrow()).size() }) == 2_u64)
        })
    );
    let s5: Value<std_span_const_int__18446744073709551615_> = Rc::new(RefCell::new({
        let __tmp_0: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2])));
        std_span_const_int__18446744073709551615_::new_2({ __tmp_0.as_pointer() })
    }));
    let s6: Value<std_span_const_int__2_> = Rc::new(RefCell::new({
        let __tmp_1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2])));
        std_span_const_int__2_::new_1({ __tmp_1.as_pointer() })
    }));
    assert!(({ (({ (*s5.borrow()).size() }) == 2_u64) }));
    assert!(({ (({ (*s6.borrow()).size() }) == 2_u64) }));
    return true;
}
pub fn testSpan_2() -> bool {
    let val: Value<Box<[f64]>> = Rc::new(RefCell::new(Box::new([0_f64, 0_f64])));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let s1: Value<std_span_double__18446744073709551615_> = Rc::new(RefCell::new(
        std_span_double__18446744073709551615_::new_1({ val.as_pointer() }),
    ));
    let s2: Value<std_span_double__2_> = Rc::new(RefCell::new(std_span_double__2_::new_1({
        val.as_pointer()
    })));
    let s3: Value<std_span_const_double__18446744073709551615_> = Rc::new(RefCell::new(
        std_span_const_double__18446744073709551615_::new_1({ val.as_pointer() }),
    ));
    let s4: Value<std_span_const_double__2_> =
        Rc::new(RefCell::new(std_span_const_double__2_::new_1({
            val.as_pointer()
        })));
    assert!(
        ({
            (({ (*s1.borrow()).data() }) == (val.as_pointer() as Ptr<f64>))
                && (({ (*s1.borrow()).size() }) == 2_u64)
        })
    );
    assert!(
        ({
            (({ (*s2.borrow()).data() }) == (val.as_pointer() as Ptr<f64>))
                && (({ (*s2.borrow()).size() }) == 2_u64)
        })
    );
    assert!(
        ({
            (({ (*s3.borrow()).data() }) == (val.as_pointer() as Ptr<f64>))
                && (({ (*s3.borrow()).size() }) == 2_u64)
        })
    );
    assert!(
        ({
            (({ (*s4.borrow()).data() }) == (val.as_pointer() as Ptr<f64>))
                && (({ (*s4.borrow()).size() }) == 2_u64)
        })
    );
    let s5: Value<std_span_const_int__18446744073709551615_> = Rc::new(RefCell::new({
        let __tmp_2: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2])));
        std_span_const_int__18446744073709551615_::new_2({ __tmp_2.as_pointer() })
    }));
    let s6: Value<std_span_const_int__2_> = Rc::new(RefCell::new({
        let __tmp_3: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2])));
        std_span_const_int__2_::new_1({ __tmp_3.as_pointer() })
    }));
    assert!(({ (({ (*s5.borrow()).size() }) == 2_u64) }));
    assert!(({ (({ (*s6.borrow()).size() }) == 2_u64) }));
    return true;
}
pub fn testSpan_3() -> bool {
    let val: Value<Box<[A]>> = Rc::new(RefCell::new(Box::new([<A>::default(), <A>::default()])));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let s1: Value<std_span_A__18446744073709551615_> =
        Rc::new(RefCell::new(std_span_A__18446744073709551615_::new_1({
            val.as_pointer()
        })));
    let s2: Value<std_span_A__2_> =
        Rc::new(RefCell::new(std_span_A__2_::new_1({ val.as_pointer() })));
    let s3: Value<std_span_const_A__18446744073709551615_> = Rc::new(RefCell::new(
        std_span_const_A__18446744073709551615_::new_1({ val.as_pointer() }),
    ));
    let s4: Value<std_span_const_A__2_> = Rc::new(RefCell::new(std_span_const_A__2_::new_1({
        val.as_pointer()
    })));
    assert!(
        ({
            (({ (*s1.borrow()).data() }) == (val.as_pointer() as Ptr<A>))
                && (({ (*s1.borrow()).size() }) == 2_u64)
        })
    );
    assert!(
        ({
            (({ (*s2.borrow()).data() }) == (val.as_pointer() as Ptr<A>))
                && (({ (*s2.borrow()).size() }) == 2_u64)
        })
    );
    assert!(
        ({
            (({ (*s3.borrow()).data() }) == (val.as_pointer() as Ptr<A>))
                && (({ (*s3.borrow()).size() }) == 2_u64)
        })
    );
    assert!(
        ({
            (({ (*s4.borrow()).data() }) == (val.as_pointer() as Ptr<A>))
                && (({ (*s4.borrow()).size() }) == 2_u64)
        })
    );
    let s5: Value<std_span_const_int__18446744073709551615_> = Rc::new(RefCell::new({
        let __tmp_4: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2])));
        std_span_const_int__18446744073709551615_::new_2({ __tmp_4.as_pointer() })
    }));
    let s6: Value<std_span_const_int__2_> = Rc::new(RefCell::new({
        let __tmp_5: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2])));
        std_span_const_int__2_::new_1({ __tmp_5.as_pointer() })
    }));
    assert!(({ (({ (*s5.borrow()).size() }) == 2_u64) }));
    assert!(({ (({ (*s6.borrow()).size() }) == 2_u64) }));
    return true;
}
pub fn testSpan_4() -> bool {
    let val: Value<Box<[Vec<i8>]>> = Rc::new(RefCell::new(Box::new([
        vec![i8::default()],
        vec![i8::default()],
    ])));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let s1 : Value<std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ > = Rc::new(RefCell::new(std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ :: new_1 ( { val .as_pointer()  } , )  )) ;
    let s2: Value<std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____2_> =
        Rc::new(RefCell::new(
            std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____2_::new_1(
                { val.as_pointer() },
            ),
        ));
    let s3 : Value<std_span_const_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ > = Rc::new(RefCell::new(std_span_const_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ :: new_1 ( { val .as_pointer()  } , )  )) ;
    let s4 : Value<std_span_const_std_basic_string_char__std_char_traits_char___std_allocator_char____2_ > = Rc::new(RefCell::new(std_span_const_std_basic_string_char__std_char_traits_char___std_allocator_char____2_ :: new_1 ( { val .as_pointer()  } , )  )) ;
    assert!(
        ({
            (({ (*s1.borrow()).data() }) == (val.as_pointer() as Ptr<Vec<i8>>))
                && (({ (*s1.borrow()).size() }) == 2_u64)
        })
    );
    assert!(
        ({
            (({ (*s2.borrow()).data() }) == (val.as_pointer() as Ptr<Vec<i8>>))
                && (({ (*s2.borrow()).size() }) == 2_u64)
        })
    );
    assert!(
        ({
            (({ (*s3.borrow()).data() }) == (val.as_pointer() as Ptr<Vec<i8>>))
                && (({ (*s3.borrow()).size() }) == 2_u64)
        })
    );
    assert!(
        ({
            (({ (*s4.borrow()).data() }) == (val.as_pointer() as Ptr<Vec<i8>>))
                && (({ (*s4.borrow()).size() }) == 2_u64)
        })
    );
    let s5: Value<std_span_const_int__18446744073709551615_> = Rc::new(RefCell::new({
        let __tmp_6: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2])));
        std_span_const_int__18446744073709551615_::new_2({ __tmp_6.as_pointer() })
    }));
    let s6: Value<std_span_const_int__2_> = Rc::new(RefCell::new({
        let __tmp_7: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2])));
        std_span_const_int__2_::new_1({ __tmp_7.as_pointer() })
    }));
    assert!(({ (({ (*s5.borrow()).size() }) == 2_u64) }));
    assert!(({ (({ (*s6.borrow()).size() }) == 2_u64) }));
    return true;
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
    ({ testSpan_1() });
    ({ testSpan_2() });
    ({ testSpan_3() });
    ({ testSpan_4() });
    const _: () = assert!(true, "testSpan<int>()");;
    const _: () = assert!(true, "testSpan<double>()");;
    const _: () = assert!(true, "testSpan<A>()");;
    ({ checkCV_0() });
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<std::span<int, 2>, int (&)[3]>"
        );;
    }
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<std::span<float>, int (&)[3]>"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<std::span<float, 3>, int (&)[3]>"
        );;
    }
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<std::span<int>, const int (&)[3]>"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<std::span<int>, volatile int (&)[3]>"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<std::span<int>, const volatile int (&)[3]>"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<std::span<const int>, volatile int (&)[3]>"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<std::span<const int>, const volatile int (&)[3]>"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<std::span<volatile int>, const int (&)[3]>"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<std::span<volatile int>, const volatile int (&)[3]>"
        );;
    }
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<std::span<int, 3>, const int (&)[3]>"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<std::span<int, 3>, volatile int (&)[3]>"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<std::span<int, 3>, const volatile int (&)[3]>"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<std::span<const int, 3>, volatile int (&)[3]>"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<std::span<const int, 3>, const volatile int (&)[3]>"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<std::span<volatile int, 3>, const int (&)[3]>"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<std::span<volatile int, 3>, const volatile int (&)[3]>"
        );;
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
