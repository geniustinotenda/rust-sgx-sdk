// Copyright (C) 2017-2018 Baidu, Inc. All Rights Reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
//  * Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
//  * Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in
//    the documentation and/or other materials provided with the
//    distribution.
//  * Neither the name of Baidu, Inc., nor the names of its
//    contributors may be used to endorse or promote products derived
//    from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

//! This mod provides the interface connecting Rust's memory management system
//! to the Intel's SGX SDK's malloc system.
//!
//! It is a c-style interface and self-explained. Currently we don't have much
//! time for documenting it.

pub use sgx_types::{int8_t, int16_t, int32_t, int64_t, uint8_t, uint16_t, uint32_t, uint64_t};
pub use sgx_types::{c_void, c_schar, c_char, c_uchar, c_short, c_ushort, c_int, c_uint, c_float,
                    c_double, c_longlong, c_ulonglong, intmax_t, uintmax_t, c_ulong, c_long};
pub use sgx_types::{size_t, ptrdiff_t, intptr_t, uintptr_t, ssize_t};

use core::ptr;

#[link(name = "sgx_tstdc")]
extern {

    //pub fn memchr(s: * const c_void, c: c_int, n: size_t) -> *mut c_void;
    //pub fn memrchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void;
    pub fn strlen(s: * const c_char) -> size_t;
    pub fn calloc(nobj: size_t, size: size_t) -> * mut c_void;
    pub fn malloc(size: size_t) -> * mut c_void;
    pub fn realloc(p: * mut c_void, size: size_t) -> * mut c_void;
    pub fn free(p: * mut c_void);
    pub fn posix_memalign(memptr: * mut * mut c_void, align: size_t, size: size_t) -> c_int;
    pub fn malloc_usable_size(ptr: * const c_void) -> size_t;
}

pub unsafe fn memchr(s: * const u8, c: u8, n: usize) -> * const u8 {

    let mut ret = ptr::null();
    let mut p = s;

    for _ in 0..n {
        if *p == c {
            ret = p;
            break;
        }
        p = p.offset(1);
    }
    ret
}

pub unsafe fn memrchr(s: * const u8, c: u8, n: usize) -> * const u8 {

    if n == 0 {return ptr::null();}

    let mut ret = ptr::null();
    let mut p: * const u8 = (s as usize + (n - 1)) as * const u8;
    for _ in 0..n {
        if *p == c {
            ret = p;
            break;
        }
         p = p.offset(-1);
    }
    ret
}

// intentionally not public, only used for fd_set
cfg_if! {
    if #[cfg(target_pointer_width = "32")] {
        const ULONG_SIZE: usize = 32;
    } else if #[cfg(target_pointer_width = "64")] {
        const ULONG_SIZE: usize = 64;
    } else {
        // Unknown target_pointer_width
    }
}

pub type in_addr_t = u32;
pub type in_port_t = u16;
pub type sa_family_t = u16;
pub type socklen_t = u32;
pub type off64_t = i64;
pub type time_t = i64;
pub type clockid_t = i32;
pub type suseconds_t = i64;
pub type dev_t = u64;
pub type mode_t = u32;
pub type blkcnt_t = i64;
pub type blkcnt64_t = i64;
pub type blksize_t = i64;
pub type ino_t = u64;
pub type nlink_t = u64;
pub type off_t = u64;
pub type uid_t = u32;
pub type gid_t = u32;
pub type ino64_t = u64;

