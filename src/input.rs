use sdl2::{
    event::Event,
    keyboard::Keycode,
    mouse::MouseWheelDirection, EventPump
};

pub struct Inputs {
    pub has_quit: bool,
    pub mouse_move: Option<(f32, f32)>,
    pub mouse_scroll: Option<f32>,
    pub pressed_keys: Vec<Keycode>,
    pub released_keys: Vec<Keycode>,
}

pub struct InputHandler {
    event_pump: EventPump,
}

impl InputHandler {
    pub fn new(event_pump: EventPump) -> Self {
        Self {
            event_pump,
        }
    }

    pub fn get_input_events(&mut self) -> Inputs {
        let mut has_quit = false;
        let mut mouse_move = None;
        let mut mouse_scroll = None;
        let mut pressed_keys: Vec<Keycode> = Vec::new();
        let mut released_keys: Vec<Keycode> = Vec::new();
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { timestamp: _ } => has_quit = true,
                Event::MouseMotion { xrel, yrel, timestamp: _, window_id: _, which: _, mousestate: _, x: _, y: _ } => {
                    mouse_move = Some((xrel as f32, -yrel as f32));
                },
                Event::MouseWheel { direction, y, timestamp: _, window_id: _, which: _, x: _, precise_x: _, precise_y: _ } => {
                    match direction {
                        MouseWheelDirection::Normal => {
                            mouse_scroll = Some(-y as f32);
                        },
                        _ => (),
                    };
                },
                Event::KeyDown { keycode, repeat: _, timestamp: _, window_id: _, scancode: _, keymod: _ } => {
                    if let Some(keycode) = keycode {
                        pressed_keys.push(keycode);
                    }
                },
                Event::KeyUp { keycode, repeat: _, timestamp: _, window_id: _, scancode: _, keymod: _ } => {
                    if let Some(keycode) = keycode {
                        released_keys.push(keycode);
                    }
                },
                _ => ()
            }
        }

        Inputs {
            has_quit,
            mouse_move,
            mouse_scroll,
            pressed_keys,
            released_keys,
        }
    }
}
