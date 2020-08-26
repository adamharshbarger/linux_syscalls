#![allow(non_camel_case_types)]
#![allow(dead_code)]

type dev_t      = i16;
type ino_t      = u16;
type mode_t     = u16;
type nlink_t    = i8;
type uid_t      = i16;
type gid_t      = i8;
type off_t      = u32;
type time_t     = i32;
type blksize_t  = i32;
type blkcnt_t   = i32;

pub struct Stat {
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