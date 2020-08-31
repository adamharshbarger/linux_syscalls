//Macro for calling ASM! to invoke the System Calls
#[macro_export]
macro_rules! syscall {
    ($nr:expr) => {
        unsafe { ::platform::syscall0($nr as usize) }
    };

    ($nr:expr, $a1:expr) => {
        unsafe { ::platform::syscall::syscall1($nr as usize, $a1 as usize) }
    };

    ($nr:expr, $a1:expr, $a2:expr) => {
        unsafe { ::platform::syscall::syscall2($nr as usize, $a1 as usize, $a2 as usize) }
    };

    ($nr:expr, $a1:expr, $a2:expr, $a3:expr) => {
        unsafe {
            ::platform::syscall::syscall3($nr as usize, $a1 as usize, $a2 as usize, $a3 as usize)
        }
    };

    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => {
        unsafe {
            ::platform::syscall::syscall4(
                $nr as usize,
                $a1 as usize,
                $a2 as usize,
                $a3 as usize,
                $a4 as usize,
            )
        }
    };

    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => {
        unsafe {
            ::platform::syscall::syscall5(
                $nr as usize,
                $a1 as usize,
                $a2 as usize,
                $a3 as usize,
                $a4 as usize,
                $a5 as usize,
            )
        }
    };

    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        unsafe {
            ::platform::syscall::syscall6(
                $nr as usize,
                $a1 as usize,
                $a2 as usize,
                $a3 as usize,
                $a4 as usize,
                $a5 as usize,
                $a6 as usize,
            )
        }
    };
}
