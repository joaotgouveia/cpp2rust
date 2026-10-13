extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(VaArg, FnPtrArg)]
pub struct __fsid_t {
    pub __val: [i32; 2],
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
#[derive(VaArg, FnPtrArg)]
pub struct iovec {
    pub iov_base: *mut ::libc::c_void,
    pub iov_len: usize,
}
impl Clone for iovec {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iovec {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __sigset_t {
    pub __val: [u64; 16],
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
#[derive(VaArg, FnPtrArg)]
pub struct timeval {
    pub tv_sec: i64,
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
#[derive(VaArg, FnPtrArg)]
pub struct timespec {
    pub tv_sec: i64,
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
#[derive(VaArg, FnPtrArg)]
pub struct fd_set {
    pub __fds_bits: [i64; 16],
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
#[derive(VaArg, FnPtrArg)]
pub struct __atomic_wide_counter {
    pub __value64: u64,
    pub __value32: anon_0,
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
#[derive(VaArg, FnPtrArg)]
pub struct anon_0 {
    pub __low: u32,
    pub __high: u32,
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
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
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
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_internal_slist {
    pub __next: *mut __pthread_internal_slist,
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
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_mutex_s {
    pub __lock: i32,
    pub __count: u32,
    pub __owner: i32,
    pub __nusers: u32,
    pub __kind: i32,
    pub __spins: i16,
    pub __elision: i16,
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
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_rwlock_arch_t {
    pub __readers: u32,
    pub __writers: u32,
    pub __wrphase_futex: u32,
    pub __writers_futex: u32,
    pub __pad3: u32,
    pub __pad4: u32,
    pub __cur_writer: i32,
    pub __shared: i32,
    pub __rwelision: i8,
    pub __pad1: [u8; 7],
    pub __pad2: u64,
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
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_refs: [u32; 2],
    pub __g_size: [u32; 2],
    pub __g1_orig_size: u32,
    pub __wrefs: u32,
    pub __g_signals: [u32; 2],
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
#[derive(VaArg, FnPtrArg)]
pub struct __once_flag {
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_condattr_t {
    pub __size: [libc::c_char; 4],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_attr_t {
    pub __size: [libc::c_char; 56],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [libc::c_char; 56],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_rwlockattr_t {
    pub __size: [libc::c_char; 8],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_barrier_t {
    pub __size: [libc::c_char; 32],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_barrierattr_t {
    pub __size: [libc::c_char; 4],
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
#[derive(VaArg, FnPtrArg)]
pub struct sockaddr {
    pub sa_family: u16,
    pub sa_data: [libc::c_char; 14],
}
impl Clone for sockaddr {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for sockaddr {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct sockaddr_storage {
    pub ss_family: u16,
    pub __ss_padding: [libc::c_char; 118],
    pub __ss_align: u64,
}
impl Clone for sockaddr_storage {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for sockaddr_storage {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct msghdr {
    pub msg_name: *mut ::libc::c_void,
    pub msg_namelen: u32,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: usize,
    pub msg_control: *mut ::libc::c_void,
    pub msg_controllen: usize,
    pub msg_flags: i32,
}
impl Clone for msghdr {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for msghdr {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct cmsghdr {
    pub cmsg_len: usize,
    pub cmsg_level: i32,
    pub cmsg_type: i32,
    pub __cmsg_data: [u8; 0],
}
impl Clone for cmsghdr {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for cmsghdr {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __kernel_fd_set {
    pub fds_bits: [u64; 16],
}
impl Clone for __kernel_fd_set {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __kernel_fd_set {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __kernel_fsid_t {
    pub val: [i32; 2],
}
impl Clone for __kernel_fsid_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __kernel_fsid_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct linger {
    pub l_onoff: i32,
    pub l_linger: i32,
}
impl Clone for linger {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for linger {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct osockaddr {
    pub sa_family: u16,
    pub sa_data: [u8; 14],
}
impl Clone for osockaddr {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for osockaddr {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct in_addr {
    pub s_addr: u32,
}
impl Clone for in_addr {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for in_addr {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct ip_opts {
    pub ip_dst: in_addr,
    pub ip_opts: [libc::c_char; 40],
}
impl Clone for ip_opts {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ip_opts {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct in_pktinfo {
    pub ipi_ifindex: i32,
    pub ipi_spec_dst: in_addr,
    pub ipi_addr: in_addr,
}
impl Clone for in_pktinfo {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for in_pktinfo {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct in6_addr {
    pub __in6_u: anon_1,
}
impl Clone for in6_addr {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for in6_addr {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct anon_1 {
    pub __u6_addr8: [u8; 16],
    pub __u6_addr16: [u16; 8],
    pub __u6_addr32: [u32; 4],
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
#[derive(VaArg, FnPtrArg)]
pub struct sockaddr_in {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}
impl Clone for sockaddr_in {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for sockaddr_in {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct sockaddr_in6 {
    pub sin6_family: u16,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: u32,
}
impl Clone for sockaddr_in6 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for sockaddr_in6 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct ip_mreq {
    pub imr_multiaddr: in_addr,
    pub imr_interface: in_addr,
}
impl Clone for ip_mreq {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ip_mreq {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct ip_mreqn {
    pub imr_multiaddr: in_addr,
    pub imr_address: in_addr,
    pub imr_ifindex: i32,
}
impl Clone for ip_mreqn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ip_mreqn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct ip_mreq_source {
    pub imr_multiaddr: in_addr,
    pub imr_interface: in_addr,
    pub imr_sourceaddr: in_addr,
}
impl Clone for ip_mreq_source {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ip_mreq_source {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct ipv6_mreq {
    pub ipv6mr_multiaddr: in6_addr,
    pub ipv6mr_interface: u32,
}
impl Clone for ipv6_mreq {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ipv6_mreq {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct group_req {
    pub gr_interface: u32,
    pub gr_group: sockaddr_storage,
}
impl Clone for group_req {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for group_req {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct group_source_req {
    pub gsr_interface: u32,
    pub gsr_group: sockaddr_storage,
    pub gsr_source: sockaddr_storage,
}
impl Clone for group_source_req {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for group_source_req {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct ip_msfilter {
    pub imsf_multiaddr: in_addr,
    pub imsf_interface: in_addr,
    pub imsf_fmode: u32,
    pub imsf_numsrc: u32,
    pub imsf_slist: [in_addr; 1],
}
impl Clone for ip_msfilter {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ip_msfilter {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct group_filter {
    pub gf_interface: u32,
    pub gf_group: sockaddr_storage,
    pub gf_fmode: u32,
    pub gf_numsrc: u32,
    pub gf_slist: [sockaddr_storage; 1],
}
impl Clone for group_filter {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for group_filter {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13],
    pub __ctype_b: *const u16,
    pub __ctype_tolower: *const i32,
    pub __ctype_toupper: *const i32,
    pub __names: [*const libc::c_char; 13],
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
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut buf: [u8; 16] = [0_u8; 16];
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                inet_pton_2(
                    2,
                    (c"1.2.3.4".as_ptr().cast_mut()).cast_const(),
                    (buf.as_mut_ptr() as *mut ::libc::c_void),
                )
            }) == (1)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"inet_pton(AF_INET, \"1.2.3.4\", buf) == 1"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"inet_pton_ntop.c".as_ptr().cast_mut()).cast_const(),
                        7_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((((((((((buf[(0) as usize] as i32) == (1)) as i32) != 0)
                && ((((buf[(1) as usize] as i32) == (2)) as i32) != 0))
                as i32)
                != 0)
                && ((((buf[(2) as usize] as i32) == (3)) as i32) != 0))
                as i32)
                != 0)
                && ((((buf[(3) as usize] as i32) == (4)) as i32) != 0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"buf[0] == 1 && buf[1] == 2 && buf[2] == 3 && buf[3] == 4"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"inet_pton_ntop.c".as_ptr().cast_mut()).cast_const(),
                        8_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                inet_pton_2(
                    2,
                    (c"999.1.1.1".as_ptr().cast_mut()).cast_const(),
                    (buf.as_mut_ptr() as *mut ::libc::c_void),
                )
            }) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"inet_pton(AF_INET, \"999.1.1.1\", buf) == 0"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"inet_pton_ntop.c".as_ptr().cast_mut()).cast_const(),
                        9_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                inet_pton_2(
                    2,
                    (c"not an ip".as_ptr().cast_mut()).cast_const(),
                    (buf.as_mut_ptr() as *mut ::libc::c_void),
                )
            }) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"inet_pton(AF_INET, \"not an ip\", buf) == 0"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"inet_pton_ntop.c".as_ptr().cast_mut()).cast_const(),
                        10_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                inet_pton_2(
                    10,
                    (c"::1".as_ptr().cast_mut()).cast_const(),
                    (buf.as_mut_ptr() as *mut ::libc::c_void),
                )
            }) == (1)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"inet_pton(AF_INET6, \"::1\", buf) == 1"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"inet_pton_ntop.c".as_ptr().cast_mut()).cast_const(),
                        11_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((((buf[(0) as usize] as i32) == (0)) as i32) != 0)
                && ((((buf[(15) as usize] as i32) == (1)) as i32) != 0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"buf[0] == 0 && buf[15] == 1".as_ptr().cast_mut()).cast_const(),
                        (c"inet_pton_ntop.c".as_ptr().cast_mut()).cast_const(),
                        12_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                inet_pton_2(
                    10,
                    (c"2001:db8::5".as_ptr().cast_mut()).cast_const(),
                    (buf.as_mut_ptr() as *mut ::libc::c_void),
                )
            }) == (1)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"inet_pton(AF_INET6, \"2001:db8::5\", buf) == 1"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"inet_pton_ntop.c".as_ptr().cast_mut()).cast_const(),
                        13_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((((((((buf[(0) as usize] as i32) == (32)) as i32) != 0)
                && ((((buf[(1) as usize] as i32) == (1)) as i32) != 0))
                as i32)
                != 0)
                && ((((buf[(15) as usize] as i32) == (5)) as i32) != 0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"buf[0] == 0x20 && buf[1] == 0x01 && buf[15] == 5"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"inet_pton_ntop.c".as_ptr().cast_mut()).cast_const(),
                        14_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut text: [libc::c_char; 64] = [(0 as libc::c_char); 64];
    let mut four: [u8; 4] = [10_u8, 0_u8, 0_u8, 1_u8];
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                strcmp_4(
                    (unsafe {
                        let ___buf: *mut libc::c_char = text.as_mut_ptr();
                        let ___len: u32 = (::std::mem::size_of::<[libc::c_char; 64]>() as u32);
                        inet_ntop_5(
                            2,
                            (four.as_mut_ptr() as *const ::libc::c_void),
                            ___buf,
                            ___len,
                        )
                    }),
                    (c"10.0.0.1".as_ptr().cast_mut()).cast_const(),
                )
            }) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_3 ( ( c"strcmp(inet_ntop(AF_INET, four, text, sizeof(text)), \"10.0.0.1\") == 0" .as_ptr() .cast_mut() ) .cast_const() , ( c"inet_pton_ntop.c" .as_ptr() .cast_mut() ) .cast_const() , 18_u32 , c"int main(void)" .as_ptr() , )
                });
            }
        }
    };
    let mut sixteen: [u8; 16] = [
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8,
    ];
    sixteen[(15) as usize] = 1_u8;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                strcmp_4(
                    (unsafe {
                        let ___buf: *mut libc::c_char = text.as_mut_ptr();
                        let ___len: u32 = (::std::mem::size_of::<[libc::c_char; 64]>() as u32);
                        inet_ntop_5(
                            10,
                            (sixteen.as_mut_ptr() as *const ::libc::c_void),
                            ___buf,
                            ___len,
                        )
                    }),
                    (c"::1".as_ptr().cast_mut()).cast_const(),
                )
            }) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"strcmp(inet_ntop(AF_INET6, sixteen, text, sizeof(text)), \"::1\") == 0"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"inet_pton_ntop.c".as_ptr().cast_mut()).cast_const(),
                        21_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                inet_ntop_5(
                    2,
                    (four.as_mut_ptr() as *const ::libc::c_void),
                    text.as_mut_ptr(),
                    4_u32,
                )
            })
            .is_null()) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_3(
                        (c"inet_ntop(AF_INET, four, text, 4) == 0"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"inet_pton_ntop.c".as_ptr().cast_mut()).cast_const(),
                        22_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct __locale_data;
pub unsafe fn inet_pton_2(
    mut __af: i32,
    mut __cp: *const libc::c_char,
    mut __buf: *mut ::libc::c_void,
) -> i32 {
    unimplemented!()
}
pub unsafe fn __assert_fail_3(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn strcmp_4(mut __s1: *const libc::c_char, mut __s2: *const libc::c_char) -> i32 {
    unimplemented!()
}
pub unsafe fn inet_ntop_5(
    mut __af: i32,
    mut __cp: *const ::libc::c_void,
    mut __buf: *mut libc::c_char,
    mut __len: u32,
) -> *const libc::c_char {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
