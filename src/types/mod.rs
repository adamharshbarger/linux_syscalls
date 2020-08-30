//TODO:
//  Finish Building-out Structures
//  Verify and update Type definitions
//  Look for best way to expose types for user created functions

//C Type Conversions for general use
pub type c_char = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint	= u32;
pub type c_long	= i32;
pub type c_ulong = u32;
pub type c_long_long = i64;
pub type c_ulong_long = u64;
pub type c_size_t = usize;
pub type c_float = f32;
pub type c_double = f64;
// pub type c_long_double = f128; <---f128 has been deprecated in Rust 
pub type c_bool	= bool;
pub type c_void	= ();

//C Types defined in stdint.h header
pub type c_int8_t = i8;
pub type c_uint8_t = u8;
pub type c_int16_t = i16;
pub type c_uint16_t = u16;
pub type c_int32_t = i32;
pub type c_uint32_t = u32;
pub type c_int64_t = i64;
pub type c_uint64_t	= u64;

//Types used for building Structs; Keeping Linux Names to aid					<---Update to C_Types
//in referencing from the Linux man pages
pub type dev_t = i16;     
pub type ino_t = u16;     
pub type mode_t	= u16;   
pub type nlink_t = i8;   
pub type uid_t = i16;     
pub type gid_t = i8;      
pub type off_t = u32;     
pub type time_t = i32;   
pub type blksize_t = i32;
pub type blkcnt_t = i32; 
pub type sigset_t = usize;
pub type loff_t = usize;  
pub type fd_set = usize;  
pub type key_t = usize;   

//Parameter for the STAT, FSTAT, LSTAT System Calls
pub struct stat {
    pub st_dev: dev_t,
	pub st_ino:	ino_t,
	pub st_mode: mode_t,
	pub st_nlink: nlink_t,
	pub st_uid: uid_t,
	pub st_gid:	gid_t,
	pub st_rdev: dev_t,
	pub st_size: off_t,
	pub st_atime: time_t,
	pub st_mtime: time_t,
	pub st_ctime: time_t,
	pub st_blksize:	blksize_t,
	pub st_blocks: blkcnt_t,
	pub st_attr: mode_t,
}

//Parameter for the POLL System Call
pub struct poll_fd {
	pub fd:	c_int,
	pub events:	c_short,
	pub revents: c_short,
}     

//Parameter for the RT_SIGACTION System Call
pub struct sigaction {}     //Define & Verify

//Parameter for the READV, WRITEV System Calls
pub struct iovec {}         //Define & Verify

//Parameter for the SELECT System Call
pub struct timeval {}       //Define & Verify

