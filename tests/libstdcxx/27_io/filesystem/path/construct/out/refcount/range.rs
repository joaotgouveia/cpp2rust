extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn compare_paths_0(p1: Ptr<Vec<i8>>, p2: Ptr<Vec<i8>>) {
    if !(unimplemented!()) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if !(unimplemented!()) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ({ (unimplemented!() as i32) } != { (unimplemented!() as i32) }) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ({ (unimplemented!() as i32) } != { (unimplemented!() as i32) }) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ({ (unimplemented!() as i32) } != { (unimplemented!() as i32) }) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ({ (unimplemented!() as i32) } != { (unimplemented!() as i32) }) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ({ (unimplemented!() as i32) } != { (unimplemented!() as i32) }) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ({ (unimplemented!() as i32) } != { (unimplemented!() as i32) }) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ({ (unimplemented!() as i32) } != { (unimplemented!() as i32) }) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ({ (unimplemented!() as i32) } != { (unimplemented!() as i32) }) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ({ (unimplemented!() as i32) } != { (unimplemented!() as i32) }) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ({ (unimplemented!() as i32) } != { (unimplemented!() as i32) }) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ({ (unimplemented!() as i32) } != { (unimplemented!() as i32) }) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    let mut d1: isize = unimplemented!();
    let mut d2: isize = unimplemented!();
    if (d1 != d2) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if !(unimplemented!()) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
}
thread_local!(
    pub static test_paths_1: Value<Box<[Vec<i8>]>> = Rc::new(RefCell::new(Box::new([
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"/").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"//").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"/.").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"/./").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"/a").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"/a/").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"/a//").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"/a/b/c/d").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"/a//b").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"a").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"a/b").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"a/b/").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"a/b/c").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"a/b/c.d").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"a/b/..").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"a/b/c.").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"a/b/.c").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        },
    ])));
);
pub fn root_path_2() -> Vec<i8> {
    return {
        let mut __v: Vec<i8> = (Ptr::<i8>::from_string_literal(b"/") as Ptr<i8>)
            .path_bytes()
            .into_iter()
            .map(|__b| __b as i8)
            .collect();
        __v.push(0);
        __v
    };
}
pub fn nonexistent_path_3(file: Option<Vec<i8>>) -> Vec<i8> {
    let file: Value<Vec<i8>> = Rc::new(RefCell::new(file.unwrap_or_else(|| {
        let mut __bytes = Ptr::<i8>::from_string_literal(file!().as_bytes()).to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    })));
    if (((*file.borrow()).len() - 1) > 3_usize) && (unimplemented!() == 0) {
        {
            (*file.borrow_mut()).pop();
            (*file.borrow_mut()).resize(
                (((*file.borrow()).len() - 1).wrapping_sub(3_usize)) as usize,
                0_i8,
            );
            (*file.borrow_mut()).push(0_i8)
        };
    }
    let mut pos: usize = {
        let __n = (*file.borrow()).len().saturating_sub(1);
        let __end = usize::MAX.saturating_add(1).min(__n);
        Ptr::<i8>::from_string_literal(b"/\\").with_c_str(|__lookup| {
            (*file.borrow())
                .iter()
                .take(__end)
                .rposition(|__x| __lookup.contains(__x))
                .unwrap_or(usize::MAX)
        })
    };
    if (pos != usize::MAX) {
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let p: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut fd: i32 = unimplemented!();
    if (fd == -1_i32) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    match nix::unistd::unlink(file.decay().to_rust_string().as_str()) {
        Ok(()) => 0,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    };
    FdRegistry::close(fd);
    unimplemented!();
    return unimplemented!();
}
pub type __gnu_test_scoped_file_adopt_file_t = u32;
pub const __gnu_test_scoped_file_adopt_file_t_adopt_file: __gnu_test_scoped_file_adopt_file_t = 0;
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Destructor)]
#[byte_size(40)]
pub struct __gnu_test_scoped_file {
    #[offset(0)]
    #[byte_size(40)]
    pub path: Vec<i8>,
}
impl __gnu_test_scoped_file {
    pub fn new_1(p: Option<Ptr<Vec<i8>>>) -> Self {
        let mut __p_default: Option<Value<Vec<i8>>> = None;
        let p: Ptr<Vec<i8>> = p.unwrap_or_else(|| {
            __p_default
                .insert(Rc::new(RefCell::new(({ nonexistent_path_3(None) }))))
                .as_pointer()
        });
        let __this: __gnu_test_scoped_file = Self {
            path: unimplemented!(),
        };
        ::std::fs::File::create((unimplemented!()).reinterpret_cast::<i8>().to_string())
            .expect("Failed to open file");
        __this
    }
    pub fn new_2(p: Vec<i8>, mut _a1: __gnu_test_scoped_file_adopt_file_t) -> Self {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new(p));
        Self {
            path: unimplemented!(),
        }
    }
    pub fn move_from(_a0: Ptr<__gnu_test_scoped_file>) -> Self {
        Self {
            path: unimplemented!(),
        }
    }
}
impl Default for __gnu_test_scoped_file {
    fn default() -> Self {
        { __gnu_test_scoped_file::new_1(None) }
    }
}
pub fn permissions_are_testable_5(print_msg: Option<bool>) -> bool {
    let mut print_msg: bool = print_msg.unwrap_or_else(|| true);
    let testable: Value<bool> = Rc::new(RefCell::new(false));
    if ((nix::unistd::geteuid().as_raw() as ::libc::uid_t) != (0 as ::libc::uid_t)) {
        (*testable.borrow_mut()) = true;
    }
    if (print_msg) && (!(*testable.borrow())) {
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(
                b"Skipping tests that depend on filesystem permissions",
            )
            .to_c_u8_bytes();
            __bytes.push(b'\n');
            match libcc2rs::c_stdout().with_mut(|__f| __f.write(&__bytes)) == __bytes.len() {
                true => 0,
                false => -1,
            }
        };
    }
    return (*testable.borrow());
}
thread_local!(
    static counter_4: Value<u32> = Rc::new(RefCell::new((unimplemented!() as u32)));
);
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(128)]
pub struct __gnu_test_func_callback {
    #[offset(0)]
    _M_size: i32,
    #[offset(8)]
    #[byte_size(120)]
    _M_tests: Box<[FnPtr<fn()>]>,
}
impl __gnu_test_func_callback {
    pub fn new() -> Self {
        Self {
            _M_size: 0,
            _M_tests: (0..15)
                .map(|_| FnPtr::<fn()>::null())
                .collect::<Box<[FnPtr<fn()>]>>(),
        }
    }
}
impl Default for __gnu_test_func_callback {
    fn default() -> Self {
        { __gnu_test_func_callback::new() }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(1)]
pub struct __gnu_test_object_counter {}
impl __gnu_test_object_counter {
    pub fn new() -> Self {
        let __this: __gnu_test_object_counter = Self {};
        (*count_6.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn copy_from(_a0: Ptr<__gnu_test_object_counter>) -> Self {
        let __this: __gnu_test_object_counter = Self {};
        (*count_6.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for __gnu_test_object_counter {
    fn clone(&self) -> Self {
        let __src: Value<__gnu_test_object_counter> =
            Rc::new(RefCell::new(__gnu_test_object_counter {}));
        __gnu_test_object_counter::copy_from(__src.as_pointer())
    }
}
impl Default for __gnu_test_object_counter {
    fn default() -> Self {
        { __gnu_test_object_counter::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct __gnu_test_copy_constructor {}
impl __gnu_test_copy_constructor {
    pub fn count() -> u32 {
        return count__7.with(|rc| *rc.borrow());
    }
    pub fn mark_call() {
        (*count__7.with(Value::clone).borrow_mut()).postfix_inc();
        if (count__7.with(|rc| *rc.borrow()) == throw_on__8.with(|rc| *rc.borrow())) {
            unimplemented!();
        }
    }
    pub fn reset() {
        count__7.with(|rc| *rc.borrow_mut() = 0_u32);
        throw_on__8.with(|rc| *rc.borrow_mut() = 0_u32);
    }
    pub fn throw_on(mut count: u32) {
        throw_on__8.with(|rc| *rc.borrow_mut() = count);
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct __gnu_test_assignment_operator {}
impl __gnu_test_assignment_operator {
    pub fn count() -> u32 {
        return count__9.with(|rc| *rc.borrow());
    }
    pub fn mark_call() {
        (*count__9.with(Value::clone).borrow_mut()).postfix_inc();
        if (count__9.with(|rc| *rc.borrow()) == throw_on__10.with(|rc| *rc.borrow())) {
            unimplemented!();
        }
    }
    pub fn reset() {
        count__9.with(|rc| *rc.borrow_mut() = 0_u32);
        throw_on__10.with(|rc| *rc.borrow_mut() = 0_u32);
    }
    pub fn throw_on(mut count: u32) {
        throw_on__10.with(|rc| *rc.borrow_mut() = count);
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct __gnu_test_destructor {}
impl __gnu_test_destructor {
    pub fn count() -> u32 {
        return _M_count_11.with(|rc| *rc.borrow());
    }
    pub fn mark_call() {
        (*_M_count_11.with(Value::clone).borrow_mut()).postfix_inc();
    }
    pub fn reset() {
        _M_count_11.with(|rc| *rc.borrow_mut() = 0_u32);
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(8)]
pub struct __gnu_test_copy_tracker {
    #[offset(0)]
    id_: i32,
    #[offset(4)]
    throw_on_copy_: bool,
}
impl __gnu_test_copy_tracker {
    pub fn new(id: Option<i32>, throw_on_copy: Option<bool>) -> Self {
        let mut id: i32 =
            id.unwrap_or_else(|| (*next_id__12.with(Value::clone).borrow_mut()).postfix_dec());
        let mut throw_on_copy: bool = throw_on_copy.unwrap_or_else(|| false);
        Self {
            id_: id,
            throw_on_copy_: throw_on_copy,
        }
    }
    pub fn copy_from(rhs: Ptr<__gnu_test_copy_tracker>) -> Self {
        let __this: Value<__gnu_test_copy_tracker> = Rc::new(RefCell::new(Self {
            id_: ({ __gnu_test_copy_trackerImpl::id(&rhs) }),
            throw_on_copy_: rhs.with(|__s| __s.throw_on_copy_),
        }));
        let this: Ptr<__gnu_test_copy_tracker> = __this.as_pointer();
        if this.with(|__s| __s.throw_on_copy_) {
            ({
                __gnu_test_copy_constructor::throw_on(
                    ({ __gnu_test_copy_constructor::count() }).wrapping_add(1_u32),
                )
            });
        }
        ({ __gnu_test_copy_constructor::mark_call() });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn reset() {
        ({ __gnu_test_copy_constructor::reset() });
        ({ __gnu_test_assignment_operator::reset() });
        ({ __gnu_test_destructor::reset() });
    }
}
impl CppLess for __gnu_test_copy_tracker {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_13(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for __gnu_test_copy_tracker {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_14(__lhs.clone(), __rhs.clone())
    }
}
impl Clone for __gnu_test_copy_tracker {
    fn clone(&self) -> Self {
        let __src: Value<__gnu_test_copy_tracker> =
            Rc::new(RefCell::new(__gnu_test_copy_tracker {
                id_: self.id_.clone(),
                throw_on_copy_: self.throw_on_copy_.clone(),
            }));
        __gnu_test_copy_tracker::copy_from(__src.as_pointer())
    }
}
impl Default for __gnu_test_copy_tracker {
    fn default() -> Self {
        { __gnu_test_copy_tracker::new(None, None) }
    }
}
pub fn operator_eq_14(
    lhs: Ptr<__gnu_test_copy_tracker>,
    rhs: Ptr<__gnu_test_copy_tracker>,
) -> bool {
    return ({ ({ __gnu_test_copy_trackerImpl::id(&lhs) }) } == {
        ({ __gnu_test_copy_trackerImpl::id(&rhs) })
    });
}
pub fn operator_lt_13(
    lhs: Ptr<__gnu_test_copy_tracker>,
    rhs: Ptr<__gnu_test_copy_tracker>,
) -> bool {
    return ({ ({ __gnu_test_copy_trackerImpl::id(&lhs) }) } < {
        ({ __gnu_test_copy_trackerImpl::id(&rhs) })
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct __gnu_test_semaphore {
    #[offset(0)]
    sem_set_: i32,
    #[offset(4)]
    pid_: (),
}
impl __gnu_test_semaphore {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct __gnu_test_BoundsContainer_char_ {
    #[offset(0)]
    #[byte_size(8)]
    pub first: Ptr<i8>,
    #[offset(8)]
    #[byte_size(8)]
    pub last: Ptr<i8>,
}
impl __gnu_test_BoundsContainer_char_ {
    pub fn new(mut _first: Ptr<i8>, mut _last: Ptr<i8>) -> Self {
        Self {
            first: (_first).clone(),
            last: (_last).clone(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct __gnu_test_BoundsContainer_const_char_ {
    #[offset(0)]
    #[byte_size(8)]
    pub first: Ptr<i8>,
    #[offset(8)]
    #[byte_size(8)]
    pub last: Ptr<i8>,
}
impl __gnu_test_BoundsContainer_const_char_ {
    pub fn new(mut _first: Ptr<i8>, mut _last: Ptr<i8>) -> Self {
        Self {
            first: (_first).clone(),
            last: (_last).clone(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct __gnu_test_BoundsContainer_wchar_t_ {
    #[offset(0)]
    #[byte_size(8)]
    pub first: Ptr<i32>,
    #[offset(8)]
    #[byte_size(8)]
    pub last: Ptr<i32>,
}
impl __gnu_test_BoundsContainer_wchar_t_ {
    pub fn new(mut _first: Ptr<i32>, mut _last: Ptr<i32>) -> Self {
        Self {
            first: (_first).clone(),
            last: (_last).clone(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct __gnu_test_BoundsContainer_const_wchar_t_ {
    #[offset(0)]
    #[byte_size(8)]
    pub first: Ptr<i32>,
    #[offset(8)]
    #[byte_size(8)]
    pub last: Ptr<i32>,
}
impl __gnu_test_BoundsContainer_const_wchar_t_ {
    pub fn new(mut _first: Ptr<i32>, mut _last: Ptr<i32>) -> Self {
        Self {
            first: (_first).clone(),
            last: (_last).clone(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct __gnu_test_input_iterator_wrapper_char__post_inc_proxy_deref_proxy {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<i8>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
struct __gnu_test_input_iterator_wrapper_char__post_inc_proxy {
    #[offset(0)]
    #[byte_size(8)]
    pub p: __gnu_test_input_iterator_wrapper_char__post_inc_proxy_deref_proxy,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct __gnu_test_input_iterator_wrapper_char_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<i8>,
    #[offset(8)]
    #[byte_size(8)]
    pub SharedInfo: Ptr<__gnu_test_BoundsContainer_char_>,
}
impl __gnu_test_input_iterator_wrapper_char_ {
    pub fn new_1(
        mut _ptr: Ptr<i8>,
        mut SharedInfo_in: Ptr<__gnu_test_BoundsContainer_char_>,
    ) -> Self {
        let __this: Value<__gnu_test_input_iterator_wrapper_char_> = Rc::new(RefCell::new(Self {
            ptr: (_ptr).clone(),
            SharedInfo: (SharedInfo_in).clone(),
        }));
        let this: Ptr<__gnu_test_input_iterator_wrapper_char_> = __this.as_pointer();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({ this.with(|__s| __s.ptr.clone()) } >= {
                this.with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.first.clone())
            }) && ({ this.with(|__s| __s.ptr.clone()) } <= {
                this.with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.last.clone())
            })) {
                unimplemented!();
                std::process::abort();
            }
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl CppEq for __gnu_test_input_iterator_wrapper_char_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        __gnu_test_input_iterator_wrapper_char_Impl::operator_eq(__lhs, __rhs.clone())
    }
}
impl CppAdvance for __gnu_test_input_iterator_wrapper_char_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_9();
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct __gnu_test_input_iterator_wrapper_const_char__post_inc_proxy_deref_proxy {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<i8>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
struct __gnu_test_input_iterator_wrapper_const_char__post_inc_proxy {
    #[offset(0)]
    #[byte_size(8)]
    pub p: __gnu_test_input_iterator_wrapper_const_char__post_inc_proxy_deref_proxy,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct __gnu_test_input_iterator_wrapper_const_char_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<i8>,
    #[offset(8)]
    #[byte_size(8)]
    pub SharedInfo: Ptr<__gnu_test_BoundsContainer_const_char_>,
}
impl __gnu_test_input_iterator_wrapper_const_char_ {
    pub fn new_1(
        mut _ptr: Ptr<i8>,
        mut SharedInfo_in: Ptr<__gnu_test_BoundsContainer_const_char_>,
    ) -> Self {
        let __this: Value<__gnu_test_input_iterator_wrapper_const_char_> =
            Rc::new(RefCell::new(Self {
                ptr: (_ptr).clone(),
                SharedInfo: (SharedInfo_in).clone(),
            }));
        let this: Ptr<__gnu_test_input_iterator_wrapper_const_char_> = __this.as_pointer();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({ this.with(|__s| __s.ptr.clone()) } >= {
                this.with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.first.clone())
            }) && ({ this.with(|__s| __s.ptr.clone()) } <= {
                this.with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.last.clone())
            })) {
                unimplemented!();
                std::process::abort();
            }
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl CppEq for __gnu_test_input_iterator_wrapper_const_char_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        __gnu_test_input_iterator_wrapper_const_char_Impl::operator_eq(__lhs, __rhs.clone())
    }
}
impl CppAdvance for __gnu_test_input_iterator_wrapper_const_char_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_9();
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct __gnu_test_input_iterator_wrapper_wchar_t__post_inc_proxy_deref_proxy {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<i32>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
struct __gnu_test_input_iterator_wrapper_wchar_t__post_inc_proxy {
    #[offset(0)]
    #[byte_size(8)]
    pub p: __gnu_test_input_iterator_wrapper_wchar_t__post_inc_proxy_deref_proxy,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct __gnu_test_input_iterator_wrapper_wchar_t_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<i32>,
    #[offset(8)]
    #[byte_size(8)]
    pub SharedInfo: Ptr<__gnu_test_BoundsContainer_wchar_t_>,
}
impl __gnu_test_input_iterator_wrapper_wchar_t_ {
    pub fn new_1(
        mut _ptr: Ptr<i32>,
        mut SharedInfo_in: Ptr<__gnu_test_BoundsContainer_wchar_t_>,
    ) -> Self {
        let __this: Value<__gnu_test_input_iterator_wrapper_wchar_t_> =
            Rc::new(RefCell::new(Self {
                ptr: (_ptr).clone(),
                SharedInfo: (SharedInfo_in).clone(),
            }));
        let this: Ptr<__gnu_test_input_iterator_wrapper_wchar_t_> = __this.as_pointer();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({ this.with(|__s| __s.ptr.clone()) } >= {
                this.with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.first.clone())
            }) && ({ this.with(|__s| __s.ptr.clone()) } <= {
                this.with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.last.clone())
            })) {
                unimplemented!();
                std::process::abort();
            }
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl CppEq for __gnu_test_input_iterator_wrapper_wchar_t_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        __gnu_test_input_iterator_wrapper_wchar_t_Impl::operator_eq(__lhs, __rhs.clone())
    }
}
impl CppAdvance for __gnu_test_input_iterator_wrapper_wchar_t_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_9();
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct __gnu_test_input_iterator_wrapper_const_wchar_t__post_inc_proxy_deref_proxy {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<i32>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
struct __gnu_test_input_iterator_wrapper_const_wchar_t__post_inc_proxy {
    #[offset(0)]
    #[byte_size(8)]
    pub p: __gnu_test_input_iterator_wrapper_const_wchar_t__post_inc_proxy_deref_proxy,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct __gnu_test_input_iterator_wrapper_const_wchar_t_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<i32>,
    #[offset(8)]
    #[byte_size(8)]
    pub SharedInfo: Ptr<__gnu_test_BoundsContainer_const_wchar_t_>,
}
impl __gnu_test_input_iterator_wrapper_const_wchar_t_ {
    pub fn new_1(
        mut _ptr: Ptr<i32>,
        mut SharedInfo_in: Ptr<__gnu_test_BoundsContainer_const_wchar_t_>,
    ) -> Self {
        let __this: Value<__gnu_test_input_iterator_wrapper_const_wchar_t_> =
            Rc::new(RefCell::new(Self {
                ptr: (_ptr).clone(),
                SharedInfo: (SharedInfo_in).clone(),
            }));
        let this: Ptr<__gnu_test_input_iterator_wrapper_const_wchar_t_> = __this.as_pointer();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({ this.with(|__s| __s.ptr.clone()) } >= {
                this.with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.first.clone())
            }) && ({ this.with(|__s| __s.ptr.clone()) } <= {
                this.with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.last.clone())
            })) {
                unimplemented!();
                std::process::abort();
            }
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl CppEq for __gnu_test_input_iterator_wrapper_const_wchar_t_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        __gnu_test_input_iterator_wrapper_const_wchar_t_Impl::operator_eq(__lhs, __rhs.clone())
    }
}
impl CppAdvance for __gnu_test_input_iterator_wrapper_const_wchar_t_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_9();
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct __gnu_test_test_container_char____gnu_test_input_iterator_wrapper_ {
    #[offset(0)]
    #[byte_size(16)]
    pub bounds: __gnu_test_BoundsContainer_char_,
}
impl __gnu_test_test_container_char____gnu_test_input_iterator_wrapper_ {
    pub fn new(mut _first: Ptr<i8>, mut _last: Ptr<i8>) -> Self {
        Self {
            bounds: __gnu_test_BoundsContainer_char_::new({ (_first).clone() }, {
                (_last).clone()
            }),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_ {
    #[offset(0)]
    #[byte_size(16)]
    pub bounds: __gnu_test_BoundsContainer_const_char_,
}
impl __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_ {
    pub fn new(mut _first: Ptr<i8>, mut _last: Ptr<i8>) -> Self {
        Self {
            bounds: __gnu_test_BoundsContainer_const_char_::new({ (_first).clone() }, {
                (_last).clone()
            }),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct __gnu_test_test_container_wchar_t____gnu_test_input_iterator_wrapper_ {
    #[offset(0)]
    #[byte_size(16)]
    pub bounds: __gnu_test_BoundsContainer_wchar_t_,
}
impl __gnu_test_test_container_wchar_t____gnu_test_input_iterator_wrapper_ {
    pub fn new(mut _first: Ptr<i32>, mut _last: Ptr<i32>) -> Self {
        Self {
            bounds: __gnu_test_BoundsContainer_wchar_t_::new({ (_first).clone() }, {
                (_last).clone()
            }),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct __gnu_test_test_container_const_wchar_t____gnu_test_input_iterator_wrapper_ {
    #[offset(0)]
    #[byte_size(16)]
    pub bounds: __gnu_test_BoundsContainer_const_wchar_t_,
}
impl __gnu_test_test_container_const_wchar_t____gnu_test_input_iterator_wrapper_ {
    pub fn new(mut _first: Ptr<i32>, mut _last: Ptr<i32>) -> Self {
        Self {
            bounds: __gnu_test_BoundsContainer_const_wchar_t_::new({ (_first).clone() }, {
                (_last).clone()
            }),
        }
    }
}
pub fn test01_15() {
    'loop_: for s in IterRange::new(
        (test_paths_1.with(|v| v.as_pointer()) as Ptr<Vec<i8>>),
        (test_paths_1.with(|v| v.as_pointer()) as Ptr<Vec<i8>>).offset((18_i64) as isize),
    ) {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new((*s.upgrade().deref()).clone()));
        let p1: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __v: Vec<i8> = s
                .as_pointer()
                .path_bytes()
                .into_iter()
                .map(|__b| __b as i8)
                .collect();
            __v.push(0);
            __v
        }));
        let p2: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        let p3: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __v: Vec<i8> = s
                .decay()
                .path_bytes()
                .into_iter()
                .map(|__b| __b as i8)
                .collect();
            __v.push(0);
            __v
        }));
        let p4: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        ({ compare_paths_0(p1.as_pointer(), p2.as_pointer()) });
        ({ compare_paths_0(p1.as_pointer(), p3.as_pointer()) });
        ({ compare_paths_0(p1.as_pointer(), p4.as_pointer()) });
        let ws: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
        let p5: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __v: Vec<i8> = ws
                .as_pointer()
                .path_bytes()
                .into_iter()
                .map(|__b| __b as i8)
                .collect();
            __v.push(0);
            __v
        }));
        let p6: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        let p7: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __v: Vec<i8> = ws
                .decay()
                .path_bytes()
                .into_iter()
                .map(|__b| __b as i8)
                .collect();
            __v.push(0);
            __v
        }));
        let p8: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        ({ compare_paths_0(p1.as_pointer(), p5.as_pointer()) });
        ({ compare_paths_0(p1.as_pointer(), p6.as_pointer()) });
        ({ compare_paths_0(p1.as_pointer(), p7.as_pointer()) });
        ({ compare_paths_0(p1.as_pointer(), p8.as_pointer()) });
        let r1: Value<__gnu_test_test_container_char____gnu_test_input_iterator_wrapper_> =
            Rc::new(RefCell::new(
                __gnu_test_test_container_char____gnu_test_input_iterator_wrapper_::new(
                    { s.decay().reinterpret_cast::<i8>() },
                    {
                        s.decay()
                            .reinterpret_cast::<i8>()
                            .offset(((*s.borrow()).len() - 1) as isize)
                    },
                ),
            ));
        let p9: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        ({ compare_paths_0(p1.as_pointer(), p9.as_pointer()) });
        let r2: Value<__gnu_test_test_container_char____gnu_test_input_iterator_wrapper_> =
            Rc::new(RefCell::new(
                __gnu_test_test_container_char____gnu_test_input_iterator_wrapper_::new(
                    { s.decay().reinterpret_cast::<i8>() },
                    {
                        s.decay()
                            .reinterpret_cast::<i8>()
                            .offset(((*s.borrow()).len() - 1) as isize)
                            .offset((1) as isize)
                    },
                ),
            ));
        let p10: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __v: Vec<i8> = Rc::new(RefCell::new(
                ({
                    __gnu_test_test_container_char____gnu_test_input_iterator_wrapper_Impl::begin(
                        &r2.as_pointer(),
                    )
                }),
            ))
            .as_pointer()
            .path_bytes()
            .into_iter()
            .map(|__b| __b as i8)
            .collect();
            __v.push(0);
            __v
        }));
        ({ compare_paths_0(p1.as_pointer(), p10.as_pointer()) });
        let r3: Value<__gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_> =
            Rc::new(RefCell::new(
                __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_::new(
                    { s.decay() },
                    { s.decay().offset(((*s.borrow()).len() - 1) as isize) },
                ),
            ));
        let p11: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        ({ compare_paths_0(p1.as_pointer(), p11.as_pointer()) });
        let r4: Value<__gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_> =
            Rc::new(RefCell::new(
                __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_::new(
                    { s.decay() },
                    {
                        s.decay()
                            .offset(((*s.borrow()).len() - 1) as isize)
                            .offset((1) as isize)
                    },
                ),
            ));
        let p12: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __v: Vec<i8> = Rc::new(RefCell::new(( { __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_Impl :: begin ( &r4 .as_pointer()  , ) } )  )).as_pointer().path_bytes().into_iter().map(|__b| __b as i8).collect();
            __v.push(0);
            __v
        }));
        ({ compare_paths_0(p1.as_pointer(), p12.as_pointer()) });
        let r5: Value<__gnu_test_test_container_wchar_t____gnu_test_input_iterator_wrapper_> =
            Rc::new(RefCell::new(
                __gnu_test_test_container_wchar_t____gnu_test_input_iterator_wrapper_::new(
                    { ws.decay().reinterpret_cast::<i32>() },
                    {
                        ws.decay()
                            .reinterpret_cast::<i32>()
                            .offset(((*ws.borrow()).len() - 1) as isize)
                    },
                ),
            ));
        let p13: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        ({ compare_paths_0(p1.as_pointer(), p13.as_pointer()) });
        let r6: Value<__gnu_test_test_container_wchar_t____gnu_test_input_iterator_wrapper_> =
            Rc::new(RefCell::new(
                __gnu_test_test_container_wchar_t____gnu_test_input_iterator_wrapper_::new(
                    { ws.decay().reinterpret_cast::<i32>() },
                    {
                        ws.decay()
                            .reinterpret_cast::<i32>()
                            .offset(((*ws.borrow()).len() - 1) as isize)
                            .offset((1) as isize)
                    },
                ),
            ));
        let p14: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __v: Vec<i8> = Rc::new(RefCell::new(( { __gnu_test_test_container_wchar_t____gnu_test_input_iterator_wrapper_Impl :: begin ( &r6 .as_pointer()  , ) } )  )).as_pointer().path_bytes().into_iter().map(|__b| __b as i8).collect();
            __v.push(0);
            __v
        }));
        ({ compare_paths_0(p1.as_pointer(), p14.as_pointer()) });
        let r7: Value<__gnu_test_test_container_const_wchar_t____gnu_test_input_iterator_wrapper_> =
            Rc::new(RefCell::new(
                __gnu_test_test_container_const_wchar_t____gnu_test_input_iterator_wrapper_::new(
                    { ws.decay() },
                    { ws.decay().offset(((*ws.borrow()).len() - 1) as isize) },
                ),
            ));
        let p15: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        ({ compare_paths_0(p1.as_pointer(), p15.as_pointer()) });
        let r8: Value<__gnu_test_test_container_const_wchar_t____gnu_test_input_iterator_wrapper_> =
            Rc::new(RefCell::new(
                __gnu_test_test_container_const_wchar_t____gnu_test_input_iterator_wrapper_::new(
                    { ws.decay() },
                    {
                        ws.decay()
                            .offset(((*ws.borrow()).len() - 1) as isize)
                            .offset((1) as isize)
                    },
                ),
            ));
        let p16: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __v: Vec<i8> = Rc::new(RefCell::new(( { __gnu_test_test_container_const_wchar_t____gnu_test_input_iterator_wrapper_Impl :: begin ( &r8 .as_pointer()  , ) } )  )).as_pointer().path_bytes().into_iter().map(|__b| __b as i8).collect();
            __v.push(0);
            __v
        }));
        ({ compare_paths_0(p1.as_pointer(), p16.as_pointer()) });
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test01_15() });
    0
}
impl __gnu_test_semaphore {
    pub fn new() -> Self {
        let __this: Value<__gnu_test_semaphore> = Rc::new(RefCell::new(Self {
            sem_set_: 0_i32,
            pid_: unimplemented!(),
        }));
        let this: Ptr<__gnu_test_semaphore> = __this.as_pointer();
        field!(this, pid_).write(unimplemented!());
        field!(this, sem_set_).write(unimplemented!());
        if (this.with(|__s| __s.sem_set_) == -1_i32) {
            unimplemented!();
        }
        let val: Value<__gnu_test_semun> = Rc::new(RefCell::new(<__gnu_test_semun>::default()));
        __gnu_test_semun::val(val.as_pointer()).write(0);
        if (unimplemented!() == -1_i32) {
            unimplemented!();
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
pub fn set_memory_limits_16(size: Option<f32>) {
    let mut size: f32 = size.unwrap_or_else(|| (16_f64 as f32));
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut limit: () = ((size * 1048576_f32) as ());
    unimplemented!();
    (*r.borrow_mut()).rlim_cur = limit;
    unimplemented!();
    unimplemented!();
    (*r.borrow_mut()).rlim_cur = limit;
    unimplemented!();
    if (limit < 20971520_()) {
        limit = 20971520_();
    }
    unimplemented!();
    (*r.borrow_mut()).rlim_cur = limit;
    unimplemented!();
}
pub fn set_file_limit_17(mut size: u64) {
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut limit: () = ((size) as ());
    unimplemented!();
    (*r.borrow_mut()).rlim_cur = limit;
    unimplemented!();
}
pub fn verify_demangle_18(mut mangled: Ptr<i8>, mut wanted: Ptr<i8>) {
    let status: Value<i32> = Rc::new(RefCell::new(0));
    let mut s: Ptr<i8> = Ptr::<i8>::null();
    let mut demangled: Ptr<i8> = unimplemented!();
    if !(demangled).is_null() {
        s = (demangled).clone();
    } else {
        switch!(match (*status.borrow()) {
            __v if __v == 0 => {
                s = Ptr::<i8>::from_string_literal(b"error code = 0: success");
                break;
            }
            __v if __v == -1_i32 => {
                s = Ptr::<i8>::from_string_literal(b"error code = -1: memory allocation failure");
                break;
            }
            __v if __v == -2_i32 => {
                s = Ptr::<i8>::from_string_literal(b"error code = -2: invalid mangled name");
                break;
            }
            __v if __v == -3_i32 => {
                s = Ptr::<i8>::from_string_literal(b"error code = -3: invalid arguments");
                break;
            }
            _ => {
                s = Ptr::<i8>::from_string_literal(b"error code unknown - who knows what happened");
            }
        });
    }
    let w: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = wanted.to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    }));
    if !(s.with_c_str(|__s| (*w.borrow())[..(*w.borrow()).len().saturating_sub(1)] == *__s)) {
        unimplemented!();
    }
    libcc2rs::free_refcount((demangled).to_any());
}
pub fn run_tests_wrapped_locale_19(mut name: Ptr<i8>, l: Ptr<__gnu_test_func_callback>) {
    let loc_name: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut res: Ptr<i8> = Ptr::<i8>::from_string_literal(b"C");
    if !(res).is_null() {
        let mut preLC_ALL: Vec<i8> = {
            let mut __bytes = res.to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        };
        let mut tests: Ptr<FnPtr<fn()>> = ({ __gnu_test_func_callbackImpl::tests(&l) });
        let mut i: i32 = 0;
        'loop_: while ({ i } < { ({ __gnu_test_func_callbackImpl::size(&l) }) }) {
            ({ (elem!(tests, i).read()).call() });
            i.prefix_inc();
        }
        let mut postLC_ALL: Vec<i8> = {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"C").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        };
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                std::process::abort();
            }
        }
    } else {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"LC_ALL for ").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        unimplemented!();
        unimplemented!();
    }
}
pub fn run_tests_wrapped_env_20(
    mut name: Ptr<i8>,
    mut env: Ptr<i8>,
    l: Ptr<__gnu_test_func_callback>,
) {
    let loc_name: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut oldENV: Ptr<i8> = match ::std::env::var(env.to_rust_string()) {
        Ok(__val) => Ptr::alloc_c_str(__val.as_bytes()),
        Err(_) => Ptr::null(),
    };
    if !({
        let __name = env.to_rust_string();
        if 1 != 0 || ::std::env::var_os(&__name).is_none() {
            unsafe { ::std::env::set_var(__name, name.to_rust_string()) };
        }
        0
    } != 0)
    {
        let mut tests: Ptr<FnPtr<fn()>> = ({ __gnu_test_func_callbackImpl::tests(&l) });
        let mut i: i32 = 0;
        'loop_: while ({ i } < { ({ __gnu_test_func_callbackImpl::size(&l) }) }) {
            ({ (elem!(tests, i).read()).call() });
            i.prefix_inc();
        }
        {
            let __name = env.to_rust_string();
            if 1 != 0 || ::std::env::var_os(&__name).is_none() {
                unsafe {
                    ::std::env::set_var(
                        __name,
                        if !(oldENV).is_null() {
                            (oldENV).clone()
                        } else {
                            Ptr::<i8>::from_string_literal(b"")
                        }
                        .to_rust_string(),
                    )
                };
            }
            0
        };
    } else {
        let s: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __bytes = env.to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        }));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
thread_local!(
    pub static count_6: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    static count__7: Value<u32> = Rc::new(RefCell::new(0_u32));
);
thread_local!(
    static throw_on__8: Value<u32> = Rc::new(RefCell::new(0_u32));
);
thread_local!(
    static count__9: Value<u32> = Rc::new(RefCell::new(0_u32));
);
thread_local!(
    static throw_on__10: Value<u32> = Rc::new(RefCell::new(0_u32));
);
thread_local!(
    static _M_count_11: Value<u32> = Rc::new(RefCell::new(0_u32));
);
thread_local!(
    static next_id__12: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(8)]
pub struct __gnu_test_semun {
    #[offset(0)]
    #[byte_size(8)]
    __bytes: Box<[u8]>,
}
impl __gnu_test_semun {
    pub fn val(this: Ptr<Self>) -> Ptr<i32> {
        this.reinterpret_cast()
    }
    pub fn buf(this: Ptr<Self>) -> Ptr<Ptr<()>> {
        this.reinterpret_cast()
    }
    pub fn array(this: Ptr<Self>) -> Ptr<Ptr<u16>> {
        this.reinterpret_cast()
    }
}
impl Default for __gnu_test_semun {
    fn default() -> Self {
        __gnu_test_semun {
            __bytes: Box::from([0u8; 8]),
        }
    }
}
impl __gnu_test_semaphore {}
pub fn test_tm_21(
    mut sec: i32,
    mut min: i32,
    mut hour: i32,
    mut mday: i32,
    mut mon: i32,
    mut year: i32,
    mut wday: i32,
    mut yday: i32,
    mut isdst: i32,
) -> libcc2rs::Tm {
    (*tmp_22.with(Value::clone).borrow_mut()).tm_sec = sec;
    (*tmp_22.with(Value::clone).borrow_mut()).tm_min = min;
    (*tmp_22.with(Value::clone).borrow_mut()).tm_hour = hour;
    (*tmp_22.with(Value::clone).borrow_mut()).tm_mday = mday;
    (*tmp_22.with(Value::clone).borrow_mut()).tm_mon = mon;
    (*tmp_22.with(Value::clone).borrow_mut()).tm_year = year;
    (*tmp_22.with(Value::clone).borrow_mut()).tm_wday = wday;
    (*tmp_22.with(Value::clone).borrow_mut()).tm_yday = yday;
    (*tmp_22.with(Value::clone).borrow_mut()).tm_isdst = isdst;
    return (*tmp_22.with(Value::clone).borrow()).clone();
}
thread_local!(
    static tmp_22: Value<libcc2rs::Tm> = Rc::new(RefCell::new(Default::default()));
);
pub trait __gnu_test_BoundsContainer_char_Impl {
    fn size(&self) -> usize {
        unimplemented!()
    }
}
impl __gnu_test_BoundsContainer_char_Impl for Ptr<__gnu_test_BoundsContainer_char_> {}
pub trait __gnu_test_BoundsContainer_const_char_Impl {
    fn size(&self) -> usize {
        unimplemented!()
    }
}
impl __gnu_test_BoundsContainer_const_char_Impl for Ptr<__gnu_test_BoundsContainer_const_char_> {}
pub trait __gnu_test_BoundsContainer_const_wchar_t_Impl {
    fn size(&self) -> usize {
        unimplemented!()
    }
}
impl __gnu_test_BoundsContainer_const_wchar_t_Impl
    for Ptr<__gnu_test_BoundsContainer_const_wchar_t_>
{
}
pub trait __gnu_test_BoundsContainer_wchar_t_Impl {
    fn size(&self) -> usize {
        unimplemented!()
    }
}
impl __gnu_test_BoundsContainer_wchar_t_Impl for Ptr<__gnu_test_BoundsContainer_wchar_t_> {}
pub trait __gnu_test_copy_trackerImpl {
    fn copy_assign(&self, rhs: Ptr<__gnu_test_copy_tracker>) -> Ptr<__gnu_test_copy_tracker>;
    fn destructor(&self);
    fn id(&self) -> i32;
}
impl __gnu_test_copy_trackerImpl for Ptr<__gnu_test_copy_tracker> {
    fn copy_assign(&self, rhs: Ptr<__gnu_test_copy_tracker>) -> Ptr<__gnu_test_copy_tracker> {
        field!((*self), id_).write(({ __gnu_test_copy_trackerImpl::id(&rhs) }));
        if rhs.with(|__s| __s.throw_on_copy_) {
            ({
                __gnu_test_assignment_operator::throw_on(
                    ({ __gnu_test_assignment_operator::count() }).wrapping_add(1_u32),
                )
            });
        }
        ({ __gnu_test_assignment_operator::mark_call() });
        return (*self).clone();
    }
    fn destructor(&self) {
        ({ __gnu_test_destructor::mark_call() });
    }
    fn id(&self) -> i32 {
        return (*self).with(|__s| __s.id_);
    }
}
pub trait __gnu_test_func_callbackImpl {
    fn copy_assign(&self, _a0: Ptr<__gnu_test_func_callback>) -> Ptr<__gnu_test_func_callback> {
        unimplemented!()
    }
    fn size(&self) -> i32;
    fn tests(&self) -> Ptr<FnPtr<fn()>>;
    fn push_back(&self, test: FnPtr<fn()>);
}
impl __gnu_test_func_callbackImpl for Ptr<__gnu_test_func_callback> {
    fn size(&self) -> i32 {
        return (*self).with(|__s| __s._M_size);
    }
    fn tests(&self) -> Ptr<FnPtr<fn()>> {
        return (array_field_ptr!((*self), _M_tests) as Ptr<FnPtr<fn()>>);
    }
    fn push_back(&self, mut test: FnPtr<fn()>) {
        array_elem!((*self), _M_tests, (*self).with(|__s| __s._M_size)).write((test).clone());
        field!((*self), _M_size).with_mut(|__v| __v.prefix_inc());
    }
}
pub trait __gnu_test_input_iterator_wrapper_char_Impl {
    fn operator_eq(&self, in_: Ptr<__gnu_test_input_iterator_wrapper_char_>) -> bool;
    fn operator_ne(&self, in_: Ptr<__gnu_test_input_iterator_wrapper_char_>) -> bool;
    fn operator_deref(&self) -> Ptr<i8>;
    fn operator_arrow(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn operator_inc_9(&self) -> Ptr<__gnu_test_input_iterator_wrapper_char_>;
    fn operator_post_inc_10(
        &self,
        mut _a0: i32,
    ) -> __gnu_test_input_iterator_wrapper_char__post_inc_proxy {
        unimplemented!()
    }
}
impl __gnu_test_input_iterator_wrapper_char_Impl for Ptr<__gnu_test_input_iterator_wrapper_char_> {
    fn operator_eq(&self, in_: Ptr<__gnu_test_input_iterator_wrapper_char_>) -> bool {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((!((*self).with(|__s| __s.SharedInfo.clone())).is_null())
                && ({ (*self).with(|__s| __s.SharedInfo.clone()) } == {
                    in_.with(|__s| __s.SharedInfo.clone())
                }))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({ (*self).with(|__s| __s.ptr.clone()) } >= {
                (*self)
                    .with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.first.clone())
            }) && ({ in_.with(|__s| __s.ptr.clone()) } >= {
                (*self)
                    .with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.first.clone())
            })) {
                unimplemented!();
                std::process::abort();
            }
        }
        return ({ (*self).with(|__s| __s.ptr.clone()) } == { in_.with(|__s| __s.ptr.clone()) });
    }
    fn operator_ne(&self, in_: Ptr<__gnu_test_input_iterator_wrapper_char_>) -> bool {
        return !({
            let _in_: Ptr<__gnu_test_input_iterator_wrapper_char_> = (in_).clone();
            __gnu_test_input_iterator_wrapper_char_Impl::operator_eq(&(*self), _in_)
        });
    }
    fn operator_deref(&self) -> Ptr<i8> {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((!((*self).with(|__s| __s.SharedInfo.clone())).is_null())
                && ({ (*self).with(|__s| __s.ptr.clone()) } < {
                    (*self)
                        .with(|__s| __s.SharedInfo.clone())
                        .with(|__s| __s.last.clone())
                }))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (*self).with(|__s| __s.ptr.clone()) } >= {
                (*self)
                    .with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.first.clone())
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        return (*self).with(|__s| __s.ptr.clone());
    }
    fn operator_inc_9(&self) -> Ptr<__gnu_test_input_iterator_wrapper_char_> {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((!((*self).with(|__s| __s.SharedInfo.clone())).is_null())
                && ({ (*self).with(|__s| __s.ptr.clone()) } < {
                    (*self)
                        .with(|__s| __s.SharedInfo.clone())
                        .with(|__s| __s.last.clone())
                }))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (*self).with(|__s| __s.ptr.clone()) } >= {
                (*self)
                    .with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.first.clone())
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        field!((*self), ptr).with_mut(|__v| __v.postfix_inc());
        field!((*self).with(|__s| __s.SharedInfo.clone()), first)
            .write((*self).with(|__s| __s.ptr.clone()));
        return (*self).clone();
    }
}
pub trait __gnu_test_input_iterator_wrapper_char__post_inc_proxyImpl {
    fn operator_deref(&self) -> __gnu_test_input_iterator_wrapper_char__post_inc_proxy_deref_proxy {
        unimplemented!()
    }
}
impl __gnu_test_input_iterator_wrapper_char__post_inc_proxyImpl
    for Ptr<__gnu_test_input_iterator_wrapper_char__post_inc_proxy>
{
}
pub trait __gnu_test_input_iterator_wrapper_char__post_inc_proxy_deref_proxyImpl {
    fn to_pconsti8(&self) -> Ptr<i8> {
        unimplemented!()
    }
}
impl __gnu_test_input_iterator_wrapper_char__post_inc_proxy_deref_proxyImpl
    for Ptr<__gnu_test_input_iterator_wrapper_char__post_inc_proxy_deref_proxy>
{
}
pub trait __gnu_test_input_iterator_wrapper_const_char_Impl {
    fn operator_eq(&self, in_: Ptr<__gnu_test_input_iterator_wrapper_const_char_>) -> bool;
    fn operator_ne(&self, in_: Ptr<__gnu_test_input_iterator_wrapper_const_char_>) -> bool;
    fn operator_deref(&self) -> Ptr<i8>;
    fn operator_arrow(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn operator_inc_9(&self) -> Ptr<__gnu_test_input_iterator_wrapper_const_char_>;
    fn operator_post_inc_10(
        &self,
        mut _a0: i32,
    ) -> __gnu_test_input_iterator_wrapper_const_char__post_inc_proxy {
        unimplemented!()
    }
}
impl __gnu_test_input_iterator_wrapper_const_char_Impl
    for Ptr<__gnu_test_input_iterator_wrapper_const_char_>
{
    fn operator_eq(&self, in_: Ptr<__gnu_test_input_iterator_wrapper_const_char_>) -> bool {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((!((*self).with(|__s| __s.SharedInfo.clone())).is_null())
                && ({ (*self).with(|__s| __s.SharedInfo.clone()) } == {
                    in_.with(|__s| __s.SharedInfo.clone())
                }))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({ (*self).with(|__s| __s.ptr.clone()) } >= {
                (*self)
                    .with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.first.clone())
            }) && ({ in_.with(|__s| __s.ptr.clone()) } >= {
                (*self)
                    .with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.first.clone())
            })) {
                unimplemented!();
                std::process::abort();
            }
        }
        return ({ (*self).with(|__s| __s.ptr.clone()) } == { in_.with(|__s| __s.ptr.clone()) });
    }
    fn operator_ne(&self, in_: Ptr<__gnu_test_input_iterator_wrapper_const_char_>) -> bool {
        return !({
            let _in_: Ptr<__gnu_test_input_iterator_wrapper_const_char_> = (in_).clone();
            __gnu_test_input_iterator_wrapper_const_char_Impl::operator_eq(&(*self), _in_)
        });
    }
    fn operator_deref(&self) -> Ptr<i8> {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((!((*self).with(|__s| __s.SharedInfo.clone())).is_null())
                && ({ (*self).with(|__s| __s.ptr.clone()) } < {
                    (*self)
                        .with(|__s| __s.SharedInfo.clone())
                        .with(|__s| __s.last.clone())
                }))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (*self).with(|__s| __s.ptr.clone()) } >= {
                (*self)
                    .with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.first.clone())
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        return (*self).with(|__s| __s.ptr.clone());
    }
    fn operator_inc_9(&self) -> Ptr<__gnu_test_input_iterator_wrapper_const_char_> {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((!((*self).with(|__s| __s.SharedInfo.clone())).is_null())
                && ({ (*self).with(|__s| __s.ptr.clone()) } < {
                    (*self)
                        .with(|__s| __s.SharedInfo.clone())
                        .with(|__s| __s.last.clone())
                }))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (*self).with(|__s| __s.ptr.clone()) } >= {
                (*self)
                    .with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.first.clone())
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        field!((*self), ptr).with_mut(|__v| __v.postfix_inc());
        field!((*self).with(|__s| __s.SharedInfo.clone()), first)
            .write((*self).with(|__s| __s.ptr.clone()));
        return (*self).clone();
    }
}
pub trait __gnu_test_input_iterator_wrapper_const_char__post_inc_proxyImpl {
    fn operator_deref(
        &self,
    ) -> __gnu_test_input_iterator_wrapper_const_char__post_inc_proxy_deref_proxy {
        unimplemented!()
    }
}
impl __gnu_test_input_iterator_wrapper_const_char__post_inc_proxyImpl
    for Ptr<__gnu_test_input_iterator_wrapper_const_char__post_inc_proxy>
{
}
pub trait __gnu_test_input_iterator_wrapper_const_char__post_inc_proxy_deref_proxyImpl {
    fn to_pconsti8(&self) -> Ptr<i8> {
        unimplemented!()
    }
}
impl __gnu_test_input_iterator_wrapper_const_char__post_inc_proxy_deref_proxyImpl
    for Ptr<__gnu_test_input_iterator_wrapper_const_char__post_inc_proxy_deref_proxy>
{
}
pub trait __gnu_test_input_iterator_wrapper_const_wchar_t_Impl {
    fn operator_eq(&self, in_: Ptr<__gnu_test_input_iterator_wrapper_const_wchar_t_>) -> bool;
    fn operator_ne(&self, in_: Ptr<__gnu_test_input_iterator_wrapper_const_wchar_t_>) -> bool;
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_arrow(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_9(&self) -> Ptr<__gnu_test_input_iterator_wrapper_const_wchar_t_>;
    fn operator_post_inc_10(
        &self,
        mut _a0: i32,
    ) -> __gnu_test_input_iterator_wrapper_const_wchar_t__post_inc_proxy {
        unimplemented!()
    }
}
impl __gnu_test_input_iterator_wrapper_const_wchar_t_Impl
    for Ptr<__gnu_test_input_iterator_wrapper_const_wchar_t_>
{
    fn operator_eq(&self, in_: Ptr<__gnu_test_input_iterator_wrapper_const_wchar_t_>) -> bool {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((!((*self).with(|__s| __s.SharedInfo.clone())).is_null())
                && ({ (*self).with(|__s| __s.SharedInfo.clone()) } == {
                    in_.with(|__s| __s.SharedInfo.clone())
                }))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({ (*self).with(|__s| __s.ptr.clone()) } >= {
                (*self)
                    .with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.first.clone())
            }) && ({ in_.with(|__s| __s.ptr.clone()) } >= {
                (*self)
                    .with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.first.clone())
            })) {
                unimplemented!();
                std::process::abort();
            }
        }
        return ({ (*self).with(|__s| __s.ptr.clone()) } == { in_.with(|__s| __s.ptr.clone()) });
    }
    fn operator_ne(&self, in_: Ptr<__gnu_test_input_iterator_wrapper_const_wchar_t_>) -> bool {
        return !({
            let _in_: Ptr<__gnu_test_input_iterator_wrapper_const_wchar_t_> = (in_).clone();
            __gnu_test_input_iterator_wrapper_const_wchar_t_Impl::operator_eq(&(*self), _in_)
        });
    }
    fn operator_deref(&self) -> Ptr<i32> {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((!((*self).with(|__s| __s.SharedInfo.clone())).is_null())
                && ({ (*self).with(|__s| __s.ptr.clone()) } < {
                    (*self)
                        .with(|__s| __s.SharedInfo.clone())
                        .with(|__s| __s.last.clone())
                }))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (*self).with(|__s| __s.ptr.clone()) } >= {
                (*self)
                    .with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.first.clone())
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        return (*self).with(|__s| __s.ptr.clone());
    }
    fn operator_inc_9(&self) -> Ptr<__gnu_test_input_iterator_wrapper_const_wchar_t_> {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((!((*self).with(|__s| __s.SharedInfo.clone())).is_null())
                && ({ (*self).with(|__s| __s.ptr.clone()) } < {
                    (*self)
                        .with(|__s| __s.SharedInfo.clone())
                        .with(|__s| __s.last.clone())
                }))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (*self).with(|__s| __s.ptr.clone()) } >= {
                (*self)
                    .with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.first.clone())
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        field!((*self), ptr).with_mut(|__v| __v.postfix_inc());
        field!((*self).with(|__s| __s.SharedInfo.clone()), first)
            .write((*self).with(|__s| __s.ptr.clone()));
        return (*self).clone();
    }
}
pub trait __gnu_test_input_iterator_wrapper_const_wchar_t__post_inc_proxyImpl {
    fn operator_deref(
        &self,
    ) -> __gnu_test_input_iterator_wrapper_const_wchar_t__post_inc_proxy_deref_proxy {
        unimplemented!()
    }
}
impl __gnu_test_input_iterator_wrapper_const_wchar_t__post_inc_proxyImpl
    for Ptr<__gnu_test_input_iterator_wrapper_const_wchar_t__post_inc_proxy>
{
}
pub trait __gnu_test_input_iterator_wrapper_const_wchar_t__post_inc_proxy_deref_proxyImpl {
    fn to_pconsti32(&self) -> Ptr<i32> {
        unimplemented!()
    }
}
impl __gnu_test_input_iterator_wrapper_const_wchar_t__post_inc_proxy_deref_proxyImpl
    for Ptr<__gnu_test_input_iterator_wrapper_const_wchar_t__post_inc_proxy_deref_proxy>
{
}
pub trait __gnu_test_input_iterator_wrapper_wchar_t_Impl {
    fn operator_eq(&self, in_: Ptr<__gnu_test_input_iterator_wrapper_wchar_t_>) -> bool;
    fn operator_ne(&self, in_: Ptr<__gnu_test_input_iterator_wrapper_wchar_t_>) -> bool;
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_arrow(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_9(&self) -> Ptr<__gnu_test_input_iterator_wrapper_wchar_t_>;
    fn operator_post_inc_10(
        &self,
        mut _a0: i32,
    ) -> __gnu_test_input_iterator_wrapper_wchar_t__post_inc_proxy {
        unimplemented!()
    }
}
impl __gnu_test_input_iterator_wrapper_wchar_t_Impl
    for Ptr<__gnu_test_input_iterator_wrapper_wchar_t_>
{
    fn operator_eq(&self, in_: Ptr<__gnu_test_input_iterator_wrapper_wchar_t_>) -> bool {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((!((*self).with(|__s| __s.SharedInfo.clone())).is_null())
                && ({ (*self).with(|__s| __s.SharedInfo.clone()) } == {
                    in_.with(|__s| __s.SharedInfo.clone())
                }))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({ (*self).with(|__s| __s.ptr.clone()) } >= {
                (*self)
                    .with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.first.clone())
            }) && ({ in_.with(|__s| __s.ptr.clone()) } >= {
                (*self)
                    .with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.first.clone())
            })) {
                unimplemented!();
                std::process::abort();
            }
        }
        return ({ (*self).with(|__s| __s.ptr.clone()) } == { in_.with(|__s| __s.ptr.clone()) });
    }
    fn operator_ne(&self, in_: Ptr<__gnu_test_input_iterator_wrapper_wchar_t_>) -> bool {
        return !({
            let _in_: Ptr<__gnu_test_input_iterator_wrapper_wchar_t_> = (in_).clone();
            __gnu_test_input_iterator_wrapper_wchar_t_Impl::operator_eq(&(*self), _in_)
        });
    }
    fn operator_deref(&self) -> Ptr<i32> {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((!((*self).with(|__s| __s.SharedInfo.clone())).is_null())
                && ({ (*self).with(|__s| __s.ptr.clone()) } < {
                    (*self)
                        .with(|__s| __s.SharedInfo.clone())
                        .with(|__s| __s.last.clone())
                }))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (*self).with(|__s| __s.ptr.clone()) } >= {
                (*self)
                    .with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.first.clone())
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        return (*self).with(|__s| __s.ptr.clone());
    }
    fn operator_inc_9(&self) -> Ptr<__gnu_test_input_iterator_wrapper_wchar_t_> {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((!((*self).with(|__s| __s.SharedInfo.clone())).is_null())
                && ({ (*self).with(|__s| __s.ptr.clone()) } < {
                    (*self)
                        .with(|__s| __s.SharedInfo.clone())
                        .with(|__s| __s.last.clone())
                }))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (*self).with(|__s| __s.ptr.clone()) } >= {
                (*self)
                    .with(|__s| __s.SharedInfo.clone())
                    .with(|__s| __s.first.clone())
            }) {
                unimplemented!();
                std::process::abort();
            }
        }
        field!((*self), ptr).with_mut(|__v| __v.postfix_inc());
        field!((*self).with(|__s| __s.SharedInfo.clone()), first)
            .write((*self).with(|__s| __s.ptr.clone()));
        return (*self).clone();
    }
}
pub trait __gnu_test_input_iterator_wrapper_wchar_t__post_inc_proxyImpl {
    fn operator_deref(
        &self,
    ) -> __gnu_test_input_iterator_wrapper_wchar_t__post_inc_proxy_deref_proxy {
        unimplemented!()
    }
}
impl __gnu_test_input_iterator_wrapper_wchar_t__post_inc_proxyImpl
    for Ptr<__gnu_test_input_iterator_wrapper_wchar_t__post_inc_proxy>
{
}
pub trait __gnu_test_input_iterator_wrapper_wchar_t__post_inc_proxy_deref_proxyImpl {
    fn to_pconsti32(&self) -> Ptr<i32> {
        unimplemented!()
    }
}
impl __gnu_test_input_iterator_wrapper_wchar_t__post_inc_proxy_deref_proxyImpl
    for Ptr<__gnu_test_input_iterator_wrapper_wchar_t__post_inc_proxy_deref_proxy>
{
}
pub trait __gnu_test_object_counterImpl {
    fn destructor(&self);
    fn copy_assign(&self, _a0: Ptr<__gnu_test_object_counter>) -> Ptr<__gnu_test_object_counter>;
}
impl __gnu_test_object_counterImpl for Ptr<__gnu_test_object_counter> {
    fn destructor(&self) {
        (*count_6.with(Value::clone).borrow_mut()).prefix_dec();
    }
    fn copy_assign(&self, _a0: Ptr<__gnu_test_object_counter>) -> Ptr<__gnu_test_object_counter> {
        return (*self).clone();
    }
}
pub trait __gnu_test_scoped_fileImpl {
    fn destructor(&self);
    fn move_assign(&self, _a0: Ptr<__gnu_test_scoped_file>) -> Ptr<__gnu_test_scoped_file>;
}
impl __gnu_test_scoped_fileImpl for Ptr<__gnu_test_scoped_file> {
    fn destructor(&self) {
        if !(unimplemented!()) {
            unimplemented!();
        }
    }
    fn move_assign(&self, _a0: Ptr<__gnu_test_scoped_file>) -> Ptr<__gnu_test_scoped_file> {
        unimplemented!();
        return (*self).clone();
    }
}
pub trait __gnu_test_semaphoreImpl {
    fn destructor(&self) {
        unimplemented!()
    }
    fn signal(&self) {
        unimplemented!()
    }
    fn wait(&self) {
        unimplemented!()
    }
}
impl __gnu_test_semaphoreImpl for Ptr<__gnu_test_semaphore> {
    fn destructor(&self) {
        let val: Value<__gnu_test_semun> = Rc::new(RefCell::new(<__gnu_test_semun>::default()));
        __gnu_test_semun::val(val.as_pointer()).write(0);
        if ((*self).with(|__s| __s.pid_) == unimplemented!()) {
            unimplemented!();
        }
    }
    fn signal(&self) {
        let op: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([unimplemented!()])));
        if (unimplemented!() == -1_i32) {
            unimplemented!();
        }
    }
    fn wait(&self) {
        let op: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([unimplemented!()])));
        if (unimplemented!() == -1_i32) {
            unimplemented!();
        }
    }
}
pub trait __gnu_test_test_container_char____gnu_test_input_iterator_wrapper_Impl {
    fn it_3(&self, mut pos: i32) -> __gnu_test_input_iterator_wrapper_char_ {
        unimplemented!()
    }
    fn it_4(&self, pos: Ptr<i8>) -> __gnu_test_input_iterator_wrapper_char_;
    fn val(&self, mut pos: i32) -> Ptr<i8> {
        unimplemented!()
    }
    fn begin(&self) -> __gnu_test_input_iterator_wrapper_char_;
    fn end(&self) -> __gnu_test_input_iterator_wrapper_char_;
    fn size(&self) -> usize {
        unimplemented!()
    }
}
impl __gnu_test_test_container_char____gnu_test_input_iterator_wrapper_Impl
    for Ptr<__gnu_test_test_container_char____gnu_test_input_iterator_wrapper_>
{
    fn it_4(&self, mut pos: Ptr<i8>) -> __gnu_test_input_iterator_wrapper_char_ {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({ (pos).clone() } >= { (*self).with(|__s| __s.bounds.first.clone()) })
                && ({ (pos).clone() } <= { (*self).with(|__s| __s.bounds.last.clone()) }))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        return __gnu_test_input_iterator_wrapper_char_::new_1({ (pos).clone() }, {
            (field_ptr!((*self), bounds))
        });
    }
    fn begin(&self) -> __gnu_test_input_iterator_wrapper_char_ {
        return ({
            let _pos: Ptr<i8> = (*self).with(|__s| __s.bounds.first.clone());
            __gnu_test_test_container_char____gnu_test_input_iterator_wrapper_Impl::it_4(self, _pos)
        });
    }
    fn end(&self) -> __gnu_test_input_iterator_wrapper_char_ {
        return ({
            let _pos: Ptr<i8> = (*self).with(|__s| __s.bounds.last.clone());
            __gnu_test_test_container_char____gnu_test_input_iterator_wrapper_Impl::it_4(self, _pos)
        });
    }
}
pub trait __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_Impl {
    fn it_3(&self, mut pos: i32) -> __gnu_test_input_iterator_wrapper_const_char_ {
        unimplemented!()
    }
    fn it_4(&self, pos: Ptr<i8>) -> __gnu_test_input_iterator_wrapper_const_char_;
    fn val(&self, mut pos: i32) -> Ptr<i8> {
        unimplemented!()
    }
    fn begin(&self) -> __gnu_test_input_iterator_wrapper_const_char_;
    fn end(&self) -> __gnu_test_input_iterator_wrapper_const_char_;
    fn size(&self) -> usize {
        unimplemented!()
    }
}
impl __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_Impl
    for Ptr<__gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_>
{
    fn it_4(&self, mut pos: Ptr<i8>) -> __gnu_test_input_iterator_wrapper_const_char_ {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({ (pos).clone() } >= { (*self).with(|__s| __s.bounds.first.clone()) })
                && ({ (pos).clone() } <= { (*self).with(|__s| __s.bounds.last.clone()) }))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        return __gnu_test_input_iterator_wrapper_const_char_::new_1({ (pos).clone() }, {
            (field_ptr!((*self), bounds))
        });
    }
    fn begin(&self) -> __gnu_test_input_iterator_wrapper_const_char_ {
        return ({
            let _pos: Ptr<i8> = (*self).with(|__s| __s.bounds.first.clone());
            __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_Impl::it_4(
                self, _pos,
            )
        });
    }
    fn end(&self) -> __gnu_test_input_iterator_wrapper_const_char_ {
        return ({
            let _pos: Ptr<i8> = (*self).with(|__s| __s.bounds.last.clone());
            __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_Impl::it_4(
                self, _pos,
            )
        });
    }
}
pub trait __gnu_test_test_container_const_wchar_t____gnu_test_input_iterator_wrapper_Impl {
    fn it_3(&self, mut pos: i32) -> __gnu_test_input_iterator_wrapper_const_wchar_t_ {
        unimplemented!()
    }
    fn it_4(&self, pos: Ptr<i32>) -> __gnu_test_input_iterator_wrapper_const_wchar_t_;
    fn val(&self, mut pos: i32) -> Ptr<i32> {
        unimplemented!()
    }
    fn begin(&self) -> __gnu_test_input_iterator_wrapper_const_wchar_t_;
    fn end(&self) -> __gnu_test_input_iterator_wrapper_const_wchar_t_;
    fn size(&self) -> usize {
        unimplemented!()
    }
}
impl __gnu_test_test_container_const_wchar_t____gnu_test_input_iterator_wrapper_Impl
    for Ptr<__gnu_test_test_container_const_wchar_t____gnu_test_input_iterator_wrapper_>
{
    fn it_4(&self, mut pos: Ptr<i32>) -> __gnu_test_input_iterator_wrapper_const_wchar_t_ {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({ (pos).clone() } >= { (*self).with(|__s| __s.bounds.first.clone()) })
                && ({ (pos).clone() } <= { (*self).with(|__s| __s.bounds.last.clone()) }))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        return __gnu_test_input_iterator_wrapper_const_wchar_t_::new_1({ (pos).clone() }, {
            (field_ptr!((*self), bounds))
        });
    }
    fn begin(&self) -> __gnu_test_input_iterator_wrapper_const_wchar_t_ {
        return ({
            let _pos: Ptr<i32> = (*self).with(|__s| __s.bounds.first.clone());
            __gnu_test_test_container_const_wchar_t____gnu_test_input_iterator_wrapper_Impl::it_4(
                self, _pos,
            )
        });
    }
    fn end(&self) -> __gnu_test_input_iterator_wrapper_const_wchar_t_ {
        return ({
            let _pos: Ptr<i32> = (*self).with(|__s| __s.bounds.last.clone());
            __gnu_test_test_container_const_wchar_t____gnu_test_input_iterator_wrapper_Impl::it_4(
                self, _pos,
            )
        });
    }
}
pub trait __gnu_test_test_container_wchar_t____gnu_test_input_iterator_wrapper_Impl {
    fn it_3(&self, mut pos: i32) -> __gnu_test_input_iterator_wrapper_wchar_t_ {
        unimplemented!()
    }
    fn it_4(&self, pos: Ptr<i32>) -> __gnu_test_input_iterator_wrapper_wchar_t_;
    fn val(&self, mut pos: i32) -> Ptr<i32> {
        unimplemented!()
    }
    fn begin(&self) -> __gnu_test_input_iterator_wrapper_wchar_t_;
    fn end(&self) -> __gnu_test_input_iterator_wrapper_wchar_t_;
    fn size(&self) -> usize {
        unimplemented!()
    }
}
impl __gnu_test_test_container_wchar_t____gnu_test_input_iterator_wrapper_Impl
    for Ptr<__gnu_test_test_container_wchar_t____gnu_test_input_iterator_wrapper_>
{
    fn it_4(&self, mut pos: Ptr<i32>) -> __gnu_test_input_iterator_wrapper_wchar_t_ {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({ (pos).clone() } >= { (*self).with(|__s| __s.bounds.first.clone()) })
                && ({ (pos).clone() } <= { (*self).with(|__s| __s.bounds.last.clone()) }))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        return __gnu_test_input_iterator_wrapper_wchar_t_::new_1({ (pos).clone() }, {
            (field_ptr!((*self), bounds))
        });
    }
    fn begin(&self) -> __gnu_test_input_iterator_wrapper_wchar_t_ {
        return ({
            let _pos: Ptr<i32> = (*self).with(|__s| __s.bounds.first.clone());
            __gnu_test_test_container_wchar_t____gnu_test_input_iterator_wrapper_Impl::it_4(
                self, _pos,
            )
        });
    }
    fn end(&self) -> __gnu_test_input_iterator_wrapper_wchar_t_ {
        return ({
            let _pos: Ptr<i32> = (*self).with(|__s| __s.bounds.last.clone());
            __gnu_test_test_container_wchar_t____gnu_test_input_iterator_wrapper_Impl::it_4(
                self, _pos,
            )
        });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = test_paths_1.with(|_| ());
    let _ = count_6.with(|_| ());
    let _ = count__7.with(|_| ());
    let _ = throw_on__8.with(|_| ());
    let _ = count__9.with(|_| ());
    let _ = throw_on__10.with(|_| ());
    let _ = _M_count_11.with(|_| ());
    let _ = next_id__12.with(|_| ());
}
