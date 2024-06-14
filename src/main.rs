use gl::types::{GLint, GLsizeiptr, GLuint, GLvoid};
use sdl2::{self, event::Event};
use std::path::Path;
use open_gl_test::{render_gl::{shader::Shader, Vec3, Vertex}, resources::Resources};
use image::io::Reader as ImageReader;

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

    unsafe {
        gl::Viewport(0, 0, 900, 700);
    }

    let shaders = Shader::from_resource(&res, "shaders/triangle").unwrap();

    let vertices1: Vec<Vertex> = vec![
        Vertex { position: (-0.5, -0.5, 0.0).into(), color: (1.0, 0.0, 0.0).into(), texture_coords: (0.0, 0.0).into() },
        Vertex { position: (0.0, 0.5, 0.0).into(), color: (0.0, 1.0, 0.0).into(), texture_coords: (1.0, 0.0).into() },
        Vertex { position: (0.5, -0.5, 0.0).into(), color: (0.0, 0.0, 1.0).into(), texture_coords: (0.5, 1.0).into() },
    ];
    let vertices2: Vec<Vec3> = vec![
        (0.0, -0.5, 0.0).into(),    (0.0, 0.0, 1.0).into(),
        (0.0, 0.5, 0.0).into(),     (0.0, 1.0, 0.0).into(),
        (0.5, 0.5, 0.0).into(),     (1.0, 0.0, 0.0).into(),
    ];
    // let indices: [GLuint;6] = [0, 1, 3, 1, 2, 3];

    let img = ImageReader::open(res.get_full_path("textures/wall.jpg")).unwrap().decode().unwrap();
    let width = img.width();
    let height = img.height();
    let img = img.into_rgb32f();

    let mut texture: GLuint = 0;
    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RG8.try_into().unwrap(),
            width.try_into().unwrap(),
            height.try_into().unwrap(),
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            img.as_ptr() as *const GLvoid,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }

    let mut vaos: [GLuint;2] = [0, 0];
    let mut vbos: [GLuint;2] = [0, 0];
    // let mut ebo: GLuint = 0;
    unsafe {
        gl::GenVertexArrays(2, vaos.as_mut_ptr());
        gl::GenBuffers(2, vbos.as_mut_ptr());
        gl::GenVertexArrays(2, vaos.as_mut_ptr());
        gl::GenBuffers(2, vbos.as_mut_ptr());
        // gl::GenBuffers(1, &mut ebo)
    }

    unsafe {
        // bind vao, bind vertex buffers, configure vertex attributes
        gl::BindVertexArray(vaos[0]);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbos[0]);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices1.len() * std::mem::size_of::<Vertex>()) as GLsizeiptr,
            vertices1.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );
        // position
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<Vertex>() as GLint,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
        // color
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<Vertex>() as GLint,
            std::mem::size_of::<Vec3>() as *const GLvoid,
        );
        gl::EnableVertexAttribArray(1);
        // texture
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<Vertex>() as GLint,
            (2 * std::mem::size_of::<Vec3>()) as *const GLvoid,
        );
        gl::EnableVertexAttribArray(2);

        gl::BindVertexArray(vaos[1]);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbos[1]);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices2.len() * std::mem::size_of::<Vec3>()) as GLsizeiptr,
            vertices2.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );
        // position
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            2 * std::mem::size_of::<Vec3>() as GLint,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
        // color
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            2 * std::mem::size_of::<Vec3>() as GLint,
            std::mem::size_of::<Vec3>() as *const GLvoid,
        );
        gl::EnableVertexAttribArray(1);

        /*
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<GLuint>()) as GLsizeiptr,
            indices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );
        */

        // cleanup
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
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

            shaders.start_using();
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::BindVertexArray(vaos[0]);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            // shaders.start_using();
            // gl::BindVertexArray(vaos[1]);
            // gl::DrawArrays(gl::TRIANGLES, 0, 3);
            // gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }

        window.gl_swap_window();
    }

    // cleanup
    unsafe {
        gl::DeleteVertexArrays(2, vaos.as_mut_ptr());
        gl::DeleteBuffers(2, vbos.as_mut_ptr());
        // gl::DeleteBuffers(1, &ebo);
    }
}
