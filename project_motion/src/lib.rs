/// A simple 2D point (used for both position and velocity).
#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub x: f32,
    pub y: f32,
}


#[derive(Debug, Clone, PartialEq)]
pub struct ThrowObject {
    pub initial_position: Object,
    pub initial_velocity: Object,
    pub current_position: Object,
    pub current_velocity: Object,
    pub time: f32,
}

impl ThrowObject {
    pub fn new(initial_position: Object, initial_velocity: Object) -> ThrowObject {
        ThrowObject {
            initial_position: initial_position.clone(),
            initial_velocity: initial_velocity.clone(),
            current_position: initial_position.clone(),
            current_velocity: initial_velocity.clone(),
            time: 0.0,
        }
    }
}

impl Iterator for ThrowObject {
    type Item = ThrowObject;

    fn next(&mut self) -> Option<ThrowObject> {
        self.time += 1.0;

        
        let new_position = Object {
            x: round_two(self.initial_position.x + self.initial_velocity.x * self.time),
            y: round_two(
                self.initial_position.y
                    + self.initial_velocity.y * self.time
                    - 0.5 * 9.8 * self.time * self.time,
            ),
        };

       
        let new_velocity = Object {
            x: round_two(self.initial_velocity.x),
            y: round_two(self.initial_velocity.y - 9.8 * self.time),
        };

        if new_position.y < 0.0 {
            return None;
        }

        Some(ThrowObject {
            initial_position: self.initial_position.clone(),
            initial_velocity: self.initial_velocity.clone(),
            current_position: new_position,
            current_velocity: new_velocity,
            time: self.time,
        })
    }
}

fn round_two(n: f32) -> f32 {
    (n * 100.0).round() / 100.0
}
