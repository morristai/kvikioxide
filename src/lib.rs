use autocxx::include_cpp;
use autocxx::prelude::*;

include_cpp! {
    #include "kvikio/file_handle.hpp"
    safety!(unsafe_ffi)
    generate!("kvikio::FileHandle")
    // generate!("mode_t")
}
pub struct CuFile {
    file_handle: UniquePtr<ffi::kvikio::FileHandle>,
}

impl CuFile {
    pub fn new(file_path: &str, flags: &str) -> Self {
        CuFile {
            file_handle: ffi::kvikio::FileHandle::new(
                file_path,
                flags
            ).within_unique_ptr(),
        }
    }

    pub fn close(&mut self) {
        ffi::kvikio::FileHandle::close(self.file_handle.pin_mut());
    }

    // pub fn closed(&mut self) -> bool {
    //     ffi::kvikio::FileHandle::closed(self.file_handle.pin_mut())
    // }

    // pub fn fileno(&mut self) -> i32 {
    //     ffi::kvikio::FileHandle::fd(self.file_handle.pin_mut())
    // }
    //
    // pub fn open_flags(&mut self) -> i32 {
    //     ffi::kvikio::FileHandle::fd_open_flags(self.file_handle.pin_mut())
    // }
}
