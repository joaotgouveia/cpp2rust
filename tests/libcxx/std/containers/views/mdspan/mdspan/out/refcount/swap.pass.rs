extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(4)]
pub struct MinimalElementType {
    #[offset(0)]
    pub val: (),
}
impl MinimalElementType {
    pub fn new_1(mut v: ()) -> Self {
        Self { val: v }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct not_extents_constructible_tag {}
thread_local!(
    static Wrap_0: Value<()> = Rc::new(RefCell::new(4_()));
);
pub fn operator_eq_1(
    lhs: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__>,
    rhs: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__>,
) -> () {
    return unimplemented!();
}
pub fn swap_2(
    x: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__>,
    y: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__>,
) {
    unimplemented!();
    {
        ( { layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__::swap_counter ( ) } )  .with_mut(|__v| __v. postfix_inc ()) ;
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__ {
    #[offset(0)]
    #[byte_size(4)]
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__ {
    pub fn copy_from(
        other: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__>,
    ) -> Self {
        Self {
            extents_: unimplemented!(),
        }
    }
    pub fn new_1(ext: Ptr<()>, mut _a1: not_extents_constructible_tag) -> Self {
        Self {
            extents_: unimplemented!(),
        }
    }
    pub fn swap_counter() -> Ptr<()> {
        return value_3.with(|v| v.as_pointer());
    }
}
impl std::cmp::PartialEq
    for layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__
{
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_1(Rc::new(RefCell::new(layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__ { extents_: self.extents_.clone(), })).as_pointer(), Rc::new(RefCell::new(layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__ { extents_: other.extents_.clone(), })).as_pointer())
        }
    }
}
impl std::cmp::Eq
    for layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__
{
}
impl Clone for layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__ {
    fn clone(&self) -> Self {
        let __src: Value<
            layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__,
        > = Rc::new(RefCell::new(
            layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__ {
                extents_: self.extents_.clone(),
            },
        ));
        layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__::copy_from(
            __src.as_pointer(),
        )
    }
}
impl Default for layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__ {
    fn default() -> Self {
        {
            layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__ {
                extents_: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct layout_wrapping_integral_4_ {}
thread_local!(
    static value_3: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_convertible_layout {}
pub fn test_swap_4(a: (), b: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let org_a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let org_b: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
        unimplemented!();
    }
    {
        unimplemented!();
        unimplemented!();
    }
    { {} }
}
pub fn test_swap_5(a: (), b: ()) {
    let a: Value<()> = Rc::new(RefCell::new(a));
    let b: Value<()> = Rc::new(RefCell::new(b));
    let org_a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let org_b: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
        unimplemented!();
    }
    {
        unimplemented!();
        unimplemented!();
    }
    {
        {
            unimplemented!();
        }
    }
}
pub fn test_6() -> () {
    let data_a: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..1024).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    let data_b: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..1024).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let b: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ test_swap_4(unimplemented!(), unimplemented!()) });
    }
    {
        let mut map_a : layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__  = { let __tmp_0 : Value<() > = Rc::new(RefCell::new(unimplemented!() )); layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__ :: new_1 ( { __tmp_0.as_pointer() } , { <not_extents_constructible_tag  >::default()  } , )  }  ;
        let mut map_b : layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__  = { let __tmp_1 : Value<() > = Rc::new(RefCell::new(unimplemented!() )); layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__ :: new_1 ( { __tmp_1.as_pointer() } , { <not_extents_constructible_tag  >::default()  } , )  }  ;
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let b: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ test_swap_5(unimplemented!(), unimplemented!()) });
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_6() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__Impl {
    fn copy_assign(
        &self,
        other: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__>,
    ) -> Ptr<layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__> {
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> () {
        unimplemented!()
    }
    fn is_unique(&self) -> () {
        unimplemented!()
    }
    fn is_strided(&self) -> () {
        unimplemented!()
    }
    fn stride(&self, mut r: ()) -> () {
        unimplemented!()
    }
}
impl layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__Impl
    for Ptr<layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__>
{
    fn extents(&self) -> Ptr<()> {
        return field_ptr!((*self), extents_);
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = Wrap_0.with(|_| ());
}
