#![allow(non_camel_case_types)]
#![allow(dead_code)]

//TODO:
//  Finish Building-out Structures
//  Verify and update Type definitions
//  Look for best way to expose types for user created functions

//Types used for building Structs; Keeping Linux Names to aid
//in referencing from the Linux man pages
pub type dev_t      = i16;      //Done
pub type ino_t      = u16;      //Done
pub type mode_t     = u16;      //Done
pub type nlink_t    = i8;       //Done
pub type uid_t      = i16;      //Done
pub type gid_t      = i8;       //Done
pub type off_t      = u32;      //Done
pub type time_t     = i32;      //Done
pub type blksize_t  = i32;      //Done
pub type blkcnt_t   = i32;      //Done
pub type sigset_t   = usize;    //Verify
pub type loff_t     = usize;    //Verify
pub type fd_set     = usize;    //Verify
pub type key_t      = usize;    //Verify

//Parameter for the STAT, FSTAT, LSTAT System Calls
pub struct stat {               //Done
    st_dev:     dev_t,
	st_ino:     ino_t,
	st_mode:    mode_t,
	st_nlink:   nlink_t,
	st_uid:     uid_t,
	st_gid:     gid_t,
	st_rdev:    dev_t,
	st_size:     off_t,
	st_atime:   time_t,
	st_mtime:   time_t,
	st_ctime:   time_t,
	st_blksize: blksize_t,
	st_blocks:  blkcnt_t,
	st_attr:    mode_t,
}

//Parameter for the POLL System Call
pub struct poll_fd {}       //Define & Verify

//Parameter for the RT_SIGACTION System Call
pub struct sigaction {}     //Define & Verify

//Parameter for the READV, WRITEV System Calls
pub struct iovec {}         //Define & Verify

//Parameter for the SELECT System Call
pub struct timeval {}       //Define & Verify



