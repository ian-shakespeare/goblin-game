use nalgebra_glm as glm;
use open_gl_test::{
    camera::Camera,
    components::{
        collision::CollisionComponent, gravity::GravityComponent, mesh::MeshComponent,
        rigid_body::RigidBodyComponent, texture::TextureComponent, transform::TransformComponent,
    },
    controller::Controller,
    ecs::ECS,
    input::InputHandler,
    mesh_manager::MeshManager,
    models::{cube::get_cube_mesh, plane::get_plane_mesh},
    resources::Resources,
    shader::Shader,
    systems::{physics_system::PhysicsSystem, render_system::RenderSystem},
    textures::texture_manager::{TextureId, TextureManager},
};
use sdl2::{self, keyboard::Keycode};
use std::{path::Path, sync::Mutex};

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
        .expect("Could not create video subsystem.");

    sdl.mouse().set_relative_mouse_mode(true);

    let _gl_context = window.gl_create_context().unwrap();
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::Viewport(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
        gl::Enable(gl::DEPTH_TEST);
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }

    let shader = Shader::from_resource(&res, "shaders/triangle").unwrap();
    let mut mesh_manager = MeshManager::new();
    let texture_manager = TextureManager::new(&res);

    let ecs = Mutex::new(ECS::new());
    let mut tmp = ecs.lock().expect("Could not lock ECS.");
    tmp.register_component::<TransformComponent>();
    tmp.register_component::<MeshComponent>();
    tmp.register_component::<TextureComponent>();
    tmp.register_component::<RigidBodyComponent>();
    tmp.register_component::<GravityComponent>();
    tmp.register_component::<CollisionComponent>();

    let grass_texture = texture_manager.get_texture(TextureId::Grass);
    let stone_brick_texture = texture_manager.get_texture(TextureId::StoneBricks);

    let plane_id = mesh_manager.add_mesh(get_plane_mesh(vec![stone_brick_texture]));
    let cube_id = mesh_manager.add_mesh(get_cube_mesh(vec![grass_texture]));

    // Floor
    let model = MeshComponent { id: plane_id };
    let transform = TransformComponent {
        position: glm::Vec3::new(0.0, 0.0, 0.0),
        rotation: glm::Vec4::new(0.0, 0.0, 0.0, 0.0),
        scale: glm::Vec3::new(1.0, 1.0, 1.0),
    };
    let texture = TextureComponent {
        id: TextureId::WoodPlanks,
    };
    let floor = tmp.create_entity();
    tmp.add_component(floor, model);
    tmp.add_component(floor, transform);
    tmp.add_component(floor, texture);

    // Floor Collider
    let collision = CollisionComponent {
        normal: glm::Vec3::new(0.0, 1.0, 0.0),
        position: glm::Vec3::new(0.0, 0.0, 0.0),
        vertices: [
            glm::Vec3::new(5.0, 0.0, 0.0),
            glm::Vec3::new(-5.0, 0.0, 5.0),
            glm::Vec3::new(-5.0, 0.0, -5.0),
        ],
    };
    let floor_collider = tmp.create_entity();
    tmp.add_component(floor_collider, collision);

    // Block 1
    let model = MeshComponent { id: cube_id };
    let transform = TransformComponent {
        position: glm::Vec3::new(0.0, 1.0, 0.0),
        rotation: glm::Vec4::new(0.0, 0.0, 0.0, 0.0),
        scale: glm::Vec3::new(1.0, 1.0, 1.0),
    };
    let texture = TextureComponent {
        id: TextureId::Grass,
    };
    let block1 = tmp.create_entity();
    tmp.add_component(block1, model);
    tmp.add_component(block1, transform);
    tmp.add_component(block1, texture);

    // Block 2
    let model = MeshComponent { id: cube_id };
    let transform = TransformComponent {
        position: glm::Vec3::new(1.0, 2.0, 1.0),
        rotation: glm::Vec4::new(0.0, 0.0, 0.0, 0.0),
        scale: glm::Vec3::new(1.0, 1.0, 1.0),
    };
    let texture = TextureComponent {
        id: TextureId::StoneBricks,
    };
    let block2 = tmp.create_entity();
    tmp.add_component(block2, model);
    tmp.add_component(block2, transform);
    tmp.add_component(block2, texture);

    // Falling Block
    let model = MeshComponent { id: cube_id };
    let transform = TransformComponent {
        position: glm::Vec3::new(0.0, 5.0, 0.0),
        rotation: glm::Vec4::new(45.0, 0.0, 1.0, 0.0),
        scale: glm::Vec3::new(1.0, 1.0, 1.0),
    };
    let texture = TextureComponent {
        id: TextureId::StoneBricks,
    };
    let rigid_body = RigidBodyComponent {
        acceleration: glm::Vec3::new(0.0, 0.0, 0.0),
        collision_x_offset: 1.0,
        collision_y_offset: 1.0,
        collision_z_offset: 1.0,
        velocity: glm::Vec3::new(0.0, 0.0, 0.0),
    };
    let gravity = GravityComponent {
        force: glm::Vec3::new(0.0, -0.001, 0.0),
    };
    let falling_block = tmp.create_entity();
    tmp.add_component(falling_block, model);
    tmp.add_component(falling_block, transform);
    tmp.add_component(falling_block, texture);
    tmp.add_component(falling_block, rigid_body);
    tmp.add_component(falling_block, gravity);

    // Render System
    let mut render_system = RenderSystem::init(&ecs, mesh_manager, &shader)
        .expect("Could not initialize render system.");
    render_system.add_entity(floor);
    render_system.add_entity(block1);
    render_system.add_entity(block2);
    render_system.add_entity(falling_block);

    // Physics System
    let mut physics_system =
        PhysicsSystem::init(&ecs).expect("Could not initialize physics system.");
    physics_system.add_entity(falling_block);

    // Camera
    let mut camera = Camera::new();

    let mut tick_count: u32 = 0;
    let mut last_tick_ms: f32 = start_time.elapsed().as_secs_f32() * 1000.0;

    let event_pump = sdl.event_pump().unwrap();
    let mut input_handler = InputHandler::new(event_pump);
    let mut controller = Controller::new();

    drop(tmp);

    'main: loop {
        let current_time_ms = start_time.elapsed().as_secs_f32() * 1000.0;
        let inputs = input_handler.get_input_events();
        if inputs.has_quit {
            break 'main;
        }

        if inputs.pressed_keys.contains(&Keycode::X) {
            let mut ecs = ecs.lock().expect("Couldn't lock ecs.");
            render_system.remove_entity(block1);
            ecs.destroy_entity(block1);

            render_system.remove_entity(block2);
            ecs.destroy_entity(block2);

            render_system.remove_entity(falling_block);
            physics_system.remove_entity(falling_block);
            ecs.destroy_entity(falling_block);
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

        // TICK - fixed update
        if current_time_ms >= last_tick_ms + TICK_RATE {
            let camera_translate_vec = controller.get_direction_vec(&camera.front(), &camera.up());
            if let Some(vec) = camera_translate_vec {
                camera.translate(camera.speed() * glm::Vec3::new(vec.x, vec.y, vec.z));
                // remove vec.y to not move vertically
            }

            physics_system
                .update()
                .expect("Could not update physics system.");

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
        let projection_transform =
            glm::perspective::<f32>(SCREEN_WIDTH / SCREEN_HEIGHT, camera.fov(), 0.1, 100.0);

        render_system
            .draw(&view_transform, &projection_transform)
            .expect("Couldn't draw");

        window.gl_swap_window();
    }

    let total_run_time = start_time.elapsed().as_secs_f32();
    let average_tick_rate = tick_count as f32 / total_run_time;
    println!(
        "Ran for {}s with {} ticks for a tick rate of {} per second",
        total_run_time, tick_count, average_tick_rate
    );
}