fn get_err_code(err: usize) -> &'static str {
    let err_code: [&str; 132] = [
        "EPERM - Operation not permitted",
        "ENOENT - No such file or directory",
        "ESRCH - No such process",
        "EINTR - Interrupted system call",
        "EIO - I/O error",
        "ENXIO - No such device or address",
        "E2BIG - Argument list too long",
        "ENOEXEC - Exec format error",
        "EBADF - Bad file number",
        "ECHILD - No child processes",
        "EAGAIN - Try again",
        "ENOMEM - Out of memory",
        "EACCES - Permission denied",
        "EFAULT - Bad address",
        "ENOTBLK - Block device required",
        "EBUSY - Device or resource busy",
        "EEXIST - File exists",
        "EXDEV - Cross-device link",
        "ENODEV - No such device",
        "ENOTDIR - Not a directory",
        "EISDIR - Is a directory",
        "EINVAL - Invalid argument",
        "ENFILE - File table overflow",
        "EMFILE - Too many open files",
        "ENOTTY - Not a typewriter",
        "ETXTBSY - Text file busy",
        "EFBIG - File too large",
        "ENOSPC - No space left on device",
        "ESPIPE - Illegal seek",
        "EROFS - Read-only file system",
        "EMLINK - Too many links",
        "EPIPE - Broken pipe",
        "EDOM - Math argument out of domain of func",
        "ERANGE - Math result not representable",
        "EDEADLK - Resource deadlock would occur",
        "ENAMETOOLONG - File name too long",
        "ENOLCK - No record locks available",
        "ENOSYS - Function not implemented",
        "ENOTEMPTY - Directory not empty",
        "ELOOP - Too many symbolic links encountered",
        "EWOULDBLOCK - Operation would block",
        "ENOMSG - No message of desired type",
        "EIDRM - Identifier removed",
        "ECHRNG - Channel number out of range",
        "EL2NSYNC - Level 2 not synchronized",
        "EL3HLT - Level 3 halted",
        "EL3RST - Level 3 reset",
        "ELNRNG - Link number out of range",
        "EUNATCH - Protocol driver not attached",
        "ENOCSI - No CSI structure available",
        "EL2HLT - Level 2 halted",
        "EBADE - Invalid exchange",
        "EBADR - Invalid request descriptor",
        "EXFULL - Exchange full",
        "ENOANO - No anode",
        "EBADRQC -Invalid request code",
        "EBADSLT - Invalid slot",
        "EDEADLOCK - EDEA",
        "EBFONT - Bad font file format",
        "ENOSTR - Device not a stream",
        "ENODATA - No data available",
        "ETIME - Timer expired",
        "ENOSR - Out of streams resources",
        "ENONET - Machine is not on the network",
        "ENOPKG - Package not installed",
        "EREMOTE - Object is remote",
        "ENOLINK - Link has been severed",
        "EADV - Advertise error",
        "ESRMNT - Srmount error",
        "ECOMM - Communication error on send",
        "EPROTO - Protocol error",
        "EMULTIHOP - Multihop attempted",
        "EDOTDOT - RFS specific error",
        "EBADMSG - Not a data message",
        "EOVERFLOW - Value too large for defined data type",
        "ENOTUNIQ - Name not unique on network",
        "EBADFD - File descriptor in bad state",
        "EREMCHG - Remote address changed",
        "ELIBACC - Can not access a needed shared library",
        "ELIBBAD - Accessing a corrupted shared library",
        "ELIBSCN - .lib section in a.out corrupted",
        "ELIBMAX - Attempting to link in too many shared libraries",
        "ELIBEXEC - Cannot exec a shared library directly",
        "EILSEQ - Illegal byte sequence",
        "ERESTART - Interrupted system call should be restarted",
        "ESTRPIPE - Streams pipe error",
        "EUSERS - Too many users",
        "ENOTSOCK - Socket operation on non-socket",
        "EDESTADDRREQ - Destination address required",
        "EMSGSIZE - Message too long",
        "EPROTOTYPE - Protocol wrong type for socket",
        "ENOPROTOOPT - Protocol not available",
        "EPROTONOSUPPOR - Protocol not supported",
        "ESOCKTNOSUPPOR - Socket type not supported",
        "EOPNOTSUPP - Operation not supported on transport endpoint",
        "EPFNOSUPPORT - Protocol family not supported",
        "EAFNOSUPPORT - Address family not supported by protocol",
        "EADDRINUSE	- Address already in use",
        "EADDRNOTAVAIL - Cannot assign requested address",
        "ENETDOWN - Network is down",
        "ENETUNREACH - Network is unreachable",
        "ENETRESET - Network dropped connection because of reset",
        "ECONNABORTED - Software caused connection abort",
        "ECONNRESET - Connection reset by peer",
        "ENOBUFS - No buffer space available",
        "EISCONN - Transport endpoint is already connected",
        "ENOTCONN - Transport endpoint is not connected",
        "ESHUTDOWN - Cannot send after transport endpoint shutdown",
        "ETOOMANYREFS - Too many references: cannot splice",
        "ETIMEDOUT - Connection timed out",
        "ECONNREFUSED - Connection refused",
        "EHOSTDOWN - Host is down",
        "EHOSTUNREACH - No route to host",
        "EALREADY - Operation already in progress",
        "EINPROGRESS - Operation now in progress",
        "ESTALE - Stale NFS file handle",
        "EUCLEAN - Structure needs cleaning",
        "ENOTNAM - Not a XENIX named type file",
        "ENAVAIL - No XENIX semaphores available",
        "EISNAM - Is a named type file",
        "EREMOTEIO - Remote I/O error",
        "EDQUOT - Quota exceeded",
        "ENOMEDIUM - No medium found",
        "EMEDIUMTYPE - Wrong medium type",
        "ECANCELED - Operation Canceled",
        "ENOKEY - Required key not available",
        "EKEYEXPIRED - Key has expired",
        "EKEYREVOKED - Key has been revoked",
        "EKEYREJECTED - Key was rejected by service",
        "OWNERDEAD - Owner died",
        "ENOTRECOVERABLE - State not recoverable",
        "ERFKILL - Operation not possible due to RF-kill",
    ];

    return err_code[err - 1];
}

