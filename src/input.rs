use sdl2::{event::Event, keyboard::Keycode, EventPump};

pub enum InputError {
    UnknownKeycode,
}

pub struct Input<Q, F, M> {
    event_pump: EventPump,
    on_quit: Q,
    on_key_down: F,
    on_mouse_move: M,
}

impl<Q, F, M> Input<Q, F, M>
where
    F: Fn(Keycode),
    Q: Fn(),
    M: Fn(f32, f32),
{
    pub fn new(event_pump: EventPump, on_quit: Q, on_key_down: F, on_mouse_move: M) -> Self {
        Self {
            event_pump,
            on_quit,
            on_key_down,
            on_mouse_move,
        }
    }

    pub fn handle_events(&mut self) -> Result<(), InputError> {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { timestamp: _ } => (self.on_quit)(),
                Event::KeyDown { timestamp: _, window_id: _, scancode: _, keymod: _, repeat: _, keycode } => {
                    if let Some(key) = keycode {
                        (self.on_key_down)(key);
                    } else {
                        return Err(InputError::UnknownKeycode);
                    }
                },
                Event::MouseMotion { timestamp: _, window_id: _, which: _, mousestate: _, x: _, y: _, xrel, yrel } => {
                    (self.on_mouse_move)(xrel as f32, yrel as f32);
                },
                _ => (),
            }
        }

        Ok(())
    }
}
