extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct __fsid_t {
    #[offset(0)]
    #[byte_size(8)]
    pub __val: Value<Box<[i32]>>,
}
impl Clone for __fsid_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __fsid_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(32)]
pub struct flock {
    #[offset(0)]
    pub l_type: i16,
    #[offset(2)]
    pub l_whence: i16,
    #[offset(8)]
    pub l_start: i64,
    #[offset(16)]
    pub l_len: i64,
    #[offset(24)]
    pub l_pid: i32,
}
impl Clone for flock {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for flock {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct timespec {
    #[offset(0)]
    pub tv_sec: i64,
    #[offset(8)]
    pub tv_nsec: i64,
}
impl Clone for timespec {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for timespec {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(144)]
pub struct stat {
    #[offset(0)]
    pub st_dev: u64,
    #[offset(8)]
    pub st_ino: u64,
    #[offset(16)]
    pub st_nlink: u64,
    #[offset(24)]
    pub st_mode: u32,
    #[offset(28)]
    pub st_uid: u32,
    #[offset(32)]
    pub st_gid: u32,
    #[offset(36)]
    pub __pad0: i32,
    #[offset(40)]
    pub st_rdev: u64,
    #[offset(48)]
    pub st_size: i64,
    #[offset(56)]
    pub st_blksize: i64,
    #[offset(64)]
    pub st_blocks: i64,
    #[offset(72)]
    #[byte_size(16)]
    pub st_atim: timespec,
    #[offset(88)]
    #[byte_size(16)]
    pub st_mtim: timespec,
    #[offset(104)]
    #[byte_size(16)]
    pub st_ctim: timespec,
    #[offset(120)]
    #[byte_size(24)]
    pub __glibc_reserved: Value<Box<[i64]>>,
}
impl Clone for stat {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for stat {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct __mbstate_t {
    #[offset(0)]
    pub __count: i32,
    #[offset(4)]
    #[byte_size(4)]
    pub __value: anon_0,
}
impl Clone for __mbstate_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __mbstate_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct anon_0 {
    #[offset(0)]
    pub __wch: u32,
    #[offset(0)]
    #[byte_size(4)]
    pub __wchb: Value<Box<[i8]>>,
}
impl Clone for anon_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_0 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct _G_fpos_t {
    #[offset(0)]
    pub __pos: i64,
    #[offset(8)]
    #[byte_size(8)]
    pub __state: __mbstate_t,
}
impl Clone for _G_fpos_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _G_fpos_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct _G_fpos64_t {
    #[offset(0)]
    pub __pos: i64,
    #[offset(8)]
    #[byte_size(8)]
    pub __state: __mbstate_t,
}
impl Clone for _G_fpos64_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _G_fpos64_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(216)]
pub struct _IO_FILE {
    #[offset(0)]
    pub _flags: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub _IO_read_ptr: Ptr<i8>,
    #[offset(16)]
    #[byte_size(8)]
    pub _IO_read_end: Ptr<i8>,
    #[offset(24)]
    #[byte_size(8)]
    pub _IO_read_base: Ptr<i8>,
    #[offset(32)]
    #[byte_size(8)]
    pub _IO_write_base: Ptr<i8>,
    #[offset(40)]
    #[byte_size(8)]
    pub _IO_write_ptr: Ptr<i8>,
    #[offset(48)]
    #[byte_size(8)]
    pub _IO_write_end: Ptr<i8>,
    #[offset(56)]
    #[byte_size(8)]
    pub _IO_buf_base: Ptr<i8>,
    #[offset(64)]
    #[byte_size(8)]
    pub _IO_buf_end: Ptr<i8>,
    #[offset(72)]
    #[byte_size(8)]
    pub _IO_save_base: Ptr<i8>,
    #[offset(80)]
    #[byte_size(8)]
    pub _IO_backup_base: Ptr<i8>,
    #[offset(88)]
    #[byte_size(8)]
    pub _IO_save_end: Ptr<i8>,
    #[offset(96)]
    #[byte_size(8)]
    pub _markers: Ptr<_IO_marker>,
    #[offset(104)]
    #[byte_size(8)]
    pub _chain: Ptr<_IO_FILE>,
    #[offset(112)]
    pub _fileno: i32,
    #[offset(116)]
    pub _flags2: i32,
    #[offset(120)]
    pub _old_offset: i64,
    #[offset(128)]
    pub _cur_column: u16,
    #[offset(130)]
    pub _vtable_offset: i8,
    #[offset(131)]
    #[byte_size(1)]
    pub _shortbuf: Value<Box<[i8]>>,
    #[offset(136)]
    #[byte_size(8)]
    pub _lock: AnyPtr,
    #[offset(144)]
    pub _offset: i64,
    #[offset(152)]
    #[byte_size(8)]
    pub _codecvt: Ptr<_IO_codecvt>,
    #[offset(160)]
    #[byte_size(8)]
    pub _wide_data: Ptr<_IO_wide_data>,
    #[offset(168)]
    #[byte_size(8)]
    pub _freeres_list: Ptr<_IO_FILE>,
    #[offset(176)]
    #[byte_size(8)]
    pub _freeres_buf: AnyPtr,
    #[offset(184)]
    pub __pad5: usize,
    #[offset(192)]
    pub _mode: i32,
    #[offset(196)]
    #[byte_size(20)]
    pub _unused2: Value<Box<[i8]>>,
}
impl Clone for _IO_FILE {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _IO_FILE {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(32)]
pub struct _IO_cookie_io_functions_t {
    #[offset(0)]
    #[byte_size(8)]
    pub read: FnPtr<fn(AnyPtr, Ptr<i8>, usize) -> i64>,
    #[offset(8)]
    #[byte_size(8)]
    pub write: FnPtr<fn(AnyPtr, Ptr<i8>, usize) -> i64>,
    #[offset(16)]
    #[byte_size(8)]
    pub seek: FnPtr<fn(AnyPtr, Ptr<i64>, i32) -> i32>,
    #[offset(24)]
    #[byte_size(8)]
    pub close: FnPtr<fn(AnyPtr) -> i32>,
}
impl Clone for _IO_cookie_io_functions_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _IO_cookie_io_functions_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(232)]
pub struct __locale_struct {
    #[offset(0)]
    #[byte_size(104)]
    pub __locales: Value<Box<[Ptr<__locale_data>]>>,
    #[offset(104)]
    #[byte_size(8)]
    pub __ctype_b: Ptr<u16>,
    #[offset(112)]
    #[byte_size(8)]
    pub __ctype_tolower: Ptr<i32>,
    #[offset(120)]
    #[byte_size(8)]
    pub __ctype_toupper: Ptr<i32>,
    #[offset(128)]
    #[byte_size(104)]
    pub __names: Value<Box<[Ptr<i8>]>>,
}
impl Clone for __locale_struct {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __locale_struct {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct winsize {
    #[offset(0)]
    pub ws_row: u16,
    #[offset(2)]
    pub ws_col: u16,
    #[offset(4)]
    pub ws_xpixel: u16,
    #[offset(6)]
    pub ws_ypixel: u16,
}
impl Clone for winsize {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for winsize {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(18)]
pub struct termio {
    #[offset(0)]
    pub c_iflag: u16,
    #[offset(2)]
    pub c_oflag: u16,
    #[offset(4)]
    pub c_cflag: u16,
    #[offset(6)]
    pub c_lflag: u16,
    #[offset(8)]
    pub c_line: u8,
    #[offset(9)]
    #[byte_size(8)]
    pub c_cc: Value<Box<[u8]>>,
}
impl Clone for termio {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for termio {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(128)]
pub struct __sigset_t {
    #[offset(0)]
    #[byte_size(128)]
    pub __val: Value<Box<[u64]>>,
}
impl Clone for __sigset_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __sigset_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct timeval {
    #[offset(0)]
    pub tv_sec: i64,
    #[offset(8)]
    pub tv_usec: i64,
}
impl Clone for timeval {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for timeval {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(128)]
pub struct fd_set {
    #[offset(0)]
    #[byte_size(128)]
    pub __fds_bits: Value<Box<[i64]>>,
}
impl Clone for fd_set {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for fd_set {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct __atomic_wide_counter {
    #[offset(0)]
    pub __value64: u64,
    #[offset(0)]
    #[byte_size(8)]
    pub __value32: anon_1,
}
impl Clone for __atomic_wide_counter {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __atomic_wide_counter {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct anon_1 {
    #[offset(0)]
    pub __low: u32,
    #[offset(4)]
    pub __high: u32,
}
impl Clone for anon_1 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_1 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct __pthread_internal_list {
    #[offset(0)]
    #[byte_size(8)]
    pub __prev: Ptr<__pthread_internal_list>,
    #[offset(8)]
    #[byte_size(8)]
    pub __next: Ptr<__pthread_internal_list>,
}
impl Clone for __pthread_internal_list {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_internal_list {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct __pthread_internal_slist {
    #[offset(0)]
    #[byte_size(8)]
    pub __next: Ptr<__pthread_internal_slist>,
}
impl Clone for __pthread_internal_slist {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_internal_slist {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(40)]
pub struct __pthread_mutex_s {
    #[offset(0)]
    pub __lock: i32,
    #[offset(4)]
    pub __count: u32,
    #[offset(8)]
    pub __owner: i32,
    #[offset(12)]
    pub __nusers: u32,
    #[offset(16)]
    pub __kind: i32,
    #[offset(20)]
    pub __spins: i16,
    #[offset(22)]
    pub __elision: i16,
    #[offset(24)]
    #[byte_size(16)]
    pub __list: __pthread_internal_list,
}
impl Clone for __pthread_mutex_s {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_mutex_s {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(56)]
pub struct __pthread_rwlock_arch_t {
    #[offset(0)]
    pub __readers: u32,
    #[offset(4)]
    pub __writers: u32,
    #[offset(8)]
    pub __wrphase_futex: u32,
    #[offset(12)]
    pub __writers_futex: u32,
    #[offset(16)]
    pub __pad3: u32,
    #[offset(20)]
    pub __pad4: u32,
    #[offset(24)]
    pub __cur_writer: i32,
    #[offset(28)]
    pub __shared: i32,
    #[offset(32)]
    pub __rwelision: i8,
    #[offset(33)]
    #[byte_size(7)]
    pub __pad1: Value<Box<[u8]>>,
    #[offset(40)]
    pub __pad2: u64,
    #[offset(48)]
    pub __flags: u32,
}
impl Clone for __pthread_rwlock_arch_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_rwlock_arch_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(48)]
pub struct __pthread_cond_s {
    #[offset(0)]
    #[byte_size(8)]
    pub __wseq: __atomic_wide_counter,
    #[offset(8)]
    #[byte_size(8)]
    pub __g1_start: __atomic_wide_counter,
    #[offset(16)]
    #[byte_size(8)]
    pub __g_refs: Value<Box<[u32]>>,
    #[offset(24)]
    #[byte_size(8)]
    pub __g_size: Value<Box<[u32]>>,
    #[offset(32)]
    pub __g1_orig_size: u32,
    #[offset(36)]
    pub __wrefs: u32,
    #[offset(40)]
    #[byte_size(8)]
    pub __g_signals: Value<Box<[u32]>>,
}
impl Clone for __pthread_cond_s {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_cond_s {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct __once_flag {
    #[offset(0)]
    pub __data: i32,
}
impl Clone for __once_flag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __once_flag {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct pthread_mutexattr_t {
    #[offset(0)]
    #[byte_size(4)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i32,
}
impl Clone for pthread_mutexattr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_mutexattr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct pthread_condattr_t {
    #[offset(0)]
    #[byte_size(4)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i32,
}
impl Clone for pthread_condattr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_condattr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(56)]
pub struct pthread_attr_t {
    #[offset(0)]
    #[byte_size(56)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i64,
}
impl Clone for pthread_attr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_attr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(40)]
pub struct pthread_mutex_t {
    #[offset(0)]
    #[byte_size(40)]
    pub __data: __pthread_mutex_s,
    #[offset(0)]
    #[byte_size(40)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i64,
}
impl Clone for pthread_mutex_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_mutex_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(48)]
pub struct pthread_cond_t {
    #[offset(0)]
    #[byte_size(48)]
    pub __data: __pthread_cond_s,
    #[offset(0)]
    #[byte_size(48)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i64,
}
impl Clone for pthread_cond_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_cond_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(56)]
pub struct pthread_rwlock_t {
    #[offset(0)]
    #[byte_size(56)]
    pub __data: __pthread_rwlock_arch_t,
    #[offset(0)]
    #[byte_size(56)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i64,
}
impl Clone for pthread_rwlock_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_rwlock_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct pthread_rwlockattr_t {
    #[offset(0)]
    #[byte_size(8)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i64,
}
impl Clone for pthread_rwlockattr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_rwlockattr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(32)]
pub struct pthread_barrier_t {
    #[offset(0)]
    #[byte_size(32)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i64,
}
impl Clone for pthread_barrier_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_barrier_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct pthread_barrierattr_t {
    #[offset(0)]
    #[byte_size(4)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
    pub __align: i32,
}
impl Clone for pthread_barrierattr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_barrierattr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_2 = u32;
pub const anon_2__SC_ARG_MAX: anon_2 = 0;
pub const anon_2__SC_CHILD_MAX: anon_2 = 1;
pub const anon_2__SC_CLK_TCK: anon_2 = 2;
pub const anon_2__SC_NGROUPS_MAX: anon_2 = 3;
pub const anon_2__SC_OPEN_MAX: anon_2 = 4;
pub const anon_2__SC_STREAM_MAX: anon_2 = 5;
pub const anon_2__SC_TZNAME_MAX: anon_2 = 6;
pub const anon_2__SC_JOB_CONTROL: anon_2 = 7;
pub const anon_2__SC_SAVED_IDS: anon_2 = 8;
pub const anon_2__SC_REALTIME_SIGNALS: anon_2 = 9;
pub const anon_2__SC_PRIORITY_SCHEDULING: anon_2 = 10;
pub const anon_2__SC_TIMERS: anon_2 = 11;
pub const anon_2__SC_ASYNCHRONOUS_IO: anon_2 = 12;
pub const anon_2__SC_PRIORITIZED_IO: anon_2 = 13;
pub const anon_2__SC_SYNCHRONIZED_IO: anon_2 = 14;
pub const anon_2__SC_FSYNC: anon_2 = 15;
pub const anon_2__SC_MAPPED_FILES: anon_2 = 16;
pub const anon_2__SC_MEMLOCK: anon_2 = 17;
pub const anon_2__SC_MEMLOCK_RANGE: anon_2 = 18;
pub const anon_2__SC_MEMORY_PROTECTION: anon_2 = 19;
pub const anon_2__SC_MESSAGE_PASSING: anon_2 = 20;
pub const anon_2__SC_SEMAPHORES: anon_2 = 21;
pub const anon_2__SC_SHARED_MEMORY_OBJECTS: anon_2 = 22;
pub const anon_2__SC_AIO_LISTIO_MAX: anon_2 = 23;
pub const anon_2__SC_AIO_MAX: anon_2 = 24;
pub const anon_2__SC_AIO_PRIO_DELTA_MAX: anon_2 = 25;
pub const anon_2__SC_DELAYTIMER_MAX: anon_2 = 26;
pub const anon_2__SC_MQ_OPEN_MAX: anon_2 = 27;
pub const anon_2__SC_MQ_PRIO_MAX: anon_2 = 28;
pub const anon_2__SC_VERSION: anon_2 = 29;
pub const anon_2__SC_PAGESIZE: anon_2 = 30;
pub const anon_2__SC_RTSIG_MAX: anon_2 = 31;
pub const anon_2__SC_SEM_NSEMS_MAX: anon_2 = 32;
pub const anon_2__SC_SEM_VALUE_MAX: anon_2 = 33;
pub const anon_2__SC_SIGQUEUE_MAX: anon_2 = 34;
pub const anon_2__SC_TIMER_MAX: anon_2 = 35;
pub const anon_2__SC_BC_BASE_MAX: anon_2 = 36;
pub const anon_2__SC_BC_DIM_MAX: anon_2 = 37;
pub const anon_2__SC_BC_SCALE_MAX: anon_2 = 38;
pub const anon_2__SC_BC_STRING_MAX: anon_2 = 39;
pub const anon_2__SC_COLL_WEIGHTS_MAX: anon_2 = 40;
pub const anon_2__SC_EQUIV_CLASS_MAX: anon_2 = 41;
pub const anon_2__SC_EXPR_NEST_MAX: anon_2 = 42;
pub const anon_2__SC_LINE_MAX: anon_2 = 43;
pub const anon_2__SC_RE_DUP_MAX: anon_2 = 44;
pub const anon_2__SC_CHARCLASS_NAME_MAX: anon_2 = 45;
pub const anon_2__SC_2_VERSION: anon_2 = 46;
pub const anon_2__SC_2_C_BIND: anon_2 = 47;
pub const anon_2__SC_2_C_DEV: anon_2 = 48;
pub const anon_2__SC_2_FORT_DEV: anon_2 = 49;
pub const anon_2__SC_2_FORT_RUN: anon_2 = 50;
pub const anon_2__SC_2_SW_DEV: anon_2 = 51;
pub const anon_2__SC_2_LOCALEDEF: anon_2 = 52;
pub const anon_2__SC_PII: anon_2 = 53;
pub const anon_2__SC_PII_XTI: anon_2 = 54;
pub const anon_2__SC_PII_SOCKET: anon_2 = 55;
pub const anon_2__SC_PII_INTERNET: anon_2 = 56;
pub const anon_2__SC_PII_OSI: anon_2 = 57;
pub const anon_2__SC_POLL: anon_2 = 58;
pub const anon_2__SC_SELECT: anon_2 = 59;
pub const anon_2__SC_UIO_MAXIOV: anon_2 = 60;
pub const anon_2__SC_IOV_MAX: anon_2 = 60;
pub const anon_2__SC_PII_INTERNET_STREAM: anon_2 = 61;
pub const anon_2__SC_PII_INTERNET_DGRAM: anon_2 = 62;
pub const anon_2__SC_PII_OSI_COTS: anon_2 = 63;
pub const anon_2__SC_PII_OSI_CLTS: anon_2 = 64;
pub const anon_2__SC_PII_OSI_M: anon_2 = 65;
pub const anon_2__SC_T_IOV_MAX: anon_2 = 66;
pub const anon_2__SC_THREADS: anon_2 = 67;
pub const anon_2__SC_THREAD_SAFE_FUNCTIONS: anon_2 = 68;
pub const anon_2__SC_GETGR_R_SIZE_MAX: anon_2 = 69;
pub const anon_2__SC_GETPW_R_SIZE_MAX: anon_2 = 70;
pub const anon_2__SC_LOGIN_NAME_MAX: anon_2 = 71;
pub const anon_2__SC_TTY_NAME_MAX: anon_2 = 72;
pub const anon_2__SC_THREAD_DESTRUCTOR_ITERATIONS: anon_2 = 73;
pub const anon_2__SC_THREAD_KEYS_MAX: anon_2 = 74;
pub const anon_2__SC_THREAD_STACK_MIN: anon_2 = 75;
pub const anon_2__SC_THREAD_THREADS_MAX: anon_2 = 76;
pub const anon_2__SC_THREAD_ATTR_STACKADDR: anon_2 = 77;
pub const anon_2__SC_THREAD_ATTR_STACKSIZE: anon_2 = 78;
pub const anon_2__SC_THREAD_PRIORITY_SCHEDULING: anon_2 = 79;
pub const anon_2__SC_THREAD_PRIO_INHERIT: anon_2 = 80;
pub const anon_2__SC_THREAD_PRIO_PROTECT: anon_2 = 81;
pub const anon_2__SC_THREAD_PROCESS_SHARED: anon_2 = 82;
pub const anon_2__SC_NPROCESSORS_CONF: anon_2 = 83;
pub const anon_2__SC_NPROCESSORS_ONLN: anon_2 = 84;
pub const anon_2__SC_PHYS_PAGES: anon_2 = 85;
pub const anon_2__SC_AVPHYS_PAGES: anon_2 = 86;
pub const anon_2__SC_ATEXIT_MAX: anon_2 = 87;
pub const anon_2__SC_PASS_MAX: anon_2 = 88;
pub const anon_2__SC_XOPEN_VERSION: anon_2 = 89;
pub const anon_2__SC_XOPEN_XCU_VERSION: anon_2 = 90;
pub const anon_2__SC_XOPEN_UNIX: anon_2 = 91;
pub const anon_2__SC_XOPEN_CRYPT: anon_2 = 92;
pub const anon_2__SC_XOPEN_ENH_I18N: anon_2 = 93;
pub const anon_2__SC_XOPEN_SHM: anon_2 = 94;
pub const anon_2__SC_2_CHAR_TERM: anon_2 = 95;
pub const anon_2__SC_2_C_VERSION: anon_2 = 96;
pub const anon_2__SC_2_UPE: anon_2 = 97;
pub const anon_2__SC_XOPEN_XPG2: anon_2 = 98;
pub const anon_2__SC_XOPEN_XPG3: anon_2 = 99;
pub const anon_2__SC_XOPEN_XPG4: anon_2 = 100;
pub const anon_2__SC_CHAR_BIT: anon_2 = 101;
pub const anon_2__SC_CHAR_MAX: anon_2 = 102;
pub const anon_2__SC_CHAR_MIN: anon_2 = 103;
pub const anon_2__SC_INT_MAX: anon_2 = 104;
pub const anon_2__SC_INT_MIN: anon_2 = 105;
pub const anon_2__SC_LONG_BIT: anon_2 = 106;
pub const anon_2__SC_WORD_BIT: anon_2 = 107;
pub const anon_2__SC_MB_LEN_MAX: anon_2 = 108;
pub const anon_2__SC_NZERO: anon_2 = 109;
pub const anon_2__SC_SSIZE_MAX: anon_2 = 110;
pub const anon_2__SC_SCHAR_MAX: anon_2 = 111;
pub const anon_2__SC_SCHAR_MIN: anon_2 = 112;
pub const anon_2__SC_SHRT_MAX: anon_2 = 113;
pub const anon_2__SC_SHRT_MIN: anon_2 = 114;
pub const anon_2__SC_UCHAR_MAX: anon_2 = 115;
pub const anon_2__SC_UINT_MAX: anon_2 = 116;
pub const anon_2__SC_ULONG_MAX: anon_2 = 117;
pub const anon_2__SC_USHRT_MAX: anon_2 = 118;
pub const anon_2__SC_NL_ARGMAX: anon_2 = 119;
pub const anon_2__SC_NL_LANGMAX: anon_2 = 120;
pub const anon_2__SC_NL_MSGMAX: anon_2 = 121;
pub const anon_2__SC_NL_NMAX: anon_2 = 122;
pub const anon_2__SC_NL_SETMAX: anon_2 = 123;
pub const anon_2__SC_NL_TEXTMAX: anon_2 = 124;
pub const anon_2__SC_XBS5_ILP32_OFF32: anon_2 = 125;
pub const anon_2__SC_XBS5_ILP32_OFFBIG: anon_2 = 126;
pub const anon_2__SC_XBS5_LP64_OFF64: anon_2 = 127;
pub const anon_2__SC_XBS5_LPBIG_OFFBIG: anon_2 = 128;
pub const anon_2__SC_XOPEN_LEGACY: anon_2 = 129;
pub const anon_2__SC_XOPEN_REALTIME: anon_2 = 130;
pub const anon_2__SC_XOPEN_REALTIME_THREADS: anon_2 = 131;
pub const anon_2__SC_ADVISORY_INFO: anon_2 = 132;
pub const anon_2__SC_BARRIERS: anon_2 = 133;
pub const anon_2__SC_BASE: anon_2 = 134;
pub const anon_2__SC_C_LANG_SUPPORT: anon_2 = 135;
pub const anon_2__SC_C_LANG_SUPPORT_R: anon_2 = 136;
pub const anon_2__SC_CLOCK_SELECTION: anon_2 = 137;
pub const anon_2__SC_CPUTIME: anon_2 = 138;
pub const anon_2__SC_THREAD_CPUTIME: anon_2 = 139;
pub const anon_2__SC_DEVICE_IO: anon_2 = 140;
pub const anon_2__SC_DEVICE_SPECIFIC: anon_2 = 141;
pub const anon_2__SC_DEVICE_SPECIFIC_R: anon_2 = 142;
pub const anon_2__SC_FD_MGMT: anon_2 = 143;
pub const anon_2__SC_FIFO: anon_2 = 144;
pub const anon_2__SC_PIPE: anon_2 = 145;
pub const anon_2__SC_FILE_ATTRIBUTES: anon_2 = 146;
pub const anon_2__SC_FILE_LOCKING: anon_2 = 147;
pub const anon_2__SC_FILE_SYSTEM: anon_2 = 148;
pub const anon_2__SC_MONOTONIC_CLOCK: anon_2 = 149;
pub const anon_2__SC_MULTI_PROCESS: anon_2 = 150;
pub const anon_2__SC_SINGLE_PROCESS: anon_2 = 151;
pub const anon_2__SC_NETWORKING: anon_2 = 152;
pub const anon_2__SC_READER_WRITER_LOCKS: anon_2 = 153;
pub const anon_2__SC_SPIN_LOCKS: anon_2 = 154;
pub const anon_2__SC_REGEXP: anon_2 = 155;
pub const anon_2__SC_REGEX_VERSION: anon_2 = 156;
pub const anon_2__SC_SHELL: anon_2 = 157;
pub const anon_2__SC_SIGNALS: anon_2 = 158;
pub const anon_2__SC_SPAWN: anon_2 = 159;
pub const anon_2__SC_SPORADIC_SERVER: anon_2 = 160;
pub const anon_2__SC_THREAD_SPORADIC_SERVER: anon_2 = 161;
pub const anon_2__SC_SYSTEM_DATABASE: anon_2 = 162;
pub const anon_2__SC_SYSTEM_DATABASE_R: anon_2 = 163;
pub const anon_2__SC_TIMEOUTS: anon_2 = 164;
pub const anon_2__SC_TYPED_MEMORY_OBJECTS: anon_2 = 165;
pub const anon_2__SC_USER_GROUPS: anon_2 = 166;
pub const anon_2__SC_USER_GROUPS_R: anon_2 = 167;
pub const anon_2__SC_2_PBS: anon_2 = 168;
pub const anon_2__SC_2_PBS_ACCOUNTING: anon_2 = 169;
pub const anon_2__SC_2_PBS_LOCATE: anon_2 = 170;
pub const anon_2__SC_2_PBS_MESSAGE: anon_2 = 171;
pub const anon_2__SC_2_PBS_TRACK: anon_2 = 172;
pub const anon_2__SC_SYMLOOP_MAX: anon_2 = 173;
pub const anon_2__SC_STREAMS: anon_2 = 174;
pub const anon_2__SC_2_PBS_CHECKPOINT: anon_2 = 175;
pub const anon_2__SC_V6_ILP32_OFF32: anon_2 = 176;
pub const anon_2__SC_V6_ILP32_OFFBIG: anon_2 = 177;
pub const anon_2__SC_V6_LP64_OFF64: anon_2 = 178;
pub const anon_2__SC_V6_LPBIG_OFFBIG: anon_2 = 179;
pub const anon_2__SC_HOST_NAME_MAX: anon_2 = 180;
pub const anon_2__SC_TRACE: anon_2 = 181;
pub const anon_2__SC_TRACE_EVENT_FILTER: anon_2 = 182;
pub const anon_2__SC_TRACE_INHERIT: anon_2 = 183;
pub const anon_2__SC_TRACE_LOG: anon_2 = 184;
pub const anon_2__SC_LEVEL1_ICACHE_SIZE: anon_2 = 185;
pub const anon_2__SC_LEVEL1_ICACHE_ASSOC: anon_2 = 186;
pub const anon_2__SC_LEVEL1_ICACHE_LINESIZE: anon_2 = 187;
pub const anon_2__SC_LEVEL1_DCACHE_SIZE: anon_2 = 188;
pub const anon_2__SC_LEVEL1_DCACHE_ASSOC: anon_2 = 189;
pub const anon_2__SC_LEVEL1_DCACHE_LINESIZE: anon_2 = 190;
pub const anon_2__SC_LEVEL2_CACHE_SIZE: anon_2 = 191;
pub const anon_2__SC_LEVEL2_CACHE_ASSOC: anon_2 = 192;
pub const anon_2__SC_LEVEL2_CACHE_LINESIZE: anon_2 = 193;
pub const anon_2__SC_LEVEL3_CACHE_SIZE: anon_2 = 194;
pub const anon_2__SC_LEVEL3_CACHE_ASSOC: anon_2 = 195;
pub const anon_2__SC_LEVEL3_CACHE_LINESIZE: anon_2 = 196;
pub const anon_2__SC_LEVEL4_CACHE_SIZE: anon_2 = 197;
pub const anon_2__SC_LEVEL4_CACHE_ASSOC: anon_2 = 198;
pub const anon_2__SC_LEVEL4_CACHE_LINESIZE: anon_2 = 199;
pub const anon_2__SC_IPV6: anon_2 = 235;
pub const anon_2__SC_RAW_SOCKETS: anon_2 = 236;
pub const anon_2__SC_V7_ILP32_OFF32: anon_2 = 237;
pub const anon_2__SC_V7_ILP32_OFFBIG: anon_2 = 238;
pub const anon_2__SC_V7_LP64_OFF64: anon_2 = 239;
pub const anon_2__SC_V7_LPBIG_OFFBIG: anon_2 = 240;
pub const anon_2__SC_SS_REPL_MAX: anon_2 = 241;
pub const anon_2__SC_TRACE_EVENT_NAME_MAX: anon_2 = 242;
pub const anon_2__SC_TRACE_NAME_MAX: anon_2 = 243;
pub const anon_2__SC_TRACE_SYS_MAX: anon_2 = 244;
pub const anon_2__SC_TRACE_USER_EVENT_MAX: anon_2 = 245;
pub const anon_2__SC_XOPEN_STREAMS: anon_2 = 246;
pub const anon_2__SC_THREAD_ROBUST_PRIO_INHERIT: anon_2 = 247;
pub const anon_2__SC_THREAD_ROBUST_PRIO_PROTECT: anon_2 = 248;
pub const anon_2__SC_MINSIGSTKSZ: anon_2 = 249;
pub const anon_2__SC_SIGSTKSZ: anon_2 = 250;
pub fn test_close_3() {
    let fds: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..2).map(|_| 0_i32).collect::<Box<[i32]>>()));
    {
        &(4usize);
        {
            if (((({ pipe_4((fds.as_pointer() as Ptr<i32>)) }) == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"pipe(fds) == 0"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        11_u32,
                        Ptr::<i8>::from_string_literal(b"void test_close(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ close_6((*fds.borrow())[(0) as usize]) }) == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"close(fds[0]) == 0"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        12_u32,
                        Ptr::<i8>::from_string_literal(b"void test_close(void)"),
                    )
                });
            }
        }
    };
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new((0..1).map(|_| 0_i8).collect::<Box<[i8]>>()));
    {
        &(4usize);
        {
            if (((({
                read_7(
                    (*fds.borrow())[(0) as usize],
                    ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
                    1_usize,
                )
            }) == (-1_i32 as isize)) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"read(fds[0], buf, 1) == -1"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        14_u32,
                        Ptr::<i8>::from_string_literal(b"void test_close(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ close_6((*fds.borrow())[(1) as usize]) }) == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"close(fds[1]) == 0"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        15_u32,
                        Ptr::<i8>::from_string_literal(b"void test_close(void)"),
                    )
                });
            }
        }
    };
}
pub fn test_lseek_8() {
    let mut path: Ptr<i8> = Ptr::<i8>::from_string_literal(b"cpp2rust_lseek_test.tmp");
    let mut fp: Ptr<_IO_FILE> =
        ({ fopen_9((path).clone(), Ptr::<i8>::from_string_literal(b"wb")) });
    {
        &(4usize);
        {
            if (((!((fp).is_null())) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"fp != NULL"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        21_u32,
                        Ptr::<i8>::from_string_literal(b"void test_lseek(void)"),
                    )
                });
            }
        }
    };
    ({ fputs_10(Ptr::<i8>::from_string_literal(b"hello world"), (fp).clone()) });
    {
        &(4usize);
        {
            if (((({ fclose_11((fp).clone()) }) == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"fclose(fp) == 0"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        23_u32,
                        Ptr::<i8>::from_string_literal(b"void test_lseek(void)"),
                    )
                });
            }
        }
    };
    fp = ({ fopen_9((path).clone(), Ptr::<i8>::from_string_literal(b"rb")) });
    {
        &(4usize);
        {
            if (((!((fp).is_null())) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"fp != NULL"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        25_u32,
                        Ptr::<i8>::from_string_literal(b"void test_lseek(void)"),
                    )
                });
            }
        }
    };
    let mut fd: i32 = ({ fileno_12((fp).clone()) });
    {
        &(4usize);
        {
            if (((({ lseek_13(fd, 0_i64, 2) }) == 11_i64) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"lseek(fd, 0, SEEK_END) == 11"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        27_u32,
                        Ptr::<i8>::from_string_literal(b"void test_lseek(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ lseek_13(fd, 6_i64, 0) }) == 6_i64) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"lseek(fd, 6, SEEK_SET) == 6"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        28_u32,
                        Ptr::<i8>::from_string_literal(b"void test_lseek(void)"),
                    )
                });
            }
        }
    };
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
    ])));
    {
        &(4usize);
        {
            if (((({
                read_7(
                    fd,
                    ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
                    5_usize,
                )
            }) == 5_isize) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"read(fd, buf, 5) == 5"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        30_u32,
                        Ptr::<i8>::from_string_literal(b"void test_lseek(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                memcmp_14(
                    ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
                    Ptr::<i8>::from_string_literal(b"world").to_any(),
                    5_usize,
                )
            }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"memcmp(buf, \"world\", 5) == 0"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        31_u32,
                        Ptr::<i8>::from_string_literal(b"void test_lseek(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ fclose_11((fp).clone()) }) == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"fclose(fp) == 0"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        32_u32,
                        Ptr::<i8>::from_string_literal(b"void test_lseek(void)"),
                    )
                });
            }
        }
    };
    ({ unlink_15((path).clone()) });
}
pub fn test_read_16() {
    let mut path: Ptr<i8> = Ptr::<i8>::from_string_literal(b"cpp2rust_read_test.tmp");
    let mut fp: Ptr<_IO_FILE> =
        ({ fopen_9((path).clone(), Ptr::<i8>::from_string_literal(b"wb")) });
    {
        &(4usize);
        {
            if (((!((fp).is_null())) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"fp != NULL"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        39_u32,
                        Ptr::<i8>::from_string_literal(b"void test_read(void)"),
                    )
                });
            }
        }
    };
    ({ fputs_10(Ptr::<i8>::from_string_literal(b"hello world"), (fp).clone()) });
    {
        &(4usize);
        {
            if (((({ fclose_11((fp).clone()) }) == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"fclose(fp) == 0"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        41_u32,
                        Ptr::<i8>::from_string_literal(b"void test_read(void)"),
                    )
                });
            }
        }
    };
    fp = ({ fopen_9((path).clone(), Ptr::<i8>::from_string_literal(b"rb")) });
    {
        &(4usize);
        {
            if (((!((fp).is_null())) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"fp != NULL"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        43_u32,
                        Ptr::<i8>::from_string_literal(b"void test_read(void)"),
                    )
                });
            }
        }
    };
    let mut fd: i32 = ({ fileno_12((fp).clone()) });
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
        0_i8,
    ])));
    {
        &(4usize);
        {
            if (((({
                read_7(
                    fd,
                    ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
                    16_usize,
                )
            }) == 11_isize) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"read(fd, buf, 16) == 11"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        46_u32,
                        Ptr::<i8>::from_string_literal(b"void test_read(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                memcmp_14(
                    ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
                    Ptr::<i8>::from_string_literal(b"hello world").to_any(),
                    11_usize,
                )
            }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"memcmp(buf, \"hello world\", 11) == 0"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        47_u32,
                        Ptr::<i8>::from_string_literal(b"void test_read(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ fclose_11((fp).clone()) }) == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"fclose(fp) == 0"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        48_u32,
                        Ptr::<i8>::from_string_literal(b"void test_read(void)"),
                    )
                });
            }
        }
    };
    ({ unlink_15((path).clone()) });
}
pub fn test_unlink_17() {
    let mut path: Ptr<i8> = Ptr::<i8>::from_string_literal(b"cpp2rust_unlink_test.tmp");
    let mut fp: Ptr<_IO_FILE> =
        ({ fopen_9((path).clone(), Ptr::<i8>::from_string_literal(b"wb")) });
    {
        &(4usize);
        {
            if (((!((fp).is_null())) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"fp != NULL"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        55_u32,
                        Ptr::<i8>::from_string_literal(b"void test_unlink(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ fclose_11((fp).clone()) }) == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"fclose(fp) == 0"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        56_u32,
                        Ptr::<i8>::from_string_literal(b"void test_unlink(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ unlink_15((path).clone()) }) == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"unlink(path) == 0"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        57_u32,
                        Ptr::<i8>::from_string_literal(b"void test_unlink(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ unlink_15((path).clone()) }) == -1_i32) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"unlink(path) == -1"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        58_u32,
                        Ptr::<i8>::from_string_literal(b"void test_unlink(void)"),
                    )
                });
            }
        }
    };
}
pub fn test_pipe_18() {
    let fds: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..2).map(|_| 0_i32).collect::<Box<[i32]>>()));
    {
        &(4usize);
        {
            if (((({ pipe_4((fds.as_pointer() as Ptr<i32>)) }) == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"pipe(fds) == 0"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        63_u32,
                        Ptr::<i8>::from_string_literal(b"void test_pipe(void)"),
                    )
                });
            }
        }
    };
    let mut msg: Ptr<i8> = Ptr::<i8>::from_string_literal(b"world");
    {
        &(4usize);
        {
            if (((({ write_19((*fds.borrow())[(1) as usize], (msg).to_any(), 5_usize) }) == 5_isize)
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"write(fds[1], msg, 5) == 5"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        65_u32,
                        Ptr::<i8>::from_string_literal(b"void test_pipe(void)"),
                    )
                });
            }
        }
    };
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
    ])));
    {
        &(4usize);
        {
            if (((({
                read_7(
                    (*fds.borrow())[(0) as usize],
                    ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
                    8_usize,
                )
            }) == 5_isize) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"read(fds[0], buf, 8) == 5"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        67_u32,
                        Ptr::<i8>::from_string_literal(b"void test_pipe(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                memcmp_14(
                    ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
                    (msg).to_any(),
                    5_usize,
                )
            }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"memcmp(buf, msg, 5) == 0"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        68_u32,
                        Ptr::<i8>::from_string_literal(b"void test_pipe(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ close_6((*fds.borrow())[(1) as usize]) }) == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"close(fds[1]) == 0"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        69_u32,
                        Ptr::<i8>::from_string_literal(b"void test_pipe(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                read_7(
                    (*fds.borrow())[(0) as usize],
                    ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
                    8_usize,
                )
            }) == 0_isize) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"read(fds[0], buf, 8) == 0"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        70_u32,
                        Ptr::<i8>::from_string_literal(b"void test_pipe(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ close_6((*fds.borrow())[(0) as usize]) }) == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"close(fds[0]) == 0"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        71_u32,
                        Ptr::<i8>::from_string_literal(b"void test_pipe(void)"),
                    )
                });
            }
        }
    };
}
pub fn test_ftruncate_20() {
    let mut path: Ptr<i8> = Ptr::<i8>::from_string_literal(b"cpp2rust_ftruncate_test.tmp");
    let mut fp: Ptr<_IO_FILE> =
        ({ fopen_9((path).clone(), Ptr::<i8>::from_string_literal(b"wb")) });
    {
        &(4usize);
        {
            if (((!((fp).is_null())) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"fp != NULL"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        77_u32,
                        Ptr::<i8>::from_string_literal(b"void test_ftruncate(void)"),
                    )
                });
            }
        }
    };
    ({ fputs_10(Ptr::<i8>::from_string_literal(b"hello world"), (fp).clone()) });
    ({ fflush_21((fp).clone()) });
    let mut fd: i32 = ({ fileno_12((fp).clone()) });
    {
        &(4usize);
        {
            if (((({ ftruncate_22(fd, 5_i64) }) == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"ftruncate(fd, 5) == 0"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        81_u32,
                        Ptr::<i8>::from_string_literal(b"void test_ftruncate(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ fclose_11((fp).clone()) }) == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"fclose(fp) == 0"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        82_u32,
                        Ptr::<i8>::from_string_literal(b"void test_ftruncate(void)"),
                    )
                });
            }
        }
    };
    fp = ({ fopen_9((path).clone(), Ptr::<i8>::from_string_literal(b"rb")) });
    {
        &(4usize);
        {
            if (((!((fp).is_null())) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"fp != NULL"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        84_u32,
                        Ptr::<i8>::from_string_literal(b"void test_ftruncate(void)"),
                    )
                });
            }
        }
    };
    fd = ({ fileno_12((fp).clone()) });
    {
        &(4usize);
        {
            if (((({ lseek_13(fd, 0_i64, 2) }) == 5_i64) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"lseek(fd, 0, SEEK_END) == 5"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        86_u32,
                        Ptr::<i8>::from_string_literal(b"void test_ftruncate(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ fclose_11((fp).clone()) }) == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"fclose(fp) == 0"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        87_u32,
                        Ptr::<i8>::from_string_literal(b"void test_ftruncate(void)"),
                    )
                });
            }
        }
    };
    ({ unlink_15((path).clone()) });
}
pub fn test_open_23() {
    let mut fd: i32 = ({
        open_24(
            Ptr::<i8>::from_string_literal(b"/dev/null"),
            0,
            &[(420).into()],
        )
    });
    {
        &(4usize);
        {
            if (((fd >= -1_i32) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"fd >= -1"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        93_u32,
                        Ptr::<i8>::from_string_literal(b"void test_open(void)"),
                    )
                });
            }
        }
    };
    if (((fd >= 0) as i32) != 0) {
        ({ close_6(fd) });
    }
    fd = ({ open_24(Ptr::<i8>::from_string_literal(b"/dev/null"), 0, &[]) });
    {
        &(4usize);
        {
            if (((fd >= -1_i32) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"fd >= -1"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        98_u32,
                        Ptr::<i8>::from_string_literal(b"void test_open(void)"),
                    )
                });
            }
        }
    };
    if (((fd >= 0) as i32) != 0) {
        ({ close_6(fd) });
    }
}
pub fn test_fcntl_25() {
    {
        &(4usize);
        {
            if (((({ fcntl_26(0, 1, &[]) }) >= -1_i32) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"fcntl(0, 1) >= -1"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        105_u32,
                        Ptr::<i8>::from_string_literal(b"void test_fcntl(void)"),
                    )
                });
            }
        }
    };
    let mut duped: i32 = ({ fcntl_26(0, 0, &[(100).into()]) });
    {
        &(4usize);
        {
            if (((duped >= -1_i32) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"duped >= -1"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        107_u32,
                        Ptr::<i8>::from_string_literal(b"void test_fcntl(void)"),
                    )
                });
            }
        }
    };
    if (((duped >= 0) as i32) != 0) {
        ({ close_6(duped) });
    }
}
pub fn test_ioctl_27() {
    let arg: Value<i32> = Rc::new(RefCell::new(0));
    {
        &(4usize);
        {
            if (((({ ioctl_28(0, 0_u64, &[(arg.as_pointer()).into()]) }) >= -1_i32) as i32) != 0) {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"ioctl(0, 0, &arg) >= -1"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        115_u32,
                        Ptr::<i8>::from_string_literal(b"void test_ioctl(void)"),
                    )
                });
            }
        }
    };
}
pub fn test_isatty_29() {
    println!("{}", ({ isatty_30(0,) }));
}
pub fn test_geteuid_31() {
    println!("{}", ({ geteuid_32() }));
}
pub fn test_gethostname_33() {
    let name: Value<Box<[i8]>> =
        Rc::new(RefCell::new((0..256).map(|_| 0_i8).collect::<Box<[i8]>>()));
    {
        &(4usize);
        {
            if (((({
                let ___name: Ptr<i8> = (name.as_pointer() as Ptr<i8>);
                let ___len: usize = 256usize;
                gethostname_34(___name, ___len)
            }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_5(
                        Ptr::<i8>::from_string_literal(b"gethostname(name, sizeof(name)) == 0"),
                        Ptr::<i8>::from_string_literal(b"unistd.c"),
                        124_u32,
                        Ptr::<i8>::from_string_literal(b"void test_gethostname(void)"),
                    )
                });
            }
        }
    };
    println!("{}", (name.as_pointer() as Ptr::<i8>));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_close_3() });
    ({ test_lseek_8() });
    ({ test_read_16() });
    ({ test_unlink_17() });
    ({ test_pipe_18() });
    ({ test_ftruncate_20() });
    ({ test_open_23() });
    ({ test_fcntl_25() });
    ({ test_ioctl_27() });
    ({ test_isatty_29() });
    ({ test_geteuid_31() });
    ({ test_gethostname_33() });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct _IO_codecvt;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct _IO_marker;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct _IO_wide_data;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct __locale_data;
