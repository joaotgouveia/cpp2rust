extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
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
pub struct ifaddrs {
    pub ifa_next: *mut ifaddrs,
    pub ifa_name: *mut libc::c_char,
    pub ifa_flags: u32,
    pub ifa_addr: *mut sockaddr,
    pub ifa_netmask: *mut sockaddr,
    pub ifa_ifu: anon_1,
    pub ifa_data: *mut ::libc::c_void,
}
impl Clone for ifaddrs {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ifaddrs {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct anon_1 {
    pub ifu_broadaddr: *mut sockaddr,
    pub ifu_dstaddr: *mut sockaddr,
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
pub struct if_nameindex {
    pub if_index: u32,
    pub if_name: *mut libc::c_char,
}
impl Clone for if_nameindex {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for if_nameindex {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct ifaddr {
    pub ifa_addr: sockaddr,
    pub ifa_ifu: anon_2,
    pub ifa_ifp: *mut iface,
    pub ifa_next: *mut ifaddr,
}
impl Clone for ifaddr {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ifaddr {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct anon_2 {
    pub ifu_broadaddr: sockaddr,
    pub ifu_dstaddr: sockaddr,
}
impl Clone for anon_2 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_2 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct ifmap {
    pub mem_start: u64,
    pub mem_end: u64,
    pub base_addr: u16,
    pub irq: u8,
    pub dma: u8,
    pub port: u8,
}
impl Clone for ifmap {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ifmap {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct ifreq {
    pub ifr_ifrn: anon_3,
    pub ifr_ifru: anon_4,
}
impl Clone for ifreq {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ifreq {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct anon_3 {
    pub ifrn_name: [libc::c_char; 16],
}
impl Clone for anon_3 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_3 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct anon_4 {
    pub ifru_addr: sockaddr,
    pub ifru_dstaddr: sockaddr,
    pub ifru_broadaddr: sockaddr,
    pub ifru_netmask: sockaddr,
    pub ifru_hwaddr: sockaddr,
    pub ifru_flags: i16,
    pub ifru_ivalue: i32,
    pub ifru_mtu: i32,
    pub ifru_map: ifmap,
    pub ifru_slave: [libc::c_char; 16],
    pub ifru_newname: [libc::c_char; 16],
    pub ifru_data: *mut libc::c_char,
}
impl Clone for anon_4 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_4 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct ifconf {
    pub ifc_len: i32,
    pub ifc_ifcu: anon_5,
}
impl Clone for ifconf {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ifconf {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct anon_5 {
    pub ifcu_buf: *mut libc::c_char,
    pub ifcu_req: *mut ifreq,
}
impl Clone for anon_5 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_5 {
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
    pub __in6_u: anon_6,
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
pub struct anon_6 {
    pub __u6_addr8: [u8; 16],
    pub __u6_addr16: [u16; 8],
    pub __u6_addr32: [u32; 4],
}
impl Clone for anon_6 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_6 {
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
    let mut list: *mut ifaddrs = std::ptr::null_mut();
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe { getifaddrs_7((&mut list as *mut *mut ifaddrs)) }) == (0)) as i32) != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_8(
                        (c"getifaddrs(&list) == 0".as_ptr().cast_mut()).cast_const(),
                        (c"ifaddrs.c".as_ptr().cast_mut()).cast_const(),
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
            if (((!((list).is_null())) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_8(
                        (c"list != NULL".as_ptr().cast_mut()).cast_const(),
                        (c"ifaddrs.c".as_ptr().cast_mut()).cast_const(),
                        12_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    let mut found_loopback: i32 = 0;
    let mut ifa: *mut ifaddrs = std::ptr::null_mut();
    ifa = list;
    'loop_: while (((!((ifa).is_null())) as i32) != 0) {
        {
            &(::std::mem::size_of::<i32>());
            {
                if (((!(((*ifa).ifa_name).is_null())) as i32) != 0) {
                } else {
                    (unsafe {
                        __assert_fail_8(
                            (c"ifa->ifa_name != NULL".as_ptr().cast_mut()).cast_const(),
                            (c"ifaddrs.c".as_ptr().cast_mut()).cast_const(),
                            16_u32,
                            c"int main(void)".as_ptr(),
                        )
                    });
                }
            }
        };
        if (((((*ifa).ifa_addr).is_null()) as i32) != 0) {
            ifa = (*ifa).ifa_next;
            continue 'loop_;
        }
        if (((((*(*ifa).ifa_addr).sa_family as i32) != (2)) as i32) != 0) {
            ifa = (*ifa).ifa_next;
            continue 'loop_;
        }
        let mut sin: *mut sockaddr_in = ((*ifa).ifa_addr as *mut sockaddr_in);
        let mut lo_be: [u8; 4] = [127_u8, 0_u8, 0_u8, 1_u8];
        if ((((unsafe {
            memcmp_9(
                ((&mut (*sin).sin_addr as *mut in_addr) as *const ::libc::c_void),
                (lo_be.as_mut_ptr() as *const ::libc::c_void),
                4_usize,
            )
        }) == (0)) as i32)
            != 0)
        {
            found_loopback = 1;
            {
                &(::std::mem::size_of::<i32>());
                {
                    if (((((*ifa).ifa_flags) != (0_u32)) as i32) != 0) {
                    } else {
                        (unsafe {
                            __assert_fail_8(
                                (c"ifa->ifa_flags != 0".as_ptr().cast_mut()).cast_const(),
                                (c"ifaddrs.c".as_ptr().cast_mut()).cast_const(),
                                27_u32,
                                c"int main(void)".as_ptr(),
                            )
                        });
                    }
                }
            };
            {
                &(::std::mem::size_of::<i32>());
                {
                    if (((!(((*ifa).ifa_netmask).is_null())) as i32) != 0) {
                    } else {
                        (unsafe {
                            __assert_fail_8(
                                (c"ifa->ifa_netmask != NULL".as_ptr().cast_mut()).cast_const(),
                                (c"ifaddrs.c".as_ptr().cast_mut()).cast_const(),
                                28_u32,
                                c"int main(void)".as_ptr(),
                            )
                        });
                    }
                }
            };
            let mut mask: *mut sockaddr_in = ((*ifa).ifa_netmask as *mut sockaddr_in);
            let mut mask_be: [u8; 4] = [255_u8, 0_u8, 0_u8, 0_u8];
            {
                &(::std::mem::size_of::<i32>());
                {
                    if ((((unsafe {
                        memcmp_9(
                            ((&mut (*mask).sin_addr as *mut in_addr) as *const ::libc::c_void),
                            (mask_be.as_mut_ptr() as *const ::libc::c_void),
                            4_usize,
                        )
                    }) == (0)) as i32)
                        != 0)
                    {
                    } else {
                        (unsafe {
                            __assert_fail_8(
                                (c"memcmp(&mask->sin_addr, mask_be, 4) == 0"
                                    .as_ptr()
                                    .cast_mut())
                                .cast_const(),
                                (c"ifaddrs.c".as_ptr().cast_mut()).cast_const(),
                                31_u32,
                                c"int main(void)".as_ptr(),
                            )
                        });
                    }
                }
            };
            {
                &(::std::mem::size_of::<i32>());
                {
                    if ((((unsafe { if_nametoindex_10(((*ifa).ifa_name).cast_const()) }) > (0_u32))
                        as i32)
                        != 0)
                    {
                    } else {
                        (unsafe {
                            __assert_fail_8(
                                (c"if_nametoindex(ifa->ifa_name) > 0".as_ptr().cast_mut())
                                    .cast_const(),
                                (c"ifaddrs.c".as_ptr().cast_mut()).cast_const(),
                                32_u32,
                                c"int main(void)".as_ptr(),
                            )
                        });
                    }
                }
            };
        }
        ifa = (*ifa).ifa_next;
    }
    {
        &(::std::mem::size_of::<i32>());
        {
            if (found_loopback != 0) {
            } else {
                (unsafe {
                    __assert_fail_8(
                        (c"found_loopback".as_ptr().cast_mut()).cast_const(),
                        (c"ifaddrs.c".as_ptr().cast_mut()).cast_const(),
                        35_u32,
                        c"int main(void)".as_ptr(),
                    )
                });
            }
        }
    };
    (unsafe { freeifaddrs_11(list) });
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((unsafe {
                if_nametoindex_10((c"cpp2rust_no_such_if".as_ptr().cast_mut()).cast_const())
            }) == (0_u32)) as i32)
                != 0)
            {
            } else {
                (unsafe {
                    __assert_fail_8(
                        (c"if_nametoindex(\"cpp2rust_no_such_if\") == 0"
                            .as_ptr()
                            .cast_mut())
                        .cast_const(),
                        (c"ifaddrs.c".as_ptr().cast_mut()).cast_const(),
                        38_u32,
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
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct iface;
pub unsafe fn getifaddrs_7(mut __ifap: *mut *mut ifaddrs) -> i32 {
    unimplemented!()
}
pub unsafe fn __assert_fail_8(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn memcmp_9(
    mut __s1: *const ::libc::c_void,
    mut __s2: *const ::libc::c_void,
    mut __n: usize,
) -> i32 {
    unimplemented!()
}
pub unsafe fn if_nametoindex_10(mut __ifname: *const libc::c_char) -> u32 {
    unimplemented!()
}
pub unsafe fn freeifaddrs_11(mut __ifa: *mut ifaddrs) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
