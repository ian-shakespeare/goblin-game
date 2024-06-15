use gl::types::{GLint, GLsizeiptr, GLuint, GLvoid};
use nalgebra_glm as glm;
use sdl2::{self, event::Event, keyboard::Keycode};
use std::{f32::consts::PI, path::Path};
use open_gl_test::{render_gl::{shader::Shader, vectors::Vec3, Vertex}, resources::Resources};
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

    let vertices: Vec<Vertex> = vec![
        Vertex { position: (0.5, 0.5, 0.0).into(), color: (1.0, 0.0, 0.0).into(), texture_coords: (1.0, 1.0).into() },
        Vertex { position: (0.5, -0.5, 0.0).into(), color: (0.0, 1.0, 0.0).into(), texture_coords: (1.0, 0.0).into() },
        Vertex { position: (-0.5, -0.5, 0.0).into(), color: (0.0, 0.0, 1.0).into(), texture_coords: (0.0, 0.0).into() },
        Vertex { position: (-0.5, 0.5, 0.0).into(), color: (1.0, 1.0, 0.0).into(), texture_coords: (0.0, 1.0).into() },
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
            (vertices.len() * std::mem::size_of::<Vertex>()) as GLsizeiptr,
            vertices.as_ptr() as *const GLvoid,
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

        // element buffer
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<GLuint>()) as GLsizeiptr,
            indices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );

        // cleanup
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }

    // create texture
    let mut texture1: GLuint = 0;
    unsafe {
        gl::GenTextures(1, &mut texture1);
        gl::BindTexture(gl::TEXTURE_2D, texture1);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE.try_into().unwrap());
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE.try_into().unwrap());
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR.try_into().unwrap());
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR.try_into().unwrap());
    }
    // open image
    let container_img = ImageReader::open(res.get_full_path("textures/container.jpg"))
        .unwrap()
        .decode()
        .unwrap();
    let width = container_img.width();
    let height = container_img.height();
    let container_img_data = container_img.into_rgb8();
    // bind image
    unsafe {
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB.try_into().unwrap(),
            width.try_into().unwrap(),
            height.try_into().unwrap(),
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            container_img_data.as_ptr() as *const GLvoid,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }
    // free image (it is now loaded in GPU)
    drop(container_img_data);

    let mut texture2: GLuint = 0;
    unsafe {
        gl::GenTextures(1, &mut texture2);
        gl::BindTexture(gl::TEXTURE_2D, texture2);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT.try_into().unwrap());
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT.try_into().unwrap());
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR.try_into().unwrap());
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR.try_into().unwrap());
    }

    let awesomeface_img = ImageReader::open(res.get_full_path("textures/awesomeface.png"))
        .unwrap()
        .decode()
        .unwrap();
    let width = awesomeface_img.width();
    let height = awesomeface_img.height();
    let awesomeface_img_data = awesomeface_img.flipv().into_rgba8();
    unsafe {
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA.try_into().unwrap(),
            width.try_into().unwrap(),
            height.try_into().unwrap(),
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            awesomeface_img_data.as_ptr() as *const GLvoid,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }
    drop(awesomeface_img_data);

    shaders.start_using();
    shaders.set_uniform_1i("aTexture1", 0).unwrap();
    shaders.set_uniform_1i("aTexture2", 1).unwrap();

    let mut mix_value: f32 = 0.5;
    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        // handle input events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { timestamp: _ } => break 'main,
                Event::KeyDown { keycode, timestamp: _, window_id: _, scancode: _, keymod: _, repeat: _ } => {
                    if let Some(key) = keycode {
                        match key {
                            Keycode::Up => mix_value += 0.1,
                            Keycode::Down => mix_value -= 0.1,
                            _ => (),
                        }
                    }
                },
                _ => ()
            }
        }

        // render
        unsafe {
            gl::ClearColor(0.2, 0.01, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture2);

            // create transform
            let mut transform = glm::Mat4::identity();
            let translation_vec = glm::Vec3::new(0.5, -0.5, 0.0);
            transform = glm::translate(&transform, &translation_vec);
            let rotation_vec = glm::Vec3::new(0.0, 0.0, 1.0);
            transform = glm::rotate(&transform, 0.5 * PI, &rotation_vec);

            shaders.start_using();
            shaders.set_uniform_1f("aMixValue", mix_value).unwrap();
            shaders.set_transform("transform", &transform).unwrap();
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());

            let mut transform = glm::Mat4::identity();
            let translation_vec = glm::Vec3::new(-0.5, 0.5, 0.0);
            transform = glm::translate(&transform, &translation_vec);
            let rotation_vec = glm::Vec3::new(0.0, 0.0, 1.0);
            transform = glm::rotate(&transform, 0.5 * PI, &rotation_vec);

            shaders.set_transform("transform", &transform).unwrap();
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }

        window.gl_swap_window();
    }

    // cleanup
    unsafe {
        gl::DeleteVertexArrays(1, &mut vao);
        gl::DeleteBuffers(1, &mut vbo);
        gl::DeleteBuffers(1, &ebo);
    }
}
