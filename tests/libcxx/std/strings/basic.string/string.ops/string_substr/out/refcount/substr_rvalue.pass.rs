extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn is_pointer_in_range_0(mut first: Ptr<()>, mut last: Ptr<()>, mut p: Ptr<()>) -> () {
    if ({ (first).clone() } == { (p).clone() }) {
        return true;
    }
    'loop_: while ({ (first).clone() } != { (last).clone() }) {
        if ({ (first).clone() } == { (p).clone() }) {
            return true;
        }
        first.prefix_inc();
    }
    return false;
}
pub fn is_pointer_in_range_1(mut first: Ptr<()>, mut last: Ptr<()>, mut p: Ptr<()>) -> () {
    if ({ (first).clone() } == { (p).clone() }) {
        return true;
    }
    'loop_: while ({ (first).clone() } != { (last).clone() }) {
        if ({ (first).clone() } == { (p).clone() }) {
            return true;
        }
        first.prefix_inc();
    }
    return false;
}
pub fn is_pointer_in_range_2(mut first: Ptr<()>, mut last: Ptr<()>, mut p: Ptr<()>) -> () {
    if ({ (first).clone() } == { (p).clone() }) {
        return true;
    }
    'loop_: while ({ (first).clone() } != { (last).clone() }) {
        if ({ (first).clone() } == { (p).clone() }) {
            return true;
        }
        first.prefix_inc();
    }
    return false;
}
pub fn is_pointer_in_range_3(mut first: Ptr<()>, mut last: Ptr<()>, mut p: Ptr<()>) -> () {
    if ({ (first).clone() } == { (p).clone() }) {
        return true;
    }
    'loop_: while ({ (first).clone() } != { (last).clone() }) {
        if ({ (first).clone() } == { (p).clone() }) {
            return true;
        }
        first.prefix_inc();
    }
    return false;
}
pub fn is_pointer_in_range_4(mut first: Ptr<()>, mut last: Ptr<()>, mut p: Ptr<()>) -> () {
    if ({ (first).clone() } == { (p).clone() }) {
        return true;
    }
    'loop_: while ({ (first).clone() } != { (last).clone() }) {
        if ({ (first).clone() } == { (p).clone() }) {
            return true;
        }
        first.prefix_inc();
    }
    return false;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct constexpr_char_traits_char_ {}