s! {

    pub struct stat {
        pub st_dev: dev_t,
        pub st_ino: ino_t,
        pub st_nlink: nlink_t,
        pub st_mode: mode_t,
        pub st_uid: uid_t,
        pub st_gid: gid_t,
        __pad0: c_int,
        pub st_rdev: dev_t,
        pub st_size: off_t,
        pub st_blksize: blksize_t,
        pub st_blocks: blkcnt_t,
        pub st_atime: time_t,
        pub st_atime_nsec: i64,
        pub st_mtime: time_t,
        pub st_mtime_nsec: i64,
        pub st_ctime: time_t,
        pub st_ctime_nsec: i64,
        __unused: [i64; 3],
    }

    pub struct stat64 {
        pub st_dev: dev_t,
        pub st_ino: ino64_t,
        pub st_nlink: nlink_t,
        pub st_mode: mode_t,
        pub st_uid: uid_t,
        pub st_gid: gid_t,
        __pad0: c_int,
        pub st_rdev: dev_t,
        pub st_size: off_t,
        pub st_blksize: blksize_t,
        pub st_blocks: blkcnt64_t,
        pub st_atime: time_t,
        pub st_atime_nsec: i64,
        pub st_mtime: time_t,
        pub st_mtime_nsec: i64,
        pub st_ctime: time_t,
        pub st_ctime_nsec: i64,
        __reserved: [i64; 3],
    }

    pub struct timeval {
        pub tv_sec: time_t,
        pub tv_usec: suseconds_t,
    }

    pub struct timespec {
        pub tv_sec: time_t,
        pub tv_nsec: c_long,
    }

    pub struct sockaddr {
        pub sa_family: sa_family_t,
        pub sa_data: [c_char; 14],
    }

    pub struct sockaddr_in {
        pub sin_family: sa_family_t,
        pub sin_port: in_port_t,
        pub sin_addr: in_addr,
        pub sin_zero: [u8; 8],
    }

    pub struct sockaddr_in6 {
        pub sin6_family: sa_family_t,
        pub sin6_port: in_port_t,
        pub sin6_flowinfo: u32,
        pub sin6_addr: in6_addr,
        pub sin6_scope_id: u32,
    }

    pub struct sockaddr_un {
        pub sun_family: sa_family_t,
        pub sun_path: [c_char; 108],
    }

    pub struct sockaddr_storage {
        pub ss_family: sa_family_t,
        __ss_align: size_t,
        #[cfg(target_pointer_width = "32")]
        __ss_pad2: [u8; 128 - 2 * 4],
        #[cfg(target_pointer_width = "64")]
        __ss_pad2: [u8; 128 - 2 * 8],
    }

    pub struct addrinfo {
        pub ai_flags: c_int,
        pub ai_family: c_int,
        pub ai_socktype: c_int,
        pub ai_protocol: c_int,
        pub ai_addrlen: socklen_t,

        #[cfg(any(target_os = "linux",
                  target_os = "emscripten",
                  target_os = "fuchsia"))]
        pub ai_addr: *mut sockaddr,

        pub ai_canonname: *mut c_char,

        #[cfg(target_os = "android")]
        pub ai_addr: *mut sockaddr,

        pub ai_next: *mut addrinfo,
    }

    pub struct sockaddr_nl {
        pub nl_family: sa_family_t,
        nl_pad: c_ushort,
        pub nl_pid: u32,
        pub nl_groups: u32,
    }

    pub struct sockaddr_ll {
        pub sll_family: c_ushort,
        pub sll_protocol: c_ushort,
        pub sll_ifindex: c_int,
        pub sll_hatype: c_ushort,
        pub sll_pkttype: c_uchar,
        pub sll_halen: c_uchar,
        pub sll_addr: [c_uchar; 8],
    }

    pub struct fd_set {
        fds_bits: [c_ulong; FD_SETSIZE / ULONG_SIZE],
    }

    pub struct tm {
        pub tm_sec: c_int,
        pub tm_min: c_int,
        pub tm_hour: c_int,
        pub tm_mday: c_int,
        pub tm_mon: c_int,
        pub tm_year: c_int,
        pub tm_wday: c_int,
        pub tm_yday: c_int,
        pub tm_isdst: c_int,
        pub tm_gmtoff: c_long,
        pub tm_zone: *const c_char,
    }

    #[cfg_attr(target_os = "netbsd", repr(packed))]
    pub struct in_addr {
        pub s_addr: in_addr_t,
    }

    pub struct in6_addr {
        pub s6_addr: [u8; 16],
        __align: [u32; 0],
    }

    pub struct ip_mreq {
        pub imr_multiaddr: in_addr,
        pub imr_interface: in_addr,
    }

    pub struct ipv6_mreq {
        pub ipv6mr_multiaddr: in6_addr,
        #[cfg(target_os = "android")]
        pub ipv6mr_interface: c_int,
        #[cfg(not(target_os = "android"))]
        pub ipv6mr_interface: c_uint,
    }

    pub struct hostent {
        pub h_name: *mut c_char,
        pub h_aliases: *mut *mut c_char,
        pub h_addrtype: c_int,
        pub h_length: c_int,
        pub h_addr_list: *mut *mut c_char,
    }

    pub struct iovec {
        pub iov_base: *mut c_void,
        pub iov_len: size_t,
    }

    pub struct pollfd {
        pub fd: c_int,
        pub events: c_short,
        pub revents: c_short,
    }

    pub struct winsize {
        pub ws_row: c_ushort,
        pub ws_col: c_ushort,
        pub ws_xpixel: c_ushort,
        pub ws_ypixel: c_ushort,
    }

    pub struct linger {
        pub l_onoff: c_int,
        pub l_linger: c_int,
    }

    pub struct sigval {
        // Actually a union of an int and a void*
        pub sival_ptr: *mut c_void
    }
}

pub const CLOCK_REALTIME: clockid_t = 0;
pub const CLOCK_MONOTONIC: clockid_t = 1;
pub const CLOCK_PROCESS_CPUTIME_ID: clockid_t = 2;
pub const CLOCK_THREAD_CPUTIME_ID: clockid_t = 3;
pub const CLOCK_MONOTONIC_RAW: clockid_t = 4;
pub const CLOCK_REALTIME_COARSE: clockid_t = 5;
pub const CLOCK_MONOTONIC_COARSE: clockid_t = 6;
pub const CLOCK_BOOTTIME: clockid_t = 7;
pub const CLOCK_REALTIME_ALARM: clockid_t = 8;
pub const CLOCK_BOOTTIME_ALARM: clockid_t = 9;

pub const STDIN_FILENO: c_int = 0;
pub const STDOUT_FILENO: c_int = 1;
pub const STDERR_FILENO: c_int = 2;

pub const O_APPEND: c_int = 1024;
pub const O_CREAT: c_int = 64;
pub const O_EXCL: c_int = 128;
pub const O_NOCTTY: c_int = 256;
pub const O_NONBLOCK: c_int = 2048;
pub const O_SYNC: c_int = 1_052_672;
pub const O_RSYNC: c_int = 1_052_672;
pub const O_DSYNC: c_int = 4096;
pub const O_FSYNC: c_int = 0x0010_1000;
pub const O_NOATIME: c_int = 0o1_000_000;
pub const O_PATH: c_int = 0o10_000_000;

pub const O_ASYNC: c_int = 0x2000;
pub const O_NDELAY: c_int = 0x800;

