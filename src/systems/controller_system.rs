use super::{System, SystemError};
use crate::{
    components::{camera::CameraComponent, rigid_body::RigidBodyComponent, ComponentValue},
    constants::PLAYER_MOVE_SPEED,
    ecs::ECS,
    entities::Entity,
    utils::{degree_to_radian, flatten_vector},
};
use nalgebra_glm::{self as glm, Vec3};
use sdl2::{event::Event, keyboard::Keycode, mouse::MouseWheelDirection, EventPump};
use std::{collections::HashSet, sync::Mutex};

pub struct ControllerSystem<'a> {
    ecs: &'a Mutex<ECS>,
    entities: HashSet<Entity>,
    event_pump: EventPump,
    camera_entity: Entity,
    forward_backword_input: f32,
    right_left_input: f32,
}

impl<'a> ControllerSystem<'a> {
    pub fn init(ecs: &'a Mutex<ECS>, event_pump: EventPump, camera_entity: Entity) -> Self {
        Self {
            entities: HashSet::new(),
            forward_backword_input: 0.0,
            right_left_input: 0.0,
            ecs,
            event_pump,
            camera_entity,
        }
    }
}

impl<'a> System for ControllerSystem<'a> {
    fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity);
    }

    fn remove_entity(&mut self, entity: Entity) {
        self.entities.remove(&entity);
    }

    fn update(&mut self) -> Result<(), SystemError> {
        let mut ecs = self.ecs.lock().expect("Could not lock ECS");
        let mut camera = ecs
            .get_component::<CameraComponent>(self.camera_entity)
            .expect("Missing camera component");
        let mut camera_rigid_body = ecs
            .get_component::<RigidBodyComponent>(self.camera_entity)
            .expect("Missing camera rigid body");

        let sensitivity: f32 = 0.1;

        // Poll events
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { timestamp: _ } => return Err(SystemError::RequestedQuit),
                Event::MouseMotion {
                    xrel,
                    yrel,
                    timestamp: _,
                    window_id: _,
                    which: _,
                    mousestate: _,
                    x: _,
                    y: _,
                } => {
                    let mouse_x = xrel as f32;
                    let mouse_y = -yrel as f32;
                    camera.yaw += mouse_x * sensitivity;
                    camera.pitch += (mouse_y * sensitivity).clamp(-89.0, 89.0);

                    let mut front = Vec3::identity();
                    front.x =
                        degree_to_radian(camera.yaw).cos() * degree_to_radian(camera.pitch).cos();
                    front.y = degree_to_radian(camera.pitch).sin();
                    front.z =
                        degree_to_radian(camera.yaw).sin() * degree_to_radian(camera.pitch).cos();
                    camera.front = glm::normalize(&front);
                    let right = glm::normalize(&glm::cross::<f32, glm::U3>(
                        &front,
                        &Vec3::new(0.0, 1.0, 0.0),
                    ));
                    camera.up = glm::normalize(&glm::cross::<f32, glm::U3>(&right, &front));
                }
                Event::MouseWheel {
                    direction,
                    y,
                    timestamp: _,
                    window_id: _,
                    which: _,
                    x: _,
                    precise_x: _,
                    precise_y: _,
                } => {
                    match direction {
                        MouseWheelDirection::Normal => {
                            let degrees = -y as f32;
                            camera.fov = (camera.fov + degrees).clamp(1.0, 90.0);
                        }
                        _ => (),
                    };
                }
                Event::KeyDown {
                    keycode,
                    repeat,
                    timestamp: _,
                    window_id: _,
                    scancode: _,
                    keymod: _,
                } => {
                    if repeat {
                        continue;
                    }

                    if let Some(keycode) = keycode {
                        match keycode {
                            Keycode::W => {
                                self.forward_backword_input += 1.0;
                            }
                            Keycode::A => {
                                self.right_left_input -= 1.0;
                            }
                            Keycode::S => {
                                self.forward_backword_input -= 1.0;
                            }
                            Keycode::D => {
                                self.right_left_input += 1.0;
                            }
                            Keycode::Escape => {
                                return Err(SystemError::RequestedQuit);
                            }
                            _ => (),
                        };
                    }
                }
                Event::KeyUp {
                    keycode,
                    repeat,
                    timestamp: _,
                    window_id: _,
                    scancode: _,
                    keymod: _,
                } => {
                    if repeat {
                        continue;
                    }

                    if let Some(keycode) = keycode {
                        match keycode {
                            Keycode::W => {
                                self.forward_backword_input -= 1.0;
                            }
                            Keycode::A => {
                                self.right_left_input += 1.0;
                            }
                            Keycode::S => {
                                self.forward_backword_input += 1.0;
                            }
                            Keycode::D => {
                                self.right_left_input -= 1.0;
                            }
                            _ => (),
                        };
                    }
                }
                _ => (),
            }
        }

        let front = Vec3::new(camera.front.x, 0.0, camera.front.z);
        let cross = front.cross(&camera.up);
        let right = Vec3::new(cross.x, 0.0, cross.z);

        let movement =
            flatten_vector(front * self.forward_backword_input + right * self.right_left_input);

        camera_rigid_body.force += PLAYER_MOVE_SPEED * movement;

        ecs.set_component(self.camera_entity, ComponentValue::Camera(camera));
        ecs.set_component(
            self.camera_entity,
            ComponentValue::RigidBody(camera_rigid_body),
        );

        Ok(())
    }
}
