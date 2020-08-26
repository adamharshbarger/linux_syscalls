# linux_syscalls
Rust Library for executing Linux system calls

***This is nothing more than a learning exercise for me. It is in no condition to be used in other projects.***

Much of my inspiration comes from [kmcallister/syscall.rs](https://github.com/kmcallister/syscall.rs) 

I originally cloned his repo and just updated his ASM! calls to the new syntax. I then decided that instead of building a system call with his awesome macro that I would prefer to have a function corresponding to each system call that wraps the call logic and returns a Result<> for error handling. From what I understand in the Man Pages many of the system calls return -1 for error and writes tot he errno. I have not implemented a check on errno to return any useful errors. Currently it just returns a genaric String stating that there was indeed an error.

***As I have mentioned, this is just a learning project for me as I am new to Rust and Programming. However, I am open to comments and suggestions.***