pub const EFD_NONBLOCK: c_int = 0x800;

pub const F_GETLK: c_int = 5;
pub const F_GETOWN: c_int = 9;
pub const F_SETOWN: c_int = 8;
pub const F_SETLK: c_int = 6;
pub const F_SETLKW: c_int = 7;

pub const SFD_NONBLOCK: c_int = 0x0800;

pub const SFD_CLOEXEC: c_int = 0x0008_0000;

pub const NCCS: usize = 32;

pub const O_TRUNC: c_int = 512;

pub const O_CLOEXEC: c_int = 0x80000;

pub const O_RDONLY: c_int = 0;
pub const O_WRONLY: c_int = 1;
pub const O_RDWR: c_int = 2;
pub const O_ACCMODE: c_int = 3;

pub const SOCK_CLOEXEC: c_int = O_CLOEXEC;

pub const S_IFIFO: mode_t = 4096;
pub const S_IFCHR: mode_t = 8192;
pub const S_IFBLK: mode_t = 24576;
pub const S_IFDIR: mode_t = 16384;
pub const S_IFREG: mode_t = 32768;
pub const S_IFLNK: mode_t = 40960;
pub const S_IFSOCK: mode_t = 49152;
pub const S_IFMT: mode_t = 61440;
pub const S_IRWXU: mode_t = 448;
pub const S_IXUSR: mode_t = 64;
pub const S_IWUSR: mode_t = 128;
pub const S_IRUSR: mode_t = 256;
pub const S_IRWXG: mode_t = 56;
pub const S_IXGRP: mode_t = 8;
pub const S_IWGRP: mode_t = 16;
pub const S_IRGRP: mode_t = 32;
pub const S_IRWXO: mode_t = 7;
pub const S_IXOTH: mode_t = 1;
pub const S_IWOTH: mode_t = 2;
pub const S_IROTH: mode_t = 4;
pub const F_OK: c_int = 0;
pub const R_OK: c_int = 4;
pub const W_OK: c_int = 2;
pub const X_OK: c_int = 1;
pub const SIGHUP: c_int = 1;
pub const SIGINT: c_int = 2;
pub const SIGQUIT: c_int = 3;
pub const SIGILL: c_int = 4;
pub const SIGABRT: c_int = 6;
pub const SIGFPE: c_int = 8;
pub const SIGKILL: c_int = 9;
pub const SIGSEGV: c_int = 11;
pub const SIGPIPE: c_int = 13;
pub const SIGALRM: c_int = 14;
pub const SIGTERM: c_int = 15;

pub const PROT_NONE: c_int = 0;
pub const PROT_READ: c_int = 1;
pub const PROT_WRITE: c_int = 2;
pub const PROT_EXEC: c_int = 4;

pub const SOL_SOCKET: c_int = 1;

pub const SO_REUSEADDR: c_int = 2;
pub const SO_TYPE: c_int = 3;
pub const SO_ERROR: c_int = 4;
pub const SO_DONTROUTE: c_int = 5;
pub const SO_BROADCAST: c_int = 6;
pub const SO_SNDBUF: c_int = 7;
pub const SO_RCVBUF: c_int = 8;
pub const SO_SNDBUFFORCE: c_int = 32;
pub const SO_RCVBUFFORCE: c_int = 33;
pub const SO_KEEPALIVE: c_int = 9;
pub const SO_OOBINLINE: c_int = 10;
pub const SO_NO_CHECK: c_int = 11;
pub const SO_PRIORITY: c_int = 12;
pub const SO_LINGER: c_int = 13;
pub const SO_BSDCOMPAT: c_int = 14;
pub const SO_REUSEPORT: c_int = 15;
pub const SO_PASSCRED: c_int = 16;
pub const SO_PEERCRED: c_int = 17;
pub const SO_RCVLOWAT: c_int = 18;
pub const SO_SNDLOWAT: c_int = 19;
pub const SO_RCVTIMEO: c_int = 20;
pub const SO_SNDTIMEO: c_int = 21;
pub const SO_SECURITY_AUTHENTICATION: c_int = 22;
pub const SO_SECURITY_ENCRYPTION_TRANSPORT: c_int = 23;
pub const SO_SECURITY_ENCRYPTION_NETWORK: c_int = 24;
pub const SO_BINDTODEVICE: c_int = 25;
pub const SO_ATTACH_FILTER: c_int = 26;
pub const SO_DETACH_FILTER: c_int = 27;
pub const SO_GET_FILTER: c_int = SO_ATTACH_FILTER;
pub const SO_PEERNAME: c_int = 28;
pub const SO_TIMESTAMP: c_int = 29;
pub const SO_ACCEPTCONN: c_int = 30;
pub const SO_PEERSEC: c_int = 31;
pub const SO_PASSSEC: c_int = 34;
pub const SO_TIMESTAMPNS: c_int = 35;
pub const SCM_TIMESTAMPNS: c_int = SO_TIMESTAMPNS;
pub const SO_MARK: c_int = 36;
pub const SO_TIMESTAMPING: c_int = 37;
pub const SCM_TIMESTAMPING: c_int = SO_TIMESTAMPING;
pub const SO_PROTOCOL: c_int = 38;
pub const SO_DOMAIN: c_int = 39;
pub const SO_RXQ_OVFL: c_int = 40;
pub const SO_WIFI_STATUS: c_int = 41;
pub const SCM_WIFI_STATUS: c_int = SO_WIFI_STATUS;
pub const SO_PEEK_OFF: c_int = 42;
pub const SO_NOFCS: c_int = 43;
pub const SO_LOCK_FILTER: c_int = 44;
pub const SO_SELECT_ERR_QUEUE: c_int = 45;
pub const SO_BUSY_POLL: c_int = 46;
pub const SO_MAX_PACING_RATE: c_int = 47;
pub const SO_BPF_EXTENSIONS: c_int = 48;
pub const SO_INCOMING_CPU: c_int = 49;
pub const SO_ATTACH_BPF: c_int = 50;
pub const SO_DETACH_BPF: c_int = SO_DETACH_FILTER;

