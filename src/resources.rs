use std::{ffi::CString, fs::File, io::{self, Read}, path::{Path, PathBuf}};

#[derive(Debug)]
pub enum ResourceError {
    CouldNotGetExePath,
    CouldNotLoad(io::Error),
    UnexpectedNullCharacter,
}

impl From<io::Error> for ResourceError {
    fn from(other: io::Error) -> Self {
        ResourceError::CouldNotLoad(other)
    }
}

pub struct Resources {
    root_path: PathBuf,
}

impl Resources {
    fn name_to_path(root_dir: &Path, location: &str) -> PathBuf {
        let mut path: PathBuf = root_dir.into();

        for part in location.split("/") {
            path = path.join(part);
        }

        path
    }

    pub fn from_relative_exe_path(rel_path: &Path) -> Result<Self, ResourceError> {
        let exe_file_name = std::env::current_exe().map_err(|_| ResourceError::CouldNotGetExePath)?;
        let exe_path = exe_file_name.parent().ok_or(ResourceError::CouldNotGetExePath)?;

        Ok(Self {
            root_path: exe_path.join(rel_path),
        })
    }

    pub fn get_full_path(&self, resource_name: &str) -> PathBuf {
        self.root_path.join(resource_name)
    }

    pub fn load_cstring(&self, resource_name: &str) -> Result<CString, ResourceError> {
        let mut file = File::open(
            Self::name_to_path(&self.root_path, resource_name)
        )?;

        let mut buffer: Vec<u8> = Vec::with_capacity(file.metadata()?.len() as usize + 1);
        file.read_to_end(&mut buffer)?;

        if buffer.iter().find(|byte| **byte == 0).is_some() {
            return Err(ResourceError::UnexpectedNullCharacter);
        }

        unsafe {
            Ok(CString::from_vec_with_nul_unchecked(buffer))
        }
    }
}
