extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct soccc_allocator_char_ {
    #[offset(0)]
    #[byte_size(8)]
    soccc_count: Ptr<i32>,
    #[offset(8)]
    self_soccc_count: i32,
}
impl soccc_allocator_char_ {
    pub fn new(mut soccc_count_: Ptr<i32>, self_coccc_count_: Option<i32>) -> Self {
        let mut self_coccc_count_: i32 = self_coccc_count_.unwrap_or_else(|| 0);
        Self {
            soccc_count: (soccc_count_).clone(),
            self_soccc_count: self_coccc_count_,
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct soccc_allocator_wchar_t_ {
    #[offset(0)]
    #[byte_size(8)]
    soccc_count: Ptr<i32>,
    #[offset(8)]
    self_soccc_count: i32,
}
impl soccc_allocator_wchar_t_ {
    pub fn new(mut soccc_count_: Ptr<i32>, self_coccc_count_: Option<i32>) -> Self {
        let mut self_coccc_count_: i32 = self_coccc_count_.unwrap_or_else(|| 0);
        Self {
            soccc_count: (soccc_count_).clone(),
            self_soccc_count: self_coccc_count_,
        }
    }
}
pub fn test_0() -> bool {
    {
        let soccc_lhs: Value<i32> = Rc::new(RefCell::new(0));
        let soccc_rhs: Value<i32> = Rc::new(RefCell::new(0));
        let mut lhs: Vec<i8> = unimplemented!();
        let mut rhs: Vec<i8> = unimplemented!();
        let mut r: Vec<i8> = unimplemented!();
        assert!(
            (({
                soccc_allocator_char_Impl::get_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == (soccc_lhs.as_pointer()))
        );
        assert!(
            (({
                soccc_allocator_char_Impl::get_self_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == 1)
        );
        assert!(((*soccc_lhs.borrow()) == 1));
        assert!(((*soccc_rhs.borrow()) == 0));
    }
    {
        let soccc_lhs: Value<i32> = Rc::new(RefCell::new(0));
        let soccc_rhs: Value<i32> = Rc::new(RefCell::new(0));
        let mut lhs: Vec<i8> = unimplemented!();
        let mut rhs: Vec<i8> = unimplemented!();
        let mut r: Vec<i8> = unimplemented!();
        assert!(
            (({
                soccc_allocator_char_Impl::get_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == (soccc_rhs.as_pointer()))
        );
        assert!(
            (({
                soccc_allocator_char_Impl::get_self_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == 0)
        );
        assert!(((*soccc_lhs.borrow()) == 0));
        assert!(((*soccc_rhs.borrow()) == 0));
    }
    {
        let soccc_lhs: Value<i32> = Rc::new(RefCell::new(0));
        let soccc_rhs: Value<i32> = Rc::new(RefCell::new(0));
        let mut lhs: Vec<i8> = unimplemented!();
        let mut rhs: Vec<i8> = unimplemented!();
        let mut r: Vec<i8> = unimplemented!();
        assert!(
            (({
                soccc_allocator_char_Impl::get_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == (soccc_lhs.as_pointer()))
        );
        assert!(
            (({
                soccc_allocator_char_Impl::get_self_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == 0)
        );
        assert!(((*soccc_lhs.borrow()) == 0));
        assert!(((*soccc_rhs.borrow()) == 0));
    }
    {
        let soccc_lhs: Value<i32> = Rc::new(RefCell::new(0));
        let soccc_rhs: Value<i32> = Rc::new(RefCell::new(0));
        let mut lhs: Vec<i8> = unimplemented!();
        let mut rhs: Vec<i8> = unimplemented!();
        let mut r: Vec<i8> = unimplemented!();
        assert!(
            (({
                soccc_allocator_char_Impl::get_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == (soccc_lhs.as_pointer()))
        );
        assert!(
            (({
                soccc_allocator_char_Impl::get_self_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == 0)
        );
        assert!(((*soccc_lhs.borrow()) == 0));
        assert!(((*soccc_rhs.borrow()) == 0));
    }
    {
        let soccc_lhs: Value<i32> = Rc::new(RefCell::new(0));
        let soccc_rhs: Value<i32> = Rc::new(RefCell::new(0));
        let mut lhs: Vec<i8> = unimplemented!();
        let rhs: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        let mut r: Vec<i8> = {
            let mut __r = (lhs).clone();
            __r.pop();
            rhs.decay().with_c_str(|__s| __r.extend_from_slice(__s));
            __r.push(0_i8);
            __r
        };
        assert!(
            (({
                soccc_allocator_char_Impl::get_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == (soccc_lhs.as_pointer()))
        );
        assert!(
            (({
                soccc_allocator_char_Impl::get_self_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == 1)
        );
        assert!(((*soccc_lhs.borrow()) == 1));
        assert!(((*soccc_rhs.borrow()) == 0));
    }
    {
        let soccc_lhs: Value<i32> = Rc::new(RefCell::new(0));
        let soccc_rhs: Value<i32> = Rc::new(RefCell::new(0));
        let mut lhs: Vec<i8> = unimplemented!();
        let rhs: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        let mut r: Vec<i8> = unimplemented!();
        assert!(
            (({
                soccc_allocator_char_Impl::get_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == (soccc_lhs.as_pointer()))
        );
        assert!(
            (({
                soccc_allocator_char_Impl::get_self_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == 1)
        );
        assert!(((*soccc_lhs.borrow()) == 1));
        assert!(((*soccc_rhs.borrow()) == 0));
    }
    {
        let soccc_lhs: Value<i32> = Rc::new(RefCell::new(0));
        let soccc_rhs: Value<i32> = Rc::new(RefCell::new(0));
        let mut lhs: Vec<i8> = unimplemented!();
        let rhs: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        let mut r: Vec<i8> = {
            let mut __tmp2 = (lhs).clone();
            __tmp2.pop();
            rhs.decay().with_c_str(|__s| __tmp2.extend_from_slice(__s));
            __tmp2.push(0_i8);
            __tmp2
        };
        assert!(
            (({
                soccc_allocator_char_Impl::get_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == (soccc_lhs.as_pointer()))
        );
        assert!(
            (({
                soccc_allocator_char_Impl::get_self_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == 0)
        );
        assert!(((*soccc_lhs.borrow()) == 0));
        assert!(((*soccc_rhs.borrow()) == 0));
    }
    {
        let soccc_lhs: Value<i32> = Rc::new(RefCell::new(0));
        let soccc_rhs: Value<i32> = Rc::new(RefCell::new(0));
        let mut lhs: Vec<i8> = unimplemented!();
        let rhs: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        let mut r: Vec<i8> = unimplemented!();
        assert!(
            (({
                soccc_allocator_char_Impl::get_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == (soccc_lhs.as_pointer()))
        );
        assert!(
            (({
                soccc_allocator_char_Impl::get_self_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == 0)
        );
        assert!(((*soccc_lhs.borrow()) == 0));
        assert!(((*soccc_rhs.borrow()) == 0));
    }
    {
        let soccc_lhs: Value<i32> = Rc::new(RefCell::new(0));
        let soccc_rhs: Value<i32> = Rc::new(RefCell::new(0));
        let lhs: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        let mut rhs: Vec<i8> = unimplemented!();
        let mut r: Vec<i8> = unimplemented!();
        assert!(
            (({
                soccc_allocator_char_Impl::get_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == (soccc_rhs.as_pointer()))
        );
        assert!(
            (({
                soccc_allocator_char_Impl::get_self_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == 1)
        );
        assert!(((*soccc_lhs.borrow()) == 0));
        assert!(((*soccc_rhs.borrow()) == 1));
    }
    {
        let soccc_lhs: Value<i32> = Rc::new(RefCell::new(0));
        let soccc_rhs: Value<i32> = Rc::new(RefCell::new(0));
        let lhs: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        let mut rhs: Vec<i8> = unimplemented!();
        let mut r: Vec<i8> = unimplemented!();
        assert!(
            (({
                soccc_allocator_char_Impl::get_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == (soccc_rhs.as_pointer()))
        );
        assert!(
            (({
                soccc_allocator_char_Impl::get_self_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == 1)
        );
        assert!(((*soccc_lhs.borrow()) == 0));
        assert!(((*soccc_rhs.borrow()) == 1));
    }
    {
        let soccc_lhs: Value<i32> = Rc::new(RefCell::new(0));
        let soccc_rhs: Value<i32> = Rc::new(RefCell::new(0));
        let lhs: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        let mut rhs: Vec<i8> = unimplemented!();
        let mut r: Vec<i8> = unimplemented!();
        assert!(
            (({
                soccc_allocator_char_Impl::get_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == (soccc_rhs.as_pointer()))
        );
        assert!(
            (({
                soccc_allocator_char_Impl::get_self_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == 0)
        );
        assert!(((*soccc_lhs.borrow()) == 0));
        assert!(((*soccc_rhs.borrow()) == 0));
    }
    {
        let soccc_lhs: Value<i32> = Rc::new(RefCell::new(0));
        let soccc_rhs: Value<i32> = Rc::new(RefCell::new(0));
        let lhs: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        let mut rhs: Vec<i8> = unimplemented!();
        let mut r: Vec<i8> = unimplemented!();
        assert!(
            (({
                soccc_allocator_char_Impl::get_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == (soccc_rhs.as_pointer()))
        );
        assert!(
            (({
                soccc_allocator_char_Impl::get_self_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == 0)
        );
        assert!(((*soccc_lhs.borrow()) == 0));
        assert!(((*soccc_rhs.borrow()) == 0));
    }
    return true;
}
pub fn test_1() -> bool {
    {
        let soccc_lhs: Value<i32> = Rc::new(RefCell::new(0));
        let soccc_rhs: Value<i32> = Rc::new(RefCell::new(0));
        let mut lhs: Vec<i32> = unimplemented!();
        let mut rhs: Vec<i32> = unimplemented!();
        let mut r: Vec<i32> = unimplemented!();
        assert!(
            (({
                soccc_allocator_wchar_t_Impl::get_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == (soccc_lhs.as_pointer()))
        );
        assert!(
            (({
                soccc_allocator_wchar_t_Impl::get_self_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == 1)
        );
        assert!(((*soccc_lhs.borrow()) == 1));
        assert!(((*soccc_rhs.borrow()) == 0));
    }
    {
        let soccc_lhs: Value<i32> = Rc::new(RefCell::new(0));
        let soccc_rhs: Value<i32> = Rc::new(RefCell::new(0));
        let mut lhs: Vec<i32> = unimplemented!();
        let mut rhs: Vec<i32> = unimplemented!();
        let mut r: Vec<i32> = unimplemented!();
        assert!(
            (({
                soccc_allocator_wchar_t_Impl::get_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == (soccc_rhs.as_pointer()))
        );
        assert!(
            (({
                soccc_allocator_wchar_t_Impl::get_self_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == 0)
        );
        assert!(((*soccc_lhs.borrow()) == 0));
        assert!(((*soccc_rhs.borrow()) == 0));
    }
    {
        let soccc_lhs: Value<i32> = Rc::new(RefCell::new(0));
        let soccc_rhs: Value<i32> = Rc::new(RefCell::new(0));
        let mut lhs: Vec<i32> = unimplemented!();
        let mut rhs: Vec<i32> = unimplemented!();
        let mut r: Vec<i32> = unimplemented!();
        assert!(
            (({
                soccc_allocator_wchar_t_Impl::get_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == (soccc_lhs.as_pointer()))
        );
        assert!(
            (({
                soccc_allocator_wchar_t_Impl::get_self_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == 0)
        );
        assert!(((*soccc_lhs.borrow()) == 0));
        assert!(((*soccc_rhs.borrow()) == 0));
    }
    {
        let soccc_lhs: Value<i32> = Rc::new(RefCell::new(0));
        let soccc_rhs: Value<i32> = Rc::new(RefCell::new(0));
        let mut lhs: Vec<i32> = unimplemented!();
        let mut rhs: Vec<i32> = unimplemented!();
        let mut r: Vec<i32> = unimplemented!();
        assert!(
            (({
                soccc_allocator_wchar_t_Impl::get_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == (soccc_lhs.as_pointer()))
        );
        assert!(
            (({
                soccc_allocator_wchar_t_Impl::get_self_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == 0)
        );
        assert!(((*soccc_lhs.borrow()) == 0));
        assert!(((*soccc_rhs.borrow()) == 0));
    }
    {
        let soccc_lhs: Value<i32> = Rc::new(RefCell::new(0));
        let soccc_rhs: Value<i32> = Rc::new(RefCell::new(0));
        let mut lhs: Vec<i32> = unimplemented!();
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
        let mut r: Vec<i32> = {
            let mut __r = (lhs).clone();
            __r.pop();
            rhs.decay().with_c_str(|__s| __r.extend_from_slice(__s));
            __r.push(0_i32);
            __r
        };
        assert!(
            (({
                soccc_allocator_wchar_t_Impl::get_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == (soccc_lhs.as_pointer()))
        );
        assert!(
            (({
                soccc_allocator_wchar_t_Impl::get_self_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == 1)
        );
        assert!(((*soccc_lhs.borrow()) == 1));
        assert!(((*soccc_rhs.borrow()) == 0));
    }
    {
        let soccc_lhs: Value<i32> = Rc::new(RefCell::new(0));
        let soccc_rhs: Value<i32> = Rc::new(RefCell::new(0));
        let mut lhs: Vec<i32> = unimplemented!();
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
        let mut r: Vec<i32> = unimplemented!();
        assert!(
            (({
                soccc_allocator_wchar_t_Impl::get_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == (soccc_lhs.as_pointer()))
        );
        assert!(
            (({
                soccc_allocator_wchar_t_Impl::get_self_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == 1)
        );
        assert!(((*soccc_lhs.borrow()) == 1));
        assert!(((*soccc_rhs.borrow()) == 0));
    }
    {
        let soccc_lhs: Value<i32> = Rc::new(RefCell::new(0));
        let soccc_rhs: Value<i32> = Rc::new(RefCell::new(0));
        let mut lhs: Vec<i32> = unimplemented!();
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
        let mut r: Vec<i32> = {
            let mut __tmp2 = (lhs).clone();
            __tmp2.pop();
            rhs.decay().with_c_str(|__s| __tmp2.extend_from_slice(__s));
            __tmp2.push(0_i32);
            __tmp2
        };
        assert!(
            (({
                soccc_allocator_wchar_t_Impl::get_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == (soccc_lhs.as_pointer()))
        );
        assert!(
            (({
                soccc_allocator_wchar_t_Impl::get_self_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == 0)
        );
        assert!(((*soccc_lhs.borrow()) == 0));
        assert!(((*soccc_rhs.borrow()) == 0));
    }
    {
        let soccc_lhs: Value<i32> = Rc::new(RefCell::new(0));
        let soccc_rhs: Value<i32> = Rc::new(RefCell::new(0));
        let mut lhs: Vec<i32> = unimplemented!();
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
        let mut r: Vec<i32> = unimplemented!();
        assert!(
            (({
                soccc_allocator_wchar_t_Impl::get_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == (soccc_lhs.as_pointer()))
        );
        assert!(
            (({
                soccc_allocator_wchar_t_Impl::get_self_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == 0)
        );
        assert!(((*soccc_lhs.borrow()) == 0));
        assert!(((*soccc_rhs.borrow()) == 0));
    }
    {
        let soccc_lhs: Value<i32> = Rc::new(RefCell::new(0));
        let soccc_rhs: Value<i32> = Rc::new(RefCell::new(0));
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
        let mut rhs: Vec<i32> = unimplemented!();
        let mut r: Vec<i32> = unimplemented!();
        assert!(
            (({
                soccc_allocator_wchar_t_Impl::get_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == (soccc_rhs.as_pointer()))
        );
        assert!(
            (({
                soccc_allocator_wchar_t_Impl::get_self_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == 1)
        );
        assert!(((*soccc_lhs.borrow()) == 0));
        assert!(((*soccc_rhs.borrow()) == 1));
    }
    {
        let soccc_lhs: Value<i32> = Rc::new(RefCell::new(0));
        let soccc_rhs: Value<i32> = Rc::new(RefCell::new(0));
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
        let mut rhs: Vec<i32> = unimplemented!();
        let mut r: Vec<i32> = unimplemented!();
        assert!(
            (({
                soccc_allocator_wchar_t_Impl::get_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == (soccc_rhs.as_pointer()))
        );
        assert!(
            (({
                soccc_allocator_wchar_t_Impl::get_self_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == 1)
        );
        assert!(((*soccc_lhs.borrow()) == 0));
        assert!(((*soccc_rhs.borrow()) == 1));
    }
    {
        let soccc_lhs: Value<i32> = Rc::new(RefCell::new(0));
        let soccc_rhs: Value<i32> = Rc::new(RefCell::new(0));
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
        let mut rhs: Vec<i32> = unimplemented!();
        let mut r: Vec<i32> = unimplemented!();
        assert!(
            (({
                soccc_allocator_wchar_t_Impl::get_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == (soccc_rhs.as_pointer()))
        );
        assert!(
            (({
                soccc_allocator_wchar_t_Impl::get_self_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == 0)
        );
        assert!(((*soccc_lhs.borrow()) == 0));
        assert!(((*soccc_rhs.borrow()) == 0));
    }
    {
        let soccc_lhs: Value<i32> = Rc::new(RefCell::new(0));
        let soccc_rhs: Value<i32> = Rc::new(RefCell::new(0));
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
        let mut rhs: Vec<i32> = unimplemented!();
        let mut r: Vec<i32> = unimplemented!();
        assert!(
            (({
                soccc_allocator_wchar_t_Impl::get_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == (soccc_rhs.as_pointer()))
        );
        assert!(
            (({
                soccc_allocator_wchar_t_Impl::get_self_soccc(
                    &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                )
            }) == 0)
        );
        assert!(((*soccc_lhs.borrow()) == 0));
        assert!(((*soccc_rhs.borrow()) == 0));
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
    ({ test_1() });
    const _: () = assert!(true, "test<char>()");;
    const _: () = assert!(true, "test<wchar_t>()");;
    return 0;
}
pub trait soccc_allocator_char_Impl {
    fn allocate(&self, n: usize) -> Ptr<i8>;
    fn deallocate(&self, p: Ptr<i8>, s: usize);
    fn select_on_container_copy_construction(&self) -> soccc_allocator_char_;
    fn get_soccc(&self) -> Ptr<i32>;
    fn get_self_soccc(&self) -> i32;
    fn operator_eq_8_char(&self, that: Ptr<soccc_allocator_char_>) -> bool;
}
impl soccc_allocator_char_Impl for Ptr<soccc_allocator_char_> {
    fn allocate(&self, mut n: usize) -> Ptr<i8> {
        return unimplemented!();
    }
    fn deallocate(&self, mut p: Ptr<i8>, mut s: usize) {
        unimplemented!();
    }
    fn select_on_container_copy_construction(&self) -> soccc_allocator_char_ {
        {
            (*self)
                .with(|__s| __s.soccc_count.clone())
                .with_mut(|__v| *__v = *__v + 1)
        };
        return soccc_allocator_char_::new({ (*self).with(|__s| __s.soccc_count.clone()) }, {
            Some(((*self).with(|__s| __s.self_soccc_count) + 1))
        });
    }
    fn get_soccc(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.soccc_count.clone());
    }
    fn get_self_soccc(&self) -> i32 {
        return (*self).with(|__s| __s.self_soccc_count);
    }
    fn operator_eq_8_char(&self, that: Ptr<soccc_allocator_char_>) -> bool {
        return ({ (*self).with(|__s| __s.soccc_count.clone()) } == {
            ({ soccc_allocator_char_Impl::get_soccc(&that) })
        });
    }
}
pub trait soccc_allocator_wchar_t_Impl {
    fn allocate(&self, n: usize) -> Ptr<i32>;
    fn deallocate(&self, p: Ptr<i32>, s: usize);
    fn select_on_container_copy_construction(&self) -> soccc_allocator_wchar_t_;
    fn get_soccc(&self) -> Ptr<i32>;
    fn get_self_soccc(&self) -> i32;
    fn operator_eq_8_wchar_t(&self, that: Ptr<soccc_allocator_wchar_t_>) -> bool;
}
impl soccc_allocator_wchar_t_Impl for Ptr<soccc_allocator_wchar_t_> {
    fn allocate(&self, mut n: usize) -> Ptr<i32> {
        return unimplemented!();
    }
    fn deallocate(&self, mut p: Ptr<i32>, mut s: usize) {
        unimplemented!();
    }
    fn select_on_container_copy_construction(&self) -> soccc_allocator_wchar_t_ {
        {
            (*self)
                .with(|__s| __s.soccc_count.clone())
                .with_mut(|__v| *__v = *__v + 1)
        };
        return soccc_allocator_wchar_t_::new({ (*self).with(|__s| __s.soccc_count.clone()) }, {
            Some(((*self).with(|__s| __s.self_soccc_count) + 1))
        });
    }
    fn get_soccc(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.soccc_count.clone());
    }
    fn get_self_soccc(&self) -> i32 {
        return (*self).with(|__s| __s.self_soccc_count);
    }
    fn operator_eq_8_wchar_t(&self, that: Ptr<soccc_allocator_wchar_t_>) -> bool {
        return ({ (*self).with(|__s| __s.soccc_count.clone()) } == {
            ({ soccc_allocator_wchar_t_Impl::get_soccc(&that) })
        });
    }
}
pub fn __cpp2rust_init_globals() {}