pub const SOCK_NONBLOCK: c_int = O_NONBLOCK;

pub const MAP_ANON: c_int = 0x0020;
pub const MAP_ANONYMOUS: c_int = 0x0020;
pub const MAP_DENYWRITE: c_int = 0x0800;
pub const MAP_EXECUTABLE: c_int = 0x01000;
pub const MAP_POPULATE: c_int = 0x08000;
pub const MAP_NONBLOCK: c_int = 0x0001_0000;
pub const MAP_STACK: c_int = 0x0002_0000;

pub const SOCK_STREAM: c_int = 1;
pub const SOCK_DGRAM: c_int = 2;
pub const SOCK_SEQPACKET: c_int = 5;
pub const SOCK_DCCP: c_int = 6;
pub const SOCK_PACKET: c_int = 10;

pub const TCP_COOKIE_TRANSACTIONS: c_int = 15;
pub const TCP_THIN_LINEAR_TIMEOUTS: c_int = 16;
pub const TCP_THIN_DUPACK: c_int = 17;
pub const TCP_USER_TIMEOUT: c_int = 18;
pub const TCP_REPAIR: c_int = 19;
pub const TCP_REPAIR_QUEUE: c_int = 20;
pub const TCP_QUEUE_SEQ: c_int = 21;
pub const TCP_REPAIR_OPTIONS: c_int = 22;
pub const TCP_FASTOPEN: c_int = 23;
pub const TCP_TIMESTAMP: c_int = 24;

/* DCCP socket options */
pub const DCCP_SOCKOPT_PACKET_SIZE: c_int = 1;
pub const DCCP_SOCKOPT_SERVICE: c_int = 2;
pub const DCCP_SOCKOPT_CHANGE_L: c_int = 3;
pub const DCCP_SOCKOPT_CHANGE_R: c_int = 4;
pub const DCCP_SOCKOPT_GET_CUR_MPS: c_int = 5;
pub const DCCP_SOCKOPT_SERVER_TIMEWAIT: c_int = 6;
pub const DCCP_SOCKOPT_SEND_CSCOV: c_int = 10;
pub const DCCP_SOCKOPT_RECV_CSCOV: c_int = 11;
pub const DCCP_SOCKOPT_AVAILABLE_CCIDS: c_int = 12;
pub const DCCP_SOCKOPT_CCID: c_int = 13;
pub const DCCP_SOCKOPT_TX_CCID: c_int = 14;
pub const DCCP_SOCKOPT_RX_CCID: c_int = 15;
pub const DCCP_SOCKOPT_QPOLICY_ID: c_int = 16;
pub const DCCP_SOCKOPT_QPOLICY_TXQLEN: c_int = 17;
pub const DCCP_SOCKOPT_CCID_RX_INFO: c_int = 128;
pub const DCCP_SOCKOPT_CCID_TX_INFO: c_int = 192;

/// maximum number of services provided on the same listening port
pub const DCCP_SERVICE_LIST_MAX_LEN: c_int = 32;

pub const FIOCLEX: c_int = 0x5451;
pub const FIONBIO: c_int = 0x5421;

pub const F_DUPFD: c_int = 0;
pub const F_GETFD: c_int = 1;
pub const F_SETFD: c_int = 2;
pub const F_GETFL: c_int = 3;
pub const F_SETFL: c_int = 4;

// Linux-specific fcntls
pub const F_SETLEASE: c_int = 1024;
pub const F_GETLEASE: c_int = 1025;
pub const F_NOTIFY: c_int = 1026;
pub const F_CANCELLK: c_int = 1029;
pub const F_DUPFD_CLOEXEC: c_int = 1030;
pub const F_SETPIPE_SZ: c_int = 1031;
pub const F_GETPIPE_SZ: c_int = 1032;
pub const F_ADD_SEALS: c_int = 1033;
pub const F_GET_SEALS: c_int = 1034;

pub const F_SEAL_SEAL: c_int = 0x0001;
pub const F_SEAL_SHRINK: c_int = 0x0002;
pub const F_SEAL_GROW: c_int = 0x0004;

pub const EXIT_FAILURE: c_int = 1;
pub const EXIT_SUCCESS: c_int = 0;
pub const EOF: c_int = -1;
pub const SEEK_SET: c_int = 0;
pub const SEEK_CUR: c_int = 1;
pub const SEEK_END: c_int = 2;
pub const _IOFBF: c_int = 0;
pub const _IONBF: c_int = 2;
pub const _IOLBF: c_int = 1;

pub const FILENAME_MAX: c_uint = 4096;
pub const FOPEN_MAX: c_uint = 16;


