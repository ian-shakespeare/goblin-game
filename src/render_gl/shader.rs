use std::ffi::{CStr, CString};
use gl::types::{GLchar, GLint, GLuint};
use nalgebra_glm as glm;
use crate::resources::{ResourceError, Resources};
use super::create_empty_buffer;

#[derive(Debug)]
pub enum ShaderError {
    CouldNotCompile(String),
    CouldNotLink(String),
    CouldNotLoad(ResourceError),
    InvalidUniformName,
}

#[derive(Debug)]
pub struct Shader {
    id: GLuint,
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

impl Shader {
    fn check_shader_error(id: GLuint) -> Result<(), CString> {
        let mut success: GLint = 1;
        let mut info_log_len: GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut info_log_len);
        }
        let info_log_buffer = create_empty_buffer(info_log_len as usize);
        unsafe {
            let error: CString = CString::from_vec_unchecked(info_log_buffer);
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                gl::GetShaderInfoLog(
                    id,
                    info_log_len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut GLchar,
                );
                return Err(error);
            }
        }

        Ok(())
    }

    fn check_program_err(id: GLuint) -> Result<(), CString> {
        let mut success: GLint = 1;
        let mut info_log_len: GLint = 0;
        unsafe {
            gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut info_log_len);
        }
        let info_log_buffer = create_empty_buffer(info_log_len as usize);
        unsafe {
            let error: CString = CString::from_vec_unchecked(info_log_buffer);
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
            if success == 0 {
                gl::GetProgramInfoLog(
                    id,
                    info_log_len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut GLchar,
                );
                return Err(error);
            }
        }

        Ok(())
    }

    pub fn from_source(vertex_source: &CStr, fragment_source: &CStr) -> Result<Self, ShaderError> {
        let vertex_shader: GLuint = unsafe {
            gl::CreateShader(gl::VERTEX_SHADER)
        };
        unsafe {
            gl::ShaderSource(
                vertex_shader,
                1,
                &vertex_source.as_ptr(),
                std::ptr::null(),
            );
            gl::CompileShader(vertex_shader);
        }

        Shader::check_shader_error(vertex_shader).map_err(|e| ShaderError::CouldNotCompile(e.to_string_lossy().to_string()))?;

        let fragment_shader: GLuint = unsafe {
            gl::CreateShader(gl::FRAGMENT_SHADER)
        };
        unsafe {
            gl::ShaderSource(
                fragment_shader,
                1,
                &fragment_source.as_ptr(),
                std::ptr::null(),
            );
            gl::CompileShader(fragment_shader);
        }

        Shader::check_shader_error(fragment_shader).map_err(|e| ShaderError::CouldNotCompile(e.to_string_lossy().to_string()))?;

        let shader_program = unsafe {
            gl::CreateProgram()
        };

        unsafe {
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);
        }

        Shader::check_program_err(shader_program).map_err(|e| ShaderError::CouldNotLink(e.to_string_lossy().to_string()))?;

        unsafe {
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }

        Ok(Self {
            id: shader_program,
        })
    }

    pub fn from_resource(resources: &Resources, resource_name: &str) -> Result<Self, ShaderError> {
        let vertex_name = format!("{}.vert", resource_name);
        let fragment_name = format!("{}.frag", resource_name);

        let vertex_source = resources
            .load_cstring(&vertex_name)
            .map_err(|e| ShaderError::CouldNotLoad(e))?;
        let fragment_source = resources
            .load_cstring(&fragment_name)
            .map_err(|e| ShaderError::CouldNotLoad(e))?;

        Shader::from_source(&vertex_source, &fragment_source)
    }

    pub fn start_using(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn set_uniform_1i(&self, name: &str, value: GLint) -> Result<(), ShaderError> {
        let null_terminated_name = format!("{}\0", name);
        let name = CStr::from_bytes_with_nul(null_terminated_name.as_bytes())
            .map_err(|_| ShaderError::InvalidUniformName)?;
        let location = unsafe {
            gl::GetUniformLocation(self.id, name.as_ptr() as *const GLchar)
        };
        unsafe {
            gl::Uniform1i(location, value);
        }
        Ok(())
    }

    pub fn set_uniform_1f(&self, name: &str, value: f32) -> Result<(), ShaderError> {
        let null_terminated_name = format!("{}\0", name);
        let name = CStr::from_bytes_with_nul(null_terminated_name.as_bytes())
            .map_err(|_| ShaderError::InvalidUniformName)?;
        let location = unsafe {
            gl::GetUniformLocation(self.id, name.as_ptr() as *const GLchar)
        };
        unsafe {
            gl::Uniform1f(location, value);
        }
        Ok(())
    }

    pub fn set_transform(&self, name: &str, mat: &glm::Mat4) -> Result<(), ShaderError> {
        let null_terminated_name = format!("{}\0", name);
        let name = CStr::from_bytes_with_nul(null_terminated_name.as_bytes())
            .map_err(|_| ShaderError::InvalidUniformName)?;
        let location = unsafe {
            gl::GetUniformLocation(self.id, name.as_ptr())
        };
        unsafe {
            gl::UniformMatrix4fv(location, 1, gl::FALSE, glm::value_ptr(mat).as_ptr());
        }
        Ok(())
    }
}
