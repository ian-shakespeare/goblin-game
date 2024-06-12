use std::{
    ffi::CString,
    fs::File,
    io::{self, Read},
    path::{Path, PathBuf}
};

#[derive(Debug)]
pub enum ResourceError {
    Io(io::Error),
    FileContainsNil,
    FailedToGetExePath,
}

impl From<io::Error> for ResourceError {
    fn from(other: io::Error) -> Self {
        ResourceError::Io(other)
    }
}

pub struct Resources {
    root_path: PathBuf,
}

impl Resources {
    pub fn from_relative_exe_path(relative_path: &Path) -> Result<Resources, ResourceError> {
        let exe_file_name = std::env::current_exe().map_err(|_| ResourceError::FailedToGetExePath)?;
        let exe_path = exe_file_name.parent().ok_or(ResourceError::FailedToGetExePath)?;
        Ok(Resources { root_path: exe_path.join(relative_path) })
    }

    pub fn load_cstring(&self, resource_name: &str) -> Result<CString, ResourceError> {
        let mut file = File::open(
            resource_name_to_path(&self.root_path, resource_name)
        )?;

        let mut buffer: Vec<u8> = Vec::with_capacity(file.metadata()?.len() as usize + 1);
        file.read_to_end(&mut buffer)?;

        if buffer.iter().find(|i| **i == 0).is_some() {
            return Err(ResourceError::FileContainsNil);
        }

        Ok(unsafe { CString::from_vec_unchecked(buffer) })
    }
}

fn resource_name_to_path(root_dir: &Path, location: &str) -> PathBuf {
    let mut path: PathBuf = root_dir.into();

    for part in location.split("/") {
        path = path.join(part);
    }

    path
}