pub const IFF_UP: c_int = 0x1;
pub const IFF_BROADCAST: c_int = 0x2;
pub const IFF_DEBUG: c_int = 0x4;
pub const IFF_LOOPBACK: c_int = 0x8;
pub const IFF_POINTOPOINT: c_int = 0x10;
pub const IFF_NOTRAILERS: c_int = 0x20;
pub const IFF_RUNNING: c_int = 0x40;
pub const IFF_NOARP: c_int = 0x80;
pub const IFF_PROMISC: c_int = 0x100;
pub const IFF_ALLMULTI: c_int = 0x200;
pub const IFF_MASTER: c_int = 0x400;
pub const IFF_SLAVE: c_int = 0x800;
pub const IFF_MULTICAST: c_int = 0x1000;
pub const IFF_PORTSEL: c_int = 0x2000;
pub const IFF_AUTOMEDIA: c_int = 0x4000;
pub const IFF_DYNAMIC: c_int = 0x8000;

pub const SOL_IP: c_int = 0;
pub const SOL_TCP: c_int = 6;
pub const SOL_UDP: c_int = 17;
pub const SOL_IPV6: c_int = 41;
pub const SOL_ICMPV6: c_int = 58;
pub const SOL_RAW: c_int = 255;
pub const SOL_DECNET: c_int = 261;
pub const SOL_X25: c_int = 262;
pub const SOL_PACKET: c_int = 263;
pub const SOL_ATM: c_int = 264;
pub const SOL_AAL: c_int = 265;
pub const SOL_IRDA: c_int = 266;
pub const SOL_NETBEUI: c_int = 267;
pub const SOL_LLC: c_int = 268;
pub const SOL_DCCP: c_int = 269;
pub const SOL_NETLINK: c_int = 270;
pub const SOL_TIPC: c_int = 271;

pub const AF_UNSPEC: c_int = 0;
pub const AF_UNIX: c_int = 1;
pub const AF_LOCAL: c_int = 1;
pub const AF_INET: c_int = 2;
pub const AF_AX25: c_int = 3;
pub const AF_IPX: c_int = 4;
pub const AF_APPLETALK: c_int = 5;
pub const AF_NETROM: c_int = 6;
pub const AF_BRIDGE: c_int = 7;
pub const AF_ATMPVC: c_int = 8;
pub const AF_X25: c_int = 9;
pub const AF_INET6: c_int = 10;
pub const AF_ROSE: c_int = 11;
pub const AF_DECnet: c_int = 12;
pub const AF_NETBEUI: c_int = 13;
pub const AF_SECURITY: c_int = 14;
pub const AF_KEY: c_int = 15;
pub const AF_NETLINK: c_int = 16;
pub const AF_ROUTE: c_int = AF_NETLINK;
pub const AF_PACKET: c_int = 17;
pub const AF_ASH: c_int = 18;
pub const AF_ECONET: c_int = 19;
pub const AF_ATMSVC: c_int = 20;
pub const AF_RDS: c_int = 21;
pub const AF_SNA: c_int = 22;
pub const AF_IRDA: c_int = 23;
pub const AF_PPPOX: c_int = 24;
pub const AF_WANPIPE: c_int = 25;
pub const AF_LLC: c_int = 26;
pub const AF_CAN: c_int = 29;
pub const AF_TIPC: c_int = 30;
pub const AF_BLUETOOTH: c_int = 31;
pub const AF_IUCV: c_int = 32;
pub const AF_RXRPC: c_int = 33;
pub const AF_ISDN: c_int = 34;
pub const AF_PHONET: c_int = 35;
pub const AF_IEEE802154: c_int = 36;
pub const AF_CAIF: c_int = 37;
pub const AF_ALG: c_int = 38;

pub const PF_UNSPEC: c_int = AF_UNSPEC;
pub const PF_UNIX: c_int = AF_UNIX;
pub const PF_LOCAL: c_int = AF_LOCAL;
pub const PF_INET: c_int = AF_INET;
pub const PF_AX25: c_int = AF_AX25;
pub const PF_IPX: c_int = AF_IPX;
pub const PF_APPLETALK: c_int = AF_APPLETALK;
pub const PF_NETROM: c_int = AF_NETROM;
pub const PF_BRIDGE: c_int = AF_BRIDGE;
pub const PF_ATMPVC: c_int = AF_ATMPVC;
pub const PF_X25: c_int = AF_X25;
pub const PF_INET6: c_int = AF_INET6;
pub const PF_ROSE: c_int = AF_ROSE;
pub const PF_DECnet: c_int = AF_DECnet;
pub const PF_NETBEUI: c_int = AF_NETBEUI;
pub const PF_SECURITY: c_int = AF_SECURITY;
pub const PF_KEY: c_int = AF_KEY;
pub const PF_NETLINK: c_int = AF_NETLINK;
pub const PF_ROUTE: c_int = AF_ROUTE;
pub const PF_PACKET: c_int = AF_PACKET;
pub const PF_ASH: c_int = AF_ASH;
pub const PF_ECONET: c_int = AF_ECONET;
pub const PF_ATMSVC: c_int = AF_ATMSVC;
pub const PF_RDS: c_int = AF_RDS;
pub const PF_SNA: c_int = AF_SNA;
pub const PF_IRDA: c_int = AF_IRDA;
pub const PF_PPPOX: c_int = AF_PPPOX;
pub const PF_WANPIPE: c_int = AF_WANPIPE;
pub const PF_LLC: c_int = AF_LLC;
pub const PF_CAN: c_int = AF_CAN;
pub const PF_TIPC: c_int = AF_TIPC;
pub const PF_BLUETOOTH: c_int = AF_BLUETOOTH;
pub const PF_IUCV: c_int = AF_IUCV;
pub const PF_RXRPC: c_int = AF_RXRPC;
pub const PF_ISDN: c_int = AF_ISDN;
pub const PF_PHONET: c_int = AF_PHONET;
pub const PF_IEEE802154: c_int = AF_IEEE802154;
pub const PF_CAIF: c_int = AF_CAIF;
pub const PF_ALG: c_int = AF_ALG;

