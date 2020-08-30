//TODO:
//  Finish Building-out Structures
//  Verify and update Type definitions
//  Decide if get_err_code & get_err_msg should be a Macro

//C Type Conversions for general use
pub type c_char = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_long = i32;
pub type c_ulong = u32;
pub type c_long_long = i64;
pub type c_ulong_long = u64;
pub type c_size_t = usize;
pub type c_float = f32;
pub type c_double = f64;
// pub type c_long_double = f128; <---f128 has been deprecated in Rust
pub type c_bool = bool;
pub type c_void = ();

//C Types defined in stdint.h header
pub type c_int8_t = i8;
pub type c_uint8_t = u8;
pub type c_int16_t = i16;
pub type c_uint16_t = u16;
pub type c_int32_t = i32;
pub type c_uint32_t = u32;
pub type c_int64_t = i64;
pub type c_uint64_t = u64;

//Types used for building Structs; Keeping Linux Names to aid					<---Update to C_Types
//in referencing from the Linux man pages
pub type dev_t = i16;
pub type ino_t = u16;
pub type mode_t = u16;
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
	pub st_ino: ino_t,
	pub st_mode: mode_t,
	pub st_nlink: nlink_t,
	pub st_uid: uid_t,
	pub st_gid: gid_t,
	pub st_rdev: dev_t,
	pub st_size: off_t,
	pub st_atime: time_t,
	pub st_mtime: time_t,
	pub st_ctime: time_t,
	pub st_blksize: blksize_t,
	pub st_blocks: blkcnt_t,
	pub st_attr: mode_t,
}

//Parameter for the POLL System Call
pub struct poll_fd {
	pub fd: c_int,
	pub events: c_short,
	pub revents: c_short,
}

//Parameter for the RT_SIGACTION System Call
pub struct sigaction {} //Define & Verify

//Parameter for the READV, WRITEV System Calls
pub struct iovec {} //Define & Verify

//Parameter for the SELECT System Call
pub struct timeval {} //Define & Verify

//Error object returned from syscall#
pub struct error {
	//This will be negative
	pub errno: isize,
	//Short Acronym for the Error
	pub err_code: &'static str,
	//Human Readable Description of Error
	pub err_msg: &'static str,
}

impl error {
	//Build error object
	pub fn new(errno: isize) -> error {
		error {
			errno: errno,
			err_code: error::get_err_code(errno),
			err_msg: error::get_err_msg(errno),
		}
	}
	//Get the Human Readable Error Message
	fn get_err_msg(errno: isize) -> &'static str {
		let err_msg: [&str; 132] = [
			"Operation not permitted",
			"No such file or directory",
			"No such process",
			"Interrupted system call",
			"I/O error",
			"No such device or address",
			"Argument list too long",
			"Exec format error",
			"Bad file number",
			"No child processes",
			"Try again",
			"Out of memory",
			"Permission denied",
			"Bad address",
			"Block device required",
			"Device or resource busy",
			"File exists",
			"Cross-device link",
			"No such device",
			"Not a directory",
			"Is a directory",
			"Invalid argument",
			"File table overflow",
			"Too many open files",
			"Not a typewriter",
			"Text file busy",
			"File too large",
			"No space left on device",
			"Illegal seek",
			"Read-only file system",
			"Too many links",
			"Broken pipe",
			"Math argument out of domain of func",
			"Math result not representable",
			"Resource deadlock would occur",
			"File name too long",
			"No record locks available",
			"Function not implemented",
			"Directory not empty",
			"Too many symbolic links encountered",
			"Operation would block",
			"No message of desired type",
			"Identifier removed",
			"Channel number out of range",
			"Level 2 not synchronized",
			"Level 3 halted",
			"Level 3 reset",
			"Link number out of range",
			"Protocol driver not attached",
			"No CSI structure available",
			"Level 2 halted",
			"Invalid exchange",
			"Invalid request descriptor",
			"Exchange full",
			"No anode",
			"Invalid request code",
			"Invalid slot",
			"EDEA",
			"Bad font file format",
			"Device not a stream",
			"No data available",
			"Timer expired",
			"Out of streams resources",
			"Machine is not on the network",
			"Package not installed",
			"Object is remote",
			"Link has been severed",
			"Advertise error",
			"Srmount error",
			"Communication error on send",
			"Protocol error",
			"Multihop attempted",
			"RFS specific error",
			"Not a data message",
			"Value too large for defined data type",
			"Name not unique on network",
			"File descriptor in bad state",
			"Remote address changed",
			"Can not access a needed shared library",
			"Accessing a corrupted shared library",
			".lib section in a.out corrupted",
			"Attempting to link in too many shared libraries",
			"Cannot exec a shared library directly",
			"Illegal byte sequence",
			"Interrupted system call should be restarted",
			"Streams pipe error",
			"Too many users",
			"Socket operation on non-socket",
			"Destination address required",
			"Message too long",
			"Protocol wrong type for socket",
			"Protocol not available",
			"Protocol not supported",
			"Socket type not supported",
			"Operation not supported on transport endpoint",
			"Protocol family not supported",
			"Address family not supported by protocol",
			"Address already in use",
			"Cannot assign requested address",
			"Network is down",
			"Network is unreachable",
			"Network dropped connection because of reset",
			"Software caused connection abort",
			"Connection reset by peer",
			"No buffer space available",
			"Transport endpoint is already connected",
			"Transport endpoint is not connected",
			"Cannot send after transport endpoint shutdown",
			"Too many references: cannot splice",
			"Connection timed out",
			"Connection refused",
			"Host is down",
			"No route to host",
			"Operation already in progress",
			"Operation now in progress",
			"Stale NFS file handle",
			"Structure needs cleaning",
			"Not a XENIX named type file",
			"No XENIX semaphores available",
			"Is a named type file",
			"Remote I/O error",
			"Quota exceeded",
			"No medium found",
			"Wrong medium type",
			"Operation Canceled",
			"Required key not available",
			"Key has expired",
			"Key has been revoked",
			"Key was rejected by service",
			"Owner died",
			"State not recoverable",
			"Operation not possible due to RF-kill",
		];
		//Invert sign of errno for array usage
		let index: usize = errno.checked_neg().unwrap() as usize;

