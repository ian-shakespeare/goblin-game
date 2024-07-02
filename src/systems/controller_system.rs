use super::{System, SystemError};
use crate::{
    components::{controllable::Controllable, rigid_body::RigidBody},
    constants::PLAYER_MOVE_SPEED,
    ecs::Ecs,
    utils::flatten_vector,
};
use sdl2::{event::Event, keyboard::Keycode, EventPump};
use std::sync::Mutex;

pub struct ControllerSystem<'a> {
    ecs: &'a Mutex<Ecs>,
    event_pump: EventPump,
}

impl<'a> ControllerSystem<'a> {
    pub fn init(ecs: &'a Mutex<Ecs>, event_pump: EventPump) -> Self {
        Self { ecs, event_pump }
    }
}

impl<'a> System for ControllerSystem<'a> {
    fn update(&mut self) -> Result<(), SystemError> {
        let ecs = self.ecs.lock().expect("Could not lock ECS");

        let mut forward_motion: f32 = 0.0;
        let mut horizontal_motion: f32 = 0.0;
        let mut rotate_x = 0.0;
        let mut rotate_y = 0.0;

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
                    rotate_x = xrel as f32;
                    rotate_y = -yrel as f32;
                }
                /*Event::MouseWheel {
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
                            camera.zoom(-y as f32);
                        }
                        _ => (),
                    };
                }*/
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
                                forward_motion += 1.0;
                            }
                            Keycode::A => {
                                horizontal_motion -= 1.0;
                            }
                            Keycode::S => {
                                forward_motion -= 1.0;
                            }
                            Keycode::D => {
                                horizontal_motion += 1.0;
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
                                forward_motion -= 1.0;
                            }
                            Keycode::A => {
                                horizontal_motion += 1.0;
                            }
                            Keycode::S => {
                                forward_motion += 1.0;
                            }
                            Keycode::D => {
                                horizontal_motion -= 1.0;
                            }
                            _ => (),
                        };
                    }
                }
                _ => (),
            }
        }

        let mut controllables = ecs
            .get_component_vec::<Controllable>()
            .expect("Could not get controlled vector");
        let mut rigid_body = ecs
            .get_component_vec::<RigidBody>()
            .expect("Could not get rigid body vector");

        let union = controllables
            .iter_mut()
            .zip(rigid_body.iter_mut())
            .filter_map(|(controlled, rigid_body)| {
                Some((controlled.as_mut()?, rigid_body.as_mut()?))
            });

        for (controlled, rigid_body) in union {
            controlled.rotate(rotate_x, rotate_y);
            controlled.apply_motion(forward_motion, horizontal_motion);

            let front = flatten_vector(controlled.facing());
            let right = flatten_vector(controlled.perpendicular());

            let movement = flatten_vector(
                front * controlled.forward_motion() + right * controlled.horizontal_motion(),
            );

            rigid_body.apply_force(PLAYER_MOVE_SPEED * movement);
        }

        Ok(())
    }
}
