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
pub type anon_0 = u32;
pub const anon_0__SC_ARG_MAX: anon_0 = 0;
pub const anon_0__SC_CHILD_MAX: anon_0 = 1;
pub const anon_0__SC_CLK_TCK: anon_0 = 2;
pub const anon_0__SC_NGROUPS_MAX: anon_0 = 3;
pub const anon_0__SC_OPEN_MAX: anon_0 = 4;
pub const anon_0__SC_STREAM_MAX: anon_0 = 5;
pub const anon_0__SC_TZNAME_MAX: anon_0 = 6;
pub const anon_0__SC_JOB_CONTROL: anon_0 = 7;
pub const anon_0__SC_SAVED_IDS: anon_0 = 8;
pub const anon_0__SC_REALTIME_SIGNALS: anon_0 = 9;
pub const anon_0__SC_PRIORITY_SCHEDULING: anon_0 = 10;
pub const anon_0__SC_TIMERS: anon_0 = 11;
pub const anon_0__SC_ASYNCHRONOUS_IO: anon_0 = 12;
pub const anon_0__SC_PRIORITIZED_IO: anon_0 = 13;
pub const anon_0__SC_SYNCHRONIZED_IO: anon_0 = 14;
pub const anon_0__SC_FSYNC: anon_0 = 15;
pub const anon_0__SC_MAPPED_FILES: anon_0 = 16;
pub const anon_0__SC_MEMLOCK: anon_0 = 17;
pub const anon_0__SC_MEMLOCK_RANGE: anon_0 = 18;
pub const anon_0__SC_MEMORY_PROTECTION: anon_0 = 19;
pub const anon_0__SC_MESSAGE_PASSING: anon_0 = 20;
pub const anon_0__SC_SEMAPHORES: anon_0 = 21;
pub const anon_0__SC_SHARED_MEMORY_OBJECTS: anon_0 = 22;
pub const anon_0__SC_AIO_LISTIO_MAX: anon_0 = 23;
pub const anon_0__SC_AIO_MAX: anon_0 = 24;
pub const anon_0__SC_AIO_PRIO_DELTA_MAX: anon_0 = 25;
pub const anon_0__SC_DELAYTIMER_MAX: anon_0 = 26;
pub const anon_0__SC_MQ_OPEN_MAX: anon_0 = 27;
pub const anon_0__SC_MQ_PRIO_MAX: anon_0 = 28;
pub const anon_0__SC_VERSION: anon_0 = 29;
pub const anon_0__SC_PAGESIZE: anon_0 = 30;
pub const anon_0__SC_RTSIG_MAX: anon_0 = 31;
pub const anon_0__SC_SEM_NSEMS_MAX: anon_0 = 32;
pub const anon_0__SC_SEM_VALUE_MAX: anon_0 = 33;
pub const anon_0__SC_SIGQUEUE_MAX: anon_0 = 34;
pub const anon_0__SC_TIMER_MAX: anon_0 = 35;
pub const anon_0__SC_BC_BASE_MAX: anon_0 = 36;
pub const anon_0__SC_BC_DIM_MAX: anon_0 = 37;
pub const anon_0__SC_BC_SCALE_MAX: anon_0 = 38;
pub const anon_0__SC_BC_STRING_MAX: anon_0 = 39;
pub const anon_0__SC_COLL_WEIGHTS_MAX: anon_0 = 40;
pub const anon_0__SC_EQUIV_CLASS_MAX: anon_0 = 41;
pub const anon_0__SC_EXPR_NEST_MAX: anon_0 = 42;
pub const anon_0__SC_LINE_MAX: anon_0 = 43;
pub const anon_0__SC_RE_DUP_MAX: anon_0 = 44;
pub const anon_0__SC_CHARCLASS_NAME_MAX: anon_0 = 45;
pub const anon_0__SC_2_VERSION: anon_0 = 46;
pub const anon_0__SC_2_C_BIND: anon_0 = 47;
pub const anon_0__SC_2_C_DEV: anon_0 = 48;
pub const anon_0__SC_2_FORT_DEV: anon_0 = 49;
pub const anon_0__SC_2_FORT_RUN: anon_0 = 50;
pub const anon_0__SC_2_SW_DEV: anon_0 = 51;
pub const anon_0__SC_2_LOCALEDEF: anon_0 = 52;
pub const anon_0__SC_PII: anon_0 = 53;
pub const anon_0__SC_PII_XTI: anon_0 = 54;
pub const anon_0__SC_PII_SOCKET: anon_0 = 55;
pub const anon_0__SC_PII_INTERNET: anon_0 = 56;
pub const anon_0__SC_PII_OSI: anon_0 = 57;
pub const anon_0__SC_POLL: anon_0 = 58;
pub const anon_0__SC_SELECT: anon_0 = 59;
pub const anon_0__SC_UIO_MAXIOV: anon_0 = 60;
pub const anon_0__SC_IOV_MAX: anon_0 = 60;
pub const anon_0__SC_PII_INTERNET_STREAM: anon_0 = 61;
pub const anon_0__SC_PII_INTERNET_DGRAM: anon_0 = 62;
pub const anon_0__SC_PII_OSI_COTS: anon_0 = 63;
pub const anon_0__SC_PII_OSI_CLTS: anon_0 = 64;
pub const anon_0__SC_PII_OSI_M: anon_0 = 65;
pub const anon_0__SC_T_IOV_MAX: anon_0 = 66;
pub const anon_0__SC_THREADS: anon_0 = 67;
pub const anon_0__SC_THREAD_SAFE_FUNCTIONS: anon_0 = 68;
pub const anon_0__SC_GETGR_R_SIZE_MAX: anon_0 = 69;
pub const anon_0__SC_GETPW_R_SIZE_MAX: anon_0 = 70;
pub const anon_0__SC_LOGIN_NAME_MAX: anon_0 = 71;
pub const anon_0__SC_TTY_NAME_MAX: anon_0 = 72;
pub const anon_0__SC_THREAD_DESTRUCTOR_ITERATIONS: anon_0 = 73;
pub const anon_0__SC_THREAD_KEYS_MAX: anon_0 = 74;
pub const anon_0__SC_THREAD_STACK_MIN: anon_0 = 75;
pub const anon_0__SC_THREAD_THREADS_MAX: anon_0 = 76;
pub const anon_0__SC_THREAD_ATTR_STACKADDR: anon_0 = 77;
pub const anon_0__SC_THREAD_ATTR_STACKSIZE: anon_0 = 78;
pub const anon_0__SC_THREAD_PRIORITY_SCHEDULING: anon_0 = 79;
pub const anon_0__SC_THREAD_PRIO_INHERIT: anon_0 = 80;
pub const anon_0__SC_THREAD_PRIO_PROTECT: anon_0 = 81;
pub const anon_0__SC_THREAD_PROCESS_SHARED: anon_0 = 82;
pub const anon_0__SC_NPROCESSORS_CONF: anon_0 = 83;
pub const anon_0__SC_NPROCESSORS_ONLN: anon_0 = 84;
pub const anon_0__SC_PHYS_PAGES: anon_0 = 85;
pub const anon_0__SC_AVPHYS_PAGES: anon_0 = 86;
pub const anon_0__SC_ATEXIT_MAX: anon_0 = 87;
pub const anon_0__SC_PASS_MAX: anon_0 = 88;
pub const anon_0__SC_XOPEN_VERSION: anon_0 = 89;
pub const anon_0__SC_XOPEN_XCU_VERSION: anon_0 = 90;
pub const anon_0__SC_XOPEN_UNIX: anon_0 = 91;
pub const anon_0__SC_XOPEN_CRYPT: anon_0 = 92;
pub const anon_0__SC_XOPEN_ENH_I18N: anon_0 = 93;
pub const anon_0__SC_XOPEN_SHM: anon_0 = 94;
pub const anon_0__SC_2_CHAR_TERM: anon_0 = 95;
pub const anon_0__SC_2_C_VERSION: anon_0 = 96;
pub const anon_0__SC_2_UPE: anon_0 = 97;
pub const anon_0__SC_XOPEN_XPG2: anon_0 = 98;
pub const anon_0__SC_XOPEN_XPG3: anon_0 = 99;
pub const anon_0__SC_XOPEN_XPG4: anon_0 = 100;
pub const anon_0__SC_CHAR_BIT: anon_0 = 101;
pub const anon_0__SC_CHAR_MAX: anon_0 = 102;
pub const anon_0__SC_CHAR_MIN: anon_0 = 103;
pub const anon_0__SC_INT_MAX: anon_0 = 104;
pub const anon_0__SC_INT_MIN: anon_0 = 105;
pub const anon_0__SC_LONG_BIT: anon_0 = 106;
pub const anon_0__SC_WORD_BIT: anon_0 = 107;
pub const anon_0__SC_MB_LEN_MAX: anon_0 = 108;
pub const anon_0__SC_NZERO: anon_0 = 109;
pub const anon_0__SC_SSIZE_MAX: anon_0 = 110;
pub const anon_0__SC_SCHAR_MAX: anon_0 = 111;
pub const anon_0__SC_SCHAR_MIN: anon_0 = 112;
pub const anon_0__SC_SHRT_MAX: anon_0 = 113;
pub const anon_0__SC_SHRT_MIN: anon_0 = 114;
pub const anon_0__SC_UCHAR_MAX: anon_0 = 115;
pub const anon_0__SC_UINT_MAX: anon_0 = 116;
pub const anon_0__SC_ULONG_MAX: anon_0 = 117;
pub const anon_0__SC_USHRT_MAX: anon_0 = 118;
pub const anon_0__SC_NL_ARGMAX: anon_0 = 119;
pub const anon_0__SC_NL_LANGMAX: anon_0 = 120;
pub const anon_0__SC_NL_MSGMAX: anon_0 = 121;
pub const anon_0__SC_NL_NMAX: anon_0 = 122;
pub const anon_0__SC_NL_SETMAX: anon_0 = 123;
pub const anon_0__SC_NL_TEXTMAX: anon_0 = 124;
pub const anon_0__SC_XBS5_ILP32_OFF32: anon_0 = 125;
pub const anon_0__SC_XBS5_ILP32_OFFBIG: anon_0 = 126;
pub const anon_0__SC_XBS5_LP64_OFF64: anon_0 = 127;
pub const anon_0__SC_XBS5_LPBIG_OFFBIG: anon_0 = 128;
pub const anon_0__SC_XOPEN_LEGACY: anon_0 = 129;
pub const anon_0__SC_XOPEN_REALTIME: anon_0 = 130;
pub const anon_0__SC_XOPEN_REALTIME_THREADS: anon_0 = 131;
pub const anon_0__SC_ADVISORY_INFO: anon_0 = 132;
pub const anon_0__SC_BARRIERS: anon_0 = 133;
pub const anon_0__SC_BASE: anon_0 = 134;
pub const anon_0__SC_C_LANG_SUPPORT: anon_0 = 135;
pub const anon_0__SC_C_LANG_SUPPORT_R: anon_0 = 136;
pub const anon_0__SC_CLOCK_SELECTION: anon_0 = 137;
pub const anon_0__SC_CPUTIME: anon_0 = 138;
pub const anon_0__SC_THREAD_CPUTIME: anon_0 = 139;
pub const anon_0__SC_DEVICE_IO: anon_0 = 140;
pub const anon_0__SC_DEVICE_SPECIFIC: anon_0 = 141;
pub const anon_0__SC_DEVICE_SPECIFIC_R: anon_0 = 142;
pub const anon_0__SC_FD_MGMT: anon_0 = 143;
pub const anon_0__SC_FIFO: anon_0 = 144;
pub const anon_0__SC_PIPE: anon_0 = 145;
pub const anon_0__SC_FILE_ATTRIBUTES: anon_0 = 146;
pub const anon_0__SC_FILE_LOCKING: anon_0 = 147;
pub const anon_0__SC_FILE_SYSTEM: anon_0 = 148;
pub const anon_0__SC_MONOTONIC_CLOCK: anon_0 = 149;
pub const anon_0__SC_MULTI_PROCESS: anon_0 = 150;
pub const anon_0__SC_SINGLE_PROCESS: anon_0 = 151;
pub const anon_0__SC_NETWORKING: anon_0 = 152;
pub const anon_0__SC_READER_WRITER_LOCKS: anon_0 = 153;
pub const anon_0__SC_SPIN_LOCKS: anon_0 = 154;
pub const anon_0__SC_REGEXP: anon_0 = 155;
pub const anon_0__SC_REGEX_VERSION: anon_0 = 156;
pub const anon_0__SC_SHELL: anon_0 = 157;
pub const anon_0__SC_SIGNALS: anon_0 = 158;
pub const anon_0__SC_SPAWN: anon_0 = 159;
pub const anon_0__SC_SPORADIC_SERVER: anon_0 = 160;
pub const anon_0__SC_THREAD_SPORADIC_SERVER: anon_0 = 161;
pub const anon_0__SC_SYSTEM_DATABASE: anon_0 = 162;
pub const anon_0__SC_SYSTEM_DATABASE_R: anon_0 = 163;
pub const anon_0__SC_TIMEOUTS: anon_0 = 164;
pub const anon_0__SC_TYPED_MEMORY_OBJECTS: anon_0 = 165;
pub const anon_0__SC_USER_GROUPS: anon_0 = 166;
pub const anon_0__SC_USER_GROUPS_R: anon_0 = 167;
pub const anon_0__SC_2_PBS: anon_0 = 168;
pub const anon_0__SC_2_PBS_ACCOUNTING: anon_0 = 169;
pub const anon_0__SC_2_PBS_LOCATE: anon_0 = 170;
pub const anon_0__SC_2_PBS_MESSAGE: anon_0 = 171;
pub const anon_0__SC_2_PBS_TRACK: anon_0 = 172;
pub const anon_0__SC_SYMLOOP_MAX: anon_0 = 173;
pub const anon_0__SC_STREAMS: anon_0 = 174;
pub const anon_0__SC_2_PBS_CHECKPOINT: anon_0 = 175;
pub const anon_0__SC_V6_ILP32_OFF32: anon_0 = 176;
pub const anon_0__SC_V6_ILP32_OFFBIG: anon_0 = 177;
pub const anon_0__SC_V6_LP64_OFF64: anon_0 = 178;
pub const anon_0__SC_V6_LPBIG_OFFBIG: anon_0 = 179;
pub const anon_0__SC_HOST_NAME_MAX: anon_0 = 180;
pub const anon_0__SC_TRACE: anon_0 = 181;
pub const anon_0__SC_TRACE_EVENT_FILTER: anon_0 = 182;
pub const anon_0__SC_TRACE_INHERIT: anon_0 = 183;
pub const anon_0__SC_TRACE_LOG: anon_0 = 184;
pub const anon_0__SC_LEVEL1_ICACHE_SIZE: anon_0 = 185;
pub const anon_0__SC_LEVEL1_ICACHE_ASSOC: anon_0 = 186;
pub const anon_0__SC_LEVEL1_ICACHE_LINESIZE: anon_0 = 187;
pub const anon_0__SC_LEVEL1_DCACHE_SIZE: anon_0 = 188;
pub const anon_0__SC_LEVEL1_DCACHE_ASSOC: anon_0 = 189;
pub const anon_0__SC_LEVEL1_DCACHE_LINESIZE: anon_0 = 190;
pub const anon_0__SC_LEVEL2_CACHE_SIZE: anon_0 = 191;
pub const anon_0__SC_LEVEL2_CACHE_ASSOC: anon_0 = 192;
pub const anon_0__SC_LEVEL2_CACHE_LINESIZE: anon_0 = 193;
pub const anon_0__SC_LEVEL3_CACHE_SIZE: anon_0 = 194;
pub const anon_0__SC_LEVEL3_CACHE_ASSOC: anon_0 = 195;
pub const anon_0__SC_LEVEL3_CACHE_LINESIZE: anon_0 = 196;
pub const anon_0__SC_LEVEL4_CACHE_SIZE: anon_0 = 197;
pub const anon_0__SC_LEVEL4_CACHE_ASSOC: anon_0 = 198;
pub const anon_0__SC_LEVEL4_CACHE_LINESIZE: anon_0 = 199;
pub const anon_0__SC_IPV6: anon_0 = 235;
pub const anon_0__SC_RAW_SOCKETS: anon_0 = 236;
pub const anon_0__SC_V7_ILP32_OFF32: anon_0 = 237;
pub const anon_0__SC_V7_ILP32_OFFBIG: anon_0 = 238;
pub const anon_0__SC_V7_LP64_OFF64: anon_0 = 239;
pub const anon_0__SC_V7_LPBIG_OFFBIG: anon_0 = 240;
pub const anon_0__SC_SS_REPL_MAX: anon_0 = 241;
pub const anon_0__SC_TRACE_EVENT_NAME_MAX: anon_0 = 242;
pub const anon_0__SC_TRACE_NAME_MAX: anon_0 = 243;
pub const anon_0__SC_TRACE_SYS_MAX: anon_0 = 244;
pub const anon_0__SC_TRACE_USER_EVENT_MAX: anon_0 = 245;
pub const anon_0__SC_XOPEN_STREAMS: anon_0 = 246;
pub const anon_0__SC_THREAD_ROBUST_PRIO_INHERIT: anon_0 = 247;
pub const anon_0__SC_THREAD_ROBUST_PRIO_PROTECT: anon_0 = 248;
pub const anon_0__SC_MINSIGSTKSZ: anon_0 = 249;
pub const anon_0__SC_SIGSTKSZ: anon_0 = 250;
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let fds: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..2).map(|_| 0_i32).collect::<Box<[i32]>>()));
    {
        &(4usize);
        {
            if (((({ pipe_1((fds.as_pointer() as Ptr<i32>)) }) == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"pipe(fds) == 0"),
                        Ptr::<i8>::from_string_literal(b"fcntl.c"),
                        7_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    let mut flags: i32 = ({ fcntl_3((*fds.borrow())[(0) as usize], 3, &[(0).into()]) });
    {
        &(4usize);
        {
            if (((flags >= 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"flags >= 0"),
                        Ptr::<i8>::from_string_literal(b"fcntl.c"),
                        9_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((((flags & 2048) == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"(flags & O_NONBLOCK) == 0"),
                        Ptr::<i8>::from_string_literal(b"fcntl.c"),
                        10_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ fcntl_3((*fds.borrow())[(0) as usize], 4, &[(flags | 2048).into()]) }) == 0)
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(
                            b"fcntl(fds[0], F_SETFL, flags | O_NONBLOCK) == 0",
                        ),
                        Ptr::<i8>::from_string_literal(b"fcntl.c"),
                        11_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    flags = ({ fcntl_3((*fds.borrow())[(0) as usize], 3, &[(0).into()]) });
    {
        &(4usize);
        {
            if ((((flags & 2048) != 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"(flags & O_NONBLOCK) != 0"),
                        Ptr::<i8>::from_string_literal(b"fcntl.c"),
                        13_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    let b: Value<i8> = Rc::new(RefCell::new(0_i8));
    {
        &(4usize);
        {
            if (((({
                read_4(
                    (*fds.borrow())[(0) as usize],
                    ((b.as_pointer()) as Ptr<i8>).to_any(),
                    1_usize,
                )
            }) == (-1_i32 as isize)) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"read(fds[0], &b, 1) == -1"),
                        Ptr::<i8>::from_string_literal(b"fcntl.c"),
                        15_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((((({ fcntl_3((*fds.borrow())[(0) as usize], 1, &[(0).into()]) }) & 1) == 0)
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(
                            b"(fcntl(fds[0], F_GETFD, 0) & FD_CLOEXEC) == 0",
                        ),
                        Ptr::<i8>::from_string_literal(b"fcntl.c"),
                        16_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ fcntl_3((*fds.borrow())[(0) as usize], 2, &[(1).into()]) }) == 0) as i32) != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"fcntl(fds[0], F_SETFD, FD_CLOEXEC) == 0"),
                        Ptr::<i8>::from_string_literal(b"fcntl.c"),
                        17_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((((({ fcntl_3((*fds.borrow())[(0) as usize], 1, &[(0).into()]) }) & 1) != 0)
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(
                            b"(fcntl(fds[0], F_GETFD, 0) & FD_CLOEXEC) != 0",
                        ),
                        Ptr::<i8>::from_string_literal(b"fcntl.c"),
                        18_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ close_5((*fds.borrow())[(0) as usize]) }) == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"close(fds[0]) == 0"),
                        Ptr::<i8>::from_string_literal(b"fcntl.c"),
                        19_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ close_5((*fds.borrow())[(1) as usize]) }) == 0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_2(
                        Ptr::<i8>::from_string_literal(b"close(fds[1]) == 0"),
                        Ptr::<i8>::from_string_literal(b"fcntl.c"),
                        20_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    return 0;
}
pub fn pipe_1(mut __pipedes: Ptr<i32>) -> i32 {
    unimplemented!()
}
pub fn __assert_fail_2(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn fcntl_3(mut __fd: i32, mut __cmd: i32, __args: &[VaArg]) -> i32 {
    unimplemented!()
}
pub fn read_4(mut __fd: i32, mut __buf: AnyPtr, mut __nbytes: usize) -> isize {
    unimplemented!()
}
pub fn close_5(mut __fd: i32) -> i32 {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
