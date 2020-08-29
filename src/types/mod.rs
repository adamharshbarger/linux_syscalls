//TODO:
//  Finish Building-out Structures
//  Verify and update Type definitions
//  Look for best way to expose types for user created functions

//C Type Conversions for general use
pub type c_char 			= i8;
pub type c_uchar			= u8;
pub type c_short			= i16;
pub type c_ushort			= u16;
pub type c_int				= i32;
pub type c_uint				= u32;
pub type c_long				= i32;
pub type c_ulong			= u32;
pub type c_long_long		= i64;
pub type c_ulong_long		= u64;
pub type c_size_t			= usize;
pub type c_float			= f32;
pub type c_double 			= f64;
// long double is not supported in rust
pub type c_bool				= bool;
pub type c_void				= ();

//C Types defined in stdint.h header
pub type c_int8_t			= i8;
pub type c_uint8_t			= u8;
pub type c_int16_t			= i16;
pub type c_uint16_t			= u16;
pub type c_int32_t			= i32;
pub type c_uint32_t			= u32;
pub type c_int64_t			= i64;
pub type c_uint64_t			= u64;

//Types used for building Structs; Keeping Linux Names to aid					<---Update to C_Types
//in referencing from the Linux man pages
pub type dev_t      		= i16;      //Done
pub type ino_t      		= u16;      //Done
pub type mode_t     		= u16;      //Done
pub type nlink_t    		= i8;       //Done
pub type uid_t      		= i16;      //Done
pub type gid_t      		= i8;       //Done
pub type off_t      		= u32;      //Done
pub type time_t     		= i32;      //Done
pub type blksize_t  		= i32;      //Done
pub type blkcnt_t   		= i32;      //Done
pub type sigset_t   		= usize;    //Verify
pub type loff_t     		= usize;    //Verify
pub type fd_set     		= usize;    //Verify
pub type key_t      		= usize;    //Verify

//Parameter for the STAT, FSTAT, LSTAT System Calls
pub struct stat {               								//Done & Verified
    pub st_dev:     		dev_t,
	pub st_ino:     		ino_t,
	pub st_mode:    		mode_t,
	pub st_nlink:   		nlink_t,
	pub st_uid:     		uid_t,
	pub st_gid:     		gid_t,
	pub st_rdev:    		dev_t,
	pub st_size:    		 off_t,
	pub st_atime:   		time_t,
	pub st_mtime:   		time_t,
	pub st_ctime:   		time_t,
	pub st_blksize: 		blksize_t,
	pub st_blocks:  		blkcnt_t,
	pub st_attr:    		mode_t,
}

//Parameter for the POLL System Call
pub struct poll_fd {											//Done & Verified
	pub fd:					c_int,
	pub events:				c_short,
	pub revents:			c_short,
}     

//Parameter for the RT_SIGACTION System Call
pub struct sigaction {}     //Define & Verify

//Parameter for the READV, WRITEV System Calls
pub struct iovec {}         //Define & Verify

//Parameter for the SELECT System Call
pub struct timeval {}       //Define & Verify



