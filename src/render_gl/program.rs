use crate::{
    render_gl::{
        create_whitespace_cstring_with_length,
        shader::{Shader, ShaderError}
    },
    resources::Resources
};
use gl::types::{GLchar, GLint, GLuint};

#[derive(Debug)]
pub enum ProgramError {
    ShaderLoad(ShaderError),
    ProgramLoad { message: String },
}

pub struct Program {
    id: GLuint,
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

impl Program {
    pub fn from_res(res: &Resources, name: &str) -> Result<Program, ProgramError> {
        const POSSIBLE_EXT: [&str; 2] = [
            ".vert",
            ".frag",
        ];

        let shaders = POSSIBLE_EXT
            .iter()
            .map(|file_extension|
                Shader::from_res(res, &format!("{}{}", name, file_extension))
            )
            .collect::<Result<Vec<Shader>, ShaderError>>()
            .map_err(|e| ProgramError::ShaderLoad(e))?;

        Program::from_shaders(&shaders)
    }

    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, ProgramError> {
        let id = unsafe {
            gl::CreateProgram()
        };

        for shader in shaders {
            unsafe {
                gl::AttachShader(id, shader.id());
            }
        }

        unsafe {
            gl::LinkProgram(id);
        }

        let mut success: GLint = 0;
        unsafe {
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut err_length: GLint = 0;
            unsafe {
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut err_length);
            }

            let error = create_whitespace_cstring_with_length(err_length as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    id,
                    err_length,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut GLchar
                );
            }

            return Err(ProgramError::ProgramLoad { message: error.to_string_lossy().into_owned() });
        }

        for shader in shaders {
            unsafe {
                gl::DetachShader(id, shader.id());
            }
        }

        Ok(Program { id })
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}
