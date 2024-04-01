use std::{os::raw::c_char, path::Path};

use libloading::{Library, Symbol};

pub struct Plugin {
    pub greet: unsafe extern "C" fn(name: *const c_char),
    lib: Library,
}

impl Plugin {
    pub fn load(lib_path: &Path) -> Result<Self, libloading::Error> {
        let lib = unsafe { Library::new(lib_path)? };

        Ok(unsafe {
            Plugin {
                greet: *(lib.get(b"greet")?),
                lib,
            }
        })
    }
}