pub const SOMAXCONN: c_int = 128;

pub const MSG_OOB: c_int = 1;
pub const MSG_PEEK: c_int = 2;
pub const MSG_DONTROUTE: c_int = 4;
pub const MSG_CTRUNC: c_int = 8;
pub const MSG_TRUNC: c_int = 0x20;
pub const MSG_DONTWAIT: c_int = 0x40;
pub const MSG_EOR: c_int = 0x80;
pub const MSG_WAITALL: c_int = 0x100;
pub const MSG_FIN: c_int = 0x200;
pub const MSG_SYN: c_int = 0x400;
pub const MSG_CONFIRM: c_int = 0x800;
pub const MSG_RST: c_int = 0x1000;
pub const MSG_ERRQUEUE: c_int = 0x2000;
pub const MSG_NOSIGNAL: c_int = 0x4000;
pub const MSG_MORE: c_int = 0x8000;
pub const MSG_WAITFORONE: c_int = 0x10000;
pub const MSG_FASTOPEN: c_int = 0x2000_0000;
pub const MSG_CMSG_CLOEXEC: c_int = 0x4000_0000;

pub const SCM_TIMESTAMP: c_int = SO_TIMESTAMP;

pub const SOCK_RAW: c_int = 3;
pub const SOCK_RDM: c_int = 4;
pub const IP_MULTICAST_IF: c_int = 32;
pub const IP_MULTICAST_TTL: c_int = 33;
pub const IP_MULTICAST_LOOP: c_int = 34;
pub const IP_TTL: c_int = 2;
pub const IP_HDRINCL: c_int = 3;
pub const IP_ADD_MEMBERSHIP: c_int = 35;
pub const IP_DROP_MEMBERSHIP: c_int = 36;
pub const IP_TRANSPARENT: c_int = 19;
pub const IPV6_ADD_MEMBERSHIP: c_int = 20;
pub const IPV6_DROP_MEMBERSHIP: c_int = 21;

pub const TCP_NODELAY: c_int = 1;
pub const TCP_MAXSEG: c_int = 2;
pub const TCP_CORK: c_int = 3;
pub const TCP_KEEPIDLE: c_int = 4;
pub const TCP_KEEPINTVL: c_int = 5;
pub const TCP_KEEPCNT: c_int = 6;
pub const TCP_SYNCNT: c_int = 7;
pub const TCP_LINGER2: c_int = 8;
pub const TCP_DEFER_ACCEPT: c_int = 9;
pub const TCP_WINDOW_CLAMP: c_int = 10;
pub const TCP_INFO: c_int = 11;
pub const TCP_QUICKACK: c_int = 12;
pub const TCP_CONGESTION: c_int = 13;

pub const IPV6_MULTICAST_LOOP: c_int = 19;
pub const IPV6_V6ONLY: c_int = 26;

pub const SO_DEBUG: c_int = 1;

pub const SHUT_RD: c_int = 0;
pub const SHUT_WR: c_int = 1;
pub const SHUT_RDWR: c_int = 2;

pub const LOCK_SH: c_int = 1;
pub const LOCK_EX: c_int = 2;
pub const LOCK_NB: c_int = 4;
pub const LOCK_UN: c_int = 8;

pub const SS_ONSTACK: c_int = 1;
pub const SS_DISABLE: c_int = 2;

pub const PATH_MAX: c_int = 4096;

pub const FD_SETSIZE: usize = 1024;

pub const EPOLLIN: c_int = 0x1;
pub const EPOLLPRI: c_int = 0x2;
pub const EPOLLOUT: c_int = 0x4;
pub const EPOLLRDNORM: c_int = 0x40;
pub const EPOLLRDBAND: c_int = 0x80;
pub const EPOLLWRNORM: c_int = 0x100;
pub const EPOLLWRBAND: c_int = 0x200;
pub const EPOLLMSG: c_int = 0x400;
pub const EPOLLERR: c_int = 0x8;
pub const EPOLLHUP: c_int = 0x10;
pub const EPOLLET: c_int = 0x8000_0000;

pub const EPOLL_CTL_ADD: c_int = 1;
pub const EPOLL_CTL_MOD: c_int = 3;
pub const EPOLL_CTL_DEL: c_int = 2;

pub const EAI_BADFLAGS: c_int = -1;
pub const EAI_NONAME: c_int = -2;
pub const EAI_AGAIN: c_int = -3;
pub const EAI_FAIL: c_int = -4;
pub const EAI_FAMILY: c_int = -6;
pub const EAI_SOCKTYPE: c_int = -7;
pub const EAI_SERVICE: c_int = -8;
pub const EAI_MEMORY: c_int = -10;
pub const EAI_OVERFLOW: c_int = -12;

pub const NI_NUMERICHOST: c_int = 1;
pub const NI_NUMERICSERV: c_int = 2;
pub const NI_NOFQDN: c_int = 4;
pub const NI_NAMEREQD: c_int = 8;
pub const NI_DGRAM: c_int = 16;