		//Must decrease by 1 for 0 based indexing
		return err_msg[index - 1];
	}
	//Error Acronym
	fn get_err_code(errno: isize) -> &'static str {
		let err_code: [&str; 132] = [
			"EPERM",
			"ENOENT",
			"ESRCH",
			"EINTR",
			"EIO",
			"ENXIO",
			"E2BIG",
			"ENOEXEC",
			"EBADF",
			"ECHILD",
			"EAGAIN",
			"ENOMEM",
			"EACCES",
			"EFAULT",
			"ENOTBLK",
			"EBUSY",
			"EEXIST",
			"EXDEV",
			"ENODEV",
			"ENOTDIR",
			"EISDIR",
			"EINVAL",
			"ENFILE",
			"EMFILE",
			"ENOTTY",
			"ETXTBSY",
			"EFBIG",
			"ENOSPC",
			"ESPIPE",
			"EROFS",
			"EMLINK",
			"EPIPE",
			"EDOM",
			"ERANGE",
			"EDEADLK",
			"ENAMETOOLONG",
			"ENOLCK",
			"ENOSYS",
			"ENOTEMPTY",
			"ELOOP",
			"EWOULDBLOCK",
			"ENOMSG",
			"EIDRM",
			"ECHRNG",
			"EL2NSYNC",
			"EL3HLT",
			"EL3RST",
			"ELNRNG",
			"EUNATCH",
			"ENOCSI",
			"EL2HLT",
			"EBADE",
			"EBADR",
			"EXFULL",
			"ENOANO",
			"EBADRQC",
			"EBADSLT",
			"EDEADLOCK",
			"EBFONT",
			"ENOSTR",
			"ENODATA",
			"ETIME",
			"ENOSR",
			"ENONET",
			"ENOPKG",
			"EREMOTE",
			"ENOLINK",
			"EADV",
			"ESRMNT",
			"ECOMM",
			"EPROTO",
			"EMULTIHOP",
			"EDOTDOT",
			"EBADMSG",
			"EOVERFLOW",
			"ENOTUNIQ",
			"EBADFD",
			"EREMCHG",
			"ELIBACC",
			"ELIBBAD",
			"ELIBSCN",
			"ELIBMAX",
			"ELIBEXEC",
			"EILSEQ",
			"ERESTART",
			"ESTRPIPE",
			"EUSERS",
			"ENOTSOCK",
			"EDESTADDRREQ",
			"EMSGSIZE",
			"EPROTOTYPE",
			"ENOPROTOOPT",
			"EPROTONOSUPPOR",
			"ESOCKTNOSUPPOR",
			"EOPNOTSUPP",
			"EPFNOSUPPORT",
			"EAFNOSUPPORT",
			"EADDRINUSE",
			"EADDRNOTAVAIL",
			"ENETDOWN",
			"ENETUNREACH",
			"ENETRESET",
			"ECONNABORTED",
			"ECONNRESET",
			"ENOBUFS",
			"EISCONN",
			"ENOTCONN",
			"ESHUTDOWN",
			"ETOOMANYREFS",
			"ETIMEDOUT",
			"ECONNREFUSED",
			"EHOSTDOWN",
			"EHOSTUNREACH",
			"EALREADY",
			"EINPROGRESS",
			"ESTALE",
			"EUCLEAN",
			"ENOTNAM",
			"ENAVAIL",
			"EISNAM",
			"EREMOTEIO",
			"EDQUOT",
			"ENOMEDIUM",
			"EMEDIUMTYPE",
			"ECANCELED",
			"ENOKEY",
			"EKEYEXPIRED",
			"EKEYREVOKED",
			"EKEYREJECTED",
			"OWNERDEAD",
			"ENOTRECOVERABLE",
			"ERFKILL",
		];
		//Invert sign of errno for array usage
		let index: usize = errno.checked_neg().unwrap() as usize;

		//Must decrease by 1 for 0 based indexing
		return err_code[index - 1];
	}
}
