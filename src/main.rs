use gl::types::{GLchar, GLint, GLsizeiptr, GLuint, GLvoid};
use sdl2::{self, event::Event};
use std::ffi::{CStr, CString};

const VERTEX_SHADER: &'static str = r#"
#version 330 core

layout (location = 0) in vec3 Position;

void main() {
  gl_Position = vec4(Position, 1.0);
}
"#;

const FRAGMENT_SHADER: &'static str = r#"
#version 330 core

out vec4 FragColor;

void main() {
  FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
}
"#;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let window = video_subsystem
        .window("Flappy Crab", 900, 700)
        .opengl()
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::Viewport(0, 0, 900, 700);
    }

    let mut vertex_shader: GLuint = 0;
    unsafe {
        vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(
            vertex_shader,
            1,
            &CStr::from_bytes_with_nul_unchecked(VERTEX_SHADER.as_bytes()).as_ptr(),
            std::ptr::null()
        );
        gl::CompileShader(vertex_shader);

        let mut success: GLint = 1;
        let mut info_log_len: GLint = 0;
        gl::GetShaderiv(vertex_shader, gl::INFO_LOG_LENGTH, &mut info_log_len);
        let mut info_log_buffer: Vec<u8> = Vec::with_capacity(info_log_len as usize + 1);
        info_log_buffer.extend([b' '].iter().cycle().take(info_log_len as usize));
        let error: CString = CString::from_vec_unchecked(info_log_buffer);
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            gl::GetShaderInfoLog(
                vertex_shader,
                info_log_len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut GLchar,
            );
            panic!("Shader Compilation Error: {}", error.to_string_lossy());
        }
    }

    let mut fragment_shader: GLuint = 0;
    unsafe {
        fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(
            fragment_shader,
            1,
            &CStr::from_bytes_with_nul_unchecked(FRAGMENT_SHADER.as_bytes()).as_ptr(),
            std::ptr::null()
        );
        gl::CompileShader(fragment_shader);

        let mut success: GLint = 1;
        let mut info_log_len: GLint = 0;
        gl::GetShaderiv(vertex_shader, gl::INFO_LOG_LENGTH, &mut info_log_len);
        let mut info_log_buffer: Vec<u8> = Vec::with_capacity(info_log_len as usize + 1);
        info_log_buffer.extend([b' '].iter().cycle().take(info_log_len as usize));
        let error: CString = CString::from_vec_unchecked(info_log_buffer);
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            gl::GetShaderInfoLog(
                vertex_shader,
                info_log_len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut GLchar,
            );
            panic!("Shader Compilation Error: {}", error.to_string_lossy());
        }
    }

    let mut shader_program: GLuint = 0;
    unsafe {
        shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);
    }

    let mut success: GLint = 1;
    let mut info_log_len: GLint = 0;
    unsafe {
        gl::GetProgramiv(shader_program, gl::INFO_LOG_LENGTH, &mut info_log_len);
    }
    let mut info_log_buffer: Vec<u8> = Vec::with_capacity(info_log_len as usize + 1);
    info_log_buffer.extend([b' '].iter().cycle().take(info_log_len as usize));
    unsafe {
        let error: CString = CString::from_vec_unchecked(info_log_buffer);
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success == 0 {
            gl::GetProgramInfoLog(
                shader_program,
                info_log_len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut GLchar,
            );
            panic!("Shader Compilation Error: {}", error.to_string_lossy());
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }

    let vertices: Vec<f32> = vec![
        0.5, 0.5, 0.0,
        0.5, -0.5, 0.0,
        -0.5, -0.5, 0.0,
        -0.5, 0.5, 0.0,
    ];
    let indices: [GLuint;6] = [0, 1, 3, 1, 2, 3];

    let mut vao: GLuint = 0;
    let mut vbo: GLuint = 0;
    let mut ebo: GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo)
    }

    unsafe {
        // bind vao, bind vertex buffers, configure vertex attributes
        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
            vertices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<GLuint>()) as GLsizeiptr,
            indices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * std::mem::size_of::<f32>()) as GLint,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        // cleanup
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        // handle input events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { timestamp: _ } => break 'main,
                _ => ()
            }
        }

        // render
        unsafe {
            gl::ClearColor(0.2, 0.01, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }

        window.gl_swap_window();
    }

    // cleanup
    unsafe {
        gl::DeleteVertexArrays(1, &vao);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteBuffers(1, &ebo);
        gl::DeleteProgram(shader_program);
    }
}
