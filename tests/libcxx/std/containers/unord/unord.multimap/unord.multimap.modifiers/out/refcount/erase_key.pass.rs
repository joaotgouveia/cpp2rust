extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn CheckConsecutiveKeys_0(pos: (), end: (), mut key: (), values: Ptr<()>) {
    let pos: Value<()> = Rc::new(RefCell::new(pos));
    let end: Value<()> = Rc::new(RefCell::new(end));
    'loop_: while !(unimplemented!()) {
        assert!((*pos.borrow()) != (*end.borrow()));
        assert!(((*(*(*pos.borrow()).iter_deref().upgrade().deref()).0.borrow()) == key));
        assert!(unimplemented!() != unimplemented!());
        unimplemented!();
        (*pos.borrow_mut()).iter_inc();
    }
    assert!(
        ((*pos.borrow()) == (*end.borrow()))
            || ((*(*(*pos.borrow()).iter_deref().upgrade().deref()).0.borrow()) != key)
    );
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> usize {
        assert!((alloc_count_2.with(|rc| *rc.borrow()) >= dealloc_count_3.with(|rc| *rc.borrow())));
        return ((alloc_count_2.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_3.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_4.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_1.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_2.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_3.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
thread_local!(
    pub static outstanding_bytes_1: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_2: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_3: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_4: Value<bool> = Rc::new(RefCell::new(false));
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
pub fn only_deletions_5(whole: Ptr<()>, part: Ptr<()>) -> bool {
    let w: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    'loop_: while ((*w.borrow()) != unimplemented!()) && ((*p.borrow()) != unimplemented!()) {
        if unimplemented!() {
            (*p.borrow_mut()).iter_post_inc();
        }
        (*w.borrow_mut()).iter_post_inc();
    }
    return (*p.borrow()) == unimplemented!();
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
        assert!((unimplemented!() == 0_()));
        assert!((unimplemented!() == 6_()));
        let eq: Value<(Value<()>, Value<()>)> = Rc::new(RefCell::new({
            let __a0 = unimplemented!();
            (
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a0.0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a0.1.as_pointer(),
                ))),
            )
        }));
        assert!((unimplemented!() == 2_isize));
        let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        ({
            let _pos: () = <()>::from(unimplemented!());
            let _end: () = <()>::from(unimplemented!());
            CheckConsecutiveKeys_0(_pos, _end, (1 as ()), s.as_pointer())
        });
        {
            let __a1 = unimplemented!();
            eq.as_pointer().write((
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.1.as_pointer(),
                ))),
            ))
        };
        assert!((unimplemented!() == 2_isize));
        unimplemented!();
        unimplemented!();
        ({
            let _pos: () = <()>::from(unimplemented!());
            let _end: () = <()>::from(unimplemented!());
            CheckConsecutiveKeys_0(_pos, _end, (2 as ()), s.as_pointer())
        });
        {
            let __a1 = unimplemented!();
            eq.as_pointer().write((
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.1.as_pointer(),
                ))),
            ))
        };
        assert!((unimplemented!() == 1_isize));
        let k: Value<()> = Rc::new(RefCell::new((*(*eq.borrow()).0.borrow()).clone()));
        assert!(((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3));
        assert!(Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
            (*(*(*k.borrow()).iter_deref().upgrade().deref()).1.borrow())[..(*(*(*k.borrow())
                .iter_deref()
                .upgrade()
                .deref())
            .1
            .borrow())
            .len()
            .saturating_sub(1)]
                == *__s
        }));
        {
            let __a1 = unimplemented!();
            eq.as_pointer().write((
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.1.as_pointer(),
                ))),
            ))
        };
        assert!((unimplemented!() == 1_isize));
        (*k.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
        assert!(((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 4));
        assert!(Ptr::<i8>::from_string_literal(b"four").with_c_str(|__s| {
            (*(*(*k.borrow()).iter_deref().upgrade().deref()).1.borrow())[..(*(*(*k.borrow())
                .iter_deref()
                .upgrade()
                .deref())
            .1
            .borrow())
            .len()
            .saturating_sub(1)]
                == *__s
        }));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!((unimplemented!() == 2_()));
        assert!((unimplemented!() == 4_()));
        {
            let __a1 = unimplemented!();
            eq.as_pointer().write((
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.1.as_pointer(),
                ))),
            ))
        };
        assert!((unimplemented!() == 2_isize));
        unimplemented!();
        unimplemented!();
        ({
            let _pos: () = <()>::from(unimplemented!());
            let _end: () = <()>::from(unimplemented!());
            CheckConsecutiveKeys_0(_pos, _end, (1 as ()), s.as_pointer())
        });
        {
            let __a1 = unimplemented!();
            eq.as_pointer().write((
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.1.as_pointer(),
                ))),
            ))
        };
        assert!((unimplemented!() == 1_isize));
        (*k.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
        assert!(((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3));
        assert!(Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
            (*(*(*k.borrow()).iter_deref().upgrade().deref()).1.borrow())[..(*(*(*k.borrow())
                .iter_deref()
                .upgrade()
                .deref())
            .1
            .borrow())
            .len()
            .saturating_sub(1)]
                == *__s
        }));
        {
            let __a1 = unimplemented!();
            eq.as_pointer().write((
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.1.as_pointer(),
                ))),
            ))
        };
        assert!((unimplemented!() == 1_isize));
        (*k.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
        assert!(((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 4));
        assert!(Ptr::<i8>::from_string_literal(b"four").with_c_str(|__s| {
            (*(*(*k.borrow()).iter_deref().upgrade().deref()).1.borrow())[..(*(*(*k.borrow())
                .iter_deref()
                .upgrade()
                .deref())
            .1
            .borrow())
            .len()
            .saturating_sub(1)]
                == *__s
        }));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!((unimplemented!() == 0_()));
        assert!((unimplemented!() == 4_()));
        {
            let __a1 = unimplemented!();
            eq.as_pointer().write((
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.1.as_pointer(),
                ))),
            ))
        };
        assert!((unimplemented!() == 2_isize));
        unimplemented!();
        unimplemented!();
        ({
            let _pos: () = <()>::from(unimplemented!());
            let _end: () = <()>::from(unimplemented!());
            CheckConsecutiveKeys_0(_pos, _end, (1 as ()), s.as_pointer())
        });
        {
            let __a1 = unimplemented!();
            eq.as_pointer().write((
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.1.as_pointer(),
                ))),
            ))
        };
        assert!((unimplemented!() == 1_isize));
        (*k.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
        assert!(((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3));
        assert!(Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
            (*(*(*k.borrow()).iter_deref().upgrade().deref()).1.borrow())[..(*(*(*k.borrow())
                .iter_deref()
                .upgrade()
                .deref())
            .1
            .borrow())
            .len()
            .saturating_sub(1)]
                == *__s
        }));
        {
            let __a1 = unimplemented!();
            eq.as_pointer().write((
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.1.as_pointer(),
                ))),
            ))
        };
        assert!((unimplemented!() == 1_isize));
        (*k.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
        assert!(((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 4));
        assert!(Ptr::<i8>::from_string_literal(b"four").with_c_str(|__s| {
            (*(*(*k.borrow()).iter_deref().upgrade().deref()).1.borrow())[..(*(*(*k.borrow())
                .iter_deref()
                .upgrade()
                .deref())
            .1
            .borrow())
            .len()
            .saturating_sub(1)]
                == *__s
        }));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!((unimplemented!() == 1_()));
        assert!((unimplemented!() == 3_()));
        {
            let __a1 = unimplemented!();
            eq.as_pointer().write((
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.1.as_pointer(),
                ))),
            ))
        };
        assert!((unimplemented!() == 2_isize));
        unimplemented!();
        unimplemented!();
        ({
            let _pos: () = <()>::from(unimplemented!());
            let _end: () = <()>::from(unimplemented!());
            CheckConsecutiveKeys_0(_pos, _end, (1 as ()), s.as_pointer())
        });
        {
            let __a1 = unimplemented!();
            eq.as_pointer().write((
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.1.as_pointer(),
                ))),
            ))
        };
        assert!((unimplemented!() == 1_isize));
        (*k.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
        assert!(((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3));
        assert!(Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
            (*(*(*k.borrow()).iter_deref().upgrade().deref()).1.borrow())[..(*(*(*k.borrow())
                .iter_deref()
                .upgrade()
                .deref())
            .1
            .borrow())
            .len()
            .saturating_sub(1)]
                == *__s
        }));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!((unimplemented!() == 0_()));
        assert!((unimplemented!() == 3_()));
        {
            let __a1 = unimplemented!();
            eq.as_pointer().write((
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.1.as_pointer(),
                ))),
            ))
        };
        assert!((unimplemented!() == 2_isize));
        unimplemented!();
        unimplemented!();
        ({
            let _pos: () = <()>::from(unimplemented!());
            let _end: () = <()>::from(unimplemented!());
            CheckConsecutiveKeys_0(_pos, _end, (1 as ()), s.as_pointer())
        });
        {
            let __a1 = unimplemented!();
            eq.as_pointer().write((
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.1.as_pointer(),
                ))),
            ))
        };
        assert!((unimplemented!() == 1_isize));
        (*k.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
        assert!(((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3));
        assert!(Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
            (*(*(*k.borrow()).iter_deref().upgrade().deref()).1.borrow())[..(*(*(*k.borrow())
                .iter_deref()
                .upgrade()
                .deref())
            .1
            .borrow())
            .len()
            .saturating_sub(1)]
                == *__s
        }));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!((unimplemented!() == 2_()));
        assert!((unimplemented!() == 1_()));
        {
            let __a1 = unimplemented!();
            eq.as_pointer().write((
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.1.as_pointer(),
                ))),
            ))
        };
        assert!((unimplemented!() == 1_isize));
        (*k.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
        assert!(((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3));
        assert!(Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
            (*(*(*k.borrow()).iter_deref().upgrade().deref()).1.borrow())[..(*(*(*k.borrow())
                .iter_deref()
                .upgrade()
                .deref())
            .1
            .borrow())
            .len()
            .saturating_sub(1)]
                == *__s
        }));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!((unimplemented!() == 0_()));
        assert!((unimplemented!() == 1_()));
        {
            let __a1 = unimplemented!();
            eq.as_pointer().write((
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.1.as_pointer(),
                ))),
            ))
        };
        assert!((unimplemented!() == 1_isize));
        (*k.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
        assert!(((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3));
        assert!(Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
            (*(*(*k.borrow()).iter_deref().upgrade().deref()).1.borrow())[..(*(*(*k.borrow())
                .iter_deref()
                .upgrade()
                .deref())
            .1
            .borrow())
            .len()
            .saturating_sub(1)]
                == *__s
        }));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!((unimplemented!() == 1_()));
        assert!((unimplemented!() == 0_()));
        {
            let __a1 = unimplemented!();
            eq.as_pointer().write((
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.1.as_pointer(),
                ))),
            ))
        };
        assert!((unimplemented!() == 0_isize));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!((unimplemented!() == 0_()));
        assert!((unimplemented!() == 0_()));
        {
            let __a1 = unimplemented!();
            eq.as_pointer().write((
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.1.as_pointer(),
                ))),
            ))
        };
        assert!((unimplemented!() == 0_isize));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
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
        assert!((unimplemented!() == 0_()));
        assert!((unimplemented!() == 6_()));
        let eq: Value<(Value<()>, Value<()>)> = Rc::new(RefCell::new({
            let __a0 = unimplemented!();
            (
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a0.0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a0.1.as_pointer(),
                ))),
            )
        }));
        assert!((unimplemented!() == 2_isize));
        let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        ({
            let _pos: () = <()>::from(unimplemented!());
            let _end: () = <()>::from(unimplemented!());
            CheckConsecutiveKeys_0(_pos, _end, (1 as ()), s.as_pointer())
        });
        {
            let __a1 = unimplemented!();
            eq.as_pointer().write((
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.1.as_pointer(),
                ))),
            ))
        };
        assert!((unimplemented!() == 2_isize));
        unimplemented!();
        unimplemented!();
        ({
            let _pos: () = <()>::from(unimplemented!());
            let _end: () = <()>::from(unimplemented!());
            CheckConsecutiveKeys_0(_pos, _end, (2 as ()), s.as_pointer())
        });
        {
            let __a1 = unimplemented!();
            eq.as_pointer().write((
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.1.as_pointer(),
                ))),
            ))
        };
        assert!((unimplemented!() == 1_isize));
        let k: Value<()> = Rc::new(RefCell::new((*(*eq.borrow()).0.borrow()).clone()));
        assert!(((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3));
        assert!(Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
            (*(*(*k.borrow()).iter_deref().upgrade().deref()).1.borrow())[..(*(*(*k.borrow())
                .iter_deref()
                .upgrade()
                .deref())
            .1
            .borrow())
            .len()
            .saturating_sub(1)]
                == *__s
        }));
        {
            let __a1 = unimplemented!();
            eq.as_pointer().write((
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.1.as_pointer(),
                ))),
            ))
        };
        assert!((unimplemented!() == 1_isize));
        (*k.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
        assert!(((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 4));
        assert!(Ptr::<i8>::from_string_literal(b"four").with_c_str(|__s| {
            (*(*(*k.borrow()).iter_deref().upgrade().deref()).1.borrow())[..(*(*(*k.borrow())
                .iter_deref()
                .upgrade()
                .deref())
            .1
            .borrow())
            .len()
            .saturating_sub(1)]
                == *__s
        }));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!((unimplemented!() == 2_()));
        assert!((unimplemented!() == 4_()));
        {
            let __a1 = unimplemented!();
            eq.as_pointer().write((
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.1.as_pointer(),
                ))),
            ))
        };
        assert!((unimplemented!() == 2_isize));
        unimplemented!();
        unimplemented!();
        ({
            let _pos: () = <()>::from(unimplemented!());
            let _end: () = <()>::from(unimplemented!());
            CheckConsecutiveKeys_0(_pos, _end, (1 as ()), s.as_pointer())
        });
        {
            let __a1 = unimplemented!();
            eq.as_pointer().write((
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.1.as_pointer(),
                ))),
            ))
        };
        assert!((unimplemented!() == 1_isize));
        (*k.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
        assert!(((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3));
        assert!(Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
            (*(*(*k.borrow()).iter_deref().upgrade().deref()).1.borrow())[..(*(*(*k.borrow())
                .iter_deref()
                .upgrade()
                .deref())
            .1
            .borrow())
            .len()
            .saturating_sub(1)]
                == *__s
        }));
        {
            let __a1 = unimplemented!();
            eq.as_pointer().write((
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.1.as_pointer(),
                ))),
            ))
        };
        assert!((unimplemented!() == 1_isize));
        (*k.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
        assert!(((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 4));
        assert!(Ptr::<i8>::from_string_literal(b"four").with_c_str(|__s| {
            (*(*(*k.borrow()).iter_deref().upgrade().deref()).1.borrow())[..(*(*(*k.borrow())
                .iter_deref()
                .upgrade()
                .deref())
            .1
            .borrow())
            .len()
            .saturating_sub(1)]
                == *__s
        }));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!((unimplemented!() == 0_()));
        assert!((unimplemented!() == 4_()));
        {
            let __a1 = unimplemented!();
            eq.as_pointer().write((
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.1.as_pointer(),
                ))),
            ))
        };
        assert!((unimplemented!() == 2_isize));
        unimplemented!();
        unimplemented!();
        ({
            let _pos: () = <()>::from(unimplemented!());
            let _end: () = <()>::from(unimplemented!());
            CheckConsecutiveKeys_0(_pos, _end, (1 as ()), s.as_pointer())
        });
        {
            let __a1 = unimplemented!();
            eq.as_pointer().write((
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.1.as_pointer(),
                ))),
            ))
        };
        assert!((unimplemented!() == 1_isize));
        (*k.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
        assert!(((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3));
        assert!(Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
            (*(*(*k.borrow()).iter_deref().upgrade().deref()).1.borrow())[..(*(*(*k.borrow())
                .iter_deref()
                .upgrade()
                .deref())
            .1
            .borrow())
            .len()
            .saturating_sub(1)]
                == *__s
        }));
        {
            let __a1 = unimplemented!();
            eq.as_pointer().write((
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.1.as_pointer(),
                ))),
            ))
        };
        assert!((unimplemented!() == 1_isize));
        (*k.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
        assert!(((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 4));
        assert!(Ptr::<i8>::from_string_literal(b"four").with_c_str(|__s| {
            (*(*(*k.borrow()).iter_deref().upgrade().deref()).1.borrow())[..(*(*(*k.borrow())
                .iter_deref()
                .upgrade()
                .deref())
            .1
            .borrow())
            .len()
            .saturating_sub(1)]
                == *__s
        }));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!((unimplemented!() == 1_()));
        assert!((unimplemented!() == 3_()));
        {
            let __a1 = unimplemented!();
            eq.as_pointer().write((
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.1.as_pointer(),
                ))),
            ))
        };
        assert!((unimplemented!() == 2_isize));
        unimplemented!();
        unimplemented!();
        ({
            let _pos: () = <()>::from(unimplemented!());
            let _end: () = <()>::from(unimplemented!());
            CheckConsecutiveKeys_0(_pos, _end, (1 as ()), s.as_pointer())
        });
        {
            let __a1 = unimplemented!();
            eq.as_pointer().write((
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.1.as_pointer(),
                ))),
            ))
        };
        assert!((unimplemented!() == 1_isize));
        (*k.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
        assert!(((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3));
        assert!(Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
            (*(*(*k.borrow()).iter_deref().upgrade().deref()).1.borrow())[..(*(*(*k.borrow())
                .iter_deref()
                .upgrade()
                .deref())
            .1
            .borrow())
            .len()
            .saturating_sub(1)]
                == *__s
        }));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!((unimplemented!() == 0_()));
        assert!((unimplemented!() == 3_()));
        {
            let __a1 = unimplemented!();
            eq.as_pointer().write((
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.1.as_pointer(),
                ))),
            ))
        };
        assert!((unimplemented!() == 2_isize));
        unimplemented!();
        unimplemented!();
        ({
            let _pos: () = <()>::from(unimplemented!());
            let _end: () = <()>::from(unimplemented!());
            CheckConsecutiveKeys_0(_pos, _end, (1 as ()), s.as_pointer())
        });
        {
            let __a1 = unimplemented!();
            eq.as_pointer().write((
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.1.as_pointer(),
                ))),
            ))
        };
        assert!((unimplemented!() == 1_isize));
        (*k.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
        assert!(((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3));
        assert!(Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
            (*(*(*k.borrow()).iter_deref().upgrade().deref()).1.borrow())[..(*(*(*k.borrow())
                .iter_deref()
                .upgrade()
                .deref())
            .1
            .borrow())
            .len()
            .saturating_sub(1)]
                == *__s
        }));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!((unimplemented!() == 2_()));
        assert!((unimplemented!() == 1_()));
        {
            let __a1 = unimplemented!();
            eq.as_pointer().write((
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.1.as_pointer(),
                ))),
            ))
        };
        assert!((unimplemented!() == 1_isize));
        (*k.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
        assert!(((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3));
        assert!(Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
            (*(*(*k.borrow()).iter_deref().upgrade().deref()).1.borrow())[..(*(*(*k.borrow())
                .iter_deref()
                .upgrade()
                .deref())
            .1
            .borrow())
            .len()
            .saturating_sub(1)]
                == *__s
        }));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!((unimplemented!() == 0_()));
        assert!((unimplemented!() == 1_()));
        {
            let __a1 = unimplemented!();
            eq.as_pointer().write((
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.1.as_pointer(),
                ))),
            ))
        };
        assert!((unimplemented!() == 1_isize));
        (*k.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
        assert!(((*(*(*k.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3));
        assert!(Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
            (*(*(*k.borrow()).iter_deref().upgrade().deref()).1.borrow())[..(*(*(*k.borrow())
                .iter_deref()
                .upgrade()
                .deref())
            .1
            .borrow())
            .len()
            .saturating_sub(1)]
                == *__s
        }));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!((unimplemented!() == 1_()));
        assert!((unimplemented!() == 0_()));
        {
            let __a1 = unimplemented!();
            eq.as_pointer().write((
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.1.as_pointer(),
                ))),
            ))
        };
        assert!((unimplemented!() == 0_isize));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!((unimplemented!() == 0_()));
        assert!((unimplemented!() == 0_()));
        {
            let __a1 = unimplemented!();
            eq.as_pointer().write((
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                    __a1.1.as_pointer(),
                ))),
            ))
        };
        assert!((unimplemented!() == 0_isize));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
    }
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let m2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < 10) {
            let j: Value<i32> = Rc::new(RefCell::new(0));
            'loop_: while ((*j.borrow()) < 2) {
                unimplemented!();
                unimplemented!();
                (*j.borrow_mut()).prefix_inc();
            }
            (*i.borrow_mut()).prefix_inc();
        }
        let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut ctr: i32 = 0;
        'loop_: while (*i.borrow()) != unimplemented!() {
            if ((ctr.postfix_inc() % 2) == 0) {
                unimplemented!();
            } else {
                (*i.borrow_mut()).iter_inc();
            }
        }
        assert!(({ only_deletions_5(m.as_pointer(), m2.as_pointer(),) }));
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
    let _ = outstanding_bytes_1.with(|_| ());
    let _ = alloc_count_2.with(|_| ());
    let _ = dealloc_count_3.with(|_| ());
    let _ = disable_default_constructor_4.with(|_| ());
}
