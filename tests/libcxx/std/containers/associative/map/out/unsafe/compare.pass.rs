extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
impl Key {
    pub unsafe fn new(_a0: *const i32) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, ptr)]
pub struct Key {}
impl Key {
    pub unsafe fn operator_lt(&self, _a0: *const Key) -> bool {
        return false;
    }
}
impl std::cmp::Ord for Key {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if Key::operator_lt(self, other as *const Key) {
                std::cmp::Ordering::Less
            } else if Key::operator_lt(other, self as *const Key) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(Key::operator_lt(self, other as *const Key))
                && !(Key::operator_lt(other, self as *const Key))
        }
    }
}
impl std::cmp::Eq for Key {}
pub unsafe fn test_0() -> bool {
    {
        let mut m_empty: UnsafeMap<Key, (Key, i32)> = UnsafeMap::new();
        let mut m_contains: UnsafeMap<Key, (Key, i32)> = UnsafeMap::new();
        (*m_contains.subscript_move(
            &mut {
                let mut __tmp_0: i32 = 0;
                Key::new({ &mut __tmp_0 })
            },
            |__k| cpp_move_unsafe!(__k),
        )) = 42;
        let mut it: UnsafeBidirectionalIterator<(Key, i32)> = m_empty.find(&{
            let mut __tmp_1: i32 = 0;
            Key::new({ &mut __tmp_1 })
        });
        assert!(it == m_empty.end());
        it = m_contains.find(&{
            let mut __tmp_2: i32 = 0;
            Key::new({ &mut __tmp_2 })
        });
        assert!(it != m_contains.end());
    }
    {
        let mut map: UnsafeMap<Key, (Key, i32)> = UnsafeMap::new();
        let mut result: (UnsafeBidirectionalIterator<(Key, i32)>, bool) = unimplemented!();
        assert!((result.1 as bool));
        assert!((((*result.0.iter_deref()).1) == (42)));
        let mut result2: (UnsafeBidirectionalIterator<(Key, i32)>, bool) = unimplemented!();
        assert!(!(result2.1));
        assert!(
            ((*map.subscript_move(
                &mut {
                    let mut __tmp_3: i32 = 0;
                    Key::new({ &mut __tmp_3 })
                },
                |__k| cpp_move_unsafe!(__k)
            )) == (42))
        );
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
    (unsafe { test_0() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