pub const SYNC_FILE_RANGE_WAIT_BEFORE: c_uint = 1;
pub const SYNC_FILE_RANGE_WRITE: c_uint = 2;
pub const SYNC_FILE_RANGE_WAIT_AFTER: c_uint = 4;

pub const EAI_SYSTEM: c_int = -11;

pub const AIO_CANCELED: c_int = 0;
pub const AIO_NOTCANCELED: c_int = 1;
pub const AIO_ALLDONE: c_int = 2;
pub const LIO_READ: c_int = 0;
pub const LIO_WRITE: c_int = 1;
pub const LIO_NOP: c_int = 2;
pub const LIO_WAIT: c_int = 0;
pub const LIO_NOWAIT: c_int = 1;

// netinet/in.h
// NOTE: These are in addition to the constants defined in src/unix/mod.rs

pub const IPPROTO_ICMP: c_int = 1;
pub const IPPROTO_ICMPV6: c_int = 58;
pub const IPPROTO_TCP: c_int = 6;
pub const IPPROTO_UDP: c_int = 17;
pub const IPPROTO_IP: c_int = 0;
pub const IPPROTO_IPV6: c_int = 41;

// IPPROTO_IP defined in src/unix/mod.rs
/// Hop-by-hop option header
pub const IPPROTO_HOPOPTS: c_int = 0;
// IPPROTO_ICMP defined in src/unix/mod.rs
/// group mgmt protocol
pub const IPPROTO_IGMP: c_int = 2;
/// for compatibility
pub const IPPROTO_IPIP: c_int = 4;
// IPPROTO_TCP defined in src/unix/mod.rs
/// exterior gateway protocol
pub const IPPROTO_EGP: c_int = 8;
/// pup
pub const IPPROTO_PUP: c_int = 12;
// IPPROTO_UDP defined in src/unix/mod.rs
/// xns idp
pub const IPPROTO_IDP: c_int = 22;
/// tp-4 w/ class negotiation
pub const IPPROTO_TP: c_int = 29;
/// DCCP
pub const IPPROTO_DCCP: c_int = 33;
// IPPROTO_IPV6 defined in src/unix/mod.rs
/// IP6 routing header
pub const IPPROTO_ROUTING: c_int = 43;
/// IP6 fragmentation header
pub const IPPROTO_FRAGMENT: c_int = 44;
/// resource reservation
pub const IPPROTO_RSVP: c_int = 46;
/// General Routing Encap.
pub const IPPROTO_GRE: c_int = 47;
/// IP6 Encap Sec. Payload
pub const IPPROTO_ESP: c_int = 50;
/// IP6 Auth Header
pub const IPPROTO_AH: c_int = 51;
// IPPROTO_ICMPV6 defined in src/unix/mod.rs
/// IP6 no next header
pub const IPPROTO_NONE: c_int = 59;
/// IP6 destination option
pub const IPPROTO_DSTOPTS: c_int = 60;
pub const IPPROTO_MTP: c_int = 92;
pub const IPPROTO_BEETPH: c_int = 94;
/// encapsulation header
pub const IPPROTO_ENCAP: c_int = 98;
/// Protocol indep. multicast
pub const IPPROTO_PIM: c_int = 103;
/// IP Payload Comp. Protocol
pub const IPPROTO_COMP: c_int = 108;
/// SCTP
pub const IPPROTO_SCTP: c_int = 132;
pub const IPPROTO_MH: c_int = 135;
pub const IPPROTO_UDPLITE: c_int = 136;
pub const IPPROTO_MPLS: c_int = 137;
/// raw IP packet
pub const IPPROTO_RAW: c_int = 255;
pub const IPPROTO_MAX: c_int = 256;

