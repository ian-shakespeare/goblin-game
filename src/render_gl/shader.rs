use crate::{render_gl::create_whitespace_cstring_with_length, resources::{ResourceError, Resources}};
use gl::types::{GLchar, GLenum, GLint, GLuint};
use std::ffi::CStr;

#[derive(Debug)]
pub enum ShaderError {
    ResourceLoad(ResourceError),
    UnrecognizedShaderType,
    CompileError { message: String },
    LinkError { message: String },
}

pub struct Shader {
    id: GLuint,
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

impl Shader {
    pub fn from_res(res: &Resources, name: &str) -> Result<Shader, ShaderError> {
        const POSSIBLE_EXT: [(&str, GLenum); 2] = [
            (".vert", gl::VERTEX_SHADER),
            (".frag", gl::FRAGMENT_SHADER),
        ];

        let kind = POSSIBLE_EXT
            .iter()
            .find(|&&(file_extension, _)| {name.ends_with(file_extension)})
            .map(|&(_, kind)| kind)
            .ok_or_else(|| ShaderError::UnrecognizedShaderType)?;

        let source = res
            .load_cstring(name)
            .map_err(|e| ShaderError::ResourceLoad(e))?;

        Shader::from_source(&source, kind)
    }

    pub fn from_source(source: &CStr, kind: GLenum) -> Result<Shader, ShaderError> {
        let id = unsafe {
            gl::CreateShader(kind)
        };
        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }
        let mut success: GLint = 1;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success)
        }
        match success {
            0 => {
                let mut err_length: GLint = 0;
                unsafe {
                    gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut err_length);
                }
                let error = create_whitespace_cstring_with_length(err_length as usize);
                unsafe {
                    gl::GetShaderInfoLog(
                        id,
                        err_length,
                        std::ptr::null_mut(),
                        error.as_ptr() as *mut GLchar
                    );
                }
                Err(ShaderError::CompileError { message: error.to_string_lossy().into_owned() })
            },
            _ => Ok(Shader { id })
        }
    }

    pub fn from_vert_source(source: &CStr) -> Result<Shader, ShaderError> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(source: &CStr) -> Result<Shader, ShaderError> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }

    pub fn id(&self) -> GLuint {
        self.id
    }
}
