#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
extern crate libc;

pub mod ffi {
    pub use libc;

    pub const ARCHIVE_EOF: i32 = 1;
    pub const ARCHIVE_OK: i32 = 0;
    pub const ARCHIVE_RETRY: i32 = -10;
    pub const ARCHIVE_WARN: i32 = -20;
    pub const ARCHIVE_FAILED: i32 = -25;
    pub const ARCHIVE_FATAL: i32 = -30;

    pub const ARCHIVE_EXTRACT_OWNER: i32 = 0x0001;
    pub const ARCHIVE_EXTRACT_PERM: i32 = 0x0002;
    pub const ARCHIVE_EXTRACT_TIME: i32 = 0x0004;
    pub const ARCHIVE_EXTRACT_NO_OVERWRITE: i32 = 0x0008;
    pub const ARCHIVE_EXTRACT_UNLINK: i32 = 0x0010;
    pub const ARCHIVE_EXTRACT_ACL: i32 = 0x0020;
    pub const ARCHIVE_EXTRACT_FFLAGS: i32 = 0x0040;
    pub const ARCHIVE_EXTRACT_XATTR: i32 = 0x0080;
    pub const ARCHIVE_EXTRACT_SECURE_SYMLINKS: i32 = 0x0100;
    pub const ARCHIVE_EXTRACT_SECURE_NODOTDOT: i32 = 0x0200;
    pub const ARCHIVE_EXTRACT_NO_AUTODIR: i32 = 0x0400;
    pub const ARCHIVE_EXTRACT_NO_OVERWRITE_NEWER: i32 = 0x0800;
    pub const ARCHIVE_EXTRACT_SPARSE: i32 = 0x1000;
    pub const ARCHIVE_EXTRACT_MAC_METADATA: i32 = 0x2000;
    pub const ARCHIVE_EXTRACT_NO_HFS_COMPRESSION: i32 = 0x4000;
    pub const ARCHIVE_EXTRACT_HFS_COMPRESSION_FORCED: i32 = 0x8000;
    pub const ARCHIVE_EXTRACT_SECURE_NOABSOLUTEPATHS: i32 = 0x10000;
    pub const ARCHIVE_EXTRACT_CLEAR_NOCHANGE_FFLAGS: i32 = 0x20000;

    include!("ffi.rs");
}
