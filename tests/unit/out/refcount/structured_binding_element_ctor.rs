extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static copies_0: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static moves_1: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct Elem {
    #[offset(0)]
    pub v: i32,
}
impl Elem {
    pub fn new(mut v: i32) -> Self {
        Self { v: v }
    }
    pub fn copy_from(other: Ptr<Elem>) -> Self {
        let __this: Elem = Self {
            v: (other.with(|__s| __s.v) + 100),
        };
        (*copies_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn move_from(other: Ptr<Elem>) -> Self {
        let __this: Elem = Self {
            v: (other.with(|__s| __s.v) + 1000),
        };
        field!(other, v).write(-1_i32);
        (*moves_1.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for Elem {
    fn clone(&self) -> Self {
        let __src: Value<Elem> = Rc::new(RefCell::new(Elem { v: self.v.clone() }));
        Elem::copy_from(__src.as_pointer())
    }
}
impl CppMove for Elem {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(8)]
pub struct Two {
    #[offset(0)]
    #[byte_size(4)]
    pub a: Elem,
    #[offset(4)]
    #[byte_size(4)]
    pub b: Elem,
}
impl Two {
    pub fn move_from(_a0: Ptr<Two>) -> Self {
        Self {
            a: Elem::move_from({ field_ptr!(_a0, a) }),
            b: Elem::move_from({ field_ptr!(_a0, b) }),
        }
    }
}
impl CppMove for Two {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn make_two_2() -> Two {
    return Two {
        a: Elem::new({ 1 }),
        b: Elem::new({ 2 }),
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let t: Value<Two> = Rc::new(RefCell::new(Two {
        a: Elem::new({ 1 }),
        b: Elem::new({ 2 }),
    }));
    let mut base_copies: i32 = copies_0.with(|rc| *rc.borrow());
    let mut base_moves: i32 = moves_1.with(|rc| *rc.borrow());
    let __decomp_3: Value<Two> = Rc::new(RefCell::new((*t.borrow()).clone()));
    ({ cpp2rust_assert_fail_4((copies_0.with(|rc| *rc.borrow()) == (base_copies + 2))) });
    ({ cpp2rust_assert_fail_4((moves_1.with(|rc| *rc.borrow()) == base_moves)) });
    ({ cpp2rust_assert_fail_4(({ (*__decomp_3.borrow()).a.v } == 101)) });
    ({ cpp2rust_assert_fail_4(({ (*__decomp_3.borrow()).b.v } == 102)) });
    let __decomp_5: Ptr<Two> = t.as_pointer();
    ({ cpp2rust_assert_fail_4((copies_0.with(|rc| *rc.borrow()) == (base_copies + 2))) });
    ({ cpp2rust_assert_fail_4(({ (*__decomp_5.upgrade().deref()).a.v } == 1)) });
    field!(field!(__decomp_5, a), v).write(50);
    ({ cpp2rust_assert_fail_4(({ (*t.borrow()).a.v } == 50)) });
    let __decomp_6: Value<Two> = Rc::new(RefCell::new(Two::move_from({ t.as_pointer() })));
    ({ cpp2rust_assert_fail_4((moves_1.with(|rc| *rc.borrow()) == (base_moves + 2))) });
    ({ cpp2rust_assert_fail_4(({ (*__decomp_6.borrow()).a.v } == 1050)) });
    ({ cpp2rust_assert_fail_4(({ (*__decomp_6.borrow()).b.v } == 1002)) });
    ({ cpp2rust_assert_fail_4(({ (*t.borrow()).a.v } == -1_i32)) });
    let mut before_copies: i32 = copies_0.with(|rc| *rc.borrow());
    let mut before_moves: i32 = moves_1.with(|rc| *rc.borrow());
    let __decomp_7: Value<Two> = Rc::new(RefCell::new(({ make_two_2() })));
    ({ cpp2rust_assert_fail_4((copies_0.with(|rc| *rc.borrow()) == before_copies)) });
    ({ cpp2rust_assert_fail_4((moves_1.with(|rc| *rc.borrow()) == before_moves)) });
    ({ cpp2rust_assert_fail_4(({ (*__decomp_7.borrow()).a.v } == 1)) });
    let p: Value<std_pair_Elem__Elem_> = Rc::new(RefCell::new({
        let __tmp_0: Value<Elem> = Rc::new(RefCell::new(Elem::new({ 7 })));
        let __tmp_1: Value<Elem> = Rc::new(RefCell::new(Elem::new({ 8 })));
        std_pair_Elem__Elem_::new_1({ __tmp_0.as_pointer() }, { __tmp_1.as_pointer() })
    }));
    before_copies = copies_0.with(|rc| *rc.borrow());
    let __decomp_8: Value<std_pair_Elem__Elem_> =
        Rc::new(RefCell::new(std_pair_Elem__Elem_::new_2({
            p.as_pointer()
        })));
    let x: Ptr<Elem> = ({ get_9(__decomp_8.as_pointer()) });
    let y: Ptr<Elem> = ({ get_10(__decomp_8.as_pointer()) });
    ({ cpp2rust_assert_fail_4((copies_0.with(|rc| *rc.borrow()) == (before_copies + 2))) });
    ({
        cpp2rust_assert_fail_4(({ (*x.upgrade().deref()).v } == ({ (*p.borrow()).first.v } + 100)))
    });
    ({
        cpp2rust_assert_fail_4(({ (*y.upgrade().deref()).v } == ({ (*p.borrow()).second.v } + 100)))
    });
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = copies_0.with(|_| ());
    let _ = moves_1.with(|_| ());
}
