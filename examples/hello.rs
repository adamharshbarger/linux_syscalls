
extern crate linux_syscalls;
use linux_syscalls::platform::*;

fn main() 
{
    //Example using WRITE System Call
    let string = "Hello World!\n";

    match WRITE(1, string.as_bytes(), string.len())
    {
        Ok(number) => println!("Bytes written: {}", number),
        Err(e) => println!("{}", e)
    }   
    
    //Example using READ System Call
    let mut buf: [u8; 10] = [0; 10];

    match READ(1, &mut buf, 10)
    {
        Ok(number) => println!("Bytes written to buffer: {}", number),
        Err(e) => println!("{}", e)
    }  

    for x in &buf
    {
        println!("{}", x);
    }
}
