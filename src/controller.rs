use sdl2::keyboard::Keycode;
use crate::input::Inputs;
use nalgebra_glm as glm;

pub const QUIT_KEY: Keycode = Keycode::Escape;
pub const FORWARD_KEY: Keycode = Keycode::W;
pub const BACKWARD_KEY: Keycode = Keycode::S;
pub const LEFT_KEY: Keycode = Keycode::A;
pub const RIGHT_KEY: Keycode = Keycode::D;

pub enum ControllerDirection {
    Stationary,
    Forward,
    ForwardRight,
    Right,
    BackwardRight,
    Backward,
    BackwardLeft,
    Left,
    ForwardLeft,
}

pub struct Controller {
    has_requested_quit: bool,
    is_moving_forward: bool,
    is_moving_backward: bool,
    is_moving_left: bool,
    is_moving_right: bool,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            has_requested_quit: false,
            is_moving_forward: false,
            is_moving_backward: false,
            is_moving_left: false,
            is_moving_right: false,
        }
    }

    pub fn process_input(&mut self, inputs: &Inputs) {
        if inputs.has_quit {
            self.has_requested_quit = true;
        }
        for key in &inputs.pressed_keys {
            match *key {
                QUIT_KEY => {
                    self.has_requested_quit = true;
                },
                FORWARD_KEY => {
                    self.is_moving_forward = true;
                },
                BACKWARD_KEY => {
                    self.is_moving_backward = true;
                },
                LEFT_KEY => {
                    self.is_moving_left = true;
                },
                RIGHT_KEY => {
                    self.is_moving_right = true;
                },
                _ => (),
            }
        }
        for key in &inputs.released_keys {
            match *key {
                FORWARD_KEY => {
                    self.is_moving_forward = false;
                },
                BACKWARD_KEY => {
                    self.is_moving_backward = false;
                },
                LEFT_KEY => {
                    self.is_moving_left = false;
                },
                RIGHT_KEY => {
                    self.is_moving_right = false;
                },
                _ => (),
            }
        }
    }

    pub fn get_direction_vec(&self, front: &glm::Vec3, up: &glm::Vec3) -> Option<glm::Vec3> {
        let mut net_direction = glm::Vec3::new(0.0, 0.0, 0.0);

        if self.is_moving_forward {
            let forward_vec = front;
            net_direction += forward_vec;
        }
        if self.is_moving_backward {
            let backward_vec = -front;
            net_direction += backward_vec;
        }
        if self.is_moving_left {
            let cross = glm::cross::<f32, glm::U3>(front, up);
            let left_vec = -glm::normalize(&cross);
            net_direction += left_vec;
        }
        if self.is_moving_right {
            let cross = glm::cross::<f32, glm::U3>(front, up);
            let right_vec = glm::normalize(&cross);
            net_direction += right_vec;
        }

        if net_direction.x == 0.0 && net_direction.y == 0.0 && net_direction.z == 0.0 {
            None
        } else {
            Some(net_direction)
        }
    }

    pub fn has_requested_quit(&self) -> bool {
        self.has_requested_quit
    }
}
