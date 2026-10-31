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
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> usize {
        assert!((alloc_count_1.with(|rc| *rc.borrow()) >= dealloc_count_2.with(|rc| *rc.borrow())));
        return ((alloc_count_1.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_2.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_3.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_0.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_1.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_2.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
thread_local!(
    pub static outstanding_bytes_0: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_1: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_2: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_3: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__
{
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<(Value<i32>, Value<Vec<i8>>)>,
}
impl min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__ { }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: AnyPtr,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
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
        let a: Value<Box<[(Value<i32>, Value<Vec<i8>>)]>> = Rc::new(RefCell::new(Box::new([
            (
                Rc::new(RefCell::new(1)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"one").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            ),
            (
                Rc::new(RefCell::new(2)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"two").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            ),
            (
                Rc::new(RefCell::new(3)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"three").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            ),
            (
                Rc::new(RefCell::new(4)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            ),
            (
                Rc::new(RefCell::new(1)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            ),
            (
                Rc::new(RefCell::new(2)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            ),
        ])));
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut bc: usize = (unimplemented!() as usize);
        assert!((bc >= 5_usize));
        let mut i: usize = 0_usize;
        'loop_: while (i < 13_usize) {
            const _: () = assert!(true, "");;
            i.prefix_inc();
        }
    }
    {
        let a: Value<Box<[(Value<i32>, Value<Vec<i8>>)]>> = Rc::new(RefCell::new(Box::new([
            (
                Rc::new(RefCell::new(1)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"one").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            ),
            (
                Rc::new(RefCell::new(2)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"two").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            ),
            (
                Rc::new(RefCell::new(3)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"three").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            ),
            (
                Rc::new(RefCell::new(4)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            ),
            (
                Rc::new(RefCell::new(1)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            ),
            (
                Rc::new(RefCell::new(2)),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                    __bytes.push(0_i8);
                    __bytes
                })),
            ),
        ])));
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut bc: usize = (unimplemented!() as usize);
        assert!((bc >= 5_usize));
        let mut i: usize = 0_usize;
        'loop_: while (i < 13_usize) {
            const _: () = assert!(true, "");;
            i.prefix_inc();
        }
    }
    return 0;
}
pub trait min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____Impl
{
    fn allocate ( &self , mut n : usize  , ) -> min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p : min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____Impl for Ptr<min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____> {
}
pub trait min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__Impl
{
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<(Value<i32>, Value<Vec<i8>>)> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<(Value<i32>, Value<Vec<i8>>)> {
        unimplemented!()
    }    fn operator_inc_8 ( &self , ) -> Ptr< min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_inc_9 ( &self , mut _a0 : i32  , ) -> min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_dec_10 ( &self , ) -> Ptr< min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_dec_11 ( &self , mut _a0 : i32  , ) -> min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_add_assign ( &self , mut n : isize  , ) -> Ptr< min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_sub_assign ( &self , mut n : isize  , ) -> Ptr< min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_add ( &self , mut n : isize  , ) -> min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_sub ( &self , mut n : isize  , ) -> min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<(Value<i32>, Value<Vec<i8>>)> {
        unimplemented!()
    }
}
impl min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__Impl for Ptr<min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__> {
}
pub trait min_pointer_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_void__std_integral_constant_unsigned_long__0__>
{
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
}
