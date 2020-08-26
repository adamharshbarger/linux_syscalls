#![allow(non_snake_case)]

pub mod types;
pub mod util;

//System Calls return a Result<>. OK will return the value returned by the sytem call except
//in cases where the system call returns something more complex (ie Brk() returns an address)
//In those cases I have hard-coded a return value of 1. Err is returning a genaric error string.
//It will be up to the user to probe errno for further details on the error type._SYSCTL()

//ToDo: 
//  Finishing writing System Call functions
//  Verify correct selection of argument types
//  Refactor and Extract reusable code into new modules
//  Verify Return Logic & Types

//#region READ System Call
pub fn READ(fd: usize, buf: &[u8], count: usize) -> Result<isize, &'static str> {
    let ret: isize;
    unsafe {
        asm!("syscall",
          in("rax") 0,
          in("rdi") fd,
          in("rsi") buf.as_ptr(),
          in("rdx") count,
          out("rcx") _,
          out("r11") _,
          lateout("rax") ret,
        );
    }
    util::Syscall_Return(ret)
}
//#endregion

//#region WRITE System Call
pub fn WRITE(fd: usize, buf: &[u8], count: usize) -> Result<isize, &'static str> {
    let ret: isize;
    unsafe {
        asm!("syscall",
          in("rax") 1,
          in("rdi") fd,
          in("rsi") buf.as_ptr(),
          in("rdx") count,
          out("rcx") _,
          out("r11") _,
          lateout("rax") ret,
        );
    }
    util::Syscall_Return(ret)
}
//#endregion

//#region OPEN System Call
pub fn OPEN(filename: &str, flags: usize, mode: usize) -> Result<isize, &'static str> {
    let ret: isize;
    if mode != 0
    {
        unsafe {
            asm!("syscall",
              in("rax") 2,
              in("rdi") filename.as_ptr(),
              in("rsi") flags,
              in("rdx") mode,
              out("rcx") _,
              out("r11") _,
              lateout("rax") ret,
            );
        }
        return util::Syscall_Return(ret);
    }
    unsafe {
        asm!("syscall",
          in("rax") 1,
          in("rdi") filename.as_ptr(),
          in("rsi") flags,
          out("rcx") _,
          out("r11") _,
          lateout("rax") ret,
        );
    }
    util::Syscall_Return(ret)
}
//#endregion

//#region CLOSE System Call
pub fn CLOSE(fd: usize) -> Result<isize, &'static str> {
    let ret: isize;
    unsafe {
        asm!("syscall",
          in("rax") 3,
          in("rdi") fd,
          out("rcx") _,
          out("r11") _,
          lateout("rax") ret,
        );
    }
    util::Syscall_Return(ret)
}
//#endregion

//#region STAT System Call
pub fn STAT(filename: &str, statbuf: &types::Stat ) -> Result<isize, &'static str> {
    let ret: isize;
    unsafe {
        asm!("syscall",
          in("rax") 4,
          in("rdi") filename.as_ptr(),
          in("rsi") statbuf as *const _,
          out("rcx") _,
          out("r11") _,
          lateout("rax") ret,
        );
    }
    util::Syscall_Return(ret)
}
//#endregion

//#region FSTAT System Call
pub fn FSTAT(fd: usize, statbuf: &types::Stat) -> Result<isize, &'static str> {
    let ret: isize;
    unsafe {
        asm!("syscall",
          in("rax") 5,
          in("rdi") fd,
          in("rsi") statbuf as *const _,
          out("rcx") _,
          out("r11") _,
          lateout("rax") ret,
        );
    }
    util::Syscall_Return(ret)
}
//#endregion

//#region LSTAT System Call
pub fn LSTAT(filename: &str, statbuf: &types::Stat ) -> Result<isize, &'static str> {
    let ret: isize;
    unsafe {
        asm!("syscall",
          in("rax") 6,
          in("rdi") filename.as_ptr(),
          in("rsi") statbuf as *const _,
          out("rcx") _,
          out("r11") _,
          lateout("rax") ret,
        );
    }
    util::Syscall_Return(ret)
}
//#endregion

//#region POLL System Call
pub fn POLL() {}
//#endregion

