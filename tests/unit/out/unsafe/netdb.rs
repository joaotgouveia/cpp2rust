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
pub type __socket_type = u32;
pub const __socket_type_SOCK_STREAM: __socket_type = 1;
pub const __socket_type_SOCK_DGRAM: __socket_type = 2;
pub const __socket_type_SOCK_RAW: __socket_type = 3;
pub const __socket_type_SOCK_RDM: __socket_type = 4;
pub const __socket_type_SOCK_SEQPACKET: __socket_type = 5;
pub const __socket_type_SOCK_DCCP: __socket_type = 6;
pub const __socket_type_SOCK_PACKET: __socket_type = 10;
pub const __socket_type_SOCK_CLOEXEC: __socket_type = 524288;
pub const __socket_type_SOCK_NONBLOCK: __socket_type = 2048;
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
pub struct rpcent {
    pub r_name: *mut libc::c_char,
    pub r_aliases: *mut *mut libc::c_char,
    pub r_number: i32,
}
impl Clone for rpcent {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rpcent {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct netent {
    pub n_name: *mut libc::c_char,
    pub n_aliases: *mut *mut libc::c_char,
    pub n_addrtype: i32,
    pub n_net: u32,
}
impl Clone for netent {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for netent {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct hostent {
    pub h_name: *mut libc::c_char,
    pub h_aliases: *mut *mut libc::c_char,
    pub h_addrtype: i32,
    pub h_length: i32,
    pub h_addr_list: *mut *mut libc::c_char,
}
impl Clone for hostent {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hostent {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct servent {
    pub s_name: *mut libc::c_char,
    pub s_aliases: *mut *mut libc::c_char,
    pub s_port: i32,
    pub s_proto: *mut libc::c_char,
}
impl Clone for servent {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for servent {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct protoent {
    pub p_name: *mut libc::c_char,
    pub p_aliases: *mut *mut libc::c_char,
    pub p_proto: i32,
}
impl Clone for protoent {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for protoent {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct addrinfo {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: u32,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut libc::c_char,
    pub ai_next: *mut addrinfo,
}
impl Clone for addrinfo {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for addrinfo {
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
pub unsafe fn test_ipv4_literal_2() {
    let mut hints: addrinfo = unsafe { std::mem::zeroed::<addrinfo>() };
    hints.ai_flags = 0;
    hints.ai_protocol = 0;
    hints.ai_addrlen = 0_u32;
    hints.ai_addr = std::ptr::null_mut();
    hints.ai_canonname = std::ptr::null_mut();
    hints.ai_next = std::ptr::null_mut();
    hints.ai_family = 2;
    hints.ai_socktype = (__socket_type_SOCK_STREAM as i32);
    let mut res: *mut addrinfo = std::ptr::null_mut();
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                getaddrinfo_3(
                    (c"127.0.0.1".as_ptr().cast_mut()).cast_const(),
                    (c"8080".as_ptr().cast_mut()).cast_const(),
                    (&mut hints as *mut addrinfo).cast_const(),
                    (&mut res as *mut *mut addrinfo),
                )
            }) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"getaddrinfo(\"127.0.0.1\", \"8080\", &hints, &res) == 0"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"netdb.c".as_ptr().cast_mut()).cast_const(),
                        19_u32,
                        c"void test_ipv4_literal(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((!((res).is_null())) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"res != NULL".as_ptr().cast_mut()).cast_const(),
                        (c"netdb.c".as_ptr().cast_mut()).cast_const(),
                        20_u32,
                        c"void test_ipv4_literal(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*res).ai_family) == (2)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"res->ai_family == AF_INET".as_ptr().cast_mut()).cast_const(),
                        (c"netdb.c".as_ptr().cast_mut()).cast_const(),
                        21_u32,
                        c"void test_ipv4_literal(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*res).ai_socktype) == (__socket_type_SOCK_STREAM as i32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"res->ai_socktype == SOCK_STREAM".as_ptr().cast_mut()).cast_const(),
                        (c"netdb.c".as_ptr().cast_mut()).cast_const(),
                        22_u32,
                        c"void test_ipv4_literal(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*res).ai_addrlen as usize) == (::std::mem::size_of::<sockaddr_in>())) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"res->ai_addrlen == sizeof(struct sockaddr_in)"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"netdb.c".as_ptr().cast_mut()).cast_const(),
                        23_u32,
                        c"void test_ipv4_literal(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((!(((*res).ai_addr).is_null())) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"res->ai_addr != NULL".as_ptr().cast_mut()).cast_const(),
                        (c"netdb.c".as_ptr().cast_mut()).cast_const(),
                        24_u32,
                        c"void test_ipv4_literal(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut sin: *mut sockaddr_in = ((*res).ai_addr as *mut sockaddr_in);
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*sin).sin_family as i32) == (2)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"sin->sin_family == AF_INET".as_ptr().cast_mut()).cast_const(),
                        (c"netdb.c".as_ptr().cast_mut()).cast_const(),
                        26_u32,
                        c"void test_ipv4_literal(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut port_be: [u8; 2] = [(((8080) / (256)) as u8), (((8080) % (256)) as u8)];
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                memcmp_5(
                    ((&mut (*sin).sin_port as *mut u16) as *const ::libc::c_void),
                    (port_be.as_mut_ptr() as *const ::libc::c_void),
                    2_usize,
                )
            }) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"memcmp(&sin->sin_port, port_be, 2) == 0"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"netdb.c".as_ptr().cast_mut()).cast_const(),
                        28_u32,
                        c"void test_ipv4_literal(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut addr_be: [u8; 4] = [127_u8, 0_u8, 0_u8, 1_u8];
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                memcmp_5(
                    ((&mut (*sin).sin_addr as *mut in_addr) as *const ::libc::c_void),
                    (addr_be.as_mut_ptr() as *const ::libc::c_void),
                    4_usize,
                )
            }) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"memcmp(&sin->sin_addr, addr_be, 4) == 0"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"netdb.c".as_ptr().cast_mut()).cast_const(),
                        30_u32,
                        c"void test_ipv4_literal(void)".as_ptr(),
                    )
                });
            }
        }
    };
    (unsafe { freeaddrinfo_6(res) });
}
pub unsafe fn test_ipv6_literal_7() {
    let mut hints: addrinfo = unsafe { std::mem::zeroed::<addrinfo>() };
    hints.ai_flags = 0;
    hints.ai_protocol = 0;
    hints.ai_addrlen = 0_u32;
    hints.ai_addr = std::ptr::null_mut();
    hints.ai_canonname = std::ptr::null_mut();
    hints.ai_next = std::ptr::null_mut();
    hints.ai_family = 10;
    hints.ai_socktype = (__socket_type_SOCK_STREAM as i32);
    let mut res: *mut addrinfo = std::ptr::null_mut();
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                getaddrinfo_3(
                    (c"::1".as_ptr().cast_mut()).cast_const(),
                    (c"443".as_ptr().cast_mut()).cast_const(),
                    (&mut hints as *mut addrinfo).cast_const(),
                    (&mut res as *mut *mut addrinfo),
                )
            }) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"getaddrinfo(\"::1\", \"443\", &hints, &res) == 0"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"netdb.c".as_ptr().cast_mut()).cast_const(),
                        45_u32,
                        c"void test_ipv6_literal(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((!((res).is_null())) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"res != NULL".as_ptr().cast_mut()).cast_const(),
                        (c"netdb.c".as_ptr().cast_mut()).cast_const(),
                        46_u32,
                        c"void test_ipv6_literal(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*res).ai_family) == (10)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"res->ai_family == AF_INET6".as_ptr().cast_mut()).cast_const(),
                        (c"netdb.c".as_ptr().cast_mut()).cast_const(),
                        47_u32,
                        c"void test_ipv6_literal(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*res).ai_addrlen as usize) == (::std::mem::size_of::<sockaddr_in6>())) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"res->ai_addrlen == sizeof(struct sockaddr_in6)"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"netdb.c".as_ptr().cast_mut()).cast_const(),
                        48_u32,
                        c"void test_ipv6_literal(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((!(((*res).ai_addr).is_null())) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"res->ai_addr != NULL".as_ptr().cast_mut()).cast_const(),
                        (c"netdb.c".as_ptr().cast_mut()).cast_const(),
                        49_u32,
                        c"void test_ipv6_literal(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut sin6: *mut sockaddr_in6 = ((*res).ai_addr as *mut sockaddr_in6);
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*sin6).sin6_family as i32) == (10)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"sin6->sin6_family == AF_INET6".as_ptr().cast_mut()).cast_const(),
                        (c"netdb.c".as_ptr().cast_mut()).cast_const(),
                        51_u32,
                        c"void test_ipv6_literal(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut port_be: [u8; 2] = [(((443) / (256)) as u8), (((443) % (256)) as u8)];
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                memcmp_5(
                    ((&mut (*sin6).sin6_port as *mut u16) as *const ::libc::c_void),
                    (port_be.as_mut_ptr() as *const ::libc::c_void),
                    2_usize,
                )
            }) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"memcmp(&sin6->sin6_port, port_be, 2) == 0"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"netdb.c".as_ptr().cast_mut()).cast_const(),
                        53_u32,
                        c"void test_ipv6_literal(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut addr_be: [u8; 16] = [
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        1_u8,
    ];
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                memcmp_5(
                    ((&mut (*sin6).sin6_addr as *mut in6_addr) as *const ::libc::c_void),
                    (addr_be.as_mut_ptr() as *const ::libc::c_void),
                    16_usize,
                )
            }) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"memcmp(&sin6->sin6_addr, addr_be, 16) == 0"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"netdb.c".as_ptr().cast_mut()).cast_const(),
                        55_u32,
                        c"void test_ipv6_literal(void)".as_ptr(),
                    )
                });
            }
        }
    };
    (unsafe { freeaddrinfo_6(res) });
}
pub unsafe fn test_null_hints_8() {
    let mut res: *mut addrinfo = std::ptr::null_mut();
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                getaddrinfo_3(
                    (c"127.0.0.1".as_ptr().cast_mut()).cast_const(),
                    (c"80".as_ptr().cast_mut()).cast_const(),
                    std::ptr::null(),
                    (&mut res as *mut *mut addrinfo),
                )
            }) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"getaddrinfo(\"127.0.0.1\", \"80\", NULL, &res) == 0"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"netdb.c".as_ptr().cast_mut()).cast_const(),
                        61_u32,
                        c"void test_null_hints(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((!((res).is_null())) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"res != NULL".as_ptr().cast_mut()).cast_const(),
                        (c"netdb.c".as_ptr().cast_mut()).cast_const(),
                        62_u32,
                        c"void test_null_hints(void)".as_ptr(),
                    )
                });
            }
        }
    };
    {
        &(::std::mem::size_of::<i32>());
        {
            if (((((*res).ai_family) == (2)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"res->ai_family == AF_INET".as_ptr().cast_mut()).cast_const(),
                        (c"netdb.c".as_ptr().cast_mut()).cast_const(),
                        63_u32,
                        c"void test_null_hints(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut sin: *mut sockaddr_in = ((*res).ai_addr as *mut sockaddr_in);
    let mut addr_be: [u8; 4] = [127_u8, 0_u8, 0_u8, 1_u8];
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                memcmp_5(
                    ((&mut (*sin).sin_addr as *mut in_addr) as *const ::libc::c_void),
                    (addr_be.as_mut_ptr() as *const ::libc::c_void),
                    4_usize,
                )
            }) == (0)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"memcmp(&sin->sin_addr, addr_be, 4) == 0"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"netdb.c".as_ptr().cast_mut()).cast_const(),
                        66_u32,
                        c"void test_null_hints(void)".as_ptr(),
                    )
                });
            }
        }
    };
    (unsafe { freeaddrinfo_6(res) });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_ipv4_literal_2() });
    (unsafe { test_ipv6_literal_7() });
    (unsafe { test_null_hints_8() });
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct __locale_data;
pub unsafe fn getaddrinfo_3(
    mut __name: *const libc::c_char,
    mut __service: *const libc::c_char,
    mut __req: *const addrinfo,
    mut __pai: *mut *mut addrinfo,
) -> i32 {
    unimplemented!()
}
pub unsafe fn __assert_fail_4(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn memcmp_5(
    mut __s1: *const ::libc::c_void,
    mut __s2: *const ::libc::c_void,
    mut __n: usize,
) -> i32 {
    unimplemented!()
}
pub unsafe fn freeaddrinfo_6(mut __ai: *mut addrinfo) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
