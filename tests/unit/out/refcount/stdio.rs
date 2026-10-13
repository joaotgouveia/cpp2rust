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
thread_local!(
    pub static stdin_1: Value<Ptr<_IO_FILE>> = unimplemented!();
);
thread_local!(
    pub static stdout_2: Value<Ptr<_IO_FILE>> = unimplemented!();
);
thread_local!(
    pub static stderr_3: Value<Ptr<_IO_FILE>> = unimplemented!();
);
pub type anon_4 = u32;
pub const anon_4__SC_ARG_MAX: anon_4 = 0;
pub const anon_4__SC_CHILD_MAX: anon_4 = 1;
pub const anon_4__SC_CLK_TCK: anon_4 = 2;
pub const anon_4__SC_NGROUPS_MAX: anon_4 = 3;
pub const anon_4__SC_OPEN_MAX: anon_4 = 4;
pub const anon_4__SC_STREAM_MAX: anon_4 = 5;
pub const anon_4__SC_TZNAME_MAX: anon_4 = 6;
pub const anon_4__SC_JOB_CONTROL: anon_4 = 7;
pub const anon_4__SC_SAVED_IDS: anon_4 = 8;
pub const anon_4__SC_REALTIME_SIGNALS: anon_4 = 9;
pub const anon_4__SC_PRIORITY_SCHEDULING: anon_4 = 10;
pub const anon_4__SC_TIMERS: anon_4 = 11;
pub const anon_4__SC_ASYNCHRONOUS_IO: anon_4 = 12;
pub const anon_4__SC_PRIORITIZED_IO: anon_4 = 13;
pub const anon_4__SC_SYNCHRONIZED_IO: anon_4 = 14;
pub const anon_4__SC_FSYNC: anon_4 = 15;
pub const anon_4__SC_MAPPED_FILES: anon_4 = 16;
pub const anon_4__SC_MEMLOCK: anon_4 = 17;
pub const anon_4__SC_MEMLOCK_RANGE: anon_4 = 18;
pub const anon_4__SC_MEMORY_PROTECTION: anon_4 = 19;
pub const anon_4__SC_MESSAGE_PASSING: anon_4 = 20;
pub const anon_4__SC_SEMAPHORES: anon_4 = 21;
pub const anon_4__SC_SHARED_MEMORY_OBJECTS: anon_4 = 22;
pub const anon_4__SC_AIO_LISTIO_MAX: anon_4 = 23;
pub const anon_4__SC_AIO_MAX: anon_4 = 24;
pub const anon_4__SC_AIO_PRIO_DELTA_MAX: anon_4 = 25;
pub const anon_4__SC_DELAYTIMER_MAX: anon_4 = 26;
pub const anon_4__SC_MQ_OPEN_MAX: anon_4 = 27;
pub const anon_4__SC_MQ_PRIO_MAX: anon_4 = 28;
pub const anon_4__SC_VERSION: anon_4 = 29;
pub const anon_4__SC_PAGESIZE: anon_4 = 30;
pub const anon_4__SC_RTSIG_MAX: anon_4 = 31;
pub const anon_4__SC_SEM_NSEMS_MAX: anon_4 = 32;
pub const anon_4__SC_SEM_VALUE_MAX: anon_4 = 33;
pub const anon_4__SC_SIGQUEUE_MAX: anon_4 = 34;
pub const anon_4__SC_TIMER_MAX: anon_4 = 35;
pub const anon_4__SC_BC_BASE_MAX: anon_4 = 36;
pub const anon_4__SC_BC_DIM_MAX: anon_4 = 37;
pub const anon_4__SC_BC_SCALE_MAX: anon_4 = 38;
pub const anon_4__SC_BC_STRING_MAX: anon_4 = 39;
pub const anon_4__SC_COLL_WEIGHTS_MAX: anon_4 = 40;
pub const anon_4__SC_EQUIV_CLASS_MAX: anon_4 = 41;
pub const anon_4__SC_EXPR_NEST_MAX: anon_4 = 42;
pub const anon_4__SC_LINE_MAX: anon_4 = 43;
pub const anon_4__SC_RE_DUP_MAX: anon_4 = 44;
pub const anon_4__SC_CHARCLASS_NAME_MAX: anon_4 = 45;
pub const anon_4__SC_2_VERSION: anon_4 = 46;
pub const anon_4__SC_2_C_BIND: anon_4 = 47;
pub const anon_4__SC_2_C_DEV: anon_4 = 48;
pub const anon_4__SC_2_FORT_DEV: anon_4 = 49;
pub const anon_4__SC_2_FORT_RUN: anon_4 = 50;
pub const anon_4__SC_2_SW_DEV: anon_4 = 51;
pub const anon_4__SC_2_LOCALEDEF: anon_4 = 52;
pub const anon_4__SC_PII: anon_4 = 53;
pub const anon_4__SC_PII_XTI: anon_4 = 54;
pub const anon_4__SC_PII_SOCKET: anon_4 = 55;
pub const anon_4__SC_PII_INTERNET: anon_4 = 56;
pub const anon_4__SC_PII_OSI: anon_4 = 57;
pub const anon_4__SC_POLL: anon_4 = 58;
pub const anon_4__SC_SELECT: anon_4 = 59;
pub const anon_4__SC_UIO_MAXIOV: anon_4 = 60;
pub const anon_4__SC_IOV_MAX: anon_4 = 60;
pub const anon_4__SC_PII_INTERNET_STREAM: anon_4 = 61;
pub const anon_4__SC_PII_INTERNET_DGRAM: anon_4 = 62;
pub const anon_4__SC_PII_OSI_COTS: anon_4 = 63;
pub const anon_4__SC_PII_OSI_CLTS: anon_4 = 64;
pub const anon_4__SC_PII_OSI_M: anon_4 = 65;
pub const anon_4__SC_T_IOV_MAX: anon_4 = 66;
pub const anon_4__SC_THREADS: anon_4 = 67;
pub const anon_4__SC_THREAD_SAFE_FUNCTIONS: anon_4 = 68;
pub const anon_4__SC_GETGR_R_SIZE_MAX: anon_4 = 69;
pub const anon_4__SC_GETPW_R_SIZE_MAX: anon_4 = 70;
pub const anon_4__SC_LOGIN_NAME_MAX: anon_4 = 71;
pub const anon_4__SC_TTY_NAME_MAX: anon_4 = 72;
pub const anon_4__SC_THREAD_DESTRUCTOR_ITERATIONS: anon_4 = 73;
pub const anon_4__SC_THREAD_KEYS_MAX: anon_4 = 74;
pub const anon_4__SC_THREAD_STACK_MIN: anon_4 = 75;
pub const anon_4__SC_THREAD_THREADS_MAX: anon_4 = 76;
pub const anon_4__SC_THREAD_ATTR_STACKADDR: anon_4 = 77;
pub const anon_4__SC_THREAD_ATTR_STACKSIZE: anon_4 = 78;
pub const anon_4__SC_THREAD_PRIORITY_SCHEDULING: anon_4 = 79;
pub const anon_4__SC_THREAD_PRIO_INHERIT: anon_4 = 80;
pub const anon_4__SC_THREAD_PRIO_PROTECT: anon_4 = 81;
pub const anon_4__SC_THREAD_PROCESS_SHARED: anon_4 = 82;
pub const anon_4__SC_NPROCESSORS_CONF: anon_4 = 83;
pub const anon_4__SC_NPROCESSORS_ONLN: anon_4 = 84;
pub const anon_4__SC_PHYS_PAGES: anon_4 = 85;
pub const anon_4__SC_AVPHYS_PAGES: anon_4 = 86;
pub const anon_4__SC_ATEXIT_MAX: anon_4 = 87;
pub const anon_4__SC_PASS_MAX: anon_4 = 88;
pub const anon_4__SC_XOPEN_VERSION: anon_4 = 89;
pub const anon_4__SC_XOPEN_XCU_VERSION: anon_4 = 90;
pub const anon_4__SC_XOPEN_UNIX: anon_4 = 91;
pub const anon_4__SC_XOPEN_CRYPT: anon_4 = 92;
pub const anon_4__SC_XOPEN_ENH_I18N: anon_4 = 93;
pub const anon_4__SC_XOPEN_SHM: anon_4 = 94;
pub const anon_4__SC_2_CHAR_TERM: anon_4 = 95;
pub const anon_4__SC_2_C_VERSION: anon_4 = 96;
pub const anon_4__SC_2_UPE: anon_4 = 97;
pub const anon_4__SC_XOPEN_XPG2: anon_4 = 98;
pub const anon_4__SC_XOPEN_XPG3: anon_4 = 99;
pub const anon_4__SC_XOPEN_XPG4: anon_4 = 100;
pub const anon_4__SC_CHAR_BIT: anon_4 = 101;
pub const anon_4__SC_CHAR_MAX: anon_4 = 102;
pub const anon_4__SC_CHAR_MIN: anon_4 = 103;
pub const anon_4__SC_INT_MAX: anon_4 = 104;
pub const anon_4__SC_INT_MIN: anon_4 = 105;
pub const anon_4__SC_LONG_BIT: anon_4 = 106;
pub const anon_4__SC_WORD_BIT: anon_4 = 107;
pub const anon_4__SC_MB_LEN_MAX: anon_4 = 108;
pub const anon_4__SC_NZERO: anon_4 = 109;
pub const anon_4__SC_SSIZE_MAX: anon_4 = 110;
pub const anon_4__SC_SCHAR_MAX: anon_4 = 111;
pub const anon_4__SC_SCHAR_MIN: anon_4 = 112;
pub const anon_4__SC_SHRT_MAX: anon_4 = 113;
pub const anon_4__SC_SHRT_MIN: anon_4 = 114;
pub const anon_4__SC_UCHAR_MAX: anon_4 = 115;
pub const anon_4__SC_UINT_MAX: anon_4 = 116;
pub const anon_4__SC_ULONG_MAX: anon_4 = 117;
pub const anon_4__SC_USHRT_MAX: anon_4 = 118;
pub const anon_4__SC_NL_ARGMAX: anon_4 = 119;
pub const anon_4__SC_NL_LANGMAX: anon_4 = 120;
pub const anon_4__SC_NL_MSGMAX: anon_4 = 121;
pub const anon_4__SC_NL_NMAX: anon_4 = 122;
pub const anon_4__SC_NL_SETMAX: anon_4 = 123;
pub const anon_4__SC_NL_TEXTMAX: anon_4 = 124;
pub const anon_4__SC_XBS5_ILP32_OFF32: anon_4 = 125;
pub const anon_4__SC_XBS5_ILP32_OFFBIG: anon_4 = 126;
pub const anon_4__SC_XBS5_LP64_OFF64: anon_4 = 127;
pub const anon_4__SC_XBS5_LPBIG_OFFBIG: anon_4 = 128;
pub const anon_4__SC_XOPEN_LEGACY: anon_4 = 129;
pub const anon_4__SC_XOPEN_REALTIME: anon_4 = 130;
pub const anon_4__SC_XOPEN_REALTIME_THREADS: anon_4 = 131;
pub const anon_4__SC_ADVISORY_INFO: anon_4 = 132;
pub const anon_4__SC_BARRIERS: anon_4 = 133;
pub const anon_4__SC_BASE: anon_4 = 134;
pub const anon_4__SC_C_LANG_SUPPORT: anon_4 = 135;
pub const anon_4__SC_C_LANG_SUPPORT_R: anon_4 = 136;
pub const anon_4__SC_CLOCK_SELECTION: anon_4 = 137;
pub const anon_4__SC_CPUTIME: anon_4 = 138;
pub const anon_4__SC_THREAD_CPUTIME: anon_4 = 139;
pub const anon_4__SC_DEVICE_IO: anon_4 = 140;
pub const anon_4__SC_DEVICE_SPECIFIC: anon_4 = 141;
pub const anon_4__SC_DEVICE_SPECIFIC_R: anon_4 = 142;
pub const anon_4__SC_FD_MGMT: anon_4 = 143;
pub const anon_4__SC_FIFO: anon_4 = 144;
pub const anon_4__SC_PIPE: anon_4 = 145;
pub const anon_4__SC_FILE_ATTRIBUTES: anon_4 = 146;
pub const anon_4__SC_FILE_LOCKING: anon_4 = 147;
pub const anon_4__SC_FILE_SYSTEM: anon_4 = 148;
pub const anon_4__SC_MONOTONIC_CLOCK: anon_4 = 149;
pub const anon_4__SC_MULTI_PROCESS: anon_4 = 150;
pub const anon_4__SC_SINGLE_PROCESS: anon_4 = 151;
pub const anon_4__SC_NETWORKING: anon_4 = 152;
pub const anon_4__SC_READER_WRITER_LOCKS: anon_4 = 153;
pub const anon_4__SC_SPIN_LOCKS: anon_4 = 154;
pub const anon_4__SC_REGEXP: anon_4 = 155;
pub const anon_4__SC_REGEX_VERSION: anon_4 = 156;
pub const anon_4__SC_SHELL: anon_4 = 157;
pub const anon_4__SC_SIGNALS: anon_4 = 158;
pub const anon_4__SC_SPAWN: anon_4 = 159;
pub const anon_4__SC_SPORADIC_SERVER: anon_4 = 160;
pub const anon_4__SC_THREAD_SPORADIC_SERVER: anon_4 = 161;
pub const anon_4__SC_SYSTEM_DATABASE: anon_4 = 162;
pub const anon_4__SC_SYSTEM_DATABASE_R: anon_4 = 163;
pub const anon_4__SC_TIMEOUTS: anon_4 = 164;
pub const anon_4__SC_TYPED_MEMORY_OBJECTS: anon_4 = 165;
pub const anon_4__SC_USER_GROUPS: anon_4 = 166;
pub const anon_4__SC_USER_GROUPS_R: anon_4 = 167;
pub const anon_4__SC_2_PBS: anon_4 = 168;
pub const anon_4__SC_2_PBS_ACCOUNTING: anon_4 = 169;
pub const anon_4__SC_2_PBS_LOCATE: anon_4 = 170;
pub const anon_4__SC_2_PBS_MESSAGE: anon_4 = 171;
pub const anon_4__SC_2_PBS_TRACK: anon_4 = 172;
pub const anon_4__SC_SYMLOOP_MAX: anon_4 = 173;
pub const anon_4__SC_STREAMS: anon_4 = 174;
pub const anon_4__SC_2_PBS_CHECKPOINT: anon_4 = 175;
pub const anon_4__SC_V6_ILP32_OFF32: anon_4 = 176;
pub const anon_4__SC_V6_ILP32_OFFBIG: anon_4 = 177;
pub const anon_4__SC_V6_LP64_OFF64: anon_4 = 178;
pub const anon_4__SC_V6_LPBIG_OFFBIG: anon_4 = 179;
pub const anon_4__SC_HOST_NAME_MAX: anon_4 = 180;
pub const anon_4__SC_TRACE: anon_4 = 181;
pub const anon_4__SC_TRACE_EVENT_FILTER: anon_4 = 182;
pub const anon_4__SC_TRACE_INHERIT: anon_4 = 183;
pub const anon_4__SC_TRACE_LOG: anon_4 = 184;
pub const anon_4__SC_LEVEL1_ICACHE_SIZE: anon_4 = 185;
pub const anon_4__SC_LEVEL1_ICACHE_ASSOC: anon_4 = 186;
pub const anon_4__SC_LEVEL1_ICACHE_LINESIZE: anon_4 = 187;
pub const anon_4__SC_LEVEL1_DCACHE_SIZE: anon_4 = 188;
pub const anon_4__SC_LEVEL1_DCACHE_ASSOC: anon_4 = 189;
pub const anon_4__SC_LEVEL1_DCACHE_LINESIZE: anon_4 = 190;
pub const anon_4__SC_LEVEL2_CACHE_SIZE: anon_4 = 191;
pub const anon_4__SC_LEVEL2_CACHE_ASSOC: anon_4 = 192;
pub const anon_4__SC_LEVEL2_CACHE_LINESIZE: anon_4 = 193;
pub const anon_4__SC_LEVEL3_CACHE_SIZE: anon_4 = 194;
pub const anon_4__SC_LEVEL3_CACHE_ASSOC: anon_4 = 195;
pub const anon_4__SC_LEVEL3_CACHE_LINESIZE: anon_4 = 196;
pub const anon_4__SC_LEVEL4_CACHE_SIZE: anon_4 = 197;
pub const anon_4__SC_LEVEL4_CACHE_ASSOC: anon_4 = 198;
pub const anon_4__SC_LEVEL4_CACHE_LINESIZE: anon_4 = 199;
pub const anon_4__SC_IPV6: anon_4 = 235;
pub const anon_4__SC_RAW_SOCKETS: anon_4 = 236;
pub const anon_4__SC_V7_ILP32_OFF32: anon_4 = 237;
pub const anon_4__SC_V7_ILP32_OFFBIG: anon_4 = 238;
pub const anon_4__SC_V7_LP64_OFF64: anon_4 = 239;
pub const anon_4__SC_V7_LPBIG_OFFBIG: anon_4 = 240;
pub const anon_4__SC_SS_REPL_MAX: anon_4 = 241;
pub const anon_4__SC_TRACE_EVENT_NAME_MAX: anon_4 = 242;
pub const anon_4__SC_TRACE_NAME_MAX: anon_4 = 243;
pub const anon_4__SC_TRACE_SYS_MAX: anon_4 = 244;
pub const anon_4__SC_TRACE_USER_EVENT_MAX: anon_4 = 245;
pub const anon_4__SC_XOPEN_STREAMS: anon_4 = 246;
pub const anon_4__SC_THREAD_ROBUST_PRIO_INHERIT: anon_4 = 247;
pub const anon_4__SC_THREAD_ROBUST_PRIO_PROTECT: anon_4 = 248;
pub const anon_4__SC_MINSIGSTKSZ: anon_4 = 249;
pub const anon_4__SC_SIGSTKSZ: anon_4 = 250;
pub fn test_fputc_5() {
    ({
        fputc_6(
            ('H' as i32),
            (*stdout_2.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        fputc_6(
            ('i' as i32),
            (*stdout_2.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        fputc_6(
            ('\n' as i32),
            (*stdout_2.with(Value::clone).borrow()).clone(),
        )
    });
}
pub fn test_fputs_7() {
    ({
        fputs_8(
            Ptr::<i8>::from_string_literal(b"hello"),
            (*stdout_2.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        fputc_6(
            ('\n' as i32),
            (*stdout_2.with(Value::clone).borrow()).clone(),
        )
    });
    let mut s: Ptr<i8> = Ptr::<i8>::from_string_literal(b"from variable");
    ({ fputs_8((s).clone(), (*stdout_2.with(Value::clone).borrow()).clone()) });
    ({
        fputc_6(
            ('\n' as i32),
            (*stdout_2.with(Value::clone).borrow()).clone(),
        )
    });
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        (('b' as i32) as i8),
        (('u' as i32) as i8),
        (('f' as i32) as i8),
        (('\0' as i32) as i8),
    ])));
    ({
        fputs_8(
            (buf.as_pointer() as Ptr<i8>),
            (*stdout_2.with(Value::clone).borrow()).clone(),
        )
    });
    ({
        fputc_6(
            ('\n' as i32),
            (*stdout_2.with(Value::clone).borrow()).clone(),
        )
    });
}
pub fn test_puts_9() {
    ({ puts_10(Ptr::<i8>::from_string_literal(b"puts hello")) });
    let mut s: Ptr<i8> = Ptr::<i8>::from_string_literal(b"puts variable");
    ({ puts_10((s).clone()) });
}
pub fn test_fileno_11() {
    {
        &(4usize);
        {
            if (((({ fileno_12((*stdin_1.with(Value::clone).borrow()).clone()) }) == 0) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_13(
                        Ptr::<i8>::from_string_literal(b"fileno(stdin) == 0"),
                        Ptr::<i8>::from_string_literal(b"stdio.c"),
                        29_u32,
                        Ptr::<i8>::from_string_literal(b"void test_fileno(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ fileno_12((*stdout_2.with(Value::clone).borrow()).clone()) }) == 1) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_13(
                        Ptr::<i8>::from_string_literal(b"fileno(stdout) == 1"),
                        Ptr::<i8>::from_string_literal(b"stdio.c"),
                        30_u32,
                        Ptr::<i8>::from_string_literal(b"void test_fileno(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ fileno_12((*stderr_3.with(Value::clone).borrow()).clone()) }) == 2) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_13(
                        Ptr::<i8>::from_string_literal(b"fileno(stderr) == 2"),
                        Ptr::<i8>::from_string_literal(b"stdio.c"),
                        31_u32,
                        Ptr::<i8>::from_string_literal(b"void test_fileno(void)"),
                    )
                });
            }
        }
    };
    let mut file: Ptr<i8> = Ptr::<i8>::from_string_literal(b"cpp2rust_fileno_test.tmp");
    let mut fp: Ptr<_IO_FILE> =
        ({ fopen_14((file).clone(), Ptr::<i8>::from_string_literal(b"wb")) });
    {
        &(4usize);
        {
            if (((!((fp).is_null())) as i32) != 0) {
            } else {
                ({
                    __assert_fail_13(
                        Ptr::<i8>::from_string_literal(b"fp != NULL"),
                        Ptr::<i8>::from_string_literal(b"stdio.c"),
                        34_u32,
                        Ptr::<i8>::from_string_literal(b"void test_fileno(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ fileno_12((fp).clone()) }) > 2) as i32) != 0) {
            } else {
                ({
                    __assert_fail_13(
                        Ptr::<i8>::from_string_literal(b"fileno(fp) > 2"),
                        Ptr::<i8>::from_string_literal(b"stdio.c"),
                        35_u32,
                        Ptr::<i8>::from_string_literal(b"void test_fileno(void)"),
                    )
                });
            }
        }
    };
    ({ fclose_15((fp).clone()) });
    {
        &(4usize);
        {
            if (((({ unlink_16((file).clone()) }) == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_13(
                        Ptr::<i8>::from_string_literal(b"unlink(file) == 0"),
                        Ptr::<i8>::from_string_literal(b"stdio.c"),
                        37_u32,
                        Ptr::<i8>::from_string_literal(b"void test_fileno(void)"),
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
    ({ test_fputc_5() });
    ({ test_fputs_7() });
    ({ test_puts_9() });
    ({ test_fileno_11() });
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
pub fn fputc_6(mut __c: i32, mut __stream: Ptr<_IO_FILE>) -> i32 {
    unimplemented!()
}
pub fn fputs_8(mut __s: Ptr<i8>, mut __stream: Ptr<_IO_FILE>) -> i32 {
    unimplemented!()
}
pub fn puts_10(mut __s: Ptr<i8>) -> i32 {
    unimplemented!()
}
pub fn fileno_12(mut __stream: Ptr<_IO_FILE>) -> i32 {
    unimplemented!()
}
pub fn __assert_fail_13(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn fopen_14(mut __filename: Ptr<i8>, mut __modes: Ptr<i8>) -> Ptr<_IO_FILE> {
    unimplemented!()
}
pub fn fclose_15(mut __stream: Ptr<_IO_FILE>) -> i32 {
    unimplemented!()
}
pub fn unlink_16(mut __name: Ptr<i8>) -> i32 {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