pub fn pipe_4(mut __pipedes: Ptr<i32>) -> i32 {
    unimplemented!()
}
pub fn __assert_fail_5(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn close_6(mut __fd: i32) -> i32 {
    unimplemented!()
}
pub fn read_7(mut __fd: i32, mut __buf: AnyPtr, mut __nbytes: usize) -> isize {
    unimplemented!()
}
pub fn fopen_9(mut __filename: Ptr<i8>, mut __modes: Ptr<i8>) -> Ptr<_IO_FILE> {
    unimplemented!()
}
pub fn fputs_10(mut __s: Ptr<i8>, mut __stream: Ptr<_IO_FILE>) -> i32 {
    unimplemented!()
}
pub fn fclose_11(mut __stream: Ptr<_IO_FILE>) -> i32 {
    unimplemented!()
}
pub fn fileno_12(mut __stream: Ptr<_IO_FILE>) -> i32 {
    unimplemented!()
}
pub fn lseek_13(mut __fd: i32, mut __offset: i64, mut __whence: i32) -> i64 {
    unimplemented!()
}
pub fn memcmp_14(mut __s1: AnyPtr, mut __s2: AnyPtr, mut __n: usize) -> i32 {
    unimplemented!()
}
pub fn unlink_15(mut __name: Ptr<i8>) -> i32 {
    unimplemented!()
}
pub fn write_19(mut __fd: i32, mut __buf: AnyPtr, mut __n: usize) -> isize {
    unimplemented!()
}
pub fn fflush_21(mut __stream: Ptr<_IO_FILE>) -> i32 {
    unimplemented!()
}
pub fn ftruncate_22(mut __fd: i32, mut __length: i64) -> i32 {
    unimplemented!()
}
pub fn open_24(mut __file: Ptr<i8>, mut __oflag: i32, __args: &[VaArg]) -> i32 {
    unimplemented!()
}
pub fn fcntl_26(mut __fd: i32, mut __cmd: i32, __args: &[VaArg]) -> i32 {
    unimplemented!()
}
pub fn ioctl_28(mut __fd: i32, mut __request: u64, __args: &[VaArg]) -> i32 {
    unimplemented!()
}
pub fn isatty_30(mut __fd: i32) -> i32 {
    unimplemented!()
}
pub fn geteuid_32() -> u32 {
    unimplemented!()
}
pub fn gethostname_34(mut __name: Ptr<i8>, mut __len: usize) -> i32 {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
