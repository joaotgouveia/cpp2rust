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
pub struct test_macros_detail_is_same_int_ptr__int_ptr_ {}
impl CppMove for test_macros_detail_is_same_int_ptr__int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_sized_subrange_1() {
    let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4])));
    let r: Value<std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_> =
        Rc::new(RefCell::new(
            std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_::new_1(
                { (a.as_pointer() as Ptr<i32>) },
                { (a.as_pointer() as Ptr<i32>).offset((4) as isize) },
            ),
        ));
    assert!(true);
    {
        let __decomp_2: Value<
            std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_,
        > = Rc::new(RefCell::new((*r.borrow()).clone()));
        let mut first: Ptr<i32> = ({ get_3(__decomp_2.as_pointer()) });
        let mut last: Ptr<i32> = ({ get_4(__decomp_2.as_pointer()) });
        assert!(({ (first).clone() } == { (a.as_pointer() as Ptr::<i32>) }));
        assert!(({ (last).clone() } == { (a.as_pointer() as Ptr::<i32>).offset((4) as isize) }));
    }
    {
        let __decomp_5: Value<
            std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_,
        > = Rc::new(RefCell::new((*r.borrow_mut()).clone()));
        let mut first: Ptr<i32> = ({ get_3(__decomp_5.as_pointer()) });
        let mut last: Ptr<i32> = ({ get_4(__decomp_5.as_pointer()) });
        assert!(({ (first).clone() } == { (a.as_pointer() as Ptr::<i32>) }));
        assert!(({ (last).clone() } == { (a.as_pointer() as Ptr::<i32>).offset((4) as isize) }));
    }
    {
        let __decomp_6: Value<
            std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_,
        > = Rc::new(RefCell::new((*r.borrow()).clone()));
        let mut first: Ptr<i32> = ({ get_3(__decomp_6.as_pointer()) });
        let mut last: Ptr<i32> = ({ get_4(__decomp_6.as_pointer()) });
        assert!(({ (first).clone() } == { (a.as_pointer() as Ptr::<i32>) }));
        assert!(({ (last).clone() } == { (a.as_pointer() as Ptr::<i32>).offset((4) as isize) }));
    }
    {
        let __decomp_7: Value<
            std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_,
        > = Rc::new(RefCell::new((*r.borrow()).clone()));
        let mut first: Ptr<i32> = ({ get_3(__decomp_7.as_pointer()) });
        let mut last: Ptr<i32> = ({ get_4(__decomp_7.as_pointer()) });
        assert!(({ (first).clone() } == { (a.as_pointer() as Ptr::<i32>) }));
        assert!(({ (last).clone() } == { (a.as_pointer() as Ptr::<i32>).offset((4) as isize) }));
    }
}
pub fn test_unsized_subrange_8() {
    let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4])));
    let r : Value<std_ranges_subrange_int_ptr__std_unreachable_sentinel_t__std_ranges_subrange_kind_unsized_ > = Rc::new(RefCell::new(std_ranges_subrange_int_ptr__std_unreachable_sentinel_t__std_ranges_subrange_kind_unsized_ :: new_1 ( { ( a .as_pointer()  as Ptr::<i32> )  } , { ((*unreachable_sentinel_9.with(Value::clone).borrow()) ).clone()  } , )  )) ;
    assert!(!(false));
    {
        let __decomp_10 : Value<std_ranges_subrange_int_ptr__std_unreachable_sentinel_t__std_ranges_subrange_kind_unsized_ > = Rc::new(RefCell::new(((*r.borrow()) ).clone() )) ;
        let mut first: Ptr<i32> = ({ get_11(__decomp_10.as_pointer()) });
        let last: Value<std_unreachable_sentinel_t> =
            Rc::new(RefCell::new(({ get_12(__decomp_10.as_pointer()) })));
        assert!(({ (first).clone() } == { (a.as_pointer() as Ptr::<i32>) }));
        const _: () = assert!(true, "");;
    }
    {
        let __decomp_13 : Value<std_ranges_subrange_int_ptr__std_unreachable_sentinel_t__std_ranges_subrange_kind_unsized_ > = Rc::new(RefCell::new(((*r.borrow_mut())  ).clone())) ;
        let mut first: Ptr<i32> = ({ get_11(__decomp_13.as_pointer()) });
        let last: Value<std_unreachable_sentinel_t> =
            Rc::new(RefCell::new(({ get_12(__decomp_13.as_pointer()) })));
        assert!(({ (first).clone() } == { (a.as_pointer() as Ptr::<i32>) }));
        const _: () = assert!(true, "");;
    }
    {
        let __decomp_14 : Value<std_ranges_subrange_int_ptr__std_unreachable_sentinel_t__std_ranges_subrange_kind_unsized_ > = Rc::new(RefCell::new(((*r.borrow()) ).clone() )) ;
        let mut first: Ptr<i32> = ({ get_11(__decomp_14.as_pointer()) });
        let last: Value<std_unreachable_sentinel_t> =
            Rc::new(RefCell::new(({ get_12(__decomp_14.as_pointer()) })));
        assert!(({ (first).clone() } == { (a.as_pointer() as Ptr::<i32>) }));
        const _: () = assert!(true, "");;
    }
    {
        let __decomp_15 : Value<std_ranges_subrange_int_ptr__std_unreachable_sentinel_t__std_ranges_subrange_kind_unsized_ > = Rc::new(RefCell::new(((*r.borrow()) ).clone() )) ;
        let mut first: Ptr<i32> = ({ get_11(__decomp_15.as_pointer()) });
        let last: Value<std_unreachable_sentinel_t> =
            Rc::new(RefCell::new(({ get_12(__decomp_15.as_pointer()) })));
        assert!(({ (first).clone() } == { (a.as_pointer() as Ptr::<i32>) }));
        const _: () = assert!(true, "");;
    }
}
pub fn test_copies_not_originals_16() {
    let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4])));
    {
        let r: Value<std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_> =
            Rc::new(RefCell::new(
                std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_::new_1(
                    { (a.as_pointer() as Ptr<i32>) },
                    { (a.as_pointer() as Ptr<i32>).offset((4) as isize) },
                ),
            ));
        let __decomp_17: Ptr<
            std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_,
        > = r.as_pointer();
        let mut first: Ptr<i32> = ({ get_18((__decomp_17).clone()) });
        let mut last: Ptr<i32> = ({ get_19((__decomp_17).clone()) });
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        first = (a.as_pointer() as Ptr<i32>).offset((2) as isize);
        last = (a.as_pointer() as Ptr<i32>).offset((2) as isize);
        assert!(({ (({ (*r.borrow()).begin_8() }) == (a.as_pointer() as Ptr::<i32>)) }));
        assert!(
            ({
                (({ (*r.borrow()).end() }) == (a.as_pointer() as Ptr<i32>).offset((4) as isize))
            })
        );
    }
    {
        let r: Value<std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_> =
            Rc::new(RefCell::new(
                std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_::new_1(
                    { (a.as_pointer() as Ptr<i32>) },
                    { (a.as_pointer() as Ptr<i32>).offset((4) as isize) },
                ),
            ));
        let __decomp_20: Ptr<
            std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_,
        > = r.as_pointer();
        let mut first: Ptr<i32> = ({ get_18((__decomp_20).clone()) });
        let mut last: Ptr<i32> = ({ get_19((__decomp_20).clone()) });
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        first = (a.as_pointer() as Ptr<i32>).offset((2) as isize);
        last = (a.as_pointer() as Ptr<i32>).offset((2) as isize);
        assert!(({ (({ (*r.borrow()).begin_8() }) == (a.as_pointer() as Ptr::<i32>)) }));
        assert!(
            ({
                (({ (*r.borrow()).end() }) == (a.as_pointer() as Ptr<i32>).offset((4) as isize))
            })
        );
    }
}
pub fn test_21() -> bool {
    ({ test_sized_subrange_1() });
    ({ test_unsized_subrange_8() });
    ({ test_copies_not_originals_16() });
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
    ({ test_21() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
