extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe)]
pub struct CopyOnly {}
impl CopyOnly {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn copy_from(_a0: *const CopyOnly) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const CopyOnly) -> *mut CopyOnly {
        return &mut (*(self as *mut CopyOnly));
    }
}
impl Clone for CopyOnly {
    fn clone(&self) -> Self {
        unsafe { CopyOnly::copy_from(self as *const CopyOnly) }
    }
}
impl Default for CopyOnly {
    fn default() -> Self {
        unsafe { CopyOnly::new() }
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct NoexceptMoveOnly {}
impl NoexceptMoveOnly {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn move_from(_a0: *mut NoexceptMoveOnly) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut NoexceptMoveOnly) -> *mut NoexceptMoveOnly {
        return &mut (*(self as *mut NoexceptMoveOnly));
    }
}
impl Default for NoexceptMoveOnly {
    fn default() -> Self {
        unsafe { NoexceptMoveOnly::new() }
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveAssignUnsafe)]
pub struct NotMoveConstructible {}
impl NotMoveConstructible {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut NotMoveConstructible,
    ) -> *mut NotMoveConstructible {
        return &mut (*(self as *mut NotMoveConstructible));
    }
}
impl Default for NotMoveConstructible {
    fn default() -> Self {
        unsafe { NotMoveConstructible::new() }
    }
}
pub unsafe fn can_swap_0() -> bool {
    return unimplemented!();
}
pub unsafe fn can_swap_1() -> bool {
    return unimplemented!();
}
pub unsafe fn can_swap_2() -> bool {
    return unimplemented!();
}
pub unsafe fn test_unique_ptr_3() {
    let mut i: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
    let mut k: i32 = 0;
    'loop_: while ((k) < (3)) {
        {
            let _a0: *mut i32 = (Box::leak(Box::new(((k) + (1)))) as *mut i32);
            i[(k) as usize] = if _a0.is_null() {
                None
            } else {
                Some(Box::from_raw(_a0))
            }
        };
        k.prefix_inc();
    }
    let mut j: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
    let mut k: i32 = 0;
    'loop_: while ((k) < (3)) {
        {
            let _a0: *mut i32 = (Box::leak(Box::new(((k) + (4)))) as *mut i32);
            j[(k) as usize] = if _a0.is_null() {
                None
            } else {
                Some(Box::from_raw(_a0))
            }
        };
        k.prefix_inc();
    }
    unimplemented!();
    assert!(
        ((*i[(0) as usize]
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (4))
    );
    assert!(
        ((*i[(1) as usize]
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (5))
    );
    assert!(
        ((*i[(2) as usize]
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (6))
    );
    assert!(
        ((*j[(0) as usize]
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (1))
    );
    assert!(
        ((*j[(1) as usize]
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (2))
    );
    assert!(
        ((*j[(2) as usize]
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (3))
    );
}
pub unsafe fn test_4() -> bool {
    {
        let mut i: [i32; 3] = [1, 2, 3];
        let mut j: [i32; 3] = [4, 5, 6];
        unimplemented!();
        assert!(((i[(0) as usize]) == (4)));
        assert!(((i[(1) as usize]) == (5)));
        assert!(((i[(2) as usize]) == (6)));
        assert!(((j[(0) as usize]) == (1)));
        assert!(((j[(1) as usize]) == (2)));
        assert!(((j[(2) as usize]) == (3)));
    }
    {
        let mut a: [[i32; 2]; 2] = [[0, 1], [2, 3]];
        let mut b: [[i32; 2]; 2] = [[9, 8], [7, 6]];
        unimplemented!();
        assert!(((a[(0) as usize][(0) as usize]) == (9)));
        assert!(((a[(0) as usize][(1) as usize]) == (8)));
        assert!(((a[(1) as usize][(0) as usize]) == (7)));
        assert!(((a[(1) as usize][(1) as usize]) == (6)));
        assert!(((b[(0) as usize][(0) as usize]) == (0)));
        assert!(((b[(0) as usize][(1) as usize]) == (1)));
        assert!(((b[(1) as usize][(0) as usize]) == (2)));
        assert!(((b[(1) as usize][(1) as usize]) == (3)));
    }
    {
        let mut a: [[i32; 3]; 3] = [[0, 1, 2], [3, 4, 5], [6, 7, 8]];
        let mut b: [[i32; 3]; 3] = [[9, 8, 7], [6, 5, 4], [3, 2, 1]];
        unimplemented!();
        assert!(((a[(0) as usize][(0) as usize]) == (9)));
        assert!(((a[(0) as usize][(1) as usize]) == (8)));
        assert!(((a[(0) as usize][(2) as usize]) == (7)));
        assert!(((a[(1) as usize][(0) as usize]) == (6)));
        assert!(((a[(1) as usize][(1) as usize]) == (5)));
        assert!(((a[(1) as usize][(2) as usize]) == (4)));
        assert!(((a[(2) as usize][(0) as usize]) == (3)));
        assert!(((a[(2) as usize][(1) as usize]) == (2)));
        assert!(((a[(2) as usize][(2) as usize]) == (1)));
        assert!(((b[(0) as usize][(0) as usize]) == (0)));
        assert!(((b[(0) as usize][(1) as usize]) == (1)));
        assert!(((b[(0) as usize][(2) as usize]) == (2)));
        assert!(((b[(1) as usize][(0) as usize]) == (3)));
        assert!(((b[(1) as usize][(1) as usize]) == (4)));
        assert!(((b[(1) as usize][(2) as usize]) == (5)));
        assert!(((b[(2) as usize][(0) as usize]) == (6)));
        assert!(((b[(2) as usize][(1) as usize]) == (7)));
        assert!(((b[(2) as usize][(2) as usize]) == (8)));
    }
    {
        const _: () = assert!(true, "can_swap<CA&>()");;
        const _: () = assert!(true, "can_swap<MA&>()");;
        const _: () = assert!(true, "!can_swap<NA&>()");;
        let mut ca: [CopyOnly; 42] = std::array::from_fn::<_, 42, _>(|_| CopyOnly::new());
        let mut ma: [NoexceptMoveOnly; 42] =
            std::array::from_fn::<_, 42, _>(|_| NoexceptMoveOnly::new());
        const _: () = assert!(true, "!noexcept(std::swap(ca, ca))");;
        const _: () = assert!(true, "noexcept(std::swap(ma, ma))");;
    }
    {
        (unsafe { test_unique_ptr_3() });
    }
    return true;
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_4() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
