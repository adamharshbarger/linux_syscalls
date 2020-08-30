//Linux-X86_64 ABI

use types;

//These functions are hidden to the user
#[inline(always)]
pub(crate) unsafe fn syscall0(n: usize) -> Result<isize, types::error> {
    let ret: isize;
    asm!("syscall",
      in("rax") n,
      out("rcx") _,
      out("r11") _,
      lateout("rax") ret,
    );
    //Check if Return is an error
    if ret < 0 && ret > -4096 {
      //Build an error Message
      return Err(types::error::new(ret));
    }
    Ok(ret)
}

#[inline(always)]
pub(crate) unsafe fn syscall1(n: usize, a1: usize) -> Result<isize, types::error> {
    let ret: isize;
    asm!("syscall",
      in("rax") n,
      in("rdi") a1,
      out("rcx") _,
      out("r11") _,
      lateout("rax") ret,
    );
    if ret < 0 && ret > -4096 {
        return Err(types::error::new(ret));
    }
    Ok(ret)
}

#[inline(always)]
pub(crate) unsafe fn syscall2(n: usize, a1: usize, a2: usize) -> Result<isize, types::error> {
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
        return Err(types::error::new(ret));
    }
    Ok(ret)
}

#[inline(always)]
pub(crate) unsafe fn syscall3(n: usize, a1: usize, a2: usize, a3: usize) -> Result<isize, types::error> {
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
        return Err(types::error::new(ret));
    }
    Ok(ret)
}

#[inline(always)]
pub(crate) unsafe fn syscall4(n: usize, a1: usize, a2: usize, a3: usize, a4: usize) -> Result<isize, types::error> {
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
        return Err(types::error::new(ret));
    }
    Ok(ret)
}

#[inline(always)]
pub(crate) unsafe fn syscall5(n: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize) -> Result<isize, types::error> {
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
        return Err(types::error::new(ret));
    }
    Ok(ret)
}

#[inline(always)]
pub(crate) unsafe fn syscall6(n: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize, a6: usize) -> Result<isize, types::error> {
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
        return Err(types::error::new(ret));
    }
    Ok(ret)
}


