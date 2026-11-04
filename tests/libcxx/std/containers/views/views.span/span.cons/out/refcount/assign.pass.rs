extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static carr1_0: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4])));
);
thread_local!(
    pub static carr2_1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([3, 4, 5])));
);
thread_local!(
    pub static carr3_2: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([7, 8])));
);
thread_local!(
    pub static arr_3: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([5, 6, 7, 9])));
);
thread_local!(
    pub static strs_4: Value<Box<[Vec<i8>]>> = Rc::new(RefCell::new(Box::new([
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
        let spans: Value<Box<[std_span_const_int__18446744073709551615_]>> =
            Rc::new(RefCell::new(Box::new([
                <std_span_const_int__18446744073709551615_>::default(),
                std_span_const_int__18446744073709551615_::new_1(
                    { (carr1_0.with(|v| v.as_pointer()) as Ptr<i32>) },
                    { (0_usize as u64) },
                ),
                std_span_const_int__18446744073709551615_::new_1(
                    { (carr1_0.with(|v| v.as_pointer()) as Ptr<i32>) },
                    { 1_u64 },
                ),
                std_span_const_int__18446744073709551615_::new_1(
                    { (carr1_0.with(|v| v.as_pointer()) as Ptr<i32>) },
                    { 2_u64 },
                ),
                std_span_const_int__18446744073709551615_::new_1(
                    { (carr1_0.with(|v| v.as_pointer()) as Ptr<i32>) },
                    { 3_u64 },
                ),
                std_span_const_int__18446744073709551615_::new_1(
                    { (carr1_0.with(|v| v.as_pointer()) as Ptr<i32>) },
                    { 4_u64 },
                ),
                std_span_const_int__18446744073709551615_::new_1(
                    { (carr2_1.with(|v| v.as_pointer()) as Ptr<i32>) },
                    { (0_usize as u64) },
                ),
                std_span_const_int__18446744073709551615_::new_1(
                    { (carr2_1.with(|v| v.as_pointer()) as Ptr<i32>) },
                    { 1_u64 },
                ),
                std_span_const_int__18446744073709551615_::new_1(
                    { (carr2_1.with(|v| v.as_pointer()) as Ptr<i32>) },
                    { 2_u64 },
                ),
                std_span_const_int__18446744073709551615_::new_1(
                    { (carr2_1.with(|v| v.as_pointer()) as Ptr<i32>) },
                    { 3_u64 },
                ),
                std_span_const_int__18446744073709551615_::new_1(
                    { (carr3_2.with(|v| v.as_pointer()) as Ptr<i32>) },
                    { (0_usize as u64) },
                ),
                std_span_const_int__18446744073709551615_::new_1(
                    { (carr3_2.with(|v| v.as_pointer()) as Ptr<i32>) },
                    { 1_u64 },
                ),
                std_span_const_int__18446744073709551615_::new_1(
                    { (carr3_2.with(|v| v.as_pointer()) as Ptr<i32>) },
                    { 2_u64 },
                ),
            ])));
        const _: () = assert!(true, "std::size(spans) == 13");;
        const _: () = assert!(true, "doAssign(spans[0], spans[0])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[1])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[2])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[3])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[5])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[6])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[7])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[8])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[9])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[10])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[11])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[12])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[1])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[2])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[3])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[5])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[6])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[7])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[8])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[9])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[10])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[11])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[12])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[2])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[3])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[5])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[6])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[7])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[8])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[9])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[10])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[11])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[12])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[3])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[10])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[11])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[12])");;
        const _: () = assert!(true, "doAssign(spans[4], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[4], spans[5])");;
        const _: () = assert!(true, "doAssign(spans[4], spans[6])");;
        const _: () = assert!(true, "doAssign(spans[4], spans[7])");;
        const _: () = assert!(true, "doAssign(spans[4], spans[8])");;
        const _: () = assert!(true, "doAssign(spans[4], spans[9])");;
        const _: () = assert!(true, "doAssign(spans[4], spans[10])");;
        const _: () = assert!(true, "doAssign(spans[4], spans[11])");;
        const _: () = assert!(true, "doAssign(spans[4], spans[12])");;
        const _: () = assert!(true, "doAssign(spans[5], spans[5])");;
        const _: () = assert!(true, "doAssign(spans[5], spans[6])");;
        const _: () = assert!(true, "doAssign(spans[5], spans[7])");;
        const _: () = assert!(true, "doAssign(spans[5], spans[8])");;
        const _: () = assert!(true, "doAssign(spans[5], spans[9])");;
        const _: () = assert!(true, "doAssign(spans[5], spans[10])");;
        const _: () = assert!(true, "doAssign(spans[5], spans[11])");;
        const _: () = assert!(true, "doAssign(spans[5], spans[12])");;
        const _: () = assert!(true, "doAssign(spans[6], spans[6])");;
        const _: () = assert!(true, "doAssign(spans[6], spans[7])");;
        const _: () = assert!(true, "doAssign(spans[6], spans[8])");;
        const _: () = assert!(true, "doAssign(spans[6], spans[9])");;
        const _: () = assert!(true, "doAssign(spans[6], spans[10])");;
        const _: () = assert!(true, "doAssign(spans[6], spans[11])");;
        const _: () = assert!(true, "doAssign(spans[6], spans[12])");;
        const _: () = assert!(true, "doAssign(spans[7], spans[7])");;
        const _: () = assert!(true, "doAssign(spans[7], spans[8])");;
        const _: () = assert!(true, "doAssign(spans[7], spans[9])");;
        const _: () = assert!(true, "doAssign(spans[7], spans[10])");;
        const _: () = assert!(true, "doAssign(spans[7], spans[11])");;
        const _: () = assert!(true, "doAssign(spans[7], spans[12])");;
        const _: () = assert!(true, "doAssign(spans[8], spans[8])");;
        const _: () = assert!(true, "doAssign(spans[8], spans[9])");;
        const _: () = assert!(true, "doAssign(spans[8], spans[10])");;
        const _: () = assert!(true, "doAssign(spans[8], spans[11])");;
        const _: () = assert!(true, "doAssign(spans[8], spans[12])");;
        const _: () = assert!(true, "doAssign(spans[9], spans[9])");;
        const _: () = assert!(true, "doAssign(spans[9], spans[10])");;
        const _: () = assert!(true, "doAssign(spans[9], spans[11])");;
        const _: () = assert!(true, "doAssign(spans[9], spans[12])");;
        const _: () = assert!(true, "doAssign(spans[10], spans[10])");;
        const _: () = assert!(true, "doAssign(spans[10], spans[11])");;
        const _: () = assert!(true, "doAssign(spans[10], spans[12])");;
        const _: () = assert!(true, "doAssign(spans[11], spans[11])");;
        const _: () = assert!(true, "doAssign(spans[11], spans[12])");;
        const _: () = assert!(true, "doAssign(spans[12], spans[12])");;
    }
    {
        let spans: Value<Box<[std_span_const_int__2_]>> = Rc::new(RefCell::new(Box::new([
            std_span_const_int__2_::new_1({ (carr1_0.with(|v| v.as_pointer()) as Ptr<i32>) }, {
                2_u64
            }),
            std_span_const_int__2_::new_1(
                { (carr1_0.with(|v| v.as_pointer()) as Ptr<i32>).offset((1) as isize) },
                { 2_u64 },
            ),
            std_span_const_int__2_::new_1(
                { (carr1_0.with(|v| v.as_pointer()) as Ptr<i32>).offset((2) as isize) },
                { 2_u64 },
            ),
            std_span_const_int__2_::new_1({ (carr2_1.with(|v| v.as_pointer()) as Ptr<i32>) }, {
                2_u64
            }),
            std_span_const_int__2_::new_1(
                { (carr2_1.with(|v| v.as_pointer()) as Ptr<i32>).offset((1) as isize) },
                { 2_u64 },
            ),
            std_span_const_int__2_::new_1({ (carr3_2.with(|v| v.as_pointer()) as Ptr<i32>) }, {
                2_u64
            }),
        ])));
        const _: () = assert!(true, "std::size(spans) == 6");;
        const _: () = assert!(true, "doAssign(spans[0], spans[0])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[1])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[2])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[3])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[5])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[1])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[2])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[3])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[5])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[2])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[3])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[5])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[3])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[5])");;
        const _: () = assert!(true, "doAssign(spans[4], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[4], spans[5])");;
        const _: () = assert!(true, "doAssign(spans[5], spans[5])");;
    }
    {
        let spans: Value<Box<[std_span_int__18446744073709551615_]>> =
            Rc::new(RefCell::new(Box::new([
                <std_span_int__18446744073709551615_>::default(),
                std_span_int__18446744073709551615_::new_1(
                    { (arr_3.with(|v| v.as_pointer()) as Ptr<i32>) },
                    { (arr_3.with(|v| v.as_pointer()) as Ptr<i32>).offset((1) as isize) },
                ),
                std_span_int__18446744073709551615_::new_1(
                    { (arr_3.with(|v| v.as_pointer()) as Ptr<i32>) },
                    { (arr_3.with(|v| v.as_pointer()) as Ptr<i32>).offset((2) as isize) },
                ),
                std_span_int__18446744073709551615_::new_1(
                    { (arr_3.with(|v| v.as_pointer()) as Ptr<i32>) },
                    { (arr_3.with(|v| v.as_pointer()) as Ptr<i32>).offset((3) as isize) },
                ),
                std_span_int__18446744073709551615_::new_1(
                    { (arr_3.with(|v| v.as_pointer()) as Ptr<i32>).offset((1) as isize) },
                    { (arr_3.with(|v| v.as_pointer()) as Ptr<i32>).offset((3) as isize) },
                ),
            ])));
        let mut i: usize = 0_usize;
        'loop_: while (i < ({ size_5(spans.as_pointer()) })) {
            let mut j: usize = i;
            'loop_: while (j < ({ size_5(spans.as_pointer()) })) {
                assert!(
                    ({
                        ({
                            let _lhs: std_span_int__18446744073709551615_ =
                                ((*spans.borrow())[(i) as usize]).clone();
                            let _rhs: std_span_int__18446744073709551615_ =
                                ((*spans.borrow())[(j) as usize]).clone();
                            doAssign_6(_lhs, _rhs)
                        })
                    })
                );
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let spans: Value<Box<[std_span_int__2_]>> = Rc::new(RefCell::new(Box::new([
            std_span_int__2_::new_1({ (arr_3.with(|v| v.as_pointer()) as Ptr<i32>) }, {
                (arr_3.with(|v| v.as_pointer()) as Ptr<i32>).offset((2) as isize)
            }),
            std_span_int__2_::new_1(
                { (arr_3.with(|v| v.as_pointer()) as Ptr<i32>).offset((1) as isize) },
                { (arr_3.with(|v| v.as_pointer()) as Ptr<i32>).offset((3) as isize) },
            ),
            std_span_int__2_::new_1(
                { (arr_3.with(|v| v.as_pointer()) as Ptr<i32>).offset((2) as isize) },
                { (arr_3.with(|v| v.as_pointer()) as Ptr<i32>).offset((4) as isize) },
            ),
        ])));
        let mut i: usize = 0_usize;
        'loop_: while (i < ({ size_7(spans.as_pointer()) })) {
            let mut j: usize = i;
            'loop_: while (j < ({ size_7(spans.as_pointer()) })) {
                assert!(
                    ({
                        ({
                            let _lhs: std_span_int__2_ = ((*spans.borrow())[(i) as usize]).clone();
                            let _rhs: std_span_int__2_ = ((*spans.borrow())[(j) as usize]).clone();
                            doAssign_8(_lhs, _rhs)
                        })
                    })
                );
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let spans : Value<Box<[ std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_  ]> > = Rc::new(RefCell::new(Box::new( [ std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ :: new_1 ( { (( strs_4.with(|v| v.as_pointer())  as Ptr< Vec<i8>  >   ) ).reinterpret_cast::<Vec<i8  >  >()  } , { (( strs_4.with(|v| v.as_pointer())  as Ptr< Vec<i8>  >   ) ).reinterpret_cast::<Vec<i8  >  >()  } , )   , std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ :: new_1 ( { (( strs_4.with(|v| v.as_pointer())  as Ptr< Vec<i8>  >   ) ).reinterpret_cast::<Vec<i8  >  >()  } , { (( strs_4.with(|v| v.as_pointer())  as Ptr< Vec<i8>  >   ) . offset ( ( 1 ) as isize ) ).reinterpret_cast::<Vec<i8  >  >()  } , )   , std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ :: new_1 ( { (( strs_4.with(|v| v.as_pointer())  as Ptr< Vec<i8>  >   ) ).reinterpret_cast::<Vec<i8  >  >()  } , { (( strs_4.with(|v| v.as_pointer())  as Ptr< Vec<i8>  >   ) . offset ( ( 2 ) as isize ) ).reinterpret_cast::<Vec<i8  >  >()  } , )   , std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ :: new_1 ( { (( strs_4.with(|v| v.as_pointer())  as Ptr< Vec<i8>  >   ) ).reinterpret_cast::<Vec<i8  >  >()  } , { (( strs_4.with(|v| v.as_pointer())  as Ptr< Vec<i8>  >   ) . offset ( ( 3 ) as isize ) ).reinterpret_cast::<Vec<i8  >  >()  } , )   , std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ :: new_1 ( { (( strs_4.with(|v| v.as_pointer())  as Ptr< Vec<i8>  >   ) . offset ( ( 1 ) as isize ) ).reinterpret_cast::<Vec<i8  >  >()  } , { (( strs_4.with(|v| v.as_pointer())  as Ptr< Vec<i8>  >   ) . offset ( ( 1 ) as isize ) ).reinterpret_cast::<Vec<i8  >  >()  } , )   , std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ :: new_1 ( { (( strs_4.with(|v| v.as_pointer())  as Ptr< Vec<i8>  >   ) . offset ( ( 1 ) as isize ) ).reinterpret_cast::<Vec<i8  >  >()  } , { (( strs_4.with(|v| v.as_pointer())  as Ptr< Vec<i8>  >   ) . offset ( ( 2 ) as isize ) ).reinterpret_cast::<Vec<i8  >  >()  } , )   , std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ :: new_1 ( { (( strs_4.with(|v| v.as_pointer())  as Ptr< Vec<i8>  >   ) . offset ( ( 1 ) as isize ) ).reinterpret_cast::<Vec<i8  >  >()  } , { (( strs_4.with(|v| v.as_pointer())  as Ptr< Vec<i8>  >   ) . offset ( ( 3 ) as isize ) ).reinterpret_cast::<Vec<i8  >  >()  } , )   , std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ :: new_1 ( { (( strs_4.with(|v| v.as_pointer())  as Ptr< Vec<i8>  >   ) . offset ( ( 2 ) as isize ) ).reinterpret_cast::<Vec<i8  >  >()  } , { (( strs_4.with(|v| v.as_pointer())  as Ptr< Vec<i8>  >   ) . offset ( ( 2 ) as isize ) ).reinterpret_cast::<Vec<i8  >  >()  } , )   , std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ :: new_1 ( { (( strs_4.with(|v| v.as_pointer())  as Ptr< Vec<i8>  >   ) . offset ( ( 2 ) as isize ) ).reinterpret_cast::<Vec<i8  >  >()  } , { (( strs_4.with(|v| v.as_pointer())  as Ptr< Vec<i8>  >   ) . offset ( ( 3 ) as isize ) ).reinterpret_cast::<Vec<i8  >  >()  } , )   , std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_ :: new_1 ( { (( strs_4.with(|v| v.as_pointer())  as Ptr< Vec<i8>  >   ) . offset ( ( 3 ) as isize ) ).reinterpret_cast::<Vec<i8  >  >()  } , { (( strs_4.with(|v| v.as_pointer())  as Ptr< Vec<i8>  >   ) . offset ( ( 3 ) as isize ) ).reinterpret_cast::<Vec<i8  >  >()  } , )   , ] ) )) ;
        let mut i: usize = 0_usize;
        'loop_: while (i < ({ size_9(spans.as_pointer()) })) {
            let mut j: usize = i;
            'loop_: while (j < ({ size_9(spans.as_pointer()) })) {
                assert!(
                    ({
                        ({
                            let _lhs: std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_   = ((*spans.borrow()) [(i ) as usize] ).clone()  ;
                            let _rhs: std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____18446744073709551615_   = ((*spans.borrow()) [(j ) as usize] ).clone()  ;
                            doAssign_10(_lhs, _rhs)
                        })
                    })
                );
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let spans: Value<
            Box<[std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____1_]>,
        > = Rc::new(RefCell::new(Box::new([
            std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____1_::new_1(
                { (strs_4.with(|v| v.as_pointer()) as Ptr<Vec<i8>>).reinterpret_cast::<Vec<i8>>() },
                {
                    ((strs_4.with(|v| v.as_pointer()) as Ptr<Vec<i8>>).offset((1) as isize))
                        .reinterpret_cast::<Vec<i8>>()
                },
            ),
            std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____1_::new_1(
                {
                    ((strs_4.with(|v| v.as_pointer()) as Ptr<Vec<i8>>).offset((1) as isize))
                        .reinterpret_cast::<Vec<i8>>()
                },
                {
                    ((strs_4.with(|v| v.as_pointer()) as Ptr<Vec<i8>>).offset((2) as isize))
                        .reinterpret_cast::<Vec<i8>>()
                },
            ),
            std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____1_::new_1(
                {
                    ((strs_4.with(|v| v.as_pointer()) as Ptr<Vec<i8>>).offset((2) as isize))
                        .reinterpret_cast::<Vec<i8>>()
                },
                {
                    ((strs_4.with(|v| v.as_pointer()) as Ptr<Vec<i8>>).offset((3) as isize))
                        .reinterpret_cast::<Vec<i8>>()
                },
            ),
        ])));
        let mut i: usize = 0_usize;
        'loop_: while (i < ({ size_11(spans.as_pointer()) })) {
            let mut j: usize = i;
            'loop_: while (j < ({ size_11(spans.as_pointer()) })) {
                assert!(
                    ({
                        ({
                            let _lhs: std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____1_   = ((*spans.borrow()) [(i ) as usize] ).clone()  ;
                            let _rhs: std_span_std_basic_string_char__std_char_traits_char___std_allocator_char____1_   = ((*spans.borrow()) [(j ) as usize] ).clone()  ;
                            doAssign_12(_lhs, _rhs)
                        })
                    })
                );
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = carr1_0.with(|_| ());
    let _ = carr2_1.with(|_| ());
    let _ = carr3_2.with(|_| ());
    let _ = arr_3.with(|_| ());
    let _ = strs_4.with(|_| ());
}
