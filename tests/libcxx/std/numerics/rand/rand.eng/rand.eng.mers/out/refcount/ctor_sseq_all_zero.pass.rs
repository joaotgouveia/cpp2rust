extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(std_initializer_list_unsigned_int_, new_1, value)]
#[byte_size(1)]
pub struct all_zero_seed_seq {}
impl all_zero_seed_seq {
    pub fn new_2() -> Self {
        Self {}
    }
    pub fn new_1(_a0: std_initializer_list_unsigned_int_) -> Self {
        let _a0: Value<std_initializer_list_unsigned_int_> = Rc::new(RefCell::new(_a0));
        Self {}
    }
}
impl CppMove for all_zero_seed_seq {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for all_zero_seed_seq {
    fn default() -> Self {
        { all_zero_seed_seq::new_2() }
    }
}
pub fn test_1() {
    let mut state_size: u64 = 1_u64;
    let mut shift_size: u64 = 1_u64;
    let mut tempering_l: u64 = 3_u64;
    let q: Value<all_zero_seed_seq> = Rc::new(RefCell::new(all_zero_seed_seq::new_2()));
    let e: Value<
        std_mersenne_twister_engine_unsigned_short__3__1__1__0__0__0__0__0__0__0__0__3__0_,
    > = Rc::new(RefCell::new(
        std_mersenne_twister_engine_unsigned_short__3__1__1__0__0__0__0__0__0__0__0__3__0_::new_1(
            { q.as_pointer() },
        ),
    ));
    let mut Xneg1: u16 = (((1_u16 as i32) << ((3_u64 as u64).wrapping_sub(1_u64))) as u16);
    let mut Y: u16 = Xneg1;
    let mut X0: u16 = (((Xneg1 as i32) ^ ((Y as i32) >> 1)) as u16);
    ({ cpp2rust_assert_fail_2(((({ (*e.borrow_mut())() }) as i32) == (X0 as i32))) });
}
pub fn test_3() {
    let mut state_size: u64 = 1_u64;
    let mut shift_size: u64 = 1_u64;
    let mut tempering_l: u64 = 33_u64;
    let q: Value<all_zero_seed_seq> = Rc::new(RefCell::new(all_zero_seed_seq::new_2()));
    let e : Value<std_mersenne_twister_engine_unsigned_long_long__33__1__1__0__0__0__0__0__0__0__0__33__0_ > = Rc::new(RefCell::new(std_mersenne_twister_engine_unsigned_long_long__33__1__1__0__0__0__0__0__0__0__0__33__0_ :: new_1 ( { q .as_pointer()  } , )  )) ;
    let mut Xneg1: u64 = (1_u64 << ((33_u64 as u64).wrapping_sub(1_u64)));
    let mut Y: u64 = Xneg1;
    let mut X0: u64 = (Xneg1 ^ (Y >> 1));
    ({ cpp2rust_assert_fail_2((({ (*e.borrow_mut())() }) == X0)) });
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
    ({ test_1() });
    ({ test_3() });
    return 0;
}
pub trait all_zero_seed_seqImpl {
    fn size(&self) -> u64;
    fn generate_4_unsigned_int_ptr(&self, rb: Ptr<u32>, re: Ptr<u32>);
}
impl all_zero_seed_seqImpl for Ptr<all_zero_seed_seq> {
    fn size(&self) -> u64 {
        return 0_u64;
    }
    fn generate_4_unsigned_int_ptr(&self, mut rb: Ptr<u32>, mut re: Ptr<u32>) {
        ({
            let ___first: Ptr<u32> = (rb).clone();
            let ___last: Ptr<u32> = (re).clone();
            let ___value: Value<u32> = Rc::new(RefCell::new(0_u32));
            fill_0(___first, ___last, ___value.as_pointer())
        });
    }
}
pub fn __cpp2rust_init_globals() {}
