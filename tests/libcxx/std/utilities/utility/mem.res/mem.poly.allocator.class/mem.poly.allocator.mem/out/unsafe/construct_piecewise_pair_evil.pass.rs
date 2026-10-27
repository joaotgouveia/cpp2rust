extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct EvilAlloc_char_ {
    pub inner_: (),
}
impl EvilAlloc_char_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            inner_: unimplemented!(),
        };
        this
    }
    pub unsafe fn new_2(a: *mut ()) -> Self {
        let mut this = Self {
            inner_: unimplemented!(),
        };
        this
    }
}
impl Default for EvilAlloc_char_ {
    fn default() -> Self {
        unsafe { EvilAlloc_char_::new_1() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct WidgetV0 {
    value_: (),
}
impl WidgetV0 {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { value_: v };
        this
    }
    pub unsafe fn holds(&self, mut v: (), _a1: *const ()) -> () {
        return ((self.value_) == (v));
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct WidgetV1 {
    value_: (),
    alloc_: EvilAlloc_char_,
}
impl WidgetV1 {
    pub unsafe fn new_1(mut v: ()) -> Self {
        let mut this = Self {
            value_: v,
            alloc_: EvilAlloc_char_::new_1(),
        };
        this
    }
    pub unsafe fn new_2(mut _a0: (), mut a: EvilAlloc_char_, mut v: ()) -> Self {
        let mut this = Self {
            value_: v,
            alloc_: a.clone(),
        };
        this
    }
    pub unsafe fn holds(&self, mut v: (), a: *const ()) -> () {
        return ((self.value_) == (v)) && (unimplemented!());
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct WidgetV2 {
    value_: (),
    alloc_: EvilAlloc_char_,
}
impl WidgetV2 {
    pub unsafe fn new_1(mut v: ()) -> Self {
        let mut this = Self {
            value_: v,
            alloc_: EvilAlloc_char_::new_1(),
        };
        this
    }
    pub unsafe fn new_2(mut v: (), mut a: EvilAlloc_char_) -> Self {
        let mut this = Self {
            value_: v,
            alloc_: a.clone(),
        };
        this
    }
    pub unsafe fn holds(&self, mut v: (), mut a: ()) -> () {
        return ((self.value_) == (v)) && (unimplemented!());
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct WidgetV3 {
    value_: (),
    alloc_: EvilAlloc_char_,
}
impl WidgetV3 {
    pub unsafe fn new_1(mut v: ()) -> Self {
        let mut this = Self {
            value_: v,
            alloc_: EvilAlloc_char_::new_1(),
        };
        this
    }
    pub unsafe fn new_2(mut _a0: (), mut a: EvilAlloc_char_, mut v: ()) -> Self {
        let mut this = Self {
            value_: v,
            alloc_: a.clone(),
        };
        this
    }
    pub unsafe fn new_3(mut v: (), mut a: EvilAlloc_char_) -> Self {
        let mut this = Self {
            value_: v,
            alloc_: a.clone(),
        };
        this
    }
    pub unsafe fn holds(&self, mut v: (), mut a: ()) -> () {
        return ((self.value_) == (v)) && (unimplemented!());
    }
}
const _: () = assert!(
    true,
    "std::uses_allocator<WidgetV1, EvilAlloc<char>>::value"
);
const _: () = assert!(
    true,
    "std::uses_allocator<WidgetV2, EvilAlloc<char>>::value"
);
const _: () = assert!(
    true,
    "std::uses_allocator<WidgetV3, EvilAlloc<char>>::value"
);
const _: () = assert!(
    true,
    "std::uses_allocator<WidgetV1, std::pmr::polymorphic_allocator<char>>::value"
);
const _: () = assert!(
    true,
    "std::uses_allocator<WidgetV2, std::pmr::polymorphic_allocator<char>>::value"
);
const _: () = assert!(
    true,
    "std::uses_allocator<WidgetV3, std::pmr::polymorphic_allocator<char>>::value"
);
pub unsafe fn test_evil_0() {
    let mut pma: () = unimplemented!();
    {
        let mut buffer: [(); 8] = [unimplemented!(); 8];
        let mut p: *mut () = (buffer.as_mut_ptr() as *mut ());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_evil_1() {
    let mut pma: () = unimplemented!();
    {
        let mut buffer: [(); 24] = [unimplemented!(); 24];
        let mut p: *mut () = (buffer.as_mut_ptr() as *mut ());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_evil_2() {
    let mut pma: () = unimplemented!();
    {
        let mut buffer: [(); 24] = [unimplemented!(); 24];
        let mut p: *mut () = (buffer.as_mut_ptr() as *mut ());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_evil_3() {
    let mut pma: () = unimplemented!();
    {
        let mut buffer: [(); 24] = [unimplemented!(); 24];
        let mut p: *mut () = (buffer.as_mut_ptr() as *mut ());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_evil_4() {
    let mut pma: () = unimplemented!();
    {
        let mut buffer: [(); 24] = [unimplemented!(); 24];
        let mut p: *mut () = (buffer.as_mut_ptr() as *mut ());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_evil_5() {
    let mut pma: () = unimplemented!();
    {
        let mut buffer: [(); 32] = [unimplemented!(); 32];
        let mut p: *mut () = (buffer.as_mut_ptr() as *mut ());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_evil_6() {
    let mut pma: () = unimplemented!();
    {
        let mut buffer: [(); 32] = [unimplemented!(); 32];
        let mut p: *mut () = (buffer.as_mut_ptr() as *mut ());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_evil_7() {
    let mut pma: () = unimplemented!();
    {
        let mut buffer: [(); 32] = [unimplemented!(); 32];
        let mut p: *mut () = (buffer.as_mut_ptr() as *mut ());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_evil_8() {
    let mut pma: () = unimplemented!();
    {
        let mut buffer: [(); 24] = [unimplemented!(); 24];
        let mut p: *mut () = (buffer.as_mut_ptr() as *mut ());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_evil_9() {
    let mut pma: () = unimplemented!();
    {
        let mut buffer: [(); 32] = [unimplemented!(); 32];
        let mut p: *mut () = (buffer.as_mut_ptr() as *mut ());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_evil_10() {
    let mut pma: () = unimplemented!();
    {
        let mut buffer: [(); 32] = [unimplemented!(); 32];
        let mut p: *mut () = (buffer.as_mut_ptr() as *mut ());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_evil_11() {
    let mut pma: () = unimplemented!();
    {
        let mut buffer: [(); 32] = [unimplemented!(); 32];
        let mut p: *mut () = (buffer.as_mut_ptr() as *mut ());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_evil_12() {
    let mut pma: () = unimplemented!();
    {
        let mut buffer: [(); 24] = [unimplemented!(); 24];
        let mut p: *mut () = (buffer.as_mut_ptr() as *mut ());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_evil_13() {
    let mut pma: () = unimplemented!();
    {
        let mut buffer: [(); 32] = [unimplemented!(); 32];
        let mut p: *mut () = (buffer.as_mut_ptr() as *mut ());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_evil_14() {
    let mut pma: () = unimplemented!();
    {
        let mut buffer: [(); 32] = [unimplemented!(); 32];
        let mut p: *mut () = (buffer.as_mut_ptr() as *mut ());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_evil_15() {
    let mut pma: () = unimplemented!();
    {
        let mut buffer: [(); 32] = [unimplemented!(); 32];
        let mut p: *mut () = (buffer.as_mut_ptr() as *mut ());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_evil_0() });
    (unsafe { test_evil_1() });
    (unsafe { test_evil_2() });
    (unsafe { test_evil_3() });
    (unsafe { test_evil_4() });
    (unsafe { test_evil_5() });
    (unsafe { test_evil_6() });
    (unsafe { test_evil_7() });
    (unsafe { test_evil_8() });
    (unsafe { test_evil_9() });
    (unsafe { test_evil_10() });
    (unsafe { test_evil_11() });
    (unsafe { test_evil_12() });
    (unsafe { test_evil_13() });
    (unsafe { test_evil_14() });
    (unsafe { test_evil_15() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
