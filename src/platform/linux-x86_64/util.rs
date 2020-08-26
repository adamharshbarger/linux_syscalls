pub fn Syscall_Return(result: isize) -> Result<isize, &'static str>
{
    if result == -1 {
        return Err("There has been an error")
    }
    Ok(result)
}