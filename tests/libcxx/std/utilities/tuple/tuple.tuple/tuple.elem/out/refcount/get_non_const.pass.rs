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
pub struct Empty {}
impl CppMove for Empty {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(12)]
pub struct S {
    #[offset(0)]
    #[byte_size(4)]
    pub a: std_tuple_int__Empty_,
    #[offset(4)]
    pub k: i32,
    #[offset(8)]
    #[byte_size(1)]
    pub e: Empty,
}
impl S {
    pub fn new() -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            a: {
                let __tmp_0: Value<i32> = Rc::new(RefCell::new(1));
                let __tmp_1: Value<Empty> = Rc::new(RefCell::new(<Empty>::default()));
                std_tuple_int__Empty_::new_1({ __tmp_0.as_pointer() }, { __tmp_1.as_pointer() })
            },
            k: (({ get_0(field_ptr!(this, a)) }).read()),
            e: (*({ get_1(field_ptr!(this, a)) }).upgrade().deref()).clone(),
        }));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(_a0: Ptr<S>) -> Self {
        Self {
            a: field!(_a0, a).with_mut(std::mem::take),
            k: { (*_a0.upgrade().deref()).k },
            e: { (*_a0.upgrade().deref()).e.clone() },
        }
    }
}
impl CppMove for S {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
impl Default for S {
    fn default() -> Self {
        { S::new() }
    }
}
pub fn getP_2() -> std_tuple_int__int_ {
    return {
        let __tmp_2: Value<i32> = Rc::new(RefCell::new(3));
        let __tmp_3: Value<i32> = Rc::new(RefCell::new(4));
        std_tuple_int__int_::new_1({ __tmp_2.as_pointer() }, { __tmp_3.as_pointer() })
    };
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
    {
        let t: Value<std_tuple_int_> = Rc::new(RefCell::new(3));
        assert!(((({ get_3(t.as_pointer(),) }).read()) == 3));
        ({ get_3(t.as_pointer()) }).write(2);
        assert!(((({ get_3(t.as_pointer(),) }).read()) == 2));
    }
    {
        let t: Value<
            std_tuple_std_basic_string_char__std_char_traits_char___std_allocator_char____int_,
        > = Rc::new(RefCell::new({
            let __tmp_4: Value<i32> = Rc::new(RefCell::new(5));
            std_tuple_std_basic_string_char__std_char_traits_char___std_allocator_char____int_ :: new_1 ( { Ptr::<i8 >::from_string_literal(b"high" ) } , { __tmp_4.as_pointer() } , )
        }));
        assert!(
            ({
                match (({ get_4(t.as_pointer()) }),) {
                    (__tmp_5,) => Ptr::<i8>::from_string_literal(b"high").with_c_str(|__s| {
                        (*__tmp_5.upgrade().deref())
                            [..(*__tmp_5.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        assert!(((({ get_5(t.as_pointer(),) }).read()) == 5));
        match (({ get_4(t.as_pointer()) }),) {
            (__tmp_6,) => {
                let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                __bytes.push(0_i8);
                __tmp_6.write(__bytes);
                (__tmp_6).clone()
            }
        };
        ({ get_5(t.as_pointer()) }).write(4);
        assert!(
            ({
                match (({ get_4(t.as_pointer()) }),) {
                    (__tmp_7,) => Ptr::<i8>::from_string_literal(b"four").with_c_str(|__s| {
                        (*__tmp_7.upgrade().deref())
                            [..(*__tmp_7.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        assert!(((({ get_5(t.as_pointer(),) }).read()) == 4));
    }
    {
        let d: Value<f64> = Rc::new(RefCell::new(1.5_f64));
        let t : Value<std_tuple_double_ref__std_basic_string_char__std_char_traits_char___std_allocator_char____int_ > = Rc::new(RefCell::new({ let __tmp_8 : Value<i32 > = Rc::new(RefCell::new(5 )); std_tuple_double_ref__std_basic_string_char__std_char_traits_char___std_allocator_char____int_ :: new_1 ( { d .as_pointer()  } , { Ptr::<i8 >::from_string_literal(b"high" ) } , { __tmp_8.as_pointer() } , )  } )) ;
        assert!(((({ get_6(t.as_pointer(),) }).read()) == 1.5_f64));
        assert!(
            ({
                match (({ get_7(t.as_pointer()) }),) {
                    (__tmp_9,) => Ptr::<i8>::from_string_literal(b"high").with_c_str(|__s| {
                        (*__tmp_9.upgrade().deref())
                            [..(*__tmp_9.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        assert!(((({ get_8(t.as_pointer(),) }).read()) == 5));
        ({ get_6(t.as_pointer()) }).write(2.5_f64);
        match (({ get_7(t.as_pointer()) }),) {
            (__tmp_10,) => {
                let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                __bytes.push(0_i8);
                __tmp_10.write(__bytes);
                (__tmp_10).clone()
            }
        };
        ({ get_8(t.as_pointer()) }).write(4);
        assert!(((({ get_6(t.as_pointer(),) }).read()) == 2.5_f64));
        assert!(
            ({
                match (({ get_7(t.as_pointer()) }),) {
                    (__tmp_11,) => Ptr::<i8>::from_string_literal(b"four").with_c_str(|__s| {
                        (*__tmp_11.upgrade().deref())
                            [..(*__tmp_11.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        assert!(((({ get_8(t.as_pointer(),) }).read()) == 4));
        assert!(({ ((*d.borrow()) == 2.5_f64) }));
    }
    {
        const _: () = assert!(
            true,
            "std::get<0> ( std::make_tuple ( 0.0f, 1, 2.0, 3L )) == 0"
        );;
        const _: () = assert!(
            true,
            "std::get<1> ( std::make_tuple ( 0.0f, 1, 2.0, 3L )) == 1"
        );;
        const _: () = assert!(
            true,
            "std::get<2> ( std::make_tuple ( 0.0f, 1, 2.0, 3L )) == 2"
        );;
        const _: () = assert!(
            true,
            "std::get<3> ( std::make_tuple ( 0.0f, 1, 2.0, 3L )) == 3"
        );;
        const _: () = assert!(true, "S().k == 1");;
        const _: () = assert!(true, "std::get<1>(getP()) == 4");;
    }
    return 0;
}
pub trait SImpl {
    fn move_assign(&self, _a0: Ptr<S>) -> Ptr<S>;
}
impl SImpl for Ptr<S> {
    fn move_assign(&self, _a0: Ptr<S>) -> Ptr<S> {
        field!((*self), a).write({ { (*_a0.upgrade().deref()).a.clone() } });
        field!((*self), k).write({ { (*_a0.upgrade().deref()).k } });
        field!((*self), e).write({ { (*_a0.upgrade().deref()).e.clone() } });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
