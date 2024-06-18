use gl::types::{GLuint, GLvoid};
use nalgebra_glm as glm;
use sdl2;
use std::{f32::consts::PI, path::Path};
use open_gl_test::{
    camera::Camera, controller::Controller, input::InputHandler, resources::Resources, shader::Shader, texture::Texture, triangle::Triangle, vertex::{Vertex, VertexArray, VertexBuffer}
};
use image::io::Reader as ImageReader;

const SCREEN_WIDTH: f32 = 900.0;
const SCREEN_HEIGHT: f32 = 700.0;
const TICKS_PER_SECOND: f32 = 90.0;
const TICK_RATE: f32 = 1000.0 / TICKS_PER_SECOND;

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
        gl::Viewport(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
        gl::Enable(gl::DEPTH_TEST);
    }

    let shaders = Shader::from_resource(&res, "shaders/triangle").unwrap();
    let vertices: Vec<Vertex> = vec![
        Vertex::new(glm::Vec3::new(-0.5, -0.5, -0.5), glm::Vec2::new(0.0, 0.0)),
        Vertex::new(glm::Vec3::new(0.5, -0.5, -0.5), glm::Vec2::new(1.0, 0.0)),
        Vertex::new(glm::Vec3::new(0.5,  0.5, -0.5), glm::Vec2::new(1.0, 1.0)),
        Vertex::new(glm::Vec3::new(0.5,  0.5, -0.5), glm::Vec2::new(1.0, 1.0)),
        Vertex::new(glm::Vec3::new(-0.5,  0.5, -0.5), glm::Vec2::new(0.0, 1.0)),
        Vertex::new(glm::Vec3::new(-0.5, -0.5, -0.5), glm::Vec2::new(0.0, 0.0)),

        Vertex::new(glm::Vec3::new(-0.5, -0.5,  0.5), glm::Vec2::new(0.0, 0.0)),
        Vertex::new(glm::Vec3::new(0.5, -0.5,  0.5), glm::Vec2::new(1.0, 0.0)),
        Vertex::new(glm::Vec3::new(0.5,  0.5,  0.5), glm::Vec2::new(1.0, 1.0)),
        Vertex::new(glm::Vec3::new(0.5,  0.5,  0.5), glm::Vec2::new(1.0, 1.0)),
        Vertex::new(glm::Vec3::new(-0.5,  0.5,  0.5), glm::Vec2::new(0.0, 1.0)),
        Vertex::new(glm::Vec3::new(-0.5, -0.5,  0.5), glm::Vec2::new(0.0, 0.0)),

        Vertex::new(glm::Vec3::new(-0.5,  0.5,  0.5), glm::Vec2::new(1.0, 0.0)),
        Vertex::new(glm::Vec3::new(-0.5,  0.5, -0.5), glm::Vec2::new(1.0, 1.0)),
        Vertex::new(glm::Vec3::new(-0.5, -0.5, -0.5), glm::Vec2::new(0.0, 1.0)),
        Vertex::new(glm::Vec3::new(-0.5, -0.5, -0.5), glm::Vec2::new(0.0, 1.0)),
        Vertex::new(glm::Vec3::new(-0.5, -0.5,  0.5), glm::Vec2::new(0.0, 0.0)),
        Vertex::new(glm::Vec3::new(-0.5,  0.5,  0.5), glm::Vec2::new(1.0, 0.0)),

        Vertex::new(glm::Vec3::new(0.5,  0.5,  0.5), glm::Vec2::new(1.0, 0.0)),
        Vertex::new(glm::Vec3::new(0.5,  0.5, -0.5), glm::Vec2::new(1.0, 1.0)),
        Vertex::new(glm::Vec3::new(0.5, -0.5, -0.5), glm::Vec2::new(0.0, 1.0)),
        Vertex::new(glm::Vec3::new(0.5, -0.5, -0.5), glm::Vec2::new(0.0, 1.0)),
        Vertex::new(glm::Vec3::new(0.5, -0.5,  0.5), glm::Vec2::new(0.0, 0.0)),
        Vertex::new(glm::Vec3::new(0.5,  0.5,  0.5), glm::Vec2::new(1.0, 0.0)),

        Vertex::new(glm::Vec3::new(-0.5, -0.5, -0.5), glm::Vec2::new(0.0, 1.0)),
        Vertex::new(glm::Vec3::new(0.5, -0.5, -0.5), glm::Vec2::new(1.0, 1.0)),
        Vertex::new(glm::Vec3::new(0.5, -0.5,  0.5), glm::Vec2::new(1.0, 0.0)),
        Vertex::new(glm::Vec3::new(0.5, -0.5,  0.5), glm::Vec2::new(1.0, 0.0)),
        Vertex::new(glm::Vec3::new(-0.5, -0.5,  0.5), glm::Vec2::new(0.0, 0.0)),
        Vertex::new(glm::Vec3::new(-0.5, -0.5, -0.5), glm::Vec2::new(0.0, 1.0)),

        Vertex::new(glm::Vec3::new(-0.5,  0.5, -0.5), glm::Vec2::new(0.0, 1.0)),
        Vertex::new(glm::Vec3::new(0.5,  0.5, -0.5), glm::Vec2::new(1.0, 1.0)),
        Vertex::new(glm::Vec3::new(0.5,  0.5,  0.5), glm::Vec2::new(1.0, 0.0)),
        Vertex::new(glm::Vec3::new(0.5,  0.5,  0.5), glm::Vec2::new(1.0, 0.0)),
        Vertex::new(glm::Vec3::new(-0.5,  0.5,  0.5), glm::Vec2::new(0.0, 0.0)),
        Vertex::new(glm::Vec3::new(-0.5,  0.5, -0.5), glm::Vec2::new(0.0, 1.0)),
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

    let triangle = Triangle::new(&shaders, vertices);
    // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

    // create texture
    let texture1 = Texture::from_resource(&res, "textures/wall.jpg");

    shaders.start_using();
    shaders.set_uniform_1i("aTexture1", 0).unwrap();

    let mut camera = Camera::new();

    let mut tick_count: u32 = 0;
    let mut last_tick_ms: f32 = start_time.elapsed().as_secs_f32()  * 1000.0;

    let event_pump = sdl.event_pump().unwrap();
    let mut input_handler = InputHandler::new(event_pump);
    let mut controller = Controller::new();

    'main: loop {
        let current_time_ms = start_time.elapsed().as_secs_f32() * 1000.0;
        let inputs = input_handler.get_input_events();
        if inputs.has_quit {
            break 'main;
        }

        // process mouse inputs
        if let Some((mouse_x, mouse_y)) = inputs.mouse_move {
            camera.rotate(mouse_x, mouse_y, None);
        }
        if let Some(mouse_scroll) = inputs.mouse_scroll {
            camera.zoom(mouse_scroll);
        }

        // process keyboard inputs
        controller.process_input(&inputs);
        if controller.has_requested_quit() {
            break 'main;
        }

        // TICK
        if current_time_ms >= last_tick_ms + TICK_RATE {
            let camera_translate_vec = controller.get_direction_vec(&camera.front(), &camera.up());
            if let Some(vec) = camera_translate_vec {
                camera.translate(camera.speed() * glm::Vec3::new(vec.x, 0.0, vec.z));
            }

            // update tick info
            tick_count += 1;
            last_tick_ms = current_time_ms;
        }

        // render
        unsafe {
            gl::ClearColor(0.8, 0.8, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            texture1.bind();

            let view_transform = camera.get_view_matrix();

            let projection_transform = glm::perspective::<f32>(SCREEN_WIDTH / SCREEN_HEIGHT, camera.fov(), 0.1, 100.0);

            shaders.start_using();
            shaders.set_transform("view", &view_transform).unwrap();
            shaders.set_transform("projection", &projection_transform).unwrap();

            for (i, position) in cube_positions.iter().enumerate() {
                let mut model_transform = glm::Mat4::identity();
                model_transform = glm::translate(&model_transform, &position);
                let model_rotation_vec = glm::Vec3::new(1.0, 0.3, 0.5);
                let angle = match i {
                    2 => last_tick_ms * (PI / 18.0),
                    5 => last_tick_ms * (PI / 18.0),
                    8 => last_tick_ms * (PI / 18.0),
                    _ => (20.0 * i as f32) * (PI / 180.0),
                };
                model_transform = glm::rotate(&model_transform, angle, &model_rotation_vec);
                triangle.draw(model_transform).unwrap();
            }
        }

        window.gl_swap_window();
    }

    let total_run_time = start_time.elapsed().as_secs_f32();
    let average_tick_rate = tick_count as f32 / total_run_time;
    println!("Ran for {}s with {} ticks for a tick rate of {} per second", total_run_time, tick_count, average_tick_rate);
}
