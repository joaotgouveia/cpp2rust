extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct indirect_less {}
impl indirect_less {
    pub unsafe fn operator_call_1_class_std_unique_ptr_int_(
        &mut self,
        x: *const Option<Box<i32>>,
        y: *const Option<Box<i32>>,
    ) -> bool {
        return ((*(*x)
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            < (*(*y)
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32)));
    }
}
impl Callable2<*const Option<Box<i32>>, *const Option<Box<i32>>, bool> for indirect_less {
    fn call(&self, a1: *const Option<Box<i32>>, a2: *const Option<Box<i32>>) -> bool {
        unsafe {
            indirect_less::operator_call_1_class_std_unique_ptr_int_(
                &mut *(&raw const *self).cast_mut(),
                a1,
                a2,
            )
        }
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    {
        let mut v: Vec<i32> = (0..(1000_usize) as usize)
            .map(|_| 0_i32)
            .collect::<Vec<_>>();
        let mut i: i32 = 0;
        'loop_: while ((i as usize) < (v.len())) {
            (*(&mut v).as_mut_ptr().add((i as usize) as usize)) = i;
            i.prefix_inc();
        }
        unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut v: Vec<Option<Box<i32>>> =
            (0..(1000_usize) as usize).map(|_| None).collect::<Vec<_>>();
        let mut i: i32 = 0;
        'loop_: while ((i as usize) < (v.len())) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(i)) as *mut i32);
                (*(&mut v).as_mut_ptr().add((i as usize) as usize)) = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            i.prefix_inc();
        }
        unimplemented!();
        assert!(unimplemented!());
        assert!(
            ((*(*(&mut v).as_mut_ptr().add(0_usize as usize))
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*(*(&mut v).as_mut_ptr().add(1_usize as usize))
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*(*(&mut v).as_mut_ptr().add(2_usize as usize))
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
