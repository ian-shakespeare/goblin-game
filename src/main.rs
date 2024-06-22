use nalgebra_glm as glm;
use sdl2;
use std::{path::Path, rc::Rc, sync::Mutex};
use open_gl_test::{
    camera::Camera, components::component_manager::ComponentManager, controller::Controller, entity_manager::EntityManager, input::InputHandler, models::model_manager::{ModelId, ModelManager}, resources::Resources, shader::Shader, systems::render_system::RenderSystem, textures::texture_manager::{TextureId, TextureManager}
};

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
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }

    let shader = Shader::from_resource(&res, "shaders/triangle").unwrap();
    let model_manager = ModelManager::new(&shader);
    let texture_manager = TextureManager::new(&res);

    let entity_manager = Rc::new(Mutex::new(EntityManager::new()));
    let component_manager = Rc::new(Mutex::new(ComponentManager::new()));
    component_manager
        .lock()
        .expect("Could not lock mutex")
        .register_component(open_gl_test::components::ComponentKind::Transform);
    component_manager
        .lock()
        .expect("Could not lock mutex")
        .register_component(open_gl_test::components::ComponentKind::Model);
    component_manager
        .lock()
        .expect("Could not lock mutex")
        .register_component(open_gl_test::components::ComponentKind::Texture);
    let mut render_system = RenderSystem::init(entity_manager, component_manager, model_manager, texture_manager).expect("No render system :(");

    render_system.add_item(
        glm::Vec3::new(5.0, 0.0, 5.0),
        glm::Vec4::new(90.0, 1.0, 0.0, 0.0),
        glm::Vec3::new(10.0, 10.0, 1.0),
        ModelId::Plane,
        TextureId::WoodPlanks,
    ).expect("Could not add to render system");

    render_system.add_item(
        glm::Vec3::new(0.0, 1.0, 0.0),
        glm::Vec4::new(0.0, 0.0, 0.0, 0.0),
        glm::Vec3::new(1.0, 1.0, 1.0),
        ModelId::Cube,
        TextureId::Grass,
    ).expect("Could not add to render system");

    render_system.add_item(
        glm::Vec3::new(1.0, 2.0, 1.0),
        glm::Vec4::new(0.0, 0.0, 0.0, 0.0),
        glm::Vec3::new(1.0, 1.0, 1.0),
        ModelId::Cube,
        TextureId::StoneBricks,
    ).expect("Could not add to render system");

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
                camera.translate(camera.speed() * glm::Vec3::new(vec.x, vec.y, vec.z)); // remove vec.y to not move vertically
            }

            // update tick info
            tick_count += 1;
            last_tick_ms = current_time_ms;
        }

        // render
        unsafe {
            //gl::ClearColor(0.4902, 0.6784, 0.7843, 1.0);
            gl::ClearColor(0.6902, 0.8392, 0.851, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let view_transform = camera.get_view_matrix();
        let projection_transform = glm::perspective::<f32>(SCREEN_WIDTH / SCREEN_HEIGHT, camera.fov(), 0.1, 100.0);

        render_system.draw(&view_transform, &projection_transform).expect("Couldn't draw");
        // box1.draw(&view_transform, &projection_transform);
        // box2.draw(&view_transform, &projection_transform);

        window.gl_swap_window();
    }

    let total_run_time = start_time.elapsed().as_secs_f32();
    let average_tick_rate = tick_count as f32 / total_run_time;
    println!("Ran for {}s with {} ticks for a tick rate of {} per second", total_run_time, tick_count, average_tick_rate);
}