pub const EPERM: int32_t              = 1;
pub const ENOENT: int32_t             = 2;
pub const ESRCH: int32_t              = 3;
pub const EINTR: int32_t              = 4;
pub const EIO: int32_t                = 5;
pub const ENXIO: int32_t              = 6;
pub const E2BIG: int32_t              = 7;
pub const ENOEXEC: int32_t            = 8;
pub const EBADF: int32_t              = 9;
pub const ECHILD: int32_t             = 10;
pub const EAGAIN: int32_t             = 11;
pub const ENOMEM: int32_t             = 12;
pub const EACCES: int32_t             = 13;
pub const EFAULT: int32_t             = 14;
pub const ENOTBLK: int32_t            = 15;
pub const EBUSY: int32_t              = 16;
pub const EEXIST: int32_t             = 17;
pub const EXDEV: int32_t              = 18;
pub const ENODEV: int32_t             = 19;
pub const ENOTDIR: int32_t            = 20;
pub const EISDIR: int32_t             = 21;
pub const EINVAL: int32_t             = 22;
pub const ENFILE: int32_t             = 23;
pub const EMFILE: int32_t             = 24;
pub const ENOTTY: int32_t             = 25;
pub const ETXTBSY: int32_t            = 26;
pub const EFBIG: int32_t              = 27;
pub const ENOSPC: int32_t             = 28;
pub const ESPIPE: int32_t             = 29;
pub const EROFS: int32_t              = 30;
pub const EMLINK: int32_t             = 31;
pub const EPIPE: int32_t              = 32;
pub const EDOM: int32_t               = 33;
pub const ERANGE: int32_t             = 34;
pub const EDEADLK: int32_t            = 35;
pub const ENAMETOOLONG: int32_t       = 36;
pub const ENOLCK: int32_t             = 37;
pub const ENOSYS: int32_t             = 38;
pub const ENOTEMPTY: int32_t          = 39;
pub const ELOOP: int32_t              = 40;
pub const EWOULDBLOCK: int32_t        = EAGAIN;
pub const ENOMSG: int32_t             = 42;
pub const EIDRM: int32_t              = 43;
pub const ECHRNG: int32_t             = 44;
pub const EL2NSYNC: int32_t           = 45;
pub const EL3HLT: int32_t             = 46;
pub const EL3RST: int32_t             = 47;
pub const ELNRNG: int32_t             = 48;
pub const EUNATCH: int32_t            = 49;
pub const ENOCSI: int32_t             = 50;
pub const EL2HLT: int32_t             = 51;
pub const EBADE: int32_t              = 52;
pub const EBADR: int32_t              = 53;
pub const EXFULL: int32_t             = 54;
pub const ENOANO: int32_t             = 55;
pub const EBADRQC: int32_t            = 56;
pub const EBADSLT: int32_t            = 57;
pub const EDEADLOCK: int32_t          = EDEADLK;
pub const EBFONT: int32_t             = 59;
pub const ENOSTR: int32_t             = 60;
pub const ENODATA: int32_t            = 61;
pub const ETIME: int32_t              = 62;
pub const ENOSR: int32_t              = 63;
pub const ENONET: int32_t             = 64;
pub const ENOPKG: int32_t             = 65;
pub const EREMOTE: int32_t            = 66;
pub const ENOLINK: int32_t            = 67;
pub const EADV: int32_t               = 68;
pub const ESRMNT: int32_t             = 69;
pub const ECOMM: int32_t              = 70;
pub const EPROTO: int32_t             = 71;
pub const EMULTIHOP: int32_t          = 72;
pub const EDOTDOT: int32_t            = 73;
pub const EBADMSG: int32_t            = 74;
pub const EOVERFLOW: int32_t          = 75;
pub const ENOTUNIQ: int32_t           = 76;
pub const EBADFD: int32_t             = 77;
pub const EREMCHG: int32_t            = 78;
pub const ELIBACC: int32_t            = 79;
pub const ELIBBAD: int32_t            = 80;
pub const ELIBSCN: int32_t            = 81;
pub const ELIBMAX: int32_t            = 82;
pub const ELIBEXEC: int32_t           = 83;
pub const EILSEQ: int32_t             = 84;
pub const ERESTART: int32_t           = 85;
pub const ESTRPIPE: int32_t           = 86;
pub const EUSERS: int32_t             = 87;
pub const ENOTSOCK: int32_t           = 88;
pub const EDESTADDRREQ: int32_t       = 89;
pub const EMSGSIZE: int32_t           = 90;
pub const EPROTOTYPE: int32_t         = 91;
pub const ENOPROTOOPT: int32_t        = 92;
pub const EPROTONOSUPPORT: int32_t    = 93;
pub const ESOCKTNOSUPPORT: int32_t    = 94;
pub const EOPNOTSUPP: int32_t         = 95;
pub const EPFNOSUPPORT: int32_t       = 96;
pub const EAFNOSUPPORT: int32_t       = 97;
pub const EADDRINUSE: int32_t         = 98;
pub const EADDRNOTAVAIL: int32_t      = 99;
pub const ENETDOWN: int32_t           = 100;
pub const ENETUNREACH: int32_t        = 101;
pub const ENETRESET: int32_t          = 102;
pub const ECONNABORTED: int32_t       = 103;
pub const ECONNRESET: int32_t         = 104;
pub const ENOBUFS: int32_t            = 105;
pub const EISCONN: int32_t            = 106;
pub const ENOTCONN: int32_t           = 107;
pub const ESHUTDOWN: int32_t          = 108;
pub const ETOOMANYREFS: int32_t       = 109;
pub const ETIMEDOUT: int32_t          = 110;
pub const ECONNREFUSED: int32_t       = 111;
pub const EHOSTDOWN: int32_t          = 112;
pub const EHOSTUNREACH: int32_t       = 113;
pub const EALREADY: int32_t           = 114;
pub const EINPROGRESS: int32_t        = 115;
pub const ESTALE: int32_t             = 116;
pub const EUCLEAN: int32_t            = 117;
pub const ENOTNAM: int32_t            = 118;
pub const ENAVAIL: int32_t            = 119;
pub const EISNAM: int32_t             = 120;
pub const EREMOTEIO: int32_t          = 121;
pub const EDQUOT: int32_t             = 122;
pub const ENOMEDIUM: int32_t          = 123;
pub const EMEDIUMTYPE: int32_t        = 124;
pub const ECANCELED: int32_t          = 125;
pub const ENOKEY: int32_t             = 126;
pub const EKEYEXPIRED: int32_t        = 127;
pub const EKEYREVOKED: int32_t        = 128;
pub const EKEYREJECTED: int32_t       = 129;
pub const EOWNERDEAD: int32_t         = 130;
pub const ENOTRECOVERABLE: int32_t    = 131;
pub const ERFKILL: int32_t            = 132;
pub const EHWPOISON: int32_t          = 133;
pub const ENOTSUP: int32_t            = EOPNOTSUPP;
pub const ESGX: int32_t               = 0x0000_FFFF;
