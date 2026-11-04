extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct foo {
    #[offset(0)]
    val_: i32,
}
impl foo {
    pub fn new() -> Self {
        Self { val_: 3 }
    }
}
impl CppMove for foo {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for foo {
    fn default() -> Self {
        { foo::new() }
    }
}
pub fn test_0() -> bool {
    {
        let p1: Value<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> =
            Rc::new(RefCell::new(({ make_unique_1(5_u64) })));
        let mut i: i32 = 0;
        'loop_: while (i < 5) {
            ({ cpp2rust_assert_fail_2(({ (*p1.borrow())[(i as u64)] } == 0)) });
            i.prefix_inc();
        }
    }
    {
        let p2 : Value<std_unique_ptr_std_basic_string_char__std_char_traits_char___std_allocator_char__arrarr__std_default_delete_std_basic_string_char__std_char_traits_char___std_allocator_char__arrarr__ > = Rc::new(RefCell::new(( { make_unique_3 ( 5_u64  , ) } )  )) ;
        let mut i: i32 = 0;
        'loop_: while (i < 5) {
            ({ cpp2rust_assert_fail_2((({ (*p2.borrow())[(i as u64)].size() }) == 0_u64)) });
            i.prefix_inc();
        }
    }
    {
        let p3: Value<std_unique_ptr_fooarrarr__std_default_delete_fooarrarr__> =
            Rc::new(RefCell::new(({ make_unique_4(7_u64) })));
        let mut i: i32 = 0;
        'loop_: while (i < 7) {
            ({
                cpp2rust_assert_fail_2(
                    (({ fooImpl::get(&p3.as_pointer().offset((i as u64))) }) == 3),
                )
            });
            i.prefix_inc();
        }
    }
    return true;
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
    const _: () = assert!(true, "test()");;
    return 0;
}
pub trait fooImpl {
    fn get(&self) -> i32;
}
impl fooImpl for Ptr<foo> {
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.val_);
    }
}
pub fn __cpp2rust_init_globals() {}