pub fn LSEEK() {}
pub fn MMAP() {}
pub fn MPROTECT() {}
pub fn MUNMAP() {}

//#region BRK System Call
pub fn BRK(end_data_segment: *const usize) -> Result<i32, &'static str>{
    let ret: *const usize;
    unsafe {
        asm!("syscall",
          in("rax") 12,
          in("rdi") end_data_segment,
          out("rcx") _,
          out("r11") _,
          lateout("rax") ret,
        );
    }
    if ret == end_data_segment {
        return Err("There has been an error");
    }
    Ok(1)
}
//#endregion

pub fn RT_SIGACTION() {}
pub fn RT_SIGPROCMASK() {}
pub fn RT_SIGRETURN() {}
pub fn IOCTL() {}
pub fn PREAD64() {}
pub fn PWRITE64() {}
pub fn READV() {}
pub fn WRITEV() {}
pub fn ACCESS() {}
pub fn PIPE() {}
pub fn SELECT() {}
pub fn SCHED_YIELD() {}
pub fn MREMAP() {}
pub fn MSYNC() {}
pub fn MINCORE() {}
pub fn MADVISE() {}
pub fn SHMGET() {}
pub fn SHMAT() {}
pub fn SHMCTL() {}
pub fn DUP() {}
pub fn DUP2() {}
pub fn PAUSE() {}
pub fn NANOSLEEP() {}
pub fn GETITIMER() {}
pub fn ALARM() {}
pub fn SETITIMER() {}
pub fn GETPID() {}
pub fn SENDFILE() {}
pub fn SOCKET() {}
pub fn CONNECT() {}
pub fn ACCEPT() {}
pub fn SENDTO() {}
pub fn RECVFROM() {}
pub fn SENDMSG() {}
pub fn RECVMSG() {}
pub fn SHUTDOWN() {}
pub fn BIND() {}
pub fn LISTEN() {}
pub fn GETSOCKNAME() {}
pub fn GETPEERNAME() {}
pub fn SOCKETPAIR() {}
pub fn SETSOCKOPT() {}
pub fn GETSOCKOPT() {}
pub fn CLONE() {}
pub fn FORK() {}
pub fn VFORK() {}
pub fn EXECVE() {}
pub fn EXIT() {}
pub fn WAIT4() {}
pub fn KILL() {}
pub fn UNAME() {}
pub fn SEMGET() {}
pub fn SEMOP() {}
pub fn SEMCTL() {}
pub fn SHMDT() {}
pub fn MSGGET() {}
pub fn MSGSND() {}
pub fn MSGRCV() {}
pub fn MSGCTL() {}
pub fn FCNTL() {}
pub fn FLOCK() {}
pub fn FSYNC() {}
pub fn FDATASYNC() {}
pub fn TRUNCATE() {}
pub fn FTRUNCATE() {}
pub fn GETDENTS() {}
pub fn GETCWD() {}
pub fn CHDIR() {}
pub fn FCHDIR() {}
pub fn RENAME() {}
pub fn MKDIR() {}
pub fn RMDIR() {}
pub fn CREAT() {}
pub fn LINK() {}
pub fn UNLINK() {}
pub fn SYMLINK() {}
pub fn READLINK() {}
pub fn CHMOD() {}
pub fn FCHMOD() {}
pub fn CHOWN() {}
pub fn FCHOWN() {}
pub fn LCHOWN() {}
pub fn UMASK() {}
pub fn GETTIMEOFDAY() {}
pub fn GETRLIMIT() {}
pub fn GETRUSAGE() {}
pub fn SYSINFO() {}
pub fn TIMES() {}
pub fn PTRACE() {}
pub fn GETUID() {}
pub fn SYSLOG() {}
pub fn GETGID() {}
pub fn SETUID() {}
pub fn SETGID() {}
pub fn GETEUID() {}
pub fn GETEGID() {}
pub fn SETPGID() {}
pub fn GETPPID() {}
pub fn GETPGRP() {}
pub fn SETSID() {}
pub fn SETREUID() {}
pub fn SETREGID() {}
pub fn GETGROUPS() {}
pub fn SETGROUPS() {}
pub fn SETRESUID() {}
pub fn GETRESUID() {}
pub fn SETRESGID() {}
pub fn GETRESGID() {}
pub fn GETPGID() {}
pub fn SETFSUID() {}
pub fn SETFSGID() {}
pub fn GETSID() {}
pub fn CAPGET() {}
pub fn CAPSET() {}
pub fn RT_SIGPENDING() {}
pub fn RT_SIGTIMEDWAIT() {}
pub fn RT_SIGQUEUEINFO() {}
pub fn RT_SIGSUSPEND() {}
pub fn SIGALTSTACK() {}
pub fn UTIME() {}
pub fn MKNOD() {}
pub fn USELIB() {}
pub fn PERSONALITY() {}
pub fn USTAT() {}
pub fn STATFS() {}
pub fn FSTATFS() {}
pub fn SYSFS() {}
pub fn GETPRIORITY() {}
pub fn SETPRIORITY() {}
pub fn SCHED_SETPARAM() {}
pub fn SCHED_GETPARAM() {}
pub fn SCHED_SETSCHEDULER() {}
pub fn SCHED_GETSCHEDULER() {}
pub fn SCHED_GET_PRIORITY_MAX() {}
pub fn SCHED_GET_PRIORITY_MIN() {}
pub fn SCHED_RR_GET_INTERVAL() {}
pub fn MLOCK() {}
pub fn MUNLOCK() {}
pub fn MLOCKALL() {}
pub fn MUNLOCKALL() {}
pub fn VHANGUP() {}
pub fn MODIFY_LDT() {}
pub fn PIVOT_ROOT() {}
pub fn _SYSCTL() {}
pub fn PRCTL() {}
pub fn ARCH_PRCTL() {}
pub fn ADJTIMEX() {}
pub fn SETRLIMIT() {}
pub fn CHROOT() {}
pub fn SYNC() {}
pub fn ACCT() {}
pub fn SETTIMEOFDAY() {}
pub fn MOUNT() {}
pub fn UMOUNT2() {}
pub fn SWAPON() {}
pub fn SWAPOFF() {}
pub fn REBOOT() {}
pub fn SETHOSTNAME() {}
pub fn SETDOMAINNAME() {}
pub fn IOPL() {}
pub fn IOPERM() {}
pub fn CREATE_MODULE() {}
pub fn INIT_MODULE() {}
pub fn DELETE_MODULE() {}
pub fn GET_KERNEL_SYMS() {}
pub fn QUERY_MODULE() {}
pub fn QUOTACTL() {}
pub fn NFSSERVCTL() {}
pub fn GETPMSG() {}
pub fn PUTPMSG() {}
pub fn AFS_SYSCALL() {}
pub fn TUXCALL() {}
pub fn SECURITY() {}
pub fn GETTID() {}
pub fn READAHEAD() {}
pub fn SETXATTR() {}
pub fn LSETXATTR() {}
pub fn FSETXATTR() {}
pub fn GETXATTR() {}
pub fn LGETXATTR() {}
pub fn FGETXATTR() {}
pub fn LISTXATTR() {}
pub fn LLISTXATTR() {}
pub fn FLISTXATTR() {}
pub fn REMOVEXATTR() {}
pub fn LREMOVEXATTR() {}
pub fn FREMOVEXATTR() {}
pub fn TKILL() {}
pub fn TIME() {}
pub fn FUTEX() {}
pub fn SCHED_SETAFFINITY() {}
pub fn SCHED_GETAFFINITY() {}
pub fn SET_THREAD_AREA() {}
pub fn IO_SETUP() {}
pub fn IO_DESTROY() {}
pub fn IO_GETEVENTS() {}
pub fn IO_SUBMIT() {}
pub fn IO_CANCEL() {}
pub fn GET_THREAD_AREA() {}
pub fn LOOKUP_DCOOKIE() {}
pub fn EPOLL_CREATE() {}
pub fn EPOLL_CTL_OLD() {}
pub fn EPOLL_WAIT_OLD() {}
pub fn REMAP_FILE_PAGES() {}
pub fn GETDENTS64() {}
pub fn SET_TID_ADDRESS() {}
pub fn RESTART_SYSCALL() {}
pub fn SEMTIMEDOP() {}
pub fn FADVISE64() {}
pub fn TIMER_CREATE() {}
pub fn TIMER_SETTIME() {}
pub fn TIMER_GETTIME() {}
pub fn TIMER_GETOVERRUN() {}
pub fn TIMER_DELETE() {}
pub fn CLOCK_SETTIME() {}
pub fn CLOCK_GETTIME() {}
pub fn CLOCK_GETRES() {}
pub fn CLOCK_NANOSLEEP() {}
pub fn EXIT_GROUP() {}
pub fn EPOLL_WAIT() {}
pub fn EPOLL_CTL() {}
pub fn TGKILL() {}
pub fn UTIMES() {}
pub fn VSERVER() {}
pub fn MBIND() {}
pub fn SET_MEMPOLICY() {}
pub fn GET_MEMPOLICY() {}
pub fn MQ_OPEN() {}
pub fn MQ_UNLINK() {}
pub fn MQ_TIMEDSEND() {}
pub fn MQ_TIMEDRECEIVE() {}
pub fn MQ_NOTIFY() {}
pub fn MQ_GETSETATTR() {}
pub fn KEXEC_LOAD() {}
pub fn WAITID() {}
pub fn ADD_KEY() {}
pub fn REQUEST_KEY() {}
pub fn KEYCTL() {}
pub fn IOPRIO_SET() {}
pub fn IOPRIO_GET() {}
pub fn INOTIFY_INIT() {}
pub fn INOTIFY_ADD_WATCH() {}
pub fn INOTIFY_RM_WATCH() {}
pub fn MIGRATE_PAGES() {}
pub fn OPENAT() {}
pub fn MKDIRAT() {}
pub fn MKNODAT() {}
pub fn FCHOWNAT() {}
pub fn FUTIMESAT() {}
pub fn NEWFSTATAT() {}
pub fn UNLINKAT() {}
pub fn RENAMEAT() {}
pub fn LINKAT() {}
pub fn SYMLINKAT() {}
pub fn READLINKAT() {}
pub fn FCHMODAT() {}
pub fn FACCESSAT() {}
pub fn PSELECT6() {}
pub fn PPOLL() {}
pub fn UNSHARE() {}
pub fn SET_ROBUST_LIST() {}
pub fn GET_ROBUST_LIST() {}
pub fn SPLICE() {}
pub fn TEE() {}
pub fn SYNC_FILE_RANGE() {}
pub fn VMSPLICE() {}
pub fn MOVE_PAGES() {}
pub fn UTIMENSAT() {}
pub fn EPOLL_PWAIT() {}
pub fn SIGNALFD() {}
pub fn TIMERFD_CREATE() {}
pub fn EVENTFD() {}
pub fn FALLOCATE() {}
pub fn TIMERFD_SETTIME() {}
pub fn TIMERFD_GETTIME() {}
pub fn ACCEPT4() {}
pub fn SIGNALFD4() {}
pub fn EVENTFD2() {}
pub fn EPOLL_CREATE1() {}
pub fn DUP3() {}
pub fn PIPE2() {}
pub fn INOTIFY_INIT1() {}
pub fn PREADV() {}
pub fn PWRITEV() {}
pub fn RT_TGSIGQUEUEINFO() {}
pub fn PERF_EVENT_OPEN() {}
pub fn RECVMMSG() {}
pub fn FANOTIFY_INIT() {}
pub fn FANOTIFY_MARK() {}
pub fn PRLIMIT64() {}
pub fn NAME_TO_HANDLE_AT() {}
pub fn OPEN_BY_HANDLE_AT() {}
pub fn CLOCK_ADJTIME() {}
pub fn SYNCFS() {}
pub fn SENDMMSG() {}
pub fn SETNS() {}
pub fn GETCPU() {}
pub fn PROCESS_VM_READV() {}
pub fn PROCESS_VM_WRITEV() {}
pub fn KCMP() {}
pub fn FINIT_MODULE() {}
pub fn SCHED_SETATTR() {}
pub fn SCHED_GETATTR() {}
pub fn RENAMEAT2() {}
pub fn SECCOMP() {}
pub fn GETRANDOM() {}
pub fn MEMFD_CREATE() {}
pub fn KEXEC_FILE_LOAD() {}
pub fn BPF() {}
pub fn EXECVEAT() {}
