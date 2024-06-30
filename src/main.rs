use goblin_game::{
    components::{
        camera::CameraComponent, collision::CollisionComponent, gravity::GravityComponent,
        mesh::MeshComponent, rigid_body::RigidBodyComponent, transform::TransformComponent,
    },
    constants::{SCREEN_HEIGHT, SCREEN_WIDTH, TICK_RATE},
    ecs::ECS,
    mesh_manager::MeshManager,
    models::{cube::get_cube_mesh, plane::get_plane_mesh},
    resources::Resources,
    shader::Shader,
    systems::{
        controller_system::ControllerSystem, physics_system::PhysicsSystem,
        render_system::RenderSystem, System, SystemError,
    },
    textures::texture_manager::{TextureId, TextureManager},
};
use nalgebra_glm as glm;
use std::{path::Path, sync::Mutex};

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
        gl::Enable(gl::CULL_FACE);
        gl::CullFace(gl::BACK);
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }

    let shader = Shader::from_resource(&res, "shaders/triangle").unwrap();
    let mesh_manager = Mutex::new(MeshManager::new());
    let mut other_tmp = mesh_manager.lock().expect("Could not lock mesh manager.");
    let texture_manager = TextureManager::new(&res);

    let ecs = Mutex::new(ECS::new());
    let mut tmp = ecs.lock().expect("Could not lock ECS.");
    tmp.register_component::<TransformComponent>();
    tmp.register_component::<MeshComponent>();
    tmp.register_component::<RigidBodyComponent>();
    tmp.register_component::<GravityComponent>();
    tmp.register_component::<CollisionComponent>();
    tmp.register_component::<CameraComponent>();

    let grass_texture = texture_manager.get_texture(TextureId::Grass);
    let stone_brick_texture = texture_manager.get_texture(TextureId::StoneBricks);

    let plane_id = other_tmp.add_mesh(get_plane_mesh(vec![stone_brick_texture]));
    let cube_id = other_tmp.add_mesh(get_cube_mesh(vec![grass_texture]));

    drop(other_tmp);

    // Floor
    let model = MeshComponent { id: plane_id };
    let transform = TransformComponent {
        position: glm::Vec3::new(50.0, 0.0, 50.0),
        rotation: None,
        scale: Some(glm::Vec3::new(101.0, 1.0, 101.0)),
    };
    let collision = CollisionComponent { mesh_id: plane_id };
    let floor = tmp.create_entity();
    tmp.add_component(floor, model);
    tmp.add_component(floor, transform);
    tmp.add_component(floor, collision);

    // Wall 1
    let model = MeshComponent { id: plane_id };
    let transform = TransformComponent {
        position: glm::Vec3::new(50.0, 10.0, 50.0),
        rotation: Some(glm::Vec4::new(90.0, 0.0, 0.0, 1.0)),
        scale: Some(glm::Vec3::new(10.0, 1.0, 101.0)),
    };
    let collision = CollisionComponent { mesh_id: plane_id };
    let wall1 = tmp.create_entity();
    tmp.add_component(wall1, model);
    tmp.add_component(wall1, transform);
    tmp.add_component(wall1, collision);

    // Block 1
    let model = MeshComponent { id: cube_id };
    let transform = TransformComponent {
        position: glm::Vec3::new(0.0, 1.0, 0.0),
        rotation: None,
        scale: None,
    };
    let block1 = tmp.create_entity();
    tmp.add_component(block1, model);
    tmp.add_component(block1, transform);

    // Block 2
    let model = MeshComponent { id: cube_id };
    let transform = TransformComponent {
        position: glm::Vec3::new(1.0, 2.0, 1.0),
        rotation: None,
        scale: None,
    };
    let block2 = tmp.create_entity();
    tmp.add_component(block2, model);
    tmp.add_component(block2, transform);

    // Falling Block
    let model = MeshComponent { id: cube_id };
    let transform = TransformComponent {
        position: glm::Vec3::new(0.0, 5.0, 0.0),
        rotation: Some(glm::Vec4::new(45.0, 0.0, 1.0, 0.0)),
        scale: None,
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
    tmp.add_component(falling_block, rigid_body);
    tmp.add_component(falling_block, gravity);

    let transform = TransformComponent {
        position: glm::Vec3::new(0.0, 4.0, 0.0),
        rotation: None,
        scale: None,
    };
    let rigid_body = RigidBodyComponent {
        velocity: glm::Vec3::new(0.0, 0.0, 0.0),
        acceleration: glm::Vec3::new(0.0, 0.0, 0.0),
        collision_x_offset: 0.5,
        collision_y_offset: 2.0,
        collision_z_offset: 0.5,
    };
    let gravity = GravityComponent {
        force: glm::Vec3::new(0.0, -0.001, 0.0),
    };
    let camera_component = CameraComponent {
        front: glm::Vec3::new(1.0, 0.0, 0.0),
        up: glm::Vec3::new(0.0, 1.0, 0.0),
        fov: 45.0,
        yaw: 0.0,
        pitch: 0.0,
    };
    let camera = tmp.create_entity();
    tmp.add_component(camera, transform);
    tmp.add_component(camera, rigid_body);
    tmp.add_component(camera, gravity);
    tmp.add_component(camera, camera_component);

    // Render System
    let mut render_system = RenderSystem::init(&ecs, &mesh_manager, &shader, camera)
        .expect("Could not initialize render system.");
    render_system.add_entity(floor);
    render_system.add_entity(block1);
    render_system.add_entity(block2);
    render_system.add_entity(falling_block);
    render_system.add_entity(wall1);

    // Physics System
    let mut physics_system =
        PhysicsSystem::init(&ecs, &mesh_manager).expect("Could not initialize physics system.");
    physics_system.add_entity(falling_block);
    physics_system.add_entity(camera);

    let event_pump = sdl.event_pump().unwrap();
    // Controller System
    let mut controller_system = ControllerSystem::init(&ecs, event_pump, camera);

    let mut tick_count: u32 = 0;
    let mut last_tick_ms: f32 = start_time.elapsed().as_secs_f32() * 1000.0;

    drop(tmp);

    'main: loop {
        let current_time_ms = start_time.elapsed().as_secs_f32() * 1000.0;

        // TICK - fixed update
        if current_time_ms >= last_tick_ms + TICK_RATE {
            match controller_system.update() {
                Ok(_) => (),
                Err(e) => match e {
                    SystemError::RequestedQuit => break 'main,
                    _ => panic!("Could not update controller system"),
                },
            };

            physics_system
                .update()
                .expect("Could not update physics system.");

            // update tick info
            tick_count += 1;
            last_tick_ms = current_time_ms;
        }

        // render
        unsafe {
            gl::ClearColor(0.6902, 0.8392, 0.851, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        render_system
            .update()
            .expect("Couldn't update render system");

        window.gl_swap_window();
    }

    let total_run_time = start_time.elapsed().as_secs_f32();
    let average_tick_rate = tick_count as f32 / total_run_time;
    println!(
        "Ran for {}s with {} ticks for a tick rate of {} per second",
        total_run_time, tick_count, average_tick_rate
    );
}
