//Linux-X86_64 ABI

//All of the syscall function utilize the new ASM! syntax
//These functions are hidden to the user
#[inline(always)]
pub(crate) unsafe fn syscall0(n: usize) -> Result<isize, &'static str> {
    let ret: isize;
    asm!("syscall",
      in("rax") n,
      out("rcx") _,
      out("r11") _,
      lateout("rax") ret,
    );
    if ret < 0 && ret > -4096 {
        let err: usize = ret.checked_neg().unwrap() as usize;
        return Err(get_err_code(err));
    }
    Ok(ret)
}

#[inline(always)]
pub(crate) unsafe fn syscall1(n: usize, a1: usize) -> Result<isize, &'static str> {
    let ret: isize;
    asm!("syscall",
      in("rax") n,
      in("rdi") a1,
      out("rcx") _,
      out("r11") _,
      lateout("rax") ret,
    );
    if ret < 0 && ret > -4096 {
        let err: usize = ret.checked_neg().unwrap() as usize;
        return Err(get_err_code(err));
    }
    Ok(ret)
}

#[inline(always)]
pub(crate) unsafe fn syscall2(n: usize, a1: usize, a2: usize) -> Result<isize, &'static str> {
    let ret: isize;
    asm!("syscall",
      in("rax") n,
      in("rdi") a1,
      in("rsi") a2,
      out("rcx") _,
      out("r11") _,
      lateout("rax") ret,
    );
    if ret < 0 && ret > -4096 {
        let err: usize = ret.checked_neg().unwrap() as usize;
        return Err(get_err_code(err));
    }
    Ok(ret)
}

#[inline(always)]
pub(crate) unsafe fn syscall3(n: usize, a1: usize, a2: usize, a3: usize) -> Result<isize, &'static str> {
    let ret: isize;
    asm!("syscall",
      in("rax") n,
      in("rdi") a1,
      in("rsi") a2,
      in("rdx") a3,
      out("rcx") _,
      out("r11") _,
      lateout("rax") ret,
    );
    if ret < 0 && ret > -4096 {
        let err: usize = ret.checked_neg().unwrap() as usize;
        return Err(get_err_code(err));
    }
    Ok(ret)
}

#[inline(always)]
pub(crate) unsafe fn syscall4(n: usize, a1: usize, a2: usize, a3: usize, a4: usize) -> Result<isize, &'static str> {
    let ret: isize;
    asm!("syscall",
      in("rax") n,
      in("rdi") a1,
      in("rsi") a2,
      in("rdx") a3,
      in("r10") a4,
      out("rcx") _,
      out("r11") _,
      lateout("rax") ret,
    );
    if ret < 0 && ret > -4096 {
        let err: usize = ret.checked_neg().unwrap() as usize;
        return Err(get_err_code(err));
    }
    Ok(ret)
}

#[inline(always)]
pub(crate) unsafe fn syscall5(n: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize) -> Result<isize, &'static str> {
    let ret: isize;
    asm!("syscall",
      in("rax") n,
      in("rdi") a1,
      in("rsi") a2,
      in("rdx") a3,
      in("r10") a4,
      in("r8") a5,
      out("rcx") _,
      out("r11") _,
      lateout("rax") ret,
    );
    if ret < 0 && ret > -4096 {
        let err: usize = ret.checked_neg().unwrap() as usize;
        return Err(get_err_code(err));
    }
    Ok(ret)
}

#[inline(always)]
pub(crate) unsafe fn syscall6(n: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize, a6: usize) -> Result<isize, &'static str> {
    let ret: isize;
    asm!("syscall",
      in("rax") n,
      in("rdi") a1,
      in("rsi") a2,
      in("rdx") a3,
      in("r10") a4,
      in("r8") a5,
      in("r9") a6,
      out("rcx") _,
      out("r11") _,
      lateout("rax") ret,
    );
    if ret < 0 && ret > -4096 {
        let err: usize = ret.checked_neg().unwrap() as usize;
        return Err(get_err_code(err));
    }
    Ok(ret)
}

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
