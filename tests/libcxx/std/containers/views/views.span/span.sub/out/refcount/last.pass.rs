extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_int__int_ {}
impl CppMove for test_macros_detail_is_same_int__int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_basic_string_char__std_char_traits_char___std_allocator_char____std_basic_string_char__std_char_traits_char___std_allocator_char___
{}
impl CppMove for test_macros_detail_is_same_std_basic_string_char__std_char_traits_char___std_allocator_char____std_basic_string_char__std_char_traits_char___std_allocator_char___ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
thread_local!(
    pub static carr1_2: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4])));
);
thread_local!(
    pub static arr_3: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([5, 6, 7])));
);
thread_local!(
    pub static sarr_4: Value<Box<[Vec<i8>]>> = Rc::new(RefCell::new(Box::new([
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"ABC").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"DEF").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"GHI").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"JKL").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"MNO").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
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
    {
        const _: () = assert!(true, "testConstexprSpan<Sp, 0>(Sp{{}})");;
        const _: () = assert!(true, "testConstexprSpan<Sp, 0>(Sp{{carr1}})");;
        const _: () = assert!(true, "testConstexprSpan<Sp, 1>(Sp{{carr1}})");;
        const _: () = assert!(true, "testConstexprSpan<Sp, 2>(Sp{{carr1}})");;
        const _: () = assert!(true, "testConstexprSpan<Sp, 3>(Sp{{carr1}})");;
        const _: () = assert!(true, "testConstexprSpan<Sp, 4>(Sp{{carr1}})");;
    }
    {
        const _: () = assert!(true, "testConstexprSpan<Sp, 0>(Sp{{carr1}})");;
        const _: () = assert!(true, "testConstexprSpan<Sp, 1>(Sp{{carr1}})");;
        const _: () = assert!(true, "testConstexprSpan<Sp, 2>(Sp{{carr1}})");;
        const _: () = assert!(true, "testConstexprSpan<Sp, 3>(Sp{{carr1}})");;
        const _: () = assert!(true, "testConstexprSpan<Sp, 4>(Sp{{carr1}})");;
    }
    {
        ({ testRuntimeSpan_5(<std_span_int__18446744073709551615_>::default()) });
        ({
            testRuntimeSpan_5(std_span_int__18446744073709551615_::new_1({
                arr_3.with(|v| v.as_pointer())
            }))
        });
        ({
            testRuntimeSpan_6(std_span_int__18446744073709551615_::new_1({
                arr_3.with(|v| v.as_pointer())
            }))
        });
        ({
            testRuntimeSpan_7(std_span_int__18446744073709551615_::new_1({
                arr_3.with(|v| v.as_pointer())
            }))
        });
        ({
            testRuntimeSpan_8(std_span_int__18446744073709551615_::new_1({
                arr_3.with(|v| v.as_pointer())
            }))
        });
    }
    {
        ({ testRuntimeSpan_9(std_span_int__3_::new_1({ arr_3.with(|v| v.as_pointer()) })) });
        ({ testRuntimeSpan_10(std_span_int__3_::new_1({ arr_3.with(|v| v.as_pointer()) })) });
        ({ testRuntimeSpan_11(std_span_int__3_::new_1({ arr_3.with(|v| v.as_pointer()) })) });
        ({ testRuntimeSpan_12(std_span_int__3_::new_1({ arr_3.with(|v| v.as_pointer()) })) });
    }
    {
        ({
            testConstexprSpan_13 ( <std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_  >::default()  , )
        });
        ({
            testRuntimeSpan_14 ( std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ :: new_1 ( { (sarr_4.with(|v| v.as_pointer())  as Ptr< Vec<i8>  > ).reinterpret_cast::<Vec<i8  >  >()  } , )   , )
        });
        ({
            testRuntimeSpan_15 ( std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ :: new_1 ( { (sarr_4.with(|v| v.as_pointer())  as Ptr< Vec<i8>  > ).reinterpret_cast::<Vec<i8  >  >()  } , )   , )
        });
        ({
            testRuntimeSpan_16 ( std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ :: new_1 ( { (sarr_4.with(|v| v.as_pointer())  as Ptr< Vec<i8>  > ).reinterpret_cast::<Vec<i8  >  >()  } , )   , )
        });
        ({
            testRuntimeSpan_17 ( std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ :: new_1 ( { (sarr_4.with(|v| v.as_pointer())  as Ptr< Vec<i8>  > ).reinterpret_cast::<Vec<i8  >  >()  } , )   , )
        });
        ({
            testRuntimeSpan_18 ( std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ :: new_1 ( { (sarr_4.with(|v| v.as_pointer())  as Ptr< Vec<i8>  > ).reinterpret_cast::<Vec<i8  >  >()  } , )   , )
        });
        ({
            testRuntimeSpan_19 ( std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ :: new_1 ( { (sarr_4.with(|v| v.as_pointer())  as Ptr< Vec<i8>  > ).reinterpret_cast::<Vec<i8  >  >()  } , )   , )
        });
    }
    {
        ({
            testRuntimeSpan_20 ( std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____5_ :: new_1 ( { (sarr_4.with(|v| v.as_pointer())  as Ptr< Vec<i8>  > ).reinterpret_cast::<Vec<i8  >  >()  } , )   , )
        });
        ({
            testRuntimeSpan_21 ( std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____5_ :: new_1 ( { (sarr_4.with(|v| v.as_pointer())  as Ptr< Vec<i8>  > ).reinterpret_cast::<Vec<i8  >  >()  } , )   , )
        });
        ({
            testRuntimeSpan_22 ( std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____5_ :: new_1 ( { (sarr_4.with(|v| v.as_pointer())  as Ptr< Vec<i8>  > ).reinterpret_cast::<Vec<i8  >  >()  } , )   , )
        });
        ({
            testRuntimeSpan_23 ( std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____5_ :: new_1 ( { (sarr_4.with(|v| v.as_pointer())  as Ptr< Vec<i8>  > ).reinterpret_cast::<Vec<i8  >  >()  } , )   , )
        });
        ({
            testRuntimeSpan_24 ( std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____5_ :: new_1 ( { (sarr_4.with(|v| v.as_pointer())  as Ptr< Vec<i8>  > ).reinterpret_cast::<Vec<i8  >  >()  } , )   , )
        });
        ({
            testRuntimeSpan_25 ( std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____5_ :: new_1 ( { (sarr_4.with(|v| v.as_pointer())  as Ptr< Vec<i8>  > ).reinterpret_cast::<Vec<i8  >  >()  } , )   , )
        });
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = carr1_2.with(|_| ());
    let _ = arr_3.with(|_| ());
    let _ = sarr_4.with(|_| ());
}
