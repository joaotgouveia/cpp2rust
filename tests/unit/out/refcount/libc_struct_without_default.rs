extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(32)]
pub struct UserDefined {
    #[offset(0)]
    #[byte_size(4)]
    pub a: std_array_int__1_,
    #[offset(8)]
    #[byte_size(24)]
    pub v: std_vector_int__std_allocator_int__,
}
impl UserDefined {
    pub fn move_from(_a0: Ptr<UserDefined>) -> Self {
        Self {
            a: field!(_a0, a).with_mut(std::mem::take),
            v: field!(_a0, v).with_mut(std::mem::take),
        }
    }
}
impl CppMove for UserDefined {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
impl Default for UserDefined {
    fn default() -> Self {
        {
            UserDefined {
                a: <std_array_int__1_>::default(),
                v: <std_vector_int__std_allocator_int__>::default(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct FieldIsLibcType {
    #[offset(0)]
    #[byte_size(16)]
    pub addr: sockaddr,
}
impl CppMove for FieldIsLibcType {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for FieldIsLibcType {
    fn default() -> Self {
        {
            FieldIsLibcType {
                addr: <sockaddr>::default(),
            }
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let p: Value<pollfd> = Rc::new(RefCell::new(<pollfd>::default()));
    (*p.borrow_mut()).fd = -1_i32;
    (*p.borrow_mut()).events = 0_i16;
    (*p.borrow_mut()).revents = 2_i16;
    ({ cpp2rust_assert_fail_0(({ (*p.borrow()).fd } == -1_i32)) });
    ({ cpp2rust_assert_fail_0((({ (*p.borrow()).events } as i32) == 0)) });
    ({ cpp2rust_assert_fail_0((({ (*p.borrow()).revents } as i32) == 2)) });
    let ia: Value<in_addr> = Rc::new(RefCell::new(<in_addr>::default()));
    (*ia.borrow_mut()).s_addr = 1_u32;
    ({ cpp2rust_assert_fail_0(({ (*ia.borrow()).s_addr } == 1_u32)) });
    let t: Value<tm> = Rc::new(RefCell::new(<tm>::default()));
    (*t.borrow_mut()).tm_year = 124;
    (*t.borrow_mut()).tm_mon = 5;
    (*t.borrow_mut()).tm_mday = 15;
    ({ cpp2rust_assert_fail_0(({ (*t.borrow()).tm_year } == 124)) });
    ({ cpp2rust_assert_fail_0(({ (*t.borrow()).tm_mon } == 5)) });
    ({ cpp2rust_assert_fail_0(({ (*t.borrow()).tm_mday } == 15)) });
    let st: Value<stat> = Rc::new(RefCell::new(<stat>::default()));
    (*st.borrow_mut()).st_size = 1024_i64;
    ({ cpp2rust_assert_fail_0(({ (*st.borrow()).st_size } == 1024_i64)) });
    let ud: Value<UserDefined> = Rc::new(RefCell::new(<UserDefined>::default()));
    ({ cpp2rust_assert_fail_0(((elem!(field_ptr!(ud.as_pointer(), a), 0_u64).read()) == 0)) });
    ({ cpp2rust_assert_fail_0((({ (*ud.borrow()).v.size() }) == 0_u64)) });
    let mut filt: FieldIsLibcType = <FieldIsLibcType>::default();
    ({ cpp2rust_assert_fail_0(((filt.addr.sa_family as i32) == 0)) });
    return 0;
}
pub trait UserDefinedImpl {
    fn move_assign(&self, _a0: Ptr<UserDefined>) -> Ptr<UserDefined>;
}
impl UserDefinedImpl for Ptr<UserDefined> {
    fn move_assign(&self, _a0: Ptr<UserDefined>) -> Ptr<UserDefined> {
        field!((*self), a).write({ { (*_a0.upgrade().deref()).a.clone() } });
        field!((*self), v).write({ { (*_a0.upgrade().deref()).v.clone() } });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
