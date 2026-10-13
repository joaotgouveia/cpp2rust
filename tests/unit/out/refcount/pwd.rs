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
#[byte_size(48)]
pub struct passwd {
    #[offset(0)]
    #[byte_size(8)]
    pub pw_name: Ptr<i8>,
    #[offset(8)]
    #[byte_size(8)]
    pub pw_passwd: Ptr<i8>,
    #[offset(16)]
    pub pw_uid: u32,
    #[offset(20)]
    pub pw_gid: u32,
    #[offset(24)]
    #[byte_size(8)]
    pub pw_gecos: Ptr<i8>,
    #[offset(32)]
    #[byte_size(8)]
    pub pw_dir: Ptr<i8>,
    #[offset(40)]
    #[byte_size(8)]
    pub pw_shell: Ptr<i8>,
}
impl Clone for passwd {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for passwd {
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
pub fn test_getpwuid_3() {
    let mut pw: Ptr<passwd> = ({ getpwuid_4(({ geteuid_5() })) });
    {
        &(4usize);
        {
            if (((!((pw).is_null())) as i32) != 0) {
            } else {
                ({
                    __assert_fail_6(
                        Ptr::<i8>::from_string_literal(b"pw != NULL"),
                        Ptr::<i8>::from_string_literal(b"pwd.c"),
                        11_u32,
                        Ptr::<i8>::from_string_literal(b"void test_getpwuid(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((({ pw.with(|__s| __s.pw_uid) } == { ({ geteuid_5() }) }) as i32) != 0) {
            } else {
                ({
                    __assert_fail_6(
                        Ptr::<i8>::from_string_literal(b"pw->pw_uid == geteuid()"),
                        Ptr::<i8>::from_string_literal(b"pwd.c"),
                        12_u32,
                        Ptr::<i8>::from_string_literal(b"void test_getpwuid(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ strlen_7(pw.with(|__s| __s.pw_name.clone())) }) > 0_usize) as i32) != 0) {
            } else {
                ({
                    __assert_fail_6(
                        Ptr::<i8>::from_string_literal(b"strlen(pw->pw_name) > 0"),
                        Ptr::<i8>::from_string_literal(b"pwd.c"),
                        13_u32,
                        Ptr::<i8>::from_string_literal(b"void test_getpwuid(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((!((pw.with(|__s| __s.pw_dir.clone())).is_null())) as i32) != 0) {
            } else {
                ({
                    __assert_fail_6(
                        Ptr::<i8>::from_string_literal(b"pw->pw_dir != NULL"),
                        Ptr::<i8>::from_string_literal(b"pwd.c"),
                        14_u32,
                        Ptr::<i8>::from_string_literal(b"void test_getpwuid(void)"),
                    )
                });
            }
        }
    };
    println!("{}", pw.with(|__s| __s.pw_name.clone()));
}
pub fn test_getpwuid_missing_8() {
    ({ __errno_location_9() }).write(0);
    let mut pw: Ptr<passwd> = ({ getpwuid_4(2147483646_u32) });
    {
        &(4usize);
        {
            if ((((pw).is_null()) as i32) != 0) {
            } else {
                ({
                    __assert_fail_6(
                        Ptr::<i8>::from_string_literal(b"pw == NULL"),
                        Ptr::<i8>::from_string_literal(b"pwd.c"),
                        21_u32,
                        Ptr::<i8>::from_string_literal(b"void test_getpwuid_missing(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((((({ __errno_location_9() }).read()) == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_6(
                        Ptr::<i8>::from_string_literal(b"errno == 0"),
                        Ptr::<i8>::from_string_literal(b"pwd.c"),
                        22_u32,
                        Ptr::<i8>::from_string_literal(b"void test_getpwuid_missing(void)"),
                    )
                });
            }
        }
    };
}
pub fn test_getpwuid_r_10() {
    let pw: Value<passwd> = <Value<passwd>>::default();
    let buf: Value<Box<[i8]>> =
        Rc::new(RefCell::new((0..4096).map(|_| 0_i8).collect::<Box<[i8]>>()));
    let result: Value<Ptr<passwd>> = Rc::new(RefCell::new(Ptr::<passwd>::null()));
    {
        &(4usize);
        {
            if (((({
                let ___buffer: Ptr<i8> = (buf.as_pointer() as Ptr<i8>);
                let ___buflen: usize = 4096usize;
                getpwuid_r_11(
                    ({ geteuid_5() }),
                    (pw.as_pointer()),
                    ___buffer,
                    ___buflen,
                    (result.as_pointer()),
                )
            }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_6(
                        Ptr::<i8>::from_string_literal(
                            b"getpwuid_r(geteuid(), &pw, buf, sizeof(buf), &result) == 0",
                        ),
                        Ptr::<i8>::from_string_literal(b"pwd.c"),
                        29_u32,
                        Ptr::<i8>::from_string_literal(b"void test_getpwuid_r(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((({ (*result.borrow()).clone() } == { (pw.as_pointer()) }) as i32) != 0) {
            } else {
                ({
                    __assert_fail_6(
                        Ptr::<i8>::from_string_literal(b"result == &pw"),
                        Ptr::<i8>::from_string_literal(b"pwd.c"),
                        30_u32,
                        Ptr::<i8>::from_string_literal(b"void test_getpwuid_r(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((({ (*pw.borrow()).pw_uid } == ({ geteuid_5() })) as i32) != 0) {
            } else {
                ({
                    __assert_fail_6(
                        Ptr::<i8>::from_string_literal(b"pw.pw_uid == geteuid()"),
                        Ptr::<i8>::from_string_literal(b"pwd.c"),
                        31_u32,
                        Ptr::<i8>::from_string_literal(b"void test_getpwuid_r(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ strlen_7({ (*pw.borrow()).pw_name.clone() }) }) > 0_usize) as i32) != 0) {
            } else {
                ({
                    __assert_fail_6(
                        Ptr::<i8>::from_string_literal(b"strlen(pw.pw_name) > 0"),
                        Ptr::<i8>::from_string_literal(b"pwd.c"),
                        32_u32,
                        Ptr::<i8>::from_string_literal(b"void test_getpwuid_r(void)"),
                    )
                });
            }
        }
    };
    let mut pw2: Ptr<passwd> = ({ getpwuid_4(({ geteuid_5() })) });
    {
        &(4usize);
        {
            if (((!((pw2).is_null())) as i32) != 0) {
            } else {
                ({
                    __assert_fail_6(
                        Ptr::<i8>::from_string_literal(b"pw2 != NULL"),
                        Ptr::<i8>::from_string_literal(b"pwd.c"),
                        34_u32,
                        Ptr::<i8>::from_string_literal(b"void test_getpwuid_r(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({
                let ___s1: Ptr<i8> = { (*pw.borrow()).pw_name.clone() };
                let ___s2: Ptr<i8> = pw2.with(|__s| __s.pw_name.clone());
                strcmp_12(___s1, ___s2)
            }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_6(
                        Ptr::<i8>::from_string_literal(b"strcmp(pw.pw_name, pw2->pw_name) == 0"),
                        Ptr::<i8>::from_string_literal(b"pwd.c"),
                        35_u32,
                        Ptr::<i8>::from_string_literal(b"void test_getpwuid_r(void)"),
                    )
                });
            }
        }
    };
    println!("{}", { (*pw.borrow()).pw_name.clone() });
}
pub fn test_getpwuid_r_erange_13() {
    let pw: Value<passwd> = <Value<passwd>>::default();
    let tiny: Value<Box<[i8]>> = Rc::new(RefCell::new((0..1).map(|_| 0_i8).collect::<Box<[i8]>>()));
    let result: Value<Ptr<passwd>> = Rc::new(RefCell::new(Ptr::<passwd>::null()));
    {
        &(4usize);
        {
            if (((({
                let ___buffer: Ptr<i8> = (tiny.as_pointer() as Ptr<i8>);
                let ___buflen: usize = 1usize;
                getpwuid_r_11(
                    ({ geteuid_5() }),
                    (pw.as_pointer()),
                    ___buffer,
                    ___buflen,
                    (result.as_pointer()),
                )
            }) == 34) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_6(
                        Ptr::<i8>::from_string_literal(
                            b"getpwuid_r(geteuid(), &pw, tiny, sizeof(tiny), &result) == ERANGE",
                        ),
                        Ptr::<i8>::from_string_literal(b"pwd.c"),
                        43_u32,
                        Ptr::<i8>::from_string_literal(b"void test_getpwuid_r_erange(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((((*result.borrow()).is_null()) as i32) != 0) {
            } else {
                ({
                    __assert_fail_6(
                        Ptr::<i8>::from_string_literal(b"result == NULL"),
                        Ptr::<i8>::from_string_literal(b"pwd.c"),
                        44_u32,
                        Ptr::<i8>::from_string_literal(b"void test_getpwuid_r_erange(void)"),
                    )
                });
            }
        }
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_getpwuid_3() });
    ({ test_getpwuid_missing_8() });
    ({ test_getpwuid_r_10() });
    ({ test_getpwuid_r_erange_13() });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct _IO_wide_data;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct _IO_codecvt;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct _IO_marker;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct __locale_data;
pub fn getpwuid_4(mut __uid: u32) -> Ptr<passwd> {
    unimplemented!()
}
pub fn geteuid_5() -> u32 {
    unimplemented!()
}
pub fn __assert_fail_6(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn strlen_7(mut __s: Ptr<i8>) -> usize {
    unimplemented!()
}
pub fn __errno_location_9() -> Ptr<i32> {
    unimplemented!()
}
pub fn getpwuid_r_11(
    mut __uid: u32,
    mut __resultbuf: Ptr<passwd>,
    mut __buffer: Ptr<i8>,
    mut __buflen: usize,
    mut __result: Ptr<Ptr<passwd>>,
) -> i32 {
    unimplemented!()
}
pub fn strcmp_12(mut __s1: Ptr<i8>, mut __s2: Ptr<i8>) -> i32 {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
