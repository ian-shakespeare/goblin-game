use gl::types::{GLint, GLsizeiptr, GLuint, GLvoid};
use nalgebra_glm::{self as glm, U3};
use sdl2::{self, event::Event, keyboard::Keycode};
use std::{f32::consts::PI, path::Path};
use open_gl_test::{camera::Camera, shader::Shader, utils::Vertex, resources::Resources};
use image::io::Reader as ImageReader;

fn main() {
    let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();

    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let start_time = std::time::Instant::now();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let window = video_subsystem
        .window("GL Test", 900, 700)
        .opengl()
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    sdl.mouse().set_relative_mouse_mode(true);

    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::Viewport(0, 0, 900, 700);
        gl::Enable(gl::DEPTH_TEST);
    }

    let shaders = Shader::from_resource(&res, "shaders/triangle").unwrap();
    let vertices: Vec<Vertex> = vec![
        Vertex { position: glm::Vec3::new(-0.5, -0.5, -0.5),  texture_coords: glm::Vec2::new(0.0, 0.0) },
        Vertex { position: glm::Vec3::new(0.5, -0.5, -0.5),  texture_coords: glm::Vec2::new(1.0, 0.0) },
        Vertex { position: glm::Vec3::new(0.5,  0.5, -0.5),  texture_coords: glm::Vec2::new(1.0, 1.0) },
        Vertex { position: glm::Vec3::new(0.5,  0.5, -0.5),  texture_coords: glm::Vec2::new(1.0, 1.0) },
        Vertex { position: glm::Vec3::new(-0.5,  0.5, -0.5),  texture_coords: glm::Vec2::new(0.0, 1.0) },
        Vertex { position: glm::Vec3::new(-0.5, -0.5, -0.5),  texture_coords: glm::Vec2::new(0.0, 0.0) },

        Vertex { position: glm::Vec3::new(-0.5, -0.5,  0.5),  texture_coords: glm::Vec2::new(0.0, 0.0) },
        Vertex { position: glm::Vec3::new(0.5, -0.5,  0.5),  texture_coords: glm::Vec2::new(1.0, 0.0) },
        Vertex { position: glm::Vec3::new(0.5,  0.5,  0.5),  texture_coords: glm::Vec2::new(1.0, 1.0) },
        Vertex { position: glm::Vec3::new(0.5,  0.5,  0.5),  texture_coords: glm::Vec2::new(1.0, 1.0) },
        Vertex { position: glm::Vec3::new(-0.5,  0.5,  0.5),  texture_coords: glm::Vec2::new(0.0, 1.0) },
        Vertex { position: glm::Vec3::new(-0.5, -0.5,  0.5),  texture_coords: glm::Vec2::new(0.0, 0.0) },

        Vertex { position: glm::Vec3::new(-0.5,  0.5,  0.5),  texture_coords: glm::Vec2::new(1.0, 0.0) },
        Vertex { position: glm::Vec3::new(-0.5,  0.5, -0.5),  texture_coords: glm::Vec2::new(1.0, 1.0) },
        Vertex { position: glm::Vec3::new(-0.5, -0.5, -0.5),  texture_coords: glm::Vec2::new(0.0, 1.0) },
        Vertex { position: glm::Vec3::new(-0.5, -0.5, -0.5),  texture_coords: glm::Vec2::new(0.0, 1.0) },
        Vertex { position: glm::Vec3::new(-0.5, -0.5,  0.5),  texture_coords: glm::Vec2::new(0.0, 0.0) },
        Vertex { position: glm::Vec3::new(-0.5,  0.5,  0.5),  texture_coords: glm::Vec2::new(1.0, 0.0) },

        Vertex { position: glm::Vec3::new(0.5,  0.5,  0.5),  texture_coords: glm::Vec2::new(1.0, 0.0) },
        Vertex { position: glm::Vec3::new(0.5,  0.5, -0.5),  texture_coords: glm::Vec2::new(1.0, 1.0) },
        Vertex { position: glm::Vec3::new(0.5, -0.5, -0.5),  texture_coords: glm::Vec2::new(0.0, 1.0) },
        Vertex { position: glm::Vec3::new(0.5, -0.5, -0.5),  texture_coords: glm::Vec2::new(0.0, 1.0) },
        Vertex { position: glm::Vec3::new(0.5, -0.5,  0.5),  texture_coords: glm::Vec2::new(0.0, 0.0) },
        Vertex { position: glm::Vec3::new(0.5,  0.5,  0.5),  texture_coords: glm::Vec2::new(1.0, 0.0) },

        Vertex { position: glm::Vec3::new(-0.5, -0.5, -0.5),  texture_coords: glm::Vec2::new(0.0, 1.0) },
        Vertex { position: glm::Vec3::new(0.5, -0.5, -0.5),  texture_coords: glm::Vec2::new(1.0, 1.0) },
        Vertex { position: glm::Vec3::new(0.5, -0.5,  0.5),  texture_coords: glm::Vec2::new(1.0, 0.0) },
        Vertex { position: glm::Vec3::new(0.5, -0.5,  0.5),  texture_coords: glm::Vec2::new(1.0, 0.0) },
        Vertex { position: glm::Vec3::new(-0.5, -0.5,  0.5),  texture_coords: glm::Vec2::new(0.0, 0.0) },
        Vertex { position: glm::Vec3::new(-0.5, -0.5, -0.5),  texture_coords: glm::Vec2::new(0.0, 1.0) },

        Vertex { position: glm::Vec3::new(-0.5,  0.5, -0.5),  texture_coords: glm::Vec2::new(0.0, 1.0) },
        Vertex { position: glm::Vec3::new(0.5,  0.5, -0.5),  texture_coords: glm::Vec2::new(1.0, 1.0) },
        Vertex { position: glm::Vec3::new(0.5,  0.5,  0.5),  texture_coords: glm::Vec2::new(1.0, 0.0) },
        Vertex { position: glm::Vec3::new(0.5,  0.5,  0.5),  texture_coords: glm::Vec2::new(1.0, 0.0) },
        Vertex { position: glm::Vec3::new(-0.5,  0.5,  0.5),  texture_coords: glm::Vec2::new(0.0, 0.0) },
        Vertex { position: glm::Vec3::new(-0.5,  0.5, -0.5),  texture_coords: glm::Vec2::new(0.0, 1.0) },
    ];

    let cube_positions: [glm::Vec3;10] = [
        glm::Vec3::new(0.0, 0.0, 0.0), 
        glm::Vec3::new( 2.0 ,5.0 ,-15.0 ), 
        glm::Vec3::new(-1.5 ,-2.2 ,-2.5 ),  
        glm::Vec3::new(-3.8 ,-2.0 ,-12.3 ),  
        glm::Vec3::new( 2.4 ,-0.4 ,-3.5 ),  
        glm::Vec3::new(-1.7 ,3.0 ,-7.5 ),  
        glm::Vec3::new( 1.3 ,-2.0 ,-2.5 ),  
        glm::Vec3::new( 1.5 ,2.0 ,-2.5 ), 
        glm::Vec3::new( 1.5 ,0.2 ,-1.5 ), 
        glm::Vec3::new(-1.3 ,1.0 ,-1.5 )  
    ];

    let mut vao: GLuint = 0;
    let mut vbo: GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
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
        // texture
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<Vertex>() as GLint,
            std::mem::size_of::<glm::Vec3>() as *const GLvoid,
        );
        gl::EnableVertexAttribArray(1);

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

    let mut camera = Camera::new();
    let mut last_frame: f32 = start_time.elapsed().as_secs_f32();
    let mix_value: f32 = 0.5;

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        let current_frame = start_time.elapsed().as_secs_f32();
        let delta_time = current_frame - last_frame;
        last_frame = current_frame;

        let camera_speed = 50.0 * delta_time;
        // handle input events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { timestamp: _ } => break 'main,
                Event::MouseMotion { timestamp: _, window_id: _, which: _, mousestate: _, x: _, y: _, xrel, yrel } => {
                    camera.rotate(xrel as f32, -yrel as f32, None);
                },
                Event::KeyDown { keycode, timestamp: _, window_id: _, scancode: _, keymod: _, repeat: _ } => {
                    if let Some(key) = keycode {
                        match key {
                            Keycode::Q => break 'main,
                            Keycode::W => camera.translate(camera_speed * camera.front()),
                            Keycode::A => {
                                let cross = glm::cross::<f32, U3>(&camera.front(), &camera.up());
                                camera.translate(-camera_speed * glm::normalize(&cross));
                            },
                            Keycode::S => camera.translate(-camera_speed * camera.front()),
                            Keycode::D => {
                                let cross = glm::cross::<f32, U3>(&camera.front(), &camera.up());
                                camera.translate(camera_speed * glm::normalize(&cross));
                            },
                            _ => (),
                        }
                    }
                },
                _ => ()
            }
        }

        // render
        unsafe {
            gl::ClearColor(0.8, 0.8, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture2);

            let view_transform = camera.get_view_matrix();

            let projection_transform = glm::perspective::<f32>(9.0 / 7.0, 45.0 / (180.0 / PI), 0.1, 100.0);

            shaders.start_using();
            shaders.set_uniform_1f("aMixValue", mix_value).unwrap();
            shaders.set_transform("view", &view_transform).unwrap();
            shaders.set_transform("projection", &projection_transform).unwrap();
            gl::BindVertexArray(vao);

            for (i, position) in cube_positions.iter().enumerate() {
                let mut model_transform = glm::Mat4::identity();
                model_transform = glm::translate(&model_transform, &position);
                let model_rotation_vec = glm::Vec3::new(1.0, 0.3, 0.5);
                let angle = match i {
                    2 => current_frame * (PI / 18.0),
                    5 => current_frame * (PI / 18.0),
                    8 => current_frame * (PI / 18.0),
                    _ => (20.0 * i as f32) * (PI / 180.0),
                };
                model_transform = glm::rotate(&model_transform, angle, &model_rotation_vec);
                shaders.set_transform("model", &model_transform).unwrap();
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }

        window.gl_swap_window();
    }

    // cleanup
    unsafe {
        gl::DeleteVertexArrays(1, &mut vao);
        gl::DeleteBuffers(1, &mut vbo);
    }
}
