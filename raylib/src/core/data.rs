//! Data manipulation functions. Compress and Decompress with DEFLATE
use std::{
    ffi::{c_char, CString},
    path::Path,
};

use crate::{
    error::{error, Error},
    ffi,
};

/// Compress data (DEFLATE algorythm)
/// ```rust
/// use sola_raylib::prelude::*;
/// let data = compress_data(b"11111").unwrap();
/// let expected: &[u8] = &[1, 5, 0, 250, 255, 49, 49, 49, 49, 49];
/// assert_eq!(data, expected);
/// ```
pub fn compress_data(data: &[u8]) -> Result<&'static [u8], Error> {
    let mut out_length: i32 = 0;
    // CompressData doesn't actually modify the data, but the header is wrong
    let buffer = {
        unsafe { ffi::CompressData(data.as_ptr() as *mut _, data.len() as i32, &mut out_length) }
    };
    if buffer.is_null() {
        return Err(error!("could not compress data"));
    }
    let buffer = unsafe { std::slice::from_raw_parts(buffer, out_length as usize) };
    Ok(buffer)
}

/// Decompress data (DEFLATE algorythm)
/// ```rust
/// use sola_raylib::prelude::*;
/// let input: &[u8] = &[1, 5, 0, 250, 255, 49, 49, 49, 49, 49];
/// let expected: &[u8] = b"11111";
/// let data = decompress_data(input).unwrap();
/// assert_eq!(data, expected);
/// ```
pub fn decompress_data(data: &[u8]) -> Result<&'static [u8], Error> {
    println!("{:?}", data.len());

    let mut out_length: i32 = 0;
    // CompressData doesn't actually modify the data, but the header is wrong
    let buffer = {
        unsafe { ffi::DecompressData(data.as_ptr() as *mut _, data.len() as i32, &mut out_length) }
    };
    if buffer.is_null() {
        return Err(error!("could not compress data"));
    }
    let buffer = unsafe { std::slice::from_raw_parts(buffer, out_length as usize) };
    Ok(buffer)
}

#[cfg(unix)]
fn path_to_bytes<P: AsRef<Path>>(path: P) -> Vec<u8> {
    use std::os::unix::ffi::OsStrExt;
    path.as_ref().as_os_str().as_bytes().to_vec()
}

#[cfg(not(unix))]
fn path_to_bytes<P: AsRef<Path>>(path: P) -> Vec<u8> {
    path.as_ref().to_string_lossy().to_string().into_bytes()
}

/// Export data to code (.h), returns true on success
pub fn export_data_as_code(data: &[u8], file_name: impl AsRef<Path>) -> bool {
    let c_str = CString::new(path_to_bytes(file_name)).unwrap();

    unsafe { ffi::ExportDataAsCode(data.as_ptr(), data.len() as i32, c_str.as_ptr()) }
}

/// Encode data to Base64 string
pub fn encode_data_base64(data: &[u8]) -> Vec<c_char> {
    let mut output_size = 0;
    let bytes =
        unsafe { ffi::EncodeDataBase64(data.as_ptr(), data.len() as i32, &mut output_size) };

    let s = unsafe { std::slice::from_raw_parts(bytes, output_size as usize) };
    if s.contains(&0) {
        // Work around a bug in Rust's from_raw_parts function
        let mut keep = true;
        let b: Vec<c_char> = s
            .iter()
            .filter(|f| {
                if **f == 0 {
                    keep = false;
                }
                keep
            })
            .copied()
            .collect();
        b
    } else {
        s.to_vec()
    }
}

/// Compute a CRC32 checksum of `data`. Added in raylib 6.0.
pub fn compute_crc32(data: &[u8]) -> u32 {
    unsafe { ffi::ComputeCRC32(data.as_ptr() as *mut u8, data.len() as i32) }
}

/// Compute an MD5 hash of `data`. Returns 16 bytes (4 × u32). Added in
/// raylib 6.0.
///
/// Each u32 is a hash word in host byte order, mirroring raylib's
/// `ComputeMD5` return type. For the canonical 16-byte digest, apply
/// [`u32::to_le_bytes`] to each word (MD5 is little-endian).
pub fn compute_md5(data: &[u8]) -> [u32; 4] {
    unsafe {
        let ptr = ffi::ComputeMD5(data.as_ptr() as *mut u8, data.len() as i32);
        let mut out = [0u32; 4];
        std::ptr::copy_nonoverlapping(ptr, out.as_mut_ptr(), 4);
        out
    }
}

/// Compute a SHA-1 hash of `data`. Returns 20 bytes (5 × u32). Added in
/// raylib 6.0.
///
/// Each u32 is a hash word in host byte order. For the canonical 20-byte
/// digest, apply [`u32::to_be_bytes`] to each word.
pub fn compute_sha1(data: &[u8]) -> [u32; 5] {
    unsafe {
        let ptr = ffi::ComputeSHA1(data.as_ptr() as *mut u8, data.len() as i32);
        let mut out = [0u32; 5];
        std::ptr::copy_nonoverlapping(ptr, out.as_mut_ptr(), 5);
        out
    }
}

/// Compute a SHA-256 hash of `data`. Returns 32 bytes (8 × u32). Added in
/// raylib 6.0.
///
/// Each u32 is a hash word in host byte order, mirroring raylib's
/// `ComputeSHA256` return type. For the canonical 32-byte digest, apply
/// [`u32::to_be_bytes`] to each word.
///
/// ```rust
/// use sola_raylib::prelude::*;
/// let empty = compute_sha256(b"");
/// // SHA-256("") = e3b0c442 98fc1c14 9afbf4c8 996fb924
/// //              27ae41e4 649b934c a495991b 7852b855
/// assert_eq!(empty[0].to_be_bytes(), [0xe3, 0xb0, 0xc4, 0x42]);
/// assert_eq!(empty[7].to_be_bytes(), [0x78, 0x52, 0xb8, 0x55]);
/// ```
pub fn compute_sha256(data: &[u8]) -> [u32; 8] {
    unsafe {
        let ptr = ffi::ComputeSHA256(data.as_ptr() as *mut u8, data.len() as i32);
        let mut out = [0u32; 8];
        std::ptr::copy_nonoverlapping(ptr, out.as_mut_ptr(), 8);
        out
    }
}

// Decode Base64 data
pub fn decode_data_base64(data: &[u8]) -> Vec<u8> {
    let mut output_size = 0;

    let bytes = unsafe {
        ffi::DecodeDataBase64(
            data.as_ptr() as *const std::os::raw::c_char,
            &mut output_size,
        )
    };

    let s = unsafe { std::slice::from_raw_parts(bytes, output_size as usize) };
    if s.contains(&0) {
        // Work around a bug in Rust's from_raw_parts function
        let mut keep = true;
        let b: Vec<u8> = s
            .iter()
            .filter(|f| {
                if **f == 0 {
                    keep = false;
                }
                keep
            })
            .copied()
            .collect();
        b
    } else {
        s.to_vec()
    }
}
