use std::ffi::c_char;
use crate::to_java_string;

#[repr(C)]
pub struct Asset {
    path: *const c_char,
    mime_type: *const c_char,
    content: *const c_char,
}

#[no_mangle]
extern fn getAssetPath(asset: &Asset) -> *const c_char {
    asset.path
}

#[no_mangle]
extern fn assetSetMimeType(mut asset: Box<Asset>, mime_type: *const c_char) -> Box<Asset> {
    asset.mime_type = mime_type;
    asset
}

#[no_mangle]
extern fn assetSetContent(mut asset: Box<Asset>, content: *const c_char) -> Box<Asset> {
    asset.content = content;
    asset
}

impl Asset {
    pub fn new(path: &str) -> Self {
        Self {
            path: to_java_string(path),
            mime_type: to_java_string(""),
            content: to_java_string(""),
        }
    }

    pub fn mime_type(&self) -> *const c_char {
        self.mime_type
    }

    pub fn content(&self) -> *const c_char {
        self.content
    }
}
