use std::ffi::{c_char, c_int};
use crate::{to_rust_string};

#[repr(C)]
pub struct Asset {
    mime_type: String,
    content: Vec<u8>,
}

#[no_mangle]
extern fn createAsset(
    mime_type: *const c_char,
    content: *const i8,
    content_len: c_int,
) -> Box<Asset> {
    // Safety: The data is guaranteed to be set from Java's side.
    let content = unsafe {
        Vec::from(
            std::slice::from_raw_parts(
                content as *const u8,
                content_len as usize,
            )
        )
    };
    return Asset {
        mime_type: to_rust_string(mime_type),
        content: content,
    }.into();
}

/// Drops the asset and frees the memory
#[no_mangle]
extern fn dropAsset(asset: Box<Asset>) {
    drop(asset)
}

impl Asset {
    pub fn new(mime_type: String, content: Vec<u8>) -> Self {
        Self { mime_type, content }
    }

    pub fn mime_type(&self) -> &str {
        &self.mime_type
    }

    pub fn content(&self) -> &Vec<u8> {
        &self.content
    }
}