impl constexpr_char_traits_char_ {
    pub fn assign_1(c1: Ptr<()>, c2: Ptr<()>) {
        c1.write({ (c2.read()) });
    }
    pub fn eq(mut c1: (), mut c2: ()) -> () {
        return (c1 == c2);
    }
    pub fn lt(mut c1: (), mut c2: ()) -> () {
        return (c1 < c2);
    }
    pub fn compare(mut s1: Ptr<()>, mut s2: Ptr<()>, mut n: ()) -> () {
        'loop_: while (n != 0) {
            if ({
                let _c1: () = (s1.read());
                let _c2: () = (s2.read());
                constexpr_char_traits_char_::lt(_c1, _c2)
            }) {
                return -1_();
            }
            if ({
                let _c1: () = (s2.read());
                let _c2: () = (s1.read());
                constexpr_char_traits_char_::lt(_c1, _c2)
            }) {
                return 1_();
            }
            {
                {
                    n.prefix_dec();
                    s1.prefix_inc()
                };
                s2.prefix_inc()
            };
        }
        return 0_();
    }
    pub fn length(mut s: Ptr<()>) -> () {
        let len: Value<()> = Rc::new(RefCell::new(0_()));
        'loop_: while !({
            let _c1: () = (s.read());
            let _c2: () = 0_();
            constexpr_char_traits_char_::eq(_c1, _c2)
        }) {
            (*len.borrow_mut()).prefix_inc();
            s.prefix_inc();
        }
        return (*len.borrow());
    }
    pub fn move_(s1: Ptr<()>, mut s2: Ptr<()>, mut n: ()) -> Ptr<()> {
        let s1: Value<Ptr<()>> = Rc::new(RefCell::new(s1));
        if ({ (*s1.borrow()).clone() } == { (s2).clone() }) {
            return (*s1.borrow()).clone();
        }
        let r: Value<Ptr<()>> = Rc::new(RefCell::new((*s1.borrow()).clone()));
        if ({
            let _first: Ptr<()> = (*s1.borrow()).clone();
            let _last: Ptr<()> = (*s1.borrow()).offset((n) as isize);
            is_pointer_in_range_0(_first, _last, (s2).clone())
        }) {
            'loop_: while (n != 0) {
                ({
                    let _c1: Ptr<()> = (*s1.borrow_mut()).postfix_inc();
                    let _c2: Ptr<()> = s2.postfix_inc();
                    constexpr_char_traits_char_::assign_1(_c1, _c2)
                });
                n.prefix_dec();
            }
        } else {
            (*s1.borrow_mut()) += n;
            s2 += n;
            'loop_: while (n != 0) {
                ({
                    let _c1: Ptr<()> = (*s1.borrow_mut()).prefix_dec();
                    let _c2: Ptr<()> = s2.prefix_dec();
                    constexpr_char_traits_char_::assign_1(_c1, _c2)
                });
                n.prefix_dec();
            }
        }
        return (*r.borrow()).clone();
    }
    pub fn copy(mut s1: Ptr<()>, mut s2: Ptr<()>, mut n: ()) -> Ptr<()> {
        {
            unimplemented!();
        }
        let r: Value<Ptr<()>> = Rc::new(RefCell::new((s1).clone()));
        'loop_: while (n != 0) {
            ({
                let _c1: Ptr<()> = (s1).clone();
                let _c2: Ptr<()> = (s2).clone();
                constexpr_char_traits_char_::assign_1(_c1, _c2)
            });
            {
                {
                    n.prefix_dec();
                    s1.prefix_inc()
                };
                s2.prefix_inc()
            };
        }
        return (*r.borrow()).clone();
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct constexpr_char_traits_char16_t_ {}
impl constexpr_char_traits_char16_t_ {
    pub fn assign_1(c1: Ptr<()>, c2: Ptr<()>) {
        c1.write({ (c2.read()) });
    }
    pub fn eq(mut c1: (), mut c2: ()) -> () {
        return (c1 == c2);
    }
    pub fn lt(mut c1: (), mut c2: ()) -> () {
        return (c1 < c2);
    }
    pub fn compare(mut s1: Ptr<()>, mut s2: Ptr<()>, mut n: ()) -> () {
        'loop_: while (n != 0) {
            if ({
                let _c1: () = (s1.read());
                let _c2: () = (s2.read());
                constexpr_char_traits_char16_t_::lt(_c1, _c2)
            }) {
                return -1_();
            }
            if ({
                let _c1: () = (s2.read());
                let _c2: () = (s1.read());
                constexpr_char_traits_char16_t_::lt(_c1, _c2)
            }) {
                return 1_();
            }
            {
                {
                    n.prefix_dec();
                    s1.prefix_inc()
                };
                s2.prefix_inc()
            };
        }
        return 0_();
    }
    pub fn length(mut s: Ptr<()>) -> () {
        let len: Value<()> = Rc::new(RefCell::new(0_()));
        'loop_: while !({
            let _c1: () = (s.read());
            let _c2: () = 0_();
            constexpr_char_traits_char16_t_::eq(_c1, _c2)
        }) {
            (*len.borrow_mut()).prefix_inc();
            s.prefix_inc();
        }
        return (*len.borrow());
    }
    pub fn move_(s1: Ptr<()>, mut s2: Ptr<()>, mut n: ()) -> Ptr<()> {
        let s1: Value<Ptr<()>> = Rc::new(RefCell::new(s1));
        if ({ (*s1.borrow()).clone() } == { (s2).clone() }) {
            return (*s1.borrow()).clone();
        }
        let r: Value<Ptr<()>> = Rc::new(RefCell::new((*s1.borrow()).clone()));
        if ({
            let _first: Ptr<()> = (*s1.borrow()).clone();
            let _last: Ptr<()> = (*s1.borrow()).offset((n) as isize);
            is_pointer_in_range_1(_first, _last, (s2).clone())
        }) {
            'loop_: while (n != 0) {
                ({
                    let _c1: Ptr<()> = (*s1.borrow_mut()).postfix_inc();
                    let _c2: Ptr<()> = s2.postfix_inc();
                    constexpr_char_traits_char16_t_::assign_1(_c1, _c2)
                });
                n.prefix_dec();
            }
        } else {
            (*s1.borrow_mut()) += n;
            s2 += n;
            'loop_: while (n != 0) {
                ({
                    let _c1: Ptr<()> = (*s1.borrow_mut()).prefix_dec();
                    let _c2: Ptr<()> = s2.prefix_dec();
                    constexpr_char_traits_char16_t_::assign_1(_c1, _c2)
                });
                n.prefix_dec();
            }
        }
        return (*r.borrow()).clone();
    }
    pub fn copy(mut s1: Ptr<()>, mut s2: Ptr<()>, mut n: ()) -> Ptr<()> {
        {
            unimplemented!();
        }
        let r: Value<Ptr<()>> = Rc::new(RefCell::new((s1).clone()));
        'loop_: while (n != 0) {
            ({
                let _c1: Ptr<()> = (s1).clone();
                let _c2: Ptr<()> = (s2).clone();
                constexpr_char_traits_char16_t_::assign_1(_c1, _c2)
            });
            {
                {
                    n.prefix_dec();
                    s1.prefix_inc()
                };
                s2.prefix_inc()
            };
        }
        return (*r.borrow()).clone();
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct constexpr_char_traits_char32_t_ {}
impl constexpr_char_traits_char32_t_ {
    pub fn assign_1(c1: Ptr<()>, c2: Ptr<()>) {
        c1.write({ (c2.read()) });
    }
    pub fn eq(mut c1: (), mut c2: ()) -> () {
        return (c1 == c2);
    }
    pub fn lt(mut c1: (), mut c2: ()) -> () {
        return (c1 < c2);
    }
    pub fn compare(mut s1: Ptr<()>, mut s2: Ptr<()>, mut n: ()) -> () {
        'loop_: while (n != 0) {
            if ({
                let _c1: () = (s1.read());
                let _c2: () = (s2.read());
                constexpr_char_traits_char32_t_::lt(_c1, _c2)
            }) {
                return -1_();
            }
            if ({
                let _c1: () = (s2.read());
                let _c2: () = (s1.read());
                constexpr_char_traits_char32_t_::lt(_c1, _c2)
            }) {
                return 1_();
            }
            {
                {
                    n.prefix_dec();
                    s1.prefix_inc()
                };
                s2.prefix_inc()
            };
        }
        return 0_();
    }
    pub fn length(mut s: Ptr<()>) -> () {
        let len: Value<()> = Rc::new(RefCell::new(0_()));
        'loop_: while !({
            let _c1: () = (s.read());
            let _c2: () = 0_();
            constexpr_char_traits_char32_t_::eq(_c1, _c2)
        }) {
            (*len.borrow_mut()).prefix_inc();
            s.prefix_inc();
        }
        return (*len.borrow());
    }
    pub fn move_(s1: Ptr<()>, mut s2: Ptr<()>, mut n: ()) -> Ptr<()> {
        let s1: Value<Ptr<()>> = Rc::new(RefCell::new(s1));
        if ({ (*s1.borrow()).clone() } == { (s2).clone() }) {
            return (*s1.borrow()).clone();
        }
        let r: Value<Ptr<()>> = Rc::new(RefCell::new((*s1.borrow()).clone()));
        if ({
            let _first: Ptr<()> = (*s1.borrow()).clone();
            let _last: Ptr<()> = (*s1.borrow()).offset((n) as isize);
            is_pointer_in_range_2(_first, _last, (s2).clone())
        }) {
            'loop_: while (n != 0) {
                ({
                    let _c1: Ptr<()> = (*s1.borrow_mut()).postfix_inc();
                    let _c2: Ptr<()> = s2.postfix_inc();
                    constexpr_char_traits_char32_t_::assign_1(_c1, _c2)
                });
                n.prefix_dec();
            }
        } else {
            (*s1.borrow_mut()) += n;
            s2 += n;
            'loop_: while (n != 0) {
                ({
                    let _c1: Ptr<()> = (*s1.borrow_mut()).prefix_dec();
                    let _c2: Ptr<()> = s2.prefix_dec();
                    constexpr_char_traits_char32_t_::assign_1(_c1, _c2)
                });
                n.prefix_dec();
            }
        }
        return (*r.borrow()).clone();
    }
    pub fn copy(mut s1: Ptr<()>, mut s2: Ptr<()>, mut n: ()) -> Ptr<()> {
        {
            unimplemented!();
        }
        let r: Value<Ptr<()>> = Rc::new(RefCell::new((s1).clone()));
        'loop_: while (n != 0) {
            ({
                let _c1: Ptr<()> = (s1).clone();
                let _c2: Ptr<()> = (s2).clone();
                constexpr_char_traits_char32_t_::assign_1(_c1, _c2)
            });
            {
                {
                    n.prefix_dec();
                    s1.prefix_inc()
                };
                s2.prefix_inc()
            };
        }
        return (*r.borrow()).clone();
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct constexpr_char_traits_wchar_t_ {}
impl constexpr_char_traits_wchar_t_ {
    pub fn assign_1(c1: Ptr<()>, c2: Ptr<()>) {
        c1.write({ (c2.read()) });
    }
    pub fn eq(mut c1: (), mut c2: ()) -> () {
        return (c1 == c2);
    }
    pub fn lt(mut c1: (), mut c2: ()) -> () {
        return (c1 < c2);
    }
    pub fn compare(mut s1: Ptr<()>, mut s2: Ptr<()>, mut n: ()) -> () {
        'loop_: while (n != 0) {
            if ({
                let _c1: () = (s1.read());
                let _c2: () = (s2.read());
                constexpr_char_traits_wchar_t_::lt(_c1, _c2)
            }) {
                return -1_();
            }
            if ({
                let _c1: () = (s2.read());
                let _c2: () = (s1.read());
                constexpr_char_traits_wchar_t_::lt(_c1, _c2)
            }) {
                return 1_();
            }
            {
                {
                    n.prefix_dec();
                    s1.prefix_inc()
                };
                s2.prefix_inc()
            };
        }
        return 0_();
    }
    pub fn length(mut s: Ptr<()>) -> () {
        let len: Value<()> = Rc::new(RefCell::new(0_()));
        'loop_: while !({
            let _c1: () = (s.read());
            let _c2: () = 0_();
            constexpr_char_traits_wchar_t_::eq(_c1, _c2)
        }) {
            (*len.borrow_mut()).prefix_inc();
            s.prefix_inc();
        }
        return (*len.borrow());
    }
    pub fn move_(s1: Ptr<()>, mut s2: Ptr<()>, mut n: ()) -> Ptr<()> {
        let s1: Value<Ptr<()>> = Rc::new(RefCell::new(s1));
        if ({ (*s1.borrow()).clone() } == { (s2).clone() }) {
            return (*s1.borrow()).clone();
        }
        let r: Value<Ptr<()>> = Rc::new(RefCell::new((*s1.borrow()).clone()));
        if ({
            let _first: Ptr<()> = (*s1.borrow()).clone();
            let _last: Ptr<()> = (*s1.borrow()).offset((n) as isize);
            is_pointer_in_range_3(_first, _last, (s2).clone())
        }) {
            'loop_: while (n != 0) {
                ({
                    let _c1: Ptr<()> = (*s1.borrow_mut()).postfix_inc();
                    let _c2: Ptr<()> = s2.postfix_inc();
                    constexpr_char_traits_wchar_t_::assign_1(_c1, _c2)
                });
                n.prefix_dec();
            }
        } else {
            (*s1.borrow_mut()) += n;
            s2 += n;
            'loop_: while (n != 0) {
                ({
                    let _c1: Ptr<()> = (*s1.borrow_mut()).prefix_dec();
                    let _c2: Ptr<()> = s2.prefix_dec();
                    constexpr_char_traits_wchar_t_::assign_1(_c1, _c2)
                });
                n.prefix_dec();
            }
        }
        return (*r.borrow()).clone();
    }
    pub fn copy(mut s1: Ptr<()>, mut s2: Ptr<()>, mut n: ()) -> Ptr<()> {
        {
            unimplemented!();
        }
        let r: Value<Ptr<()>> = Rc::new(RefCell::new((s1).clone()));
        'loop_: while (n != 0) {
            ({
                let _c1: Ptr<()> = (s1).clone();
                let _c2: Ptr<()> = (s2).clone();
                constexpr_char_traits_wchar_t_::assign_1(_c1, _c2)
            });
            {
                {
                    n.prefix_dec();
                    s1.prefix_inc()
                };
                s2.prefix_inc()
            };
        }
        return (*r.borrow()).clone();
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct constexpr_char_traits_char8_t_ {}
impl constexpr_char_traits_char8_t_ {
    pub fn assign_1(c1: Ptr<()>, c2: Ptr<()>) {
        c1.write({ (c2.read()) });
    }
    pub fn eq(mut c1: (), mut c2: ()) -> () {
        return (c1 == c2);
    }
    pub fn lt(mut c1: (), mut c2: ()) -> () {
        return (c1 < c2);
    }
    pub fn compare(mut s1: Ptr<()>, mut s2: Ptr<()>, mut n: ()) -> () {
        'loop_: while (n != 0) {
            if ({
                let _c1: () = (s1.read());
                let _c2: () = (s2.read());
                constexpr_char_traits_char8_t_::lt(_c1, _c2)
            }) {
                return -1_();
            }
            if ({
                let _c1: () = (s2.read());
                let _c2: () = (s1.read());
                constexpr_char_traits_char8_t_::lt(_c1, _c2)
            }) {
                return 1_();
            }
            {
                {
                    n.prefix_dec();
                    s1.prefix_inc()
                };
                s2.prefix_inc()
            };
        }
        return 0_();
    }
    pub fn length(mut s: Ptr<()>) -> () {
        let len: Value<()> = Rc::new(RefCell::new(0_()));
        'loop_: while !({
            let _c1: () = (s.read());
            let _c2: () = 0_();
            constexpr_char_traits_char8_t_::eq(_c1, _c2)
        }) {
            (*len.borrow_mut()).prefix_inc();
            s.prefix_inc();
        }
        return (*len.borrow());
    }
    pub fn move_(s1: Ptr<()>, mut s2: Ptr<()>, mut n: ()) -> Ptr<()> {
        let s1: Value<Ptr<()>> = Rc::new(RefCell::new(s1));
        if ({ (*s1.borrow()).clone() } == { (s2).clone() }) {
            return (*s1.borrow()).clone();
        }
        let r: Value<Ptr<()>> = Rc::new(RefCell::new((*s1.borrow()).clone()));
        if ({
            let _first: Ptr<()> = (*s1.borrow()).clone();
            let _last: Ptr<()> = (*s1.borrow()).offset((n) as isize);
            is_pointer_in_range_4(_first, _last, (s2).clone())
        }) {
            'loop_: while (n != 0) {
                ({
                    let _c1: Ptr<()> = (*s1.borrow_mut()).postfix_inc();
                    let _c2: Ptr<()> = s2.postfix_inc();
                    constexpr_char_traits_char8_t_::assign_1(_c1, _c2)
                });
                n.prefix_dec();
            }
        } else {
            (*s1.borrow_mut()) += n;
            s2 += n;
            'loop_: while (n != 0) {
                ({
                    let _c1: Ptr<()> = (*s1.borrow_mut()).prefix_dec();
                    let _c2: Ptr<()> = s2.prefix_dec();
                    constexpr_char_traits_char8_t_::assign_1(_c1, _c2)
                });
                n.prefix_dec();
            }
        }
        return (*r.borrow()).clone();
    }
    pub fn copy(mut s1: Ptr<()>, mut s2: Ptr<()>, mut n: ()) -> Ptr<()> {
        {
            unimplemented!();
        }
        let r: Value<Ptr<()>> = Rc::new(RefCell::new((s1).clone()));
        'loop_: while (n != 0) {
            ({
                let _c1: Ptr<()> = (s1).clone();
                let _c2: Ptr<()> = (s2).clone();
                constexpr_char_traits_char8_t_::assign_1(_c1, _c2)
            });
            {
                {
                    n.prefix_dec();
                    s1.prefix_inc()
                };
                s2.prefix_inc()
            };
        }
        return (*r.borrow()).clone();
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(80)]
pub struct MultiStringType {
    #[offset(0)]
    #[byte_size(8)]
    pub w_: Ptr<()>,
    #[offset(8)]
    pub wn_: (),
    #[offset(16)]
    #[byte_size(8)]
    pub u8_: Ptr<()>,
    #[offset(24)]
    pub u8n_: (),
    #[offset(32)]
    #[byte_size(8)]
    pub u16_: Ptr<()>,
    #[offset(40)]
    pub u16n_: (),
    #[offset(48)]
    #[byte_size(8)]
    pub u32_: Ptr<()>,
    #[offset(56)]
    pub u32n_: (),
    #[offset(64)]
    #[byte_size(8)]
    pub s_: Ptr<()>,
    #[offset(72)]
    pub sn_: (),
}
impl MultiStringType {
    pub fn new(
        mut w: Ptr<()>,
        mut wn: (),
        mut u8: Ptr<()>,
        mut u8n: (),
        mut u16: Ptr<()>,
        mut u16n: (),
        mut u32: Ptr<()>,
        mut u32n: (),
        mut s: Ptr<()>,
        mut sn: (),
    ) -> Self {
        Self {
            w_: (w).clone(),
            wn_: wn,
            u8_: (u8).clone(),
            u8n_: u8n,
            u16_: (u16).clone(),
            u16n_: u16n,
            u32_: (u32).clone(),
            u32n_: u32n,
            s_: (s).clone(),
            sn_: sn,
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> () {
        unimplemented!();
        return ((alloc_count_6.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_7.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        unimplemented!();
        disable_default_constructor_8.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_5.with(|rc| *rc.borrow_mut() = 0_());
        alloc_count_6.with(|rc| *rc.borrow_mut() = 0_());
        dealloc_count_7.with(|rc| *rc.borrow_mut() = 0_());
    }
}
thread_local!(
    pub static outstanding_bytes_5: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static alloc_count_6: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_7: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static disable_default_constructor_8: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_char__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_char__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn pointer_to(t: Ptr<()>) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn new_2(mut p: min_pointer_char__std_integral_constant_unsigned_long__0__) -> Self {
        Self {
            ptr_: (p.ptr_).clone(),
        }
    }
    pub fn pointer_to(
        t: Ptr<()>,
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_const_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_char16_t__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_char16_t__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn pointer_to(
        t: Ptr<()>,
    ) -> min_pointer_char16_t__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char16_t__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn new_2(mut p: min_pointer_char16_t__std_integral_constant_unsigned_long__0__) -> Self {
        Self {
            ptr_: (p.ptr_).clone(),
        }
    }
    pub fn pointer_to(
        t: Ptr<()>,
    ) -> min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__ {
        return min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_char32_t__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_char32_t__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn pointer_to(
        t: Ptr<()>,
    ) -> min_pointer_char32_t__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char32_t__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn new_2(mut p: min_pointer_char32_t__std_integral_constant_unsigned_long__0__) -> Self {
        Self {
            ptr_: (p.ptr_).clone(),
        }
    }
    pub fn pointer_to(
        t: Ptr<()>,
    ) -> min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__ {
        return min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_wchar_t__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_wchar_t__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn pointer_to(t: Ptr<()>) -> min_pointer_wchar_t__std_integral_constant_unsigned_long__0__ {
        return min_pointer_wchar_t__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn new_2(mut p: min_pointer_wchar_t__std_integral_constant_unsigned_long__0__) -> Self {
        Self {
            ptr_: (p.ptr_).clone(),
        }
    }
    pub fn pointer_to(
        t: Ptr<()>,
    ) -> min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__ {
        return min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_char8_t__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_char8_t__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn pointer_to(t: Ptr<()>) -> min_pointer_char8_t__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char8_t__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn new_2(mut p: min_pointer_char8_t__std_integral_constant_unsigned_long__0__) -> Self {
        Self {
            ptr_: (p.ptr_).clone(),
        }
    }
    pub fn pointer_to(
        t: Ptr<()>,
    ) -> min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__ {
        return min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
pub fn operator_eq_9(mut _a0: min_allocator_char_, mut _a1: min_allocator_char_) -> () {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_char_ {}
impl std::cmp::PartialEq for min_allocator_char_ {
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_9(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_char_ {}
pub fn operator_eq_10(mut _a0: min_allocator_char16_t_, mut _a1: min_allocator_char16_t_) -> () {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_char16_t_ {}
impl std::cmp::PartialEq for min_allocator_char16_t_ {
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_10(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_char16_t_ {}
pub fn operator_eq_11(mut _a0: min_allocator_char32_t_, mut _a1: min_allocator_char32_t_) -> () {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_char32_t_ {}
impl std::cmp::PartialEq for min_allocator_char32_t_ {
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_11(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_char32_t_ {}
pub fn operator_eq_12(mut _a0: min_allocator_wchar_t_, mut _a1: min_allocator_wchar_t_) -> () {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_wchar_t_ {}
impl std::cmp::PartialEq for min_allocator_wchar_t_ {
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_12(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_wchar_t_ {}
pub fn operator_eq_13(mut _a0: min_allocator_char8_t_, mut _a1: min_allocator_char8_t_) -> () {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_char8_t_ {}
impl std::cmp::PartialEq for min_allocator_char8_t_ {
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_13(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_char8_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(40)]
pub struct test_allocator_statistics {
    #[offset(0)]
    pub time_to_throw: (),
    #[offset(4)]
    pub throw_after: (),
    #[offset(8)]
    pub count: (),
    #[offset(12)]
    pub alloc_count: (),
    #[offset(16)]
    pub allocated_size: (),
    #[offset(20)]
    pub construct_count: (),
    #[offset(24)]
    pub destroy_count: (),
    #[offset(28)]
    pub copied: (),
    #[offset(32)]
    pub moved: (),
    #[offset(36)]
    pub converted: (),
}
impl Default for test_allocator_statistics {
    fn default() -> Self {
        {
            test_allocator_statistics {
                time_to_throw: 0_(),
                throw_after: 2147483647_(),
                count: 0_(),
                alloc_count: 0_(),
                allocated_size: 0_(),
                construct_count: 0_(),
                destroy_count: 0_(),
                copied: 0_(),
                moved: 0_(),
                converted: 0_(),
            }
        }
    }
}
thread_local!(
    pub static destructed_value_14: Value<()> = Rc::new(RefCell::new(-1_()));
);
thread_local!(
    pub static moved_value_15: Value<()> = Rc::new(RefCell::new(2147483647_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
pub fn operator_eq_16(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_17(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return !({
        let _x: Ptr<test_allocator_void_> = (x).clone();
        let _y: Ptr<test_allocator_void_> = (y).clone();
        operator_eq_16(_x, _y)
    });
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(16)]
pub struct test_allocator_void_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_void_ {
    pub fn new_1(mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: 0_(),
            id_: 0_(),
            stats_: (stats).clone(),
        }
    }
    pub fn new_2(mut data: ()) -> Self {
        Self {
            data_: data,
            id_: 0_(),
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn new_3(mut data: (), mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: data,
            id_: 0_(),
            stats_: (stats).clone(),
        }
    }
    pub fn new_4(mut data: (), mut id: ()) -> Self {
        Self {
            data_: data,
            id_: id,
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn new_5(mut data: (), mut id: (), mut stats: Ptr<test_allocator_statistics>) -> Self {
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
impl std::cmp::PartialEq for test_allocator_void_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_16(
                Rc::new(RefCell::new(test_allocator_void_ {
                    data_: self.data_.clone(),
                    id_: self.id_.clone(),
                    stats_: self.stats_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(test_allocator_void_ {
                    data_: other.data_.clone(),
                    id_: other.id_.clone(),
                    stats_: other.stats_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_void_ {}
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
impl Default for test_allocator_void_ {
    fn default() -> Self {
        {
            test_allocator_void_ {
                data_: 0_(),
                id_: 0_(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
pub fn operator_eq_18(x: Ptr<test_allocator_char_>, y: Ptr<test_allocator_char_>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_char__rebind_char_ {}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_char_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
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
        unimplemented!();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::PartialEq for test_allocator_char_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_18(
                Rc::new(RefCell::new(test_allocator_char_ {
                    data_: self.data_.clone(),
                    id_: self.id_.clone(),
                    stats_: self.stats_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(test_allocator_char_ {
                    data_: other.data_.clone(),
                    id_: other.id_.clone(),
                    stats_: other.stats_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_char_ {}
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
impl Default for test_allocator_char_ {
    fn default() -> Self {
        {
            test_allocator_char_ {
                data_: 0_(),
                id_: 0_(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
pub fn operator_eq_19(x: Ptr<test_allocator_char16_t_>, y: Ptr<test_allocator_char16_t_>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_char16_t__rebind_char16_t_ {}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_char16_t_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_char16_t_ {
    pub fn copy_from(a: Ptr<test_allocator_char16_t_>) -> Self {
        let __this: Value<test_allocator_char16_t_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_char16_t_> = __this.as_pointer();
        unimplemented!();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::PartialEq for test_allocator_char16_t_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_19(
                Rc::new(RefCell::new(test_allocator_char16_t_ {
                    data_: self.data_.clone(),
                    id_: self.id_.clone(),
                    stats_: self.stats_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(test_allocator_char16_t_ {
                    data_: other.data_.clone(),
                    id_: other.id_.clone(),
                    stats_: other.stats_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_char16_t_ {}
impl Clone for test_allocator_char16_t_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_char16_t_> =
            Rc::new(RefCell::new(test_allocator_char16_t_ {
                data_: self.data_.clone(),
                id_: self.id_.clone(),
                stats_: self.stats_.clone(),
            }));
        test_allocator_char16_t_::copy_from(__src.as_pointer())
    }
}
impl Default for test_allocator_char16_t_ {
    fn default() -> Self {
        {
            test_allocator_char16_t_ {
                data_: 0_(),
                id_: 0_(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
pub fn operator_eq_20(x: Ptr<test_allocator_char32_t_>, y: Ptr<test_allocator_char32_t_>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_char32_t__rebind_char32_t_ {}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_char32_t_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_char32_t_ {
    pub fn copy_from(a: Ptr<test_allocator_char32_t_>) -> Self {
        let __this: Value<test_allocator_char32_t_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_char32_t_> = __this.as_pointer();
        unimplemented!();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::PartialEq for test_allocator_char32_t_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_20(
                Rc::new(RefCell::new(test_allocator_char32_t_ {
                    data_: self.data_.clone(),
                    id_: self.id_.clone(),
                    stats_: self.stats_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(test_allocator_char32_t_ {
                    data_: other.data_.clone(),
                    id_: other.id_.clone(),
                    stats_: other.stats_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_char32_t_ {}
impl Clone for test_allocator_char32_t_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_char32_t_> =
            Rc::new(RefCell::new(test_allocator_char32_t_ {
                data_: self.data_.clone(),
                id_: self.id_.clone(),
                stats_: self.stats_.clone(),
            }));
        test_allocator_char32_t_::copy_from(__src.as_pointer())
    }
}
impl Default for test_allocator_char32_t_ {
    fn default() -> Self {
        {
            test_allocator_char32_t_ {
                data_: 0_(),
                id_: 0_(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
pub fn operator_eq_21(x: Ptr<test_allocator_wchar_t_>, y: Ptr<test_allocator_wchar_t_>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_wchar_t__rebind_wchar_t_ {}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_wchar_t_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
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
        unimplemented!();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::PartialEq for test_allocator_wchar_t_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_21(
                Rc::new(RefCell::new(test_allocator_wchar_t_ {
                    data_: self.data_.clone(),
                    id_: self.id_.clone(),
                    stats_: self.stats_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(test_allocator_wchar_t_ {
                    data_: other.data_.clone(),
                    id_: other.id_.clone(),
                    stats_: other.stats_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_wchar_t_ {}
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
impl Default for test_allocator_wchar_t_ {
    fn default() -> Self {
        {
            test_allocator_wchar_t_ {
                data_: 0_(),
                id_: 0_(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
pub fn operator_eq_22(x: Ptr<test_allocator_char8_t_>, y: Ptr<test_allocator_char8_t_>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_char8_t__rebind_char8_t_ {}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_char8_t_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_char8_t_ {
    pub fn copy_from(a: Ptr<test_allocator_char8_t_>) -> Self {
        let __this: Value<test_allocator_char8_t_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_char8_t_> = __this.as_pointer();
        unimplemented!();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::PartialEq for test_allocator_char8_t_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_22(
                Rc::new(RefCell::new(test_allocator_char8_t_ {
                    data_: self.data_.clone(),
                    id_: self.id_.clone(),
                    stats_: self.stats_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(test_allocator_char8_t_ {
                    data_: other.data_.clone(),
                    id_: other.id_.clone(),
                    stats_: other.stats_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_char8_t_ {}
impl Clone for test_allocator_char8_t_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_char8_t_> =
            Rc::new(RefCell::new(test_allocator_char8_t_ {
                data_: self.data_.clone(),
                id_: self.id_.clone(),
                stats_: self.stats_.clone(),
            }));
        test_allocator_char8_t_::copy_from(__src.as_pointer())
    }
}
impl Default for test_allocator_char8_t_ {
    fn default() -> Self {
        {
            test_allocator_char8_t_ {
                data_: 0_(),
                id_: 0_(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Ctor_Tag {}
#[derive(Record, ByteRepr, FnPtrArg, CopyAssign, MoveAssign, Default)]
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
pub fn is_string_asan_correct_23(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_24(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_25(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_26(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_27(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_28(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_29(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_30(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_31(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_32(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_33(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_34(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_35(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_36(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_37(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_38(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_39(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_40(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_41(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_42(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_43(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_44(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_45(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_46(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_47(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_48(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_49(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_50(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_51(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_52(_a0: Ptr<()>) -> () {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct should_throw_exception_t {}
thread_local!(
    pub static should_throw_exception_53: Value<should_throw_exception_t> =
        Rc::new(RefCell::new(<should_throw_exception_t>::default()));
);
pub fn test_54(orig: (), mut pos: (), mut n: (), mut expected: ()) {
    let orig: Value<()> = Rc::new(RefCell::new(orig));
    let str: Value<()> = Rc::new(RefCell::new(
        ({ (*unimplemented!().upgrade().deref()).substr_131(pos, n) }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_55(orig: (), mut pos: (), mut n: (), mut expected: ()) {
    let orig: Value<()> = Rc::new(RefCell::new(orig));
    let str: Value<()> = Rc::new(RefCell::new(
        ({ (*unimplemented!().upgrade().deref()).substr_131(pos, n) }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_56(orig: (), mut pos: (), mut n: (), mut expected: ()) {
    let orig: Value<()> = Rc::new(RefCell::new(orig));
    let str: Value<()> = Rc::new(RefCell::new(
        ({ (*unimplemented!().upgrade().deref()).substr_131(pos, n) }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_57(orig: (), mut pos: (), mut n: (), mut expected: ()) {
    let orig: Value<()> = Rc::new(RefCell::new(orig));
    let str: Value<()> = Rc::new(RefCell::new(
        ({ (*unimplemented!().upgrade().deref()).substr_131(pos, n) }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_58(orig: (), mut pos: (), mut n: (), mut expected: ()) {
    let orig: Value<()> = Rc::new(RefCell::new(orig));
    let str: Value<()> = Rc::new(RefCell::new(
        ({ (*unimplemented!().upgrade().deref()).substr_131(pos, n) }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_59(orig: (), mut pos: (), mut n: (), mut expected: ()) {
    let orig: Value<()> = Rc::new(RefCell::new(orig));
    let str: Value<()> = Rc::new(RefCell::new(
        ({ (*unimplemented!().upgrade().deref()).substr_131(pos, n) }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_60(orig: (), mut pos: (), mut n: (), mut expected: ()) {
    let orig: Value<()> = Rc::new(RefCell::new(orig));
    let str: Value<()> = Rc::new(RefCell::new(
        ({ (*unimplemented!().upgrade().deref()).substr_131(pos, n) }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_61(orig: (), mut pos: (), mut n: (), mut expected: ()) {
    let orig: Value<()> = Rc::new(RefCell::new(orig));
    let str: Value<()> = Rc::new(RefCell::new(
        ({ (*unimplemented!().upgrade().deref()).substr_131(pos, n) }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_62(orig: (), mut pos: (), mut n: (), mut expected: ()) {
    let orig: Value<()> = Rc::new(RefCell::new(orig));
    let str: Value<()> = Rc::new(RefCell::new(
        ({ (*unimplemented!().upgrade().deref()).substr_131(pos, n) }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_63(orig: (), mut pos: (), mut n: (), mut expected: ()) {
    let orig: Value<()> = Rc::new(RefCell::new(orig));
    let str: Value<()> = Rc::new(RefCell::new(
        ({ (*unimplemented!().upgrade().deref()).substr_131(pos, n) }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_64(orig: (), mut pos: (), mut n: (), mut expected: ()) {
    let orig: Value<()> = Rc::new(RefCell::new(orig));
    let str: Value<()> = Rc::new(RefCell::new(
        ({ (*unimplemented!().upgrade().deref()).substr_131(pos, n) }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_65(orig: (), mut pos: (), mut n: (), mut expected: ()) {
    let orig: Value<()> = Rc::new(RefCell::new(orig));
    let str: Value<()> = Rc::new(RefCell::new(
        ({ (*unimplemented!().upgrade().deref()).substr_131(pos, n) }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_66(orig: (), mut pos: (), mut n: (), mut expected: ()) {
    let orig: Value<()> = Rc::new(RefCell::new(orig));
    let str: Value<()> = Rc::new(RefCell::new(
        ({ (*unimplemented!().upgrade().deref()).substr_131(pos, n) }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_67(orig: (), mut pos: (), mut n: (), mut expected: ()) {
    let orig: Value<()> = Rc::new(RefCell::new(orig));
    let str: Value<()> = Rc::new(RefCell::new(
        ({ (*unimplemented!().upgrade().deref()).substr_131(pos, n) }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_68(orig: (), mut pos: (), mut n: (), mut expected: ()) {
    let orig: Value<()> = Rc::new(RefCell::new(orig));
    let str: Value<()> = Rc::new(RefCell::new(
        ({ (*unimplemented!().upgrade().deref()).substr_131(pos, n) }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_69(orig: (), mut pos: (), mut n: (), mut expected: ()) {
    let orig: Value<()> = Rc::new(RefCell::new(orig));
    let str: Value<()> = Rc::new(RefCell::new(
        ({ (*unimplemented!().upgrade().deref()).substr_131(pos, n) }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_70(orig: (), mut pos: (), mut n: (), mut expected: ()) {
    let orig: Value<()> = Rc::new(RefCell::new(orig));
    let str: Value<()> = Rc::new(RefCell::new(
        ({ (*unimplemented!().upgrade().deref()).substr_131(pos, n) }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_71(orig: (), mut pos: (), mut n: (), mut expected: ()) {
    let orig: Value<()> = Rc::new(RefCell::new(orig));
    let str: Value<()> = Rc::new(RefCell::new(
        ({ (*unimplemented!().upgrade().deref()).substr_131(pos, n) }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_72(orig: (), mut pos: (), mut n: (), mut expected: ()) {
    let orig: Value<()> = Rc::new(RefCell::new(orig));
    let str: Value<()> = Rc::new(RefCell::new(
        ({ (*unimplemented!().upgrade().deref()).substr_131(pos, n) }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_73(orig: (), mut pos: (), mut n: (), mut expected: ()) {
    let orig: Value<()> = Rc::new(RefCell::new(orig));
    let str: Value<()> = Rc::new(RefCell::new(
        ({ (*unimplemented!().upgrade().deref()).substr_131(pos, n) }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_74(orig: (), mut pos: (), mut n: (), mut expected: ()) {
    let orig: Value<()> = Rc::new(RefCell::new(orig));
    let str: Value<()> = Rc::new(RefCell::new(
        ({ (*unimplemented!().upgrade().deref()).substr_131(pos, n) }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_75(orig: (), mut pos: (), mut n: (), mut expected: ()) {
    let orig: Value<()> = Rc::new(RefCell::new(orig));
    let str: Value<()> = Rc::new(RefCell::new(
        ({ (*unimplemented!().upgrade().deref()).substr_131(pos, n) }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_76(orig: (), mut pos: (), mut n: (), mut expected: ()) {
    let orig: Value<()> = Rc::new(RefCell::new(orig));
    let str: Value<()> = Rc::new(RefCell::new(
        ({ (*unimplemented!().upgrade().deref()).substr_131(pos, n) }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_77(orig: (), mut pos: (), mut n: (), mut expected: ()) {
    let orig: Value<()> = Rc::new(RefCell::new(orig));
    let str: Value<()> = Rc::new(RefCell::new(
        ({ (*unimplemented!().upgrade().deref()).substr_131(pos, n) }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_78(orig: (), mut pos: (), mut n: (), mut expected: ()) {
    let orig: Value<()> = Rc::new(RefCell::new(orig));
    let str: Value<()> = Rc::new(RefCell::new(
        ({ (*unimplemented!().upgrade().deref()).substr_131(pos, n) }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_79(orig: (), mut pos: (), mut n: (), mut expected: ()) {
    let orig: Value<()> = Rc::new(RefCell::new(orig));
    let str: Value<()> = Rc::new(RefCell::new(
        ({ (*unimplemented!().upgrade().deref()).substr_131(pos, n) }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_80(orig: (), mut pos: (), mut n: (), mut expected: ()) {
    let orig: Value<()> = Rc::new(RefCell::new(orig));
    let str: Value<()> = Rc::new(RefCell::new(
        ({ (*unimplemented!().upgrade().deref()).substr_131(pos, n) }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_81(orig: (), mut pos: (), mut n: (), mut expected: ()) {
    let orig: Value<()> = Rc::new(RefCell::new(orig));
    let str: Value<()> = Rc::new(RefCell::new(
        ({ (*unimplemented!().upgrade().deref()).substr_131(pos, n) }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_82(orig: (), mut pos: (), mut n: (), mut expected: ()) {
    let orig: Value<()> = Rc::new(RefCell::new(orig));
    let str: Value<()> = Rc::new(RefCell::new(
        ({ (*unimplemented!().upgrade().deref()).substr_131(pos, n) }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_83(orig: (), mut pos: (), mut n: (), mut expected: ()) {
    let orig: Value<()> = Rc::new(RefCell::new(orig));
    let str: Value<()> = Rc::new(RefCell::new(
        ({ (*unimplemented!().upgrade().deref()).substr_131(pos, n) }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_84(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub fn test_85(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub fn test_86(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub fn test_87(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub fn test_88(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub fn test_89(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub fn test_90(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub fn test_91(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub fn test_92(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub fn test_93(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub fn test_94(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub fn test_95(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub fn test_96(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub fn test_97(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub fn test_98(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub fn test_99(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub fn test_100(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub fn test_101(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub fn test_102(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub fn test_103(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub fn test_104(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub fn test_105(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub fn test_106(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub fn test_107(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub fn test_108(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub fn test_109(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub fn test_110(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub fn test_111(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub fn test_112(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub fn test_113(mut orig: (), mut pos: (), mut n: (), mut _a3: should_throw_exception_t) {
    &(orig);
    &(pos);
    &(n);
}
pub fn test_string_114() {
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_54(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_54(_orig, 0_(), 1_(), _expected)
    });
    ({
        test_84(
            unimplemented!(),
            1_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        test_84(
            unimplemented!(),
            1_(),
            1_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_54(_orig, 0_(), 1_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_54(_orig, 5_(), 5_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_54(_orig, 12_(), 5_(), _expected)
    });
    ({
        test_84(
            unimplemented!(),
            13_(),
            5_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_54(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_54(_orig, 0_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_54(_orig, 10_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_54(_orig, 20_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_54(_orig, 26_(), 10_(), _expected)
    });
    ({
        test_84(
            unimplemented!(),
            27_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
}
pub fn test_string_115() {
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_55(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_55(_orig, 0_(), 1_(), _expected)
    });
    ({
        test_85(
            unimplemented!(),
            1_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        test_85(
            unimplemented!(),
            1_(),
            1_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_55(_orig, 0_(), 1_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_55(_orig, 5_(), 5_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_55(_orig, 12_(), 5_(), _expected)
    });
    ({
        test_85(
            unimplemented!(),
            13_(),
            5_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_55(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_55(_orig, 0_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_55(_orig, 10_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_55(_orig, 20_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_55(_orig, 26_(), 10_(), _expected)
    });
    ({
        test_85(
            unimplemented!(),
            27_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
}
pub fn test_string_116() {
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_56(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_56(_orig, 0_(), 1_(), _expected)
    });
    ({
        test_86(
            unimplemented!(),
            1_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        test_86(
            unimplemented!(),
            1_(),
            1_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_56(_orig, 0_(), 1_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_56(_orig, 5_(), 5_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_56(_orig, 12_(), 5_(), _expected)
    });
    ({
        test_86(
            unimplemented!(),
            13_(),
            5_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_56(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_56(_orig, 0_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_56(_orig, 10_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_56(_orig, 20_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_56(_orig, 26_(), 10_(), _expected)
    });
    ({
        test_86(
            unimplemented!(),
            27_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
}
pub fn test_string_117() {
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_57(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_57(_orig, 0_(), 1_(), _expected)
    });
    ({
        test_87(
            unimplemented!(),
            1_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        test_87(
            unimplemented!(),
            1_(),
            1_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_57(_orig, 0_(), 1_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_57(_orig, 5_(), 5_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_57(_orig, 12_(), 5_(), _expected)
    });
    ({
        test_87(
            unimplemented!(),
            13_(),
            5_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_57(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_57(_orig, 0_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_57(_orig, 10_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_57(_orig, 20_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_57(_orig, 26_(), 10_(), _expected)
    });
    ({
        test_87(
            unimplemented!(),
            27_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
}
pub fn test_string_118() {
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_58(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_58(_orig, 0_(), 1_(), _expected)
    });
    ({
        test_88(
            unimplemented!(),
            1_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        test_88(
            unimplemented!(),
            1_(),
            1_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_58(_orig, 0_(), 1_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_58(_orig, 5_(), 5_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_58(_orig, 12_(), 5_(), _expected)
    });
    ({
        test_88(
            unimplemented!(),
            13_(),
            5_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_58(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_58(_orig, 0_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_58(_orig, 10_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_58(_orig, 20_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_58(_orig, 26_(), 10_(), _expected)
    });
    ({
        test_88(
            unimplemented!(),
            27_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
}
pub fn test_string_119() {
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_59(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_59(_orig, 0_(), 1_(), _expected)
    });
    ({
        test_89(
            unimplemented!(),
            1_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        test_89(
            unimplemented!(),
            1_(),
            1_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_59(_orig, 0_(), 1_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_59(_orig, 5_(), 5_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_59(_orig, 12_(), 5_(), _expected)
    });
    ({
        test_89(
            unimplemented!(),
            13_(),
            5_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_59(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_59(_orig, 0_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_59(_orig, 10_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_59(_orig, 20_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_59(_orig, 26_(), 10_(), _expected)
    });
    ({
        test_89(
            unimplemented!(),
            27_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
}
pub fn test_string_120() {
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_60(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_60(_orig, 0_(), 1_(), _expected)
    });
    ({
        test_90(
            unimplemented!(),
            1_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        test_90(
            unimplemented!(),
            1_(),
            1_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_60(_orig, 0_(), 1_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_60(_orig, 5_(), 5_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_60(_orig, 12_(), 5_(), _expected)
    });
    ({
        test_90(
            unimplemented!(),
            13_(),
            5_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_60(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_60(_orig, 0_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_60(_orig, 10_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_60(_orig, 20_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_60(_orig, 26_(), 10_(), _expected)
    });
    ({
        test_90(
            unimplemented!(),
            27_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
}
pub fn test_string_121() {
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_61(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_61(_orig, 0_(), 1_(), _expected)
    });
    ({
        test_91(
            unimplemented!(),
            1_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        test_91(
            unimplemented!(),
            1_(),
            1_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_61(_orig, 0_(), 1_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_61(_orig, 5_(), 5_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_61(_orig, 12_(), 5_(), _expected)
    });
    ({
        test_91(
            unimplemented!(),
            13_(),
            5_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_61(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_61(_orig, 0_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_61(_orig, 10_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_61(_orig, 20_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_61(_orig, 26_(), 10_(), _expected)
    });
    ({
        test_91(
            unimplemented!(),
            27_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
}
pub fn test_string_122() {
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_62(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_62(_orig, 0_(), 1_(), _expected)
    });
    ({
        test_92(
            unimplemented!(),
            1_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        test_92(
            unimplemented!(),
            1_(),
            1_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_62(_orig, 0_(), 1_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_62(_orig, 5_(), 5_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_62(_orig, 12_(), 5_(), _expected)
    });
    ({
        test_92(
            unimplemented!(),
            13_(),
            5_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_62(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_62(_orig, 0_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_62(_orig, 10_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_62(_orig, 20_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_62(_orig, 26_(), 10_(), _expected)
    });
    ({
        test_92(
            unimplemented!(),
            27_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
}
pub fn test_string_123() {
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_63(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_63(_orig, 0_(), 1_(), _expected)
    });
    ({
        test_93(
            unimplemented!(),
            1_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        test_93(
            unimplemented!(),
            1_(),
            1_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_63(_orig, 0_(), 1_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_63(_orig, 5_(), 5_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_63(_orig, 12_(), 5_(), _expected)
    });
    ({
        test_93(
            unimplemented!(),
            13_(),
            5_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_63(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_63(_orig, 0_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_63(_orig, 10_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_63(_orig, 20_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_63(_orig, 26_(), 10_(), _expected)
    });
    ({
        test_93(
            unimplemented!(),
            27_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
}
pub fn test_string_124() {
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_64(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_64(_orig, 0_(), 1_(), _expected)
    });
    ({
        test_94(
            unimplemented!(),
            1_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        test_94(
            unimplemented!(),
            1_(),
            1_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_64(_orig, 0_(), 1_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_64(_orig, 5_(), 5_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_64(_orig, 12_(), 5_(), _expected)
    });
    ({
        test_94(
            unimplemented!(),
            13_(),
            5_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_64(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_64(_orig, 0_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_64(_orig, 10_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_64(_orig, 20_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_64(_orig, 26_(), 10_(), _expected)
    });
    ({
        test_94(
            unimplemented!(),
            27_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
}
pub fn test_string_125() {
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_65(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_65(_orig, 0_(), 1_(), _expected)
    });
    ({
        test_95(
            unimplemented!(),
            1_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        test_95(
            unimplemented!(),
            1_(),
            1_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_65(_orig, 0_(), 1_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_65(_orig, 5_(), 5_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_65(_orig, 12_(), 5_(), _expected)
    });
    ({
        test_95(
            unimplemented!(),
            13_(),
            5_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_65(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_65(_orig, 0_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_65(_orig, 10_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_65(_orig, 20_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_65(_orig, 26_(), 10_(), _expected)
    });
    ({
        test_95(
            unimplemented!(),
            27_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
}
pub fn test_string_126() {
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_66(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_66(_orig, 0_(), 1_(), _expected)
    });
    ({
        test_96(
            unimplemented!(),
            1_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        test_96(
            unimplemented!(),
            1_(),
            1_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_66(_orig, 0_(), 1_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_66(_orig, 5_(), 5_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_66(_orig, 12_(), 5_(), _expected)
    });
    ({
        test_96(
            unimplemented!(),
            13_(),
            5_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_66(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_66(_orig, 0_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_66(_orig, 10_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_66(_orig, 20_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_66(_orig, 26_(), 10_(), _expected)
    });
    ({
        test_96(
            unimplemented!(),
            27_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
}
pub fn test_string_127() {
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_67(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_67(_orig, 0_(), 1_(), _expected)
    });
    ({
        test_97(
            unimplemented!(),
            1_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        test_97(
            unimplemented!(),
            1_(),
            1_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_67(_orig, 0_(), 1_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_67(_orig, 5_(), 5_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_67(_orig, 12_(), 5_(), _expected)
    });
    ({
        test_97(
            unimplemented!(),
            13_(),
            5_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_67(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_67(_orig, 0_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_67(_orig, 10_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_67(_orig, 20_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_67(_orig, 26_(), 10_(), _expected)
    });
    ({
        test_97(
            unimplemented!(),
            27_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
}
pub fn test_string_128() {
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_68(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_68(_orig, 0_(), 1_(), _expected)
    });
    ({
        test_98(
            unimplemented!(),
            1_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        test_98(
            unimplemented!(),
            1_(),
            1_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_68(_orig, 0_(), 1_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_68(_orig, 5_(), 5_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_68(_orig, 12_(), 5_(), _expected)
    });
    ({
        test_98(
            unimplemented!(),
            13_(),
            5_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_68(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_68(_orig, 0_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_68(_orig, 10_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_68(_orig, 20_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_68(_orig, 26_(), 10_(), _expected)
    });
    ({
        test_98(
            unimplemented!(),
            27_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
}
pub fn test_string_129() {
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_69(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_69(_orig, 0_(), 1_(), _expected)
    });
    ({
        test_99(
            unimplemented!(),
            1_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        test_99(
            unimplemented!(),
            1_(),
            1_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_69(_orig, 0_(), 1_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_69(_orig, 5_(), 5_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_69(_orig, 12_(), 5_(), _expected)
    });
    ({
        test_99(
            unimplemented!(),
            13_(),
            5_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_69(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_69(_orig, 0_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_69(_orig, 10_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_69(_orig, 20_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_69(_orig, 26_(), 10_(), _expected)
    });
    ({
        test_99(
            unimplemented!(),
            27_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
}
pub fn test_string_130() {
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_70(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_70(_orig, 0_(), 1_(), _expected)
    });
    ({
        test_100(
            unimplemented!(),
            1_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        test_100(
            unimplemented!(),
            1_(),
            1_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_70(_orig, 0_(), 1_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_70(_orig, 5_(), 5_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_70(_orig, 12_(), 5_(), _expected)
    });
    ({
        test_100(
            unimplemented!(),
            13_(),
            5_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_70(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_70(_orig, 0_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_70(_orig, 10_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_70(_orig, 20_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_70(_orig, 26_(), 10_(), _expected)
    });
    ({
        test_100(
            unimplemented!(),
            27_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
}
pub fn test_string_131() {
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_71(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_71(_orig, 0_(), 1_(), _expected)
    });
    ({
        test_101(
            unimplemented!(),
            1_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        test_101(
            unimplemented!(),
            1_(),
            1_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_71(_orig, 0_(), 1_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_71(_orig, 5_(), 5_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_71(_orig, 12_(), 5_(), _expected)
    });
    ({
        test_101(
            unimplemented!(),
            13_(),
            5_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_71(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_71(_orig, 0_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_71(_orig, 10_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_71(_orig, 20_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_71(_orig, 26_(), 10_(), _expected)
    });
    ({
        test_101(
            unimplemented!(),
            27_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
}
pub fn test_string_132() {
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_72(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_72(_orig, 0_(), 1_(), _expected)
    });
    ({
        test_102(
            unimplemented!(),
            1_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        test_102(
            unimplemented!(),
            1_(),
            1_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_72(_orig, 0_(), 1_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_72(_orig, 5_(), 5_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_72(_orig, 12_(), 5_(), _expected)
    });
    ({
        test_102(
            unimplemented!(),
            13_(),
            5_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_72(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_72(_orig, 0_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_72(_orig, 10_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_72(_orig, 20_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_72(_orig, 26_(), 10_(), _expected)
    });
    ({
        test_102(
            unimplemented!(),
            27_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
}
pub fn test_string_133() {
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_73(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_73(_orig, 0_(), 1_(), _expected)
    });
    ({
        test_103(
            unimplemented!(),
            1_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        test_103(
            unimplemented!(),
            1_(),
            1_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_73(_orig, 0_(), 1_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_73(_orig, 5_(), 5_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_73(_orig, 12_(), 5_(), _expected)
    });
    ({
        test_103(
            unimplemented!(),
            13_(),
            5_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_73(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_73(_orig, 0_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_73(_orig, 10_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_73(_orig, 20_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_73(_orig, 26_(), 10_(), _expected)
    });
    ({
        test_103(
            unimplemented!(),
            27_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
}
pub fn test_string_134() {
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_74(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_74(_orig, 0_(), 1_(), _expected)
    });
    ({
        test_104(
            unimplemented!(),
            1_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        test_104(
            unimplemented!(),
            1_(),
            1_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_74(_orig, 0_(), 1_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_74(_orig, 5_(), 5_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_74(_orig, 12_(), 5_(), _expected)
    });
    ({
        test_104(
            unimplemented!(),
            13_(),
            5_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_74(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_74(_orig, 0_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_74(_orig, 10_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_74(_orig, 20_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_74(_orig, 26_(), 10_(), _expected)
    });
    ({
        test_104(
            unimplemented!(),
            27_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
}
pub fn test_string_135() {
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_75(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_75(_orig, 0_(), 1_(), _expected)
    });
    ({
        test_105(
            unimplemented!(),
            1_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        test_105(
            unimplemented!(),
            1_(),
            1_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_75(_orig, 0_(), 1_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_75(_orig, 5_(), 5_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_75(_orig, 12_(), 5_(), _expected)
    });
    ({
        test_105(
            unimplemented!(),
            13_(),
            5_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_75(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_75(_orig, 0_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_75(_orig, 10_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_75(_orig, 20_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_75(_orig, 26_(), 10_(), _expected)
    });
    ({
        test_105(
            unimplemented!(),
            27_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
}
pub fn test_string_136() {
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_76(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_76(_orig, 0_(), 1_(), _expected)
    });
    ({
        test_106(
            unimplemented!(),
            1_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        test_106(
            unimplemented!(),
            1_(),
            1_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_76(_orig, 0_(), 1_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_76(_orig, 5_(), 5_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_76(_orig, 12_(), 5_(), _expected)
    });
    ({
        test_106(
            unimplemented!(),
            13_(),
            5_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_76(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_76(_orig, 0_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_76(_orig, 10_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_76(_orig, 20_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_76(_orig, 26_(), 10_(), _expected)
    });
    ({
        test_106(
            unimplemented!(),
            27_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
}
pub fn test_string_137() {
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_77(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_77(_orig, 0_(), 1_(), _expected)
    });
    ({
        test_107(
            unimplemented!(),
            1_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        test_107(
            unimplemented!(),
            1_(),
            1_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_77(_orig, 0_(), 1_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_77(_orig, 5_(), 5_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_77(_orig, 12_(), 5_(), _expected)
    });
    ({
        test_107(
            unimplemented!(),
            13_(),
            5_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_77(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_77(_orig, 0_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_77(_orig, 10_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_77(_orig, 20_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_77(_orig, 26_(), 10_(), _expected)
    });
    ({
        test_107(
            unimplemented!(),
            27_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
}
pub fn test_string_138() {
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_78(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_78(_orig, 0_(), 1_(), _expected)
    });
    ({
        test_108(
            unimplemented!(),
            1_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        test_108(
            unimplemented!(),
            1_(),
            1_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_78(_orig, 0_(), 1_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_78(_orig, 5_(), 5_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_78(_orig, 12_(), 5_(), _expected)
    });
    ({
        test_108(
            unimplemented!(),
            13_(),
            5_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_78(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_78(_orig, 0_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_78(_orig, 10_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_78(_orig, 20_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_78(_orig, 26_(), 10_(), _expected)
    });
    ({
        test_108(
            unimplemented!(),
            27_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
}
pub fn test_string_139() {
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_79(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_79(_orig, 0_(), 1_(), _expected)
    });
    ({
        test_109(
            unimplemented!(),
            1_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        test_109(
            unimplemented!(),
            1_(),
            1_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_79(_orig, 0_(), 1_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_79(_orig, 5_(), 5_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_79(_orig, 12_(), 5_(), _expected)
    });
    ({
        test_109(
            unimplemented!(),
            13_(),
            5_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_79(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_79(_orig, 0_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_79(_orig, 10_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_79(_orig, 20_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_79(_orig, 26_(), 10_(), _expected)
    });
    ({
        test_109(
            unimplemented!(),
            27_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
}
pub fn test_string_140() {
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_80(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_80(_orig, 0_(), 1_(), _expected)
    });
    ({
        test_110(
            unimplemented!(),
            1_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        test_110(
            unimplemented!(),
            1_(),
            1_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_80(_orig, 0_(), 1_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_80(_orig, 5_(), 5_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_80(_orig, 12_(), 5_(), _expected)
    });
    ({
        test_110(
            unimplemented!(),
            13_(),
            5_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_80(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_80(_orig, 0_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_80(_orig, 10_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_80(_orig, 20_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_80(_orig, 26_(), 10_(), _expected)
    });
    ({
        test_110(
            unimplemented!(),
            27_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
}
pub fn test_string_141() {
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_81(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_81(_orig, 0_(), 1_(), _expected)
    });
    ({
        test_111(
            unimplemented!(),
            1_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        test_111(
            unimplemented!(),
            1_(),
            1_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_81(_orig, 0_(), 1_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_81(_orig, 5_(), 5_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_81(_orig, 12_(), 5_(), _expected)
    });
    ({
        test_111(
            unimplemented!(),
            13_(),
            5_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_81(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_81(_orig, 0_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_81(_orig, 10_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_81(_orig, 20_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_81(_orig, 26_(), 10_(), _expected)
    });
    ({
        test_111(
            unimplemented!(),
            27_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
}
pub fn test_string_142() {
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_82(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_82(_orig, 0_(), 1_(), _expected)
    });
    ({
        test_112(
            unimplemented!(),
            1_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        test_112(
            unimplemented!(),
            1_(),
            1_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_82(_orig, 0_(), 1_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_82(_orig, 5_(), 5_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_82(_orig, 12_(), 5_(), _expected)
    });
    ({
        test_112(
            unimplemented!(),
            13_(),
            5_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_82(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_82(_orig, 0_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_82(_orig, 10_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_82(_orig, 20_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_82(_orig, 26_(), 10_(), _expected)
    });
    ({
        test_112(
            unimplemented!(),
            27_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
}
pub fn test_string_143() {
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_83(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_83(_orig, 0_(), 1_(), _expected)
    });
    ({
        test_113(
            unimplemented!(),
            1_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        test_113(
            unimplemented!(),
            1_(),
            1_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_83(_orig, 0_(), 1_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_83(_orig, 5_(), 5_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_83(_orig, 12_(), 5_(), _expected)
    });
    ({
        test_113(
            unimplemented!(),
            13_(),
            5_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_83(_orig, 0_(), 0_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_83(_orig, 0_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_83(_orig, 10_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_83(_orig, 20_(), 10_(), _expected)
    });
    ({
        let _orig: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_83(_orig, 26_(), 10_(), _expected)
    });
    ({
        test_113(
            unimplemented!(),
            27_(),
            0_(),
            (*should_throw_exception_53.with(Value::clone).borrow()).clone(),
        )
    });
}
pub fn test_allocators_144() {
    ({ test_string_114() });
    ({ test_string_115() });
    ({ test_string_116() });
}
pub fn test_allocators_145() {
    ({ test_string_117() });
    ({ test_string_118() });
    ({ test_string_119() });
}
pub fn test_allocators_146() {
    ({ test_string_120() });
    ({ test_string_121() });
    ({ test_string_122() });
}
pub fn test_allocators_147() {
    ({ test_string_123() });
    ({ test_string_124() });
    ({ test_string_125() });
}
pub fn test_allocators_148() {
    ({ test_string_126() });
    ({ test_string_127() });
    ({ test_string_128() });
}
pub fn test_allocators_149() {
    ({ test_string_129() });
    ({ test_string_130() });
    ({ test_string_131() });
}
pub fn test_allocators_150() {
    ({ test_string_132() });
    ({ test_string_133() });
    ({ test_string_134() });
}
pub fn test_allocators_151() {
    ({ test_string_135() });
    ({ test_string_136() });
    ({ test_string_137() });
}
pub fn test_allocators_152() {
    ({ test_string_138() });
    ({ test_string_139() });
    ({ test_string_140() });
}
pub fn test_allocators_153() {
    ({ test_string_141() });
    ({ test_string_142() });
    ({ test_string_143() });
}
pub fn test_char_traits_154() {
    ({ test_allocators_144() });
    ({ test_allocators_145() });
}
pub fn test_char_traits_155() {
    ({ test_allocators_146() });
    ({ test_allocators_147() });
}
pub fn test_char_traits_156() {
    ({ test_allocators_148() });
    ({ test_allocators_149() });
}
pub fn test_char_traits_157() {
    ({ test_allocators_150() });
    ({ test_allocators_151() });
}
pub fn test_char_traits_158() {
    ({ test_allocators_152() });
    ({ test_allocators_153() });
}
pub fn test_159() -> () {
    ({ test_char_traits_154() });
    ({ test_char_traits_155() });
    ({ test_char_traits_156() });
    ({ test_char_traits_157() });
    ({ test_char_traits_158() });
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
    ({ test_159() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait MultiStringTypeImpl {
    fn as_ptr_2(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_3(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_4(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_5(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_6(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn length_7(&self, _a0: Ptr<()>) -> ();
    fn length_8(&self, _a0: Ptr<()>) -> ();
    fn length_9(&self, _a0: Ptr<()>) -> ();
    fn length_10(&self, _a0: Ptr<()>) -> ();
    fn length_11(&self, _a0: Ptr<()>) -> ();
    fn to_pconst_12(&self) -> Ptr<()>;
    fn to_pconst_13(&self) -> Ptr<()>;
    fn to_pconst_14(&self) -> Ptr<()>;
    fn to_pconst_15(&self) -> Ptr<()>;
    fn to_pconst_16(&self) -> Ptr<()>;
}
impl MultiStringTypeImpl for Ptr<MultiStringType> {
    fn as_ptr_2(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.s_.clone());
    }
    fn as_ptr_3(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.w_.clone());
    }
    fn as_ptr_4(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.u8_.clone());
    }
    fn as_ptr_5(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.u16_.clone());
    }
    fn as_ptr_6(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.u32_.clone());
    }
    fn length_7(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.sn_);
    }
    fn length_8(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.wn_);
    }
    fn length_9(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.u8n_);
    }
    fn length_10(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.u16n_);
    }
    fn length_11(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.u32n_);
    }
    fn to_pconst_12(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.s_.clone());
    }
    fn to_pconst_13(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.w_.clone());
    }
    fn to_pconst_14(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.u8_.clone());
    }
    fn to_pconst_15(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.u16_.clone());
    }
    fn to_pconst_16(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.u32_.clone());
    }
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
pub trait min_allocator_char16_t_Impl {
    fn allocate(&self, n: ()) -> min_pointer_char16_t__std_integral_constant_unsigned_long__0__;
    fn deallocate(&self, p: min_pointer_char16_t__std_integral_constant_unsigned_long__0__, n: ());
}
impl min_allocator_char16_t_Impl for Ptr<min_allocator_char16_t_> {
    fn allocate(
        &self,
        mut n: (),
    ) -> min_pointer_char16_t__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char16_t__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_char16_t__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
pub trait min_allocator_char32_t_Impl {
    fn allocate(&self, n: ()) -> min_pointer_char32_t__std_integral_constant_unsigned_long__0__;
    fn deallocate(&self, p: min_pointer_char32_t__std_integral_constant_unsigned_long__0__, n: ());
}
impl min_allocator_char32_t_Impl for Ptr<min_allocator_char32_t_> {
    fn allocate(
        &self,
        mut n: (),
    ) -> min_pointer_char32_t__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char32_t__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_char32_t__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
pub trait min_allocator_char8_t_Impl {
    fn allocate(&self, n: ()) -> min_pointer_char8_t__std_integral_constant_unsigned_long__0__;
    fn deallocate(&self, p: min_pointer_char8_t__std_integral_constant_unsigned_long__0__, n: ());
}
impl min_allocator_char8_t_Impl for Ptr<min_allocator_char8_t_> {
    fn allocate(&self, mut n: ()) -> min_pointer_char8_t__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char8_t__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_char8_t__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
pub trait min_allocator_char_Impl {
    fn allocate(&self, n: ()) -> min_pointer_char__std_integral_constant_unsigned_long__0__;
    fn deallocate(&self, p: min_pointer_char__std_integral_constant_unsigned_long__0__, n: ());
}
impl min_allocator_char_Impl for Ptr<min_allocator_char_> {
    fn allocate(&self, mut n: ()) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_char__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
pub trait min_allocator_wchar_t_Impl {
    fn allocate(&self, n: ()) -> min_pointer_wchar_t__std_integral_constant_unsigned_long__0__;
    fn deallocate(&self, p: min_pointer_wchar_t__std_integral_constant_unsigned_long__0__, n: ());
}
impl min_allocator_wchar_t_Impl for Ptr<min_allocator_wchar_t_> {
    fn allocate(&self, mut n: ()) -> min_pointer_wchar_t__std_integral_constant_unsigned_long__0__ {
        return min_pointer_wchar_t__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_wchar_t__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
pub trait min_pointer_char16_t__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_char16_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_char16_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_char16_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_char16_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_char16_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_char16_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_char16_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_char16_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, n: ()) -> Ptr<()>;
}
impl min_pointer_char16_t__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_char16_t__std_integral_constant_unsigned_long__0__>
{
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone()).offset((n) as isize);
    }
}
pub trait min_pointer_char32_t__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_char32_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_char32_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_char32_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_char32_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_char32_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_char32_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_char32_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_char32_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, n: ()) -> Ptr<()>;
}
impl min_pointer_char32_t__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_char32_t__std_integral_constant_unsigned_long__0__>
{
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone()).offset((n) as isize);
    }
}
pub trait min_pointer_char8_t__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_8(&self) -> Ptr<min_pointer_char8_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_char8_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_char8_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_char8_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_char8_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_char8_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_char8_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_char8_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, n: ()) -> Ptr<()>;
}
impl min_pointer_char8_t__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_char8_t__std_integral_constant_unsigned_long__0__>
{
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone()).offset((n) as isize);
    }
}
pub trait min_pointer_char__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_8(&self) -> Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, n: ()) -> Ptr<()>;
}
impl min_pointer_char__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__>
{
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone()).offset((n) as isize);
    }
}
pub trait min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_10(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_11(
        &self,
    ) -> Ptr<min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_12(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_char16_t__std_integral_constant_unsigned_long__0__>
{
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
}
pub trait min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_10(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_11(
        &self,
    ) -> Ptr<min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_12(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_char32_t__std_integral_constant_unsigned_long__0__>
{
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
}
pub trait min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_10(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_11(
        &self,
    ) -> Ptr<min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_12(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_char8_t__std_integral_constant_unsigned_long__0__>
{
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
}
pub trait min_pointer_const_char__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_10(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_11(
        &self,
    ) -> Ptr<min_pointer_const_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_12(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_const_char__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_char__std_integral_constant_unsigned_long__0__>
{
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
}
pub trait min_pointer_const_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_void__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_10(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_11(
        &self,
    ) -> Ptr<min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_12(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_wchar_t__std_integral_constant_unsigned_long__0__>
{
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
}
pub trait min_pointer_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_void__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_wchar_t__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_8(&self) -> Ptr<min_pointer_wchar_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_wchar_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_wchar_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_wchar_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_wchar_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_wchar_t__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_wchar_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_wchar_t__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, n: ()) -> Ptr<()>;
}
impl min_pointer_wchar_t__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_wchar_t__std_integral_constant_unsigned_long__0__>
{
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone()).offset((n) as isize);
    }
}
pub trait test_allocator_char16_t_Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn allocate(&self, n: (), _a1: Option<()>) -> Ptr<()>;
    fn deallocate(&self, p: Ptr<()>, s: ());
    fn max_size(&self) -> ();
    fn destroy(&self, mut p: Ptr<()>) {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_char16_t_Impl for Ptr<test_allocator_char16_t_> {
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_14.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_14.with(|rc| *rc.borrow()));
    }
    fn allocate(&self, mut n: (), _a1: Option<()>) -> Ptr<()> {
        let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
        unimplemented!();
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
                unimplemented!();
            }
            field!((*self).with(|__s| __s.stats_.clone()), time_to_throw)
                .with_mut(|__v| __v.prefix_inc());
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count)
                .with_mut(|__v| __v.prefix_inc());
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                ((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size))
                .wrapping_add(n)
            });
        }
        return unimplemented!();
    }
    fn deallocate(&self, mut p: Ptr<()>, mut s: ()) {
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count)
                .with_mut(|__v| __v.prefix_dec());
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                ((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size))
                .wrapping_sub(s)
            });
        }
        unimplemented!();
    }
    fn max_size(&self) -> () {
        return (((2147483647_()).wrapping_mul((2_() as ())) as ()).wrapping_add((1_() as ())))
            .wrapping_div((::std::mem::size_of::<()>() as ()));
    }
}
pub trait test_allocator_char32_t_Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn allocate(&self, n: (), _a1: Option<()>) -> Ptr<()>;
    fn deallocate(&self, p: Ptr<()>, s: ());
    fn max_size(&self) -> ();
    fn destroy(&self, mut p: Ptr<()>) {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_char32_t_Impl for Ptr<test_allocator_char32_t_> {
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_14.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_14.with(|rc| *rc.borrow()));
    }
    fn allocate(&self, mut n: (), _a1: Option<()>) -> Ptr<()> {
        let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
        unimplemented!();
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
                unimplemented!();
            }
            field!((*self).with(|__s| __s.stats_.clone()), time_to_throw)
                .with_mut(|__v| __v.prefix_inc());
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count)
                .with_mut(|__v| __v.prefix_inc());
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                ((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size))
                .wrapping_add(n)
            });
        }
        return unimplemented!();
    }
    fn deallocate(&self, mut p: Ptr<()>, mut s: ()) {
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count)
                .with_mut(|__v| __v.prefix_dec());
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                ((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size))
                .wrapping_sub(s)
            });
        }
        unimplemented!();
    }
    fn max_size(&self) -> () {
        return (((2147483647_()).wrapping_mul((2_() as ())) as ()).wrapping_add((1_() as ())))
            .wrapping_div((::std::mem::size_of::<()>() as ()));
    }
}
pub trait test_allocator_char8_t_Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn allocate(&self, n: (), _a1: Option<()>) -> Ptr<()>;
    fn deallocate(&self, p: Ptr<()>, s: ());
    fn max_size(&self) -> ();
    fn destroy(&self, mut p: Ptr<()>) {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_char8_t_Impl for Ptr<test_allocator_char8_t_> {
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_14.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_14.with(|rc| *rc.borrow()));
    }
    fn allocate(&self, mut n: (), _a1: Option<()>) -> Ptr<()> {
        let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
        unimplemented!();
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
                unimplemented!();
            }
            field!((*self).with(|__s| __s.stats_.clone()), time_to_throw)
                .with_mut(|__v| __v.prefix_inc());
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count)
                .with_mut(|__v| __v.prefix_inc());
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                ((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size))
                .wrapping_add(n)
            });
        }
        return unimplemented!();
    }
    fn deallocate(&self, mut p: Ptr<()>, mut s: ()) {
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count)
                .with_mut(|__v| __v.prefix_dec());
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                ((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size))
                .wrapping_sub(s)
            });
        }
        unimplemented!();
    }
    fn max_size(&self) -> () {
        return (((2147483647_()).wrapping_mul((2_() as ())) as ()).wrapping_add((1_() as ())))
            .wrapping_div((::std::mem::size_of::<()>() as ()));
    }
}
pub trait test_allocator_char_Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn allocate(&self, n: (), _a1: Option<()>) -> Ptr<()>;
    fn deallocate(&self, p: Ptr<()>, s: ());
    fn max_size(&self) -> ();
    fn destroy(&self, mut p: Ptr<()>) {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_char_Impl for Ptr<test_allocator_char_> {
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_14.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_14.with(|rc| *rc.borrow()));
    }
    fn allocate(&self, mut n: (), _a1: Option<()>) -> Ptr<()> {
        let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
        unimplemented!();
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
                unimplemented!();
            }
            field!((*self).with(|__s| __s.stats_.clone()), time_to_throw)
                .with_mut(|__v| __v.prefix_inc());
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count)
                .with_mut(|__v| __v.prefix_inc());
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                ((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size))
                .wrapping_add(n)
            });
        }
        return unimplemented!();
    }
    fn deallocate(&self, mut p: Ptr<()>, mut s: ()) {
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count)
                .with_mut(|__v| __v.prefix_dec());
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                ((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size))
                .wrapping_sub(s)
            });
        }
        unimplemented!();
    }
    fn max_size(&self) -> () {
        return (((2147483647_()).wrapping_mul((2_() as ())) as ()).wrapping_add((1_() as ())))
            .wrapping_div((::std::mem::size_of::<()>() as ()));
    }
}
pub trait test_allocator_statisticsImpl {
    fn clear(&self);
    fn clear_ctor_counters(&self);
}
impl test_allocator_statisticsImpl for Ptr<test_allocator_statistics> {
    fn clear(&self) {
        unimplemented!();
        field!((*self), count).write(0_());
        field!((*self), time_to_throw).write(0_());
        field!((*self), alloc_count).write(0_());
        field!((*self), allocated_size).write(0_());
        field!((*self), construct_count).write(0_());
        field!((*self), destroy_count).write(0_());
        field!((*self), throw_after).write(2147483647_());
        ({ test_allocator_statisticsImpl::clear_ctor_counters(self) });
    }
    fn clear_ctor_counters(&self) {
        field!((*self), copied).write(0_());
        field!((*self), moved).write(0_());
        field!((*self), converted).write(0_());
    }
}
pub trait test_allocator_void_Impl {
    fn destructor(&self);
    fn get_id(&self) -> ();
    fn get_data(&self) -> ();
    fn copy_assign(&self, _a0: Ptr<test_allocator_void_>) -> Ptr<test_allocator_void_>;
}
impl test_allocator_void_Impl for Ptr<test_allocator_void_> {
    fn destructor(&self) {
        field!((*self), data_).write(destructed_value_14.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_14.with(|rc| *rc.borrow()));
    }
    fn get_id(&self) -> () {
        return (*self).with(|__s| __s.id_);
    }
    fn get_data(&self) -> () {
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
    fn address_11(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn allocate(&self, n: (), _a1: Option<()>) -> Ptr<()>;
    fn deallocate(&self, p: Ptr<()>, s: ());
    fn max_size(&self) -> ();
    fn destroy(&self, mut p: Ptr<()>) {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_wchar_t_Impl for Ptr<test_allocator_wchar_t_> {
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_14.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_14.with(|rc| *rc.borrow()));
    }
    fn allocate(&self, mut n: (), _a1: Option<()>) -> Ptr<()> {
        let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
        unimplemented!();
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
                unimplemented!();
            }
            field!((*self).with(|__s| __s.stats_.clone()), time_to_throw)
                .with_mut(|__v| __v.prefix_inc());
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count)
                .with_mut(|__v| __v.prefix_inc());
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                ((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size))
                .wrapping_add(n)
            });
        }
        return unimplemented!();
    }
    fn deallocate(&self, mut p: Ptr<()>, mut s: ()) {
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count)
                .with_mut(|__v| __v.prefix_dec());
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                ((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size))
                .wrapping_sub(s)
            });
        }
        unimplemented!();
    }
    fn max_size(&self) -> () {
        return (((2147483647_()).wrapping_mul((2_() as ())) as ()).wrapping_add((1_() as ())))
            .wrapping_div((::std::mem::size_of::<()>() as ()));
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_5.with(|_| ());
    let _ = alloc_count_6.with(|_| ());
    let _ = dealloc_count_7.with(|_| ());
    let _ = disable_default_constructor_8.with(|_| ());
    let _ = destructed_value_14.with(|_| ());
    let _ = moved_value_15.with(|_| ());
    let _ = should_throw_exception_53.with(|_| ());
}
