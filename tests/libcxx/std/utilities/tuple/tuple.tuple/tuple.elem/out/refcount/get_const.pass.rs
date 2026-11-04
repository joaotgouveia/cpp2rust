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
        assert!(((({ get_0(t.as_pointer(),) }).read()) == 3));
    }
    {
        let t: Value<
            std_tuple_std_basic_string_char__std_char_traits_char___std_allocator_char____int_,
        > = Rc::new(RefCell::new({
            let __tmp_0: Value<i32> = Rc::new(RefCell::new(5));
            std_tuple_std_basic_string_char__std_char_traits_char___std_allocator_char____int_ :: new_1 ( { Ptr::<i8 >::from_string_literal(b"high" ) } , { __tmp_0.as_pointer() } , )
        }));
        assert!(
            ({
                match (({ get_1(t.as_pointer()) }),) {
                    (__tmp_1,) => Ptr::<i8>::from_string_literal(b"high").with_c_str(|__s| {
                        (*__tmp_1.upgrade().deref())
                            [..(*__tmp_1.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        assert!(((({ get_2(t.as_pointer(),) }).read()) == 5));
    }
    {
        let t: Value<std_tuple_double__int_> = Rc::new(RefCell::new({
            let __tmp_2: Value<f64> = Rc::new(RefCell::new(2.718_f64));
            let __tmp_3: Value<i32> = Rc::new(RefCell::new(5));
            std_tuple_double__int_::new_1({ __tmp_2.as_pointer() }, { __tmp_3.as_pointer() })
        }));
        const _: () = assert!(true, "std::get<0>(t) == 2.718");;
        const _: () = assert!(true, "std::get<1>(t) == 5");;
    }
    {
        let t: Value<std_tuple_Empty_> = Rc::new(RefCell::new(<Empty>::default()));
        let mut e: Empty = (*({ get_3(t.as_pointer()) }).upgrade().deref()).clone();
        (&(e));
    }
    {
        let d: Value<f64> = Rc::new(RefCell::new(1.5_f64));
        let t : Value<std_tuple_double_ref__std_basic_string_char__std_char_traits_char___std_allocator_char____int_ > = Rc::new(RefCell::new({ let __tmp_4 : Value<i32 > = Rc::new(RefCell::new(5 )); std_tuple_double_ref__std_basic_string_char__std_char_traits_char___std_allocator_char____int_ :: new_1 ( { d .as_pointer()  } , { Ptr::<i8 >::from_string_literal(b"high" ) } , { __tmp_4.as_pointer() } , )  } )) ;
        assert!(((({ get_4(t.as_pointer(),) }).read()) == 1.5_f64));
        assert!(
            ({
                match (({ get_5(t.as_pointer()) }),) {
                    (__tmp_5,) => Ptr::<i8>::from_string_literal(b"high").with_c_str(|__s| {
                        (*__tmp_5.upgrade().deref())
                            [..(*__tmp_5.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        assert!(((({ get_6(t.as_pointer(),) }).read()) == 5));
        ({ get_4(t.as_pointer()) }).write(2.5_f64);
        assert!(((({ get_4(t.as_pointer(),) }).read()) == 2.5_f64));
        assert!(
            ({
                match (({ get_5(t.as_pointer()) }),) {
                    (__tmp_6,) => Ptr::<i8>::from_string_literal(b"high").with_c_str(|__s| {
                        (*__tmp_6.upgrade().deref())
                            [..(*__tmp_6.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        assert!(((({ get_6(t.as_pointer(),) }).read()) == 5));
        assert!(({ ((*d.borrow()) == 2.5_f64) }));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
