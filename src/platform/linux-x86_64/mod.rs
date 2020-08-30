use crate::types;

pub(super) mod syscall;

//Syscall Functions return Result<>. Ok() returns the result from the actual ASM!(rax register). 
//In most cases this will be 0 or 1. But soem (I.E. BRK() will return an address). Err() will
//return a string description of the error. 

//ToDo: 
//  Finishing writing System Call functions
//  Verify correct selection of argument types
//  Refactor and Extract reusable code into new modules 
//  Verify Return Logic & Types

//#region READ System Call #0
pub fn READ(fd: usize, buf: &[u8], count: usize) -> Result<isize, types::error> {
    return syscall!(0, fd, buf.as_ptr(), count); 
}
//#endregion

//#region WRITE System Call #1error
pub fn WRITE(fd: usize, buf: &[u8], count: usize) -> Result<isize, types::error> {
    return syscall!(1, fd, buf.as_ptr(), count)
}
//#endregion

//#region OPEN System Call #2
pub fn OPEN(filename: &str, flags: usize, mode: usize) -> Result<isize, types::error> {                 //<-Make sure this will work
    //Per man pages, flags and mode are optional. Set param to 0 when unused.
    return syscall!(2, filename.as_ptr(), flags, mode);
}
//#endregion

//#region CLOSE System Call #3
pub fn CLOSE(fd: usize) -> Result<isize, types::error> {
    return syscall!(3, fd);
}
//#endregion

//#region STAT System Call #4
pub fn STAT(filename: &str, statbuf: &types::stat ) -> Result<isize, types::error> {
    return syscall!(4, filename.as_ptr(), statbuf as *const _);
}
//#endregion

//#region FSTAT System Call #5
pub fn FSTAT(fd: usize, statbuf: &types::stat) -> Result<isize, types::error> {
    return syscall!(5, fd, statbuf as *const _);
}
//#endregion

//#region LSTAT System Call #6
pub fn LSTAT(filename: &str, statbuf: &types::stat ) -> Result<isize, types::error> {
    return syscall!(6, filename.as_ptr(), statbuf as *const _);
}
//#endregion

//#region POLL System Call #7
pub fn POLL(ufds: &[types::poll_fd], nfds: usize, timeout_msecs: usize) -> Result<isize, types::error> {
    return syscall!(7, ufds.as_ptr(), nfds, timeout_msecs);
}
//#endregion

//#region LSEEK System Call #8
pub fn LSEEK(fd: usize, offset: &types::off_t, origin: usize) -> Result<isize, types::error> {
    return syscall!(8, fd, offset as *const _, origin);
}
//#endregion

//#region MMAP System Call #9
pub fn MMAP(addr: usize, len: usize, prot: usize, flags: usize, fd: usize, off: usize) -> Result<isize, types::error> {
    return syscall!(9, addr, len, prot, flags, fd, off);
}
//#endregion

//#region MPROTECT System Call #10
pub fn MPROTECT(start: usize, len: usize, prot: usize) -> Result<isize, types::error> {
    return syscall!(10, start, len, prot);
}
//#endregion

//#region MUNMAP System Call #11
pub fn MUNMAP(addr: usize, len: usize) -> Result<isize, types::error> {
    return syscall!(11, addr, len);
}
//#endregion

//#region BRK System Call #12
// Not sure if the end_data_segment is passed correct   
pub fn BRK(end_data_segment: &usize) -> Result<isize, types::error>{            //<-Check on this***********
    return syscall!(12, end_data_segment as *const _);
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
