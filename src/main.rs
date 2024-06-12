use flappy_crab::{
    render_gl::{
        data::{Vec3, Vec4, VertexPointer},
        program::Program
    },
    resources::Resources
};
use gl::types::{GLsizeiptr, GLuint, GLvoid};
use sdl2::{self, event::Event};
use std::path::Path;

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    pos: Vec3,
    color: Vec4,
}

impl Vertex {
    fn vertex_attrib_pointers() {
        let stride = std::mem::size_of::<Self>();
        let location = 0;
        let offset = 0;
        unsafe {
            Vec3::vertex_attrib_pointer(stride, location, offset);
        }

        let location = 1;
        let offset = offset + std::mem::size_of::<Vec3>();
        unsafe {
            Vec4::vertex_attrib_pointer(stride, location, offset);
        }
    }
}

fn main() {
    let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();

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

    let shader_program = Program::from_res(&res, "shaders/triangle").unwrap();

    let vertices: Vec<Vertex> = vec![
        Vertex {
            pos: (-0.5, -0.5, 0.0).into(),
            color: (0.0, 0.0, 1.0, 1.0).into()
        }, // bottom left
        Vertex {
            pos: (0.5, -0.5, 0.0).into(),
            color: (0.0, 1.0, 0.0, 1.0).into()
        }, // bottom right
        Vertex {
            pos: (0.0, 0.5, 0.0).into(),
            color: (1.0, 0.0, 0.0, 1.0).into()
        }, // top
    ];

    let mut vbo: GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<Vertex>()) as GLsizeiptr,
            vertices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    let mut vao: GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        Vertex::vertex_attrib_pointers();

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    unsafe {
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.2, 0.01, 0.2, 1.0);
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
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        shader_program.set_used();
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.gl_swap_window();
    }
}
