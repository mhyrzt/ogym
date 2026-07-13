// Patched for MuJoCo 3.x: the original crate targeted the pre-3.0 mjVFS API
// (public nfile/filedata/filesize fields, mj_findFileVFS/mj_makeEmptyFileVFS,
// fixed-size mjMAXVFS/mjMAXVFSNAME limits). MuJoCo 3.0 made mjVFS opaque and
// replaced the two-step "reserve then copy" file API with mj_addBufferVFS,
// which both reserves and copies in one call. There is no longer a public
// accessor for reading a buffer back out of the VFS once added, so
// `get_file()` can no longer be implemented against this API and is stubbed.

use std::ffi::CString;

/// An error when adding a file to a [`Vfs`] via [`Vfs::add_file()`]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum AddError {
    RepeatedName,
    LoadFailed,
}
impl std::fmt::Display for AddError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
impl std::error::Error for AddError {}

pub struct Vfs {
    pub(crate) vfs: Box<mujoco_rs_sys::no_render::mjVFS_>,
}
impl Vfs {
    /// Initializes a new empty `Vfs`
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets a file's contents from the `Vfs`.
    ///
    /// Always returns `None`: MuJoCo 3.x's mjVFS is opaque and exposes no
    /// public API to read a buffer back out once added.
    #[allow(dead_code)]
    pub fn get_file(&self, _filename: &str) -> Option<&[u8]> {
        None
    }

    /// Deletes a file from the `Vfs` if it exists, and returns if such a file
    /// was found
    pub fn delete_file(&mut self, filename: &str) -> bool {
        let c_str = CString::new(filename).unwrap();
        let result = unsafe {
            mujoco_rs_sys::no_render::mj_deleteFileVFS(&mut *self.vfs, c_str.as_ptr())
        };
        result == 0
    }

    /// Adds a file to the `Vfs` from some given contents
    pub fn add_file(&mut self, filename: &str, contents: &[u8]) -> Result<(), AddError> {
        let c_filename = CString::new(filename).unwrap();
        let add_errno = unsafe {
            mujoco_rs_sys::no_render::mj_addBufferVFS(
                &mut *self.vfs,
                c_filename.as_ptr(),
                contents.as_ptr() as *const std::os::raw::c_void,
                contents.len() as std::os::raw::c_int,
            )
        };

        match add_errno {
            0 => Ok(()),
            2 => Err(AddError::RepeatedName),
            _ => Err(AddError::LoadFailed),
        }
    }
}
impl Default for Vfs {
    fn default() -> Self {
        let mut result = Self {
            vfs: unsafe {
                Box::from_raw(std::alloc::alloc(std::alloc::Layout::new::<
                    mujoco_rs_sys::no_render::mjVFS_,
                >()) as *mut _)
            },
        };
        unsafe { mujoco_rs_sys::no_render::mj_defaultVFS(&mut *result.vfs) };
        result
    }
}
impl Drop for Vfs {
    fn drop(&mut self) {
        unsafe { mujoco_rs_sys::no_render::mj_deleteVFS(&mut *self.vfs) }
    }
}

#[cfg(test)]
mod tests {
    use super::{AddError, Vfs};

    #[test]
    fn new() {
        Vfs::new();
    }

    #[test]
    fn add() {
        let filename = "asdf/dsdfs$@f.123";
        let content = "3klj032#$>>😮f";
        let mut vfs = Vfs::new();
        vfs.add_file(filename, content.as_bytes()).unwrap();
    }

    #[test]
    fn delete_without_add() {
        let mut vfs = Vfs::new();
        assert!(!vfs.delete_file("asdf"));
    }

    #[test]
    fn add_then_delete() {
        let filename = "file";
        let mut vfs = Vfs::new();
        vfs.add_file(filename, "asdf".as_bytes()).unwrap();
        assert!(vfs.delete_file(filename));
    }

    #[test]
    fn add_twice_protected() {
        let filename = "file";
        let mut vfs = Vfs::new();
        vfs.add_file(filename, "asdf".as_bytes()).unwrap();
        let second_add_result = vfs.add_file(filename, "asdf".as_bytes());
        assert!(second_add_result == Err(AddError::RepeatedName));
    }
}
