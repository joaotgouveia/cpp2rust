extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(40)]
pub struct test_allocator_statistics {
    #[offset(0)]
    pub time_to_throw: i32,
    #[offset(4)]
    pub throw_after: i32,
    #[offset(8)]
    pub count: i32,
    #[offset(12)]
    pub alloc_count: i32,
    #[offset(16)]
    pub allocated_size: i32,
    #[offset(20)]
    pub construct_count: i32,
    #[offset(24)]
    pub destroy_count: i32,
    #[offset(28)]
    pub copied: i32,
    #[offset(32)]
    pub moved: i32,
    #[offset(36)]
    pub converted: i32,
}
impl CppMove for test_allocator_statistics {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for test_allocator_statistics {
    fn default() -> Self {
        {
            test_allocator_statistics {
                time_to_throw: 0,
                throw_after: 2147483647,
                count: 0,
                alloc_count: 0,
                allocated_size: 0,
                construct_count: 0,
                destroy_count: 0,
                copied: 0,
                moved: 0,
                converted: 0,
            }
        }
    }
}
thread_local!(
    pub static destructed_value_0: Value<i32> = Rc::new(RefCell::new(-1_i32));
);
thread_local!(
    pub static moved_value_1: Value<i32> = Rc::new(RefCell::new(2147483647));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
impl CppMove for test_alloc_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_eq_2(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_3(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> bool {
    return !({
        let _x: Ptr<test_allocator_void_> = (x).clone();
        let _y: Ptr<test_allocator_void_> = (y).clone();
        operator_eq_2(_x, _y)
    });
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(16)]
pub struct test_allocator_void_ {
    #[offset(0)]
    data_: i32,
    #[offset(4)]
    id_: i32,
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_void_ {
    pub fn new_1(mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: 0,
            id_: 0,
            stats_: (stats).clone(),
        }
    }
    pub fn new_2(mut data: i32) -> Self {
        Self {
            data_: data,
            id_: 0,
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn new_3(mut data: i32, mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: data,
            id_: 0,
            stats_: (stats).clone(),
        }
    }
    pub fn new_4(mut data: i32, mut id: i32) -> Self {
        Self {
            data_: data,
            id_: id,
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn new_5(mut data: i32, mut id: i32, mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: data,
            id_: id,
            stats_: (stats).clone(),
        }
    }
    pub fn copy_from(a: Ptr<test_allocator_void_>) -> Self {
        Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }
    }
}
impl CppEq for test_allocator_void_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_2(__lhs.clone(), __rhs.clone())
    }
}
impl Clone for test_allocator_void_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_void_> = Rc::new(RefCell::new(test_allocator_void_ {
            data_: self.data_.clone(),
            id_: self.id_.clone(),
            stats_: self.stats_.clone(),
        }));
        test_allocator_void_::copy_from(__src.as_pointer())
    }
}
impl CppMove for test_allocator_void_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for test_allocator_void_ {
    fn default() -> Self {
        {
            test_allocator_void_ {
                data_: 0,
                id_: 0,
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Destructor)]
#[byte_size(16)]
pub struct test_allocator_char_ {
    #[offset(0)]
    data_: i32,
    #[offset(4)]
    id_: i32,
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_char_ {
    pub fn copy_from(a: Ptr<test_allocator_char_>) -> Self {
        let __this: Value<test_allocator_char_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_char_> = __this.as_pointer();
        assert!(
            (({ a.with(|__s| __s.data_) } != { destructed_value_0.with(|rc| *rc.borrow()) })
                && ({ a.with(|__s| __s.id_) } != { destructed_value_0.with(|rc| *rc.borrow()) }))
                && (!(Ptr::<i8>::from_string_literal(b"copying from destroyed allocator"))
                    .is_null())
        );
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| {
                __v.prefix_inc();
            });
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| {
                __v.prefix_inc();
            });
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(a: Ptr<test_allocator_char_>) -> Self {
        let __this: Value<test_allocator_char_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_char_> = __this.as_pointer();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| {
                __v.prefix_inc();
            });
            field!(this.with(|__s| __s.stats_.clone()), moved).with_mut(|__v| {
                __v.prefix_inc();
            });
        }
        assert!(
            (({ a.with(|__s| __s.data_) } != { destructed_value_0.with(|rc| *rc.borrow()) })
                && ({ a.with(|__s| __s.id_) } != { destructed_value_0.with(|rc| *rc.borrow()) }))
                && (!(Ptr::<i8>::from_string_literal(b"moving from destroyed allocator"))
                    .is_null())
        );
        field!(a, id_).write(moved_value_1.with(|rc| *rc.borrow()));
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for test_allocator_char_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_char_> = Rc::new(RefCell::new(test_allocator_char_ {
            data_: self.data_.clone(),
            id_: self.id_.clone(),
            stats_: self.stats_.clone(),
        }));
        test_allocator_char_::copy_from(__src.as_pointer())
    }
}
impl CppMove for test_allocator_char_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
impl Default for test_allocator_char_ {
    fn default() -> Self {
        {
            test_allocator_char_ {
                data_: 0,
                id_: 0,
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Destructor)]
#[byte_size(16)]
pub struct test_allocator_wchar_t_ {
    #[offset(0)]
    data_: i32,
    #[offset(4)]
    id_: i32,
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_wchar_t_ {
    pub fn copy_from(a: Ptr<test_allocator_wchar_t_>) -> Self {
        let __this: Value<test_allocator_wchar_t_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_wchar_t_> = __this.as_pointer();
        assert!(
            (({ a.with(|__s| __s.data_) } != { destructed_value_0.with(|rc| *rc.borrow()) })
                && ({ a.with(|__s| __s.id_) } != { destructed_value_0.with(|rc| *rc.borrow()) }))
                && (!(Ptr::<i8>::from_string_literal(b"copying from destroyed allocator"))
                    .is_null())
        );
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| {
                __v.prefix_inc();
            });
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| {
                __v.prefix_inc();
            });
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(a: Ptr<test_allocator_wchar_t_>) -> Self {
        let __this: Value<test_allocator_wchar_t_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_wchar_t_> = __this.as_pointer();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| {
                __v.prefix_inc();
            });
            field!(this.with(|__s| __s.stats_.clone()), moved).with_mut(|__v| {
                __v.prefix_inc();
            });
        }
        assert!(
            (({ a.with(|__s| __s.data_) } != { destructed_value_0.with(|rc| *rc.borrow()) })
                && ({ a.with(|__s| __s.id_) } != { destructed_value_0.with(|rc| *rc.borrow()) }))
                && (!(Ptr::<i8>::from_string_literal(b"moving from destroyed allocator"))
                    .is_null())
        );
        field!(a, id_).write(moved_value_1.with(|rc| *rc.borrow()));
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for test_allocator_wchar_t_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_wchar_t_> =
            Rc::new(RefCell::new(test_allocator_wchar_t_ {
                data_: self.data_.clone(),
                id_: self.id_.clone(),
                stats_: self.stats_.clone(),
            }));
        test_allocator_wchar_t_::copy_from(__src.as_pointer())
    }
}
impl CppMove for test_allocator_wchar_t_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
impl Default for test_allocator_wchar_t_ {
    fn default() -> Self {
        {
            test_allocator_wchar_t_ {
                data_: 0,
                id_: 0,
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Ctor_Tag {}
impl CppMove for Ctor_Tag {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Record, ByteRepr, FnPtrArg, CppFrom, CopyAssign, MoveAssign, Default)]
#[cpp_from(Ctor_Tag, new_1, value)]
#[byte_size(1)]
pub struct Tag_X {}
impl Tag_X {
    pub fn new_1(mut _a0: Ctor_Tag) -> Self {
        Self {}
    }
    pub fn new_2(mut _a0: Ctor_Tag, _a1: Ptr<Tag_X>) -> Self {
        Self {}
    }
    pub fn new_3(mut _a0: Ctor_Tag, _a1: Ptr<Tag_X>) -> Self {
        Self {}
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
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"abcdefghijk\0")));
        assert!(
            ({
                let ___e: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"cd((e)fg)hi") },
                        None,
                    )),
                );
                regex_search_4(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___e.as_pointer(),
                    None,
                )
            })
        );
        let fmt: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(
                b"prefix: $`, match: $&, suffix: $\', m[1]: $1, m[2]: $2",
            )
            .to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        let mut out: Vec<i8> = ({ (*m.borrow()).format(fmt.as_pointer(), None) });
        assert!(
            Ptr::<i8>::from_string_literal(
                b"prefix: ab, match: cdefghi, suffix: jk, m[1]: efg, m[2]: e"
            )
            .with_c_str(|__s| out[..out.len().saturating_sub(1)] == *__s)
        );
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"abcdefghijk\0")));
        assert!(
            ({
                let ___e: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"cd((e)fg)hi") },
                        None,
                    )),
                );
                regex_search_4(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___e.as_pointer(),
                    None,
                )
            })
        );
        let fmt: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(
                b"prefix: $`, match: $&, suffix: $\', m[1]: $1, m[2]: $2",
            )
            .to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        let mut out: Vec<i8> = ({
            (*m.borrow()).format(fmt.as_pointer(), Some(format_sed_5.with(|rc| *rc.borrow())))
        });
        assert!(
            Ptr::<i8>::from_string_literal(
                b"prefix: $`, match: $cdefghi, suffix: $\', m[1]: $1, m[2]: $2"
            )
            .with_c_str(|__s| out[..out.len().saturating_sub(1)] == *__s)
        );
    }
    {
        let m : Value<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_char_ptr__std_allocator_std_sub_match_const_char_ptr___  >::default() )) ;
        let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"abcdefghijk\0")));
        assert!(
            ({
                let ___e: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(
                    RefCell::new(std_basic_regex_char__std_regex_traits_char__::new_1(
                        { Ptr::<i8>::from_string_literal(b"cd((e)fg)hi") },
                        None,
                    )),
                );
                regex_search_4(
                    (s.as_pointer() as Ptr<i8>),
                    m.as_pointer(),
                    ___e.as_pointer(),
                    None,
                )
            })
        );
        let fmt: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes =
                Ptr::<i8>::from_string_literal(b"match: &, m[1]: \\1, m[2]: \\2").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        let mut out: Vec<i8> = ({
            (*m.borrow()).format(fmt.as_pointer(), Some(format_sed_5.with(|rc| *rc.borrow())))
        });
        assert!(
            Ptr::<i8>::from_string_literal(b"match: cdefghi, m[1]: efg, m[2]: e")
                .with_c_str(|__s| out[..out.len().saturating_sub(1)] == *__s)
        );
    };
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
            104 as i32, 105 as i32, 106 as i32, 107 as i32, 0 as i32,
        ])));
        assert!(
            ({
                let ___e: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                99 as i32, 100 as i32, 40 as i32, 40 as i32, 101 as i32, 41 as i32,
                                102 as i32, 103 as i32, 41 as i32, 104 as i32, 105 as i32,
                                0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_search_6(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___e.as_pointer(),
                    None,
                )
            })
        );
        let fmt: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[
                112 as i32, 114 as i32, 101 as i32, 102 as i32, 105 as i32, 120 as i32, 58 as i32,
                32 as i32, 36 as i32, 96 as i32, 44 as i32, 32 as i32, 109 as i32, 97 as i32,
                116 as i32, 99 as i32, 104 as i32, 58 as i32, 32 as i32, 36 as i32, 38 as i32,
                44 as i32, 32 as i32, 115 as i32, 117 as i32, 102 as i32, 102 as i32, 105 as i32,
                120 as i32, 58 as i32, 32 as i32, 36 as i32, 39 as i32, 44 as i32, 32 as i32,
                109 as i32, 91 as i32, 49 as i32, 93 as i32, 58 as i32, 32 as i32, 36 as i32,
                49 as i32, 44 as i32, 32 as i32, 109 as i32, 91 as i32, 50 as i32, 93 as i32,
                58 as i32, 32 as i32, 36 as i32, 50 as i32, 0 as i32,
            ])
            .to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        let mut out: Vec<i32> = ({ (*m.borrow()).format(fmt.as_pointer(), None) });
        assert!(
            Ptr::<i32>::from_string_literal(&[
                112 as i32, 114 as i32, 101 as i32, 102 as i32, 105 as i32, 120 as i32, 58 as i32,
                32 as i32, 97 as i32, 98 as i32, 44 as i32, 32 as i32, 109 as i32, 97 as i32,
                116 as i32, 99 as i32, 104 as i32, 58 as i32, 32 as i32, 99 as i32, 100 as i32,
                101 as i32, 102 as i32, 103 as i32, 104 as i32, 105 as i32, 44 as i32, 32 as i32,
                115 as i32, 117 as i32, 102 as i32, 102 as i32, 105 as i32, 120 as i32, 58 as i32,
                32 as i32, 106 as i32, 107 as i32, 44 as i32, 32 as i32, 109 as i32, 91 as i32,
                49 as i32, 93 as i32, 58 as i32, 32 as i32, 101 as i32, 102 as i32, 103 as i32,
                44 as i32, 32 as i32, 109 as i32, 91 as i32, 50 as i32, 93 as i32, 58 as i32,
                32 as i32, 101 as i32, 0 as i32,
            ])
            .with_c_str(|__s| out[..out.len().saturating_sub(1)] == *__s)
        );
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
            104 as i32, 105 as i32, 106 as i32, 107 as i32, 0 as i32,
        ])));
        assert!(
            ({
                let ___e: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                99 as i32, 100 as i32, 40 as i32, 40 as i32, 101 as i32, 41 as i32,
                                102 as i32, 103 as i32, 41 as i32, 104 as i32, 105 as i32,
                                0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_search_6(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___e.as_pointer(),
                    None,
                )
            })
        );
        let fmt: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[
                112 as i32, 114 as i32, 101 as i32, 102 as i32, 105 as i32, 120 as i32, 58 as i32,
                32 as i32, 36 as i32, 96 as i32, 44 as i32, 32 as i32, 109 as i32, 97 as i32,
                116 as i32, 99 as i32, 104 as i32, 58 as i32, 32 as i32, 36 as i32, 38 as i32,
                44 as i32, 32 as i32, 115 as i32, 117 as i32, 102 as i32, 102 as i32, 105 as i32,
                120 as i32, 58 as i32, 32 as i32, 36 as i32, 39 as i32, 44 as i32, 32 as i32,
                109 as i32, 91 as i32, 49 as i32, 93 as i32, 58 as i32, 32 as i32, 36 as i32,
                49 as i32, 44 as i32, 32 as i32, 109 as i32, 91 as i32, 50 as i32, 93 as i32,
                58 as i32, 32 as i32, 36 as i32, 50 as i32, 0 as i32,
            ])
            .to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        let mut out: Vec<i32> = ({
            (*m.borrow()).format(fmt.as_pointer(), Some(format_sed_5.with(|rc| *rc.borrow())))
        });
        assert!(
            Ptr::<i32>::from_string_literal(&[
                112 as i32, 114 as i32, 101 as i32, 102 as i32, 105 as i32, 120 as i32, 58 as i32,
                32 as i32, 36 as i32, 96 as i32, 44 as i32, 32 as i32, 109 as i32, 97 as i32,
                116 as i32, 99 as i32, 104 as i32, 58 as i32, 32 as i32, 36 as i32, 99 as i32,
                100 as i32, 101 as i32, 102 as i32, 103 as i32, 104 as i32, 105 as i32, 44 as i32,
                32 as i32, 115 as i32, 117 as i32, 102 as i32, 102 as i32, 105 as i32, 120 as i32,
                58 as i32, 32 as i32, 36 as i32, 39 as i32, 44 as i32, 32 as i32, 109 as i32,
                91 as i32, 49 as i32, 93 as i32, 58 as i32, 32 as i32, 36 as i32, 49 as i32,
                44 as i32, 32 as i32, 109 as i32, 91 as i32, 50 as i32, 93 as i32, 58 as i32,
                32 as i32, 36 as i32, 50 as i32, 0 as i32,
            ])
            .with_c_str(|__s| out[..out.len().saturating_sub(1)] == *__s)
        );
    }
    {
        let m : Value<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___ > = Rc::new(RefCell::new(<std_match_results_const_wchar_t_ptr__std_allocator_std_sub_match_const_wchar_t_ptr___  >::default() )) ;
        let s: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
            97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
            104 as i32, 105 as i32, 106 as i32, 107 as i32, 0 as i32,
        ])));
        assert!(
            ({
                let ___e: Value<std_basic_regex_wchar_t__std_regex_traits_wchar_t__> = Rc::new(
                    RefCell::new(std_basic_regex_wchar_t__std_regex_traits_wchar_t__::new_1(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                99 as i32, 100 as i32, 40 as i32, 40 as i32, 101 as i32, 41 as i32,
                                102 as i32, 103 as i32, 41 as i32, 104 as i32, 105 as i32,
                                0 as i32,
                            ])
                        },
                        None,
                    )),
                );
                regex_search_6(
                    (s.as_pointer() as Ptr<i32>),
                    m.as_pointer(),
                    ___e.as_pointer(),
                    None,
                )
            })
        );
        let fmt: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i32>::from_string_literal(&[
                109 as i32, 97 as i32, 116 as i32, 99 as i32, 104 as i32, 58 as i32, 32 as i32,
                38 as i32, 44 as i32, 32 as i32, 109 as i32, 91 as i32, 49 as i32, 93 as i32,
                58 as i32, 32 as i32, 92 as i32, 49 as i32, 44 as i32, 32 as i32, 109 as i32,
                91 as i32, 50 as i32, 93 as i32, 58 as i32, 32 as i32, 92 as i32, 50 as i32,
                0 as i32,
            ])
            .to_c_bytes();
            __bytes.push(0_i32);
            __bytes
        }));
        let mut out: Vec<i32> = ({
            (*m.borrow()).format(fmt.as_pointer(), Some(format_sed_5.with(|rc| *rc.borrow())))
        });
        assert!(
            Ptr::<i32>::from_string_literal(&[
                109 as i32, 97 as i32, 116 as i32, 99 as i32, 104 as i32, 58 as i32, 32 as i32,
                99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32, 104 as i32, 105 as i32,
                44 as i32, 32 as i32, 109 as i32, 91 as i32, 49 as i32, 93 as i32, 58 as i32,
                32 as i32, 101 as i32, 102 as i32, 103 as i32, 44 as i32, 32 as i32, 109 as i32,
                91 as i32, 50 as i32, 93 as i32, 58 as i32, 32 as i32, 101 as i32, 0 as i32,
            ])
            .with_c_str(|__s| out[..out.len().saturating_sub(1)] == *__s)
        );
    }
    return 0;
}
pub trait Tag_XImpl {
    fn copy_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X>;
    fn move_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X>;
}
impl Tag_XImpl for Ptr<Tag_X> {
    fn copy_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X> {
        return (*self).clone();
    }
    fn move_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X> {
        return (*self).clone();
    }
}
pub trait test_allocator_char_Impl {
    fn destructor(&self);
    fn address_11(&self, x: Ptr<i8>) -> Ptr<i8> {
        unimplemented!()
    }
    fn address_12(&self, x: Ptr<i8>) -> Ptr<i8> {
        unimplemented!()
    }
    fn allocate(&self, n: u32, _a1: Option<AnyPtr>) -> Ptr<i8>;
    fn deallocate(&self, p: Ptr<i8>, s: u32);
    fn max_size(&self) -> u32;
    fn destroy(&self, mut p: Ptr<i8>) {
        unimplemented!()
    }
    fn get_data(&self) -> i32 {
        unimplemented!()
    }
    fn get_id(&self) -> i32 {
        unimplemented!()
    }
}
impl test_allocator_char_Impl for Ptr<test_allocator_char_> {
    fn destructor(&self) {
        assert!(((*self).with(|__s| __s.data_) != destructed_value_0.with(|rc| *rc.borrow())));
        assert!(((*self).with(|__s| __s.id_) != destructed_value_0.with(|rc| *rc.borrow())));
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| {
                __v.prefix_dec();
            });
        }
        field!((*self), data_).write(destructed_value_0.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_0.with(|rc| *rc.borrow()));
    }
    fn allocate(&self, mut n: u32, _a1: Option<AnyPtr>) -> Ptr<i8> {
        let mut _a1: AnyPtr = _a1.unwrap_or_else(|| AnyPtr::default());
        assert!(((*self).with(|__s| __s.data_) != destructed_value_0.with(|rc| *rc.borrow())));
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            if ({
                (*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.time_to_throw)
            } >= {
                (*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.throw_after)
            }) {
                std::process::abort();
            }
            field!((*self).with(|__s| __s.stats_.clone()), time_to_throw).with_mut(|__v| {
                __v.prefix_inc();
            });
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count).with_mut(|__v| {
                __v.prefix_inc();
            });
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                (((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size) as u32)
                    .wrapping_add(n)) as i32
            });
        }
        return ({ <std_allocator_char_>::default().allocate((n as usize)) });
    }
    fn deallocate(&self, mut p: Ptr<i8>, mut s: u32) {
        assert!(((*self).with(|__s| __s.data_) != destructed_value_0.with(|rc| *rc.borrow())));
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count).with_mut(|__v| {
                __v.prefix_dec();
            });
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                (((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size) as u32)
                    .wrapping_sub(s)) as i32
            });
        }
        ({ <std_allocator_char_>::default().deallocate((p).clone(), (s as usize)) });
    }
    fn max_size(&self) -> u32 {
        return ((((((2147483647_u32).wrapping_mul((2_u32 as u32)) as u32)
            .wrapping_add((1_u32 as u32))) as usize)
            .wrapping_div((::std::mem::size_of::<i8>() as usize))) as u32);
    }
}
pub trait test_allocator_statisticsImpl {
    fn clear(&self);
    fn clear_ctor_counters(&self);
}
impl test_allocator_statisticsImpl for Ptr<test_allocator_statistics> {
    fn clear(&self) {
        assert!(
            ((*self).with(|__s| __s.count) == 0)
                && (!(Ptr::<i8>::from_string_literal(b"clearing leaking allocator data?"))
                    .is_null())
        );
        field!((*self), count).write(0);
        field!((*self), time_to_throw).write(0);
        field!((*self), alloc_count).write(0);
        field!((*self), allocated_size).write(0);
        field!((*self), construct_count).write(0);
        field!((*self), destroy_count).write(0);
        field!((*self), throw_after).write(2147483647);
        ({ test_allocator_statisticsImpl::clear_ctor_counters(self) });
    }
    fn clear_ctor_counters(&self) {
        field!((*self), copied).write(0);
        field!((*self), moved).write(0);
        field!((*self), converted).write(0);
    }
}
pub trait test_allocator_void_Impl {
    fn destructor(&self);
    fn get_id(&self) -> i32;
    fn get_data(&self) -> i32;
    fn copy_assign(&self, _a0: Ptr<test_allocator_void_>) -> Ptr<test_allocator_void_>;
}
impl test_allocator_void_Impl for Ptr<test_allocator_void_> {
    fn destructor(&self) {
        field!((*self), data_).write(destructed_value_0.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_0.with(|rc| *rc.borrow()));
    }
    fn get_id(&self) -> i32 {
        return (*self).with(|__s| __s.id_);
    }
    fn get_data(&self) -> i32 {
        return (*self).with(|__s| __s.data_);
    }
    fn copy_assign(&self, _a0: Ptr<test_allocator_void_>) -> Ptr<test_allocator_void_> {
        field!((*self), data_).write({ _a0.with(|__s| __s.data_) });
        field!((*self), id_).write({ _a0.with(|__s| __s.id_) });
        field!((*self), stats_).write({ _a0.with(|__s| __s.stats_.clone()) });
        return (*self).clone();
    }
}
pub trait test_allocator_wchar_t_Impl {
    fn destructor(&self);
    fn address_11(&self, x: Ptr<i32>) -> Ptr<i32> {
        unimplemented!()
    }
    fn address_12(&self, x: Ptr<i32>) -> Ptr<i32> {
        unimplemented!()
    }
    fn allocate(&self, n: u32, _a1: Option<AnyPtr>) -> Ptr<i32>;
    fn deallocate(&self, p: Ptr<i32>, s: u32);
    fn max_size(&self) -> u32;
    fn destroy(&self, mut p: Ptr<i32>) {
        unimplemented!()
    }
    fn get_data(&self) -> i32 {
        unimplemented!()
    }
    fn get_id(&self) -> i32 {
        unimplemented!()
    }
}
impl test_allocator_wchar_t_Impl for Ptr<test_allocator_wchar_t_> {
    fn destructor(&self) {
        assert!(((*self).with(|__s| __s.data_) != destructed_value_0.with(|rc| *rc.borrow())));
        assert!(((*self).with(|__s| __s.id_) != destructed_value_0.with(|rc| *rc.borrow())));
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| {
                __v.prefix_dec();
            });
        }
        field!((*self), data_).write(destructed_value_0.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_0.with(|rc| *rc.borrow()));
    }
    fn allocate(&self, mut n: u32, _a1: Option<AnyPtr>) -> Ptr<i32> {
        let mut _a1: AnyPtr = _a1.unwrap_or_else(|| AnyPtr::default());
        assert!(((*self).with(|__s| __s.data_) != destructed_value_0.with(|rc| *rc.borrow())));
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            if ({
                (*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.time_to_throw)
            } >= {
                (*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.throw_after)
            }) {
                std::process::abort();
            }
            field!((*self).with(|__s| __s.stats_.clone()), time_to_throw).with_mut(|__v| {
                __v.prefix_inc();
            });
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count).with_mut(|__v| {
                __v.prefix_inc();
            });
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                (((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size) as u32)
                    .wrapping_add(n)) as i32
            });
        }
        return ({ <std_allocator_wchar_t_>::default().allocate((n as usize)) });
    }
    fn deallocate(&self, mut p: Ptr<i32>, mut s: u32) {
        assert!(((*self).with(|__s| __s.data_) != destructed_value_0.with(|rc| *rc.borrow())));
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count).with_mut(|__v| {
                __v.prefix_dec();
            });
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                (((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size) as u32)
                    .wrapping_sub(s)) as i32
            });
        }
        ({ <std_allocator_wchar_t_>::default().deallocate((p).clone(), (s as usize)) });
    }
    fn max_size(&self) -> u32 {
        return ((((((2147483647_u32).wrapping_mul((2_u32 as u32)) as u32)
            .wrapping_add((1_u32 as u32))) as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = destructed_value_0.with(|_| ());
    let _ = moved_value_1.with(|_| ());
}
