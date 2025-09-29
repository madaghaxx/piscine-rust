#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThrowObject {
    pub init_position: Object,
    pub init_velocity: Object,
    pub actual_position: Object,
    pub actual_velocity: Object,
    pub time: f32,
}

impl ThrowObject {
    pub fn new(init_position: Object, init_velocity: Object) -> ThrowObject {
        ThrowObject {
            init_position: init_position.clone(),
            init_velocity: init_velocity.clone(),
            actual_position: init_position,
            actual_velocity: init_velocity,
            time: 0.0,
        }
    }
}
impl Iterator for ThrowObject {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        let dt = 1.0;
        let g = 9.8;

        let new_vx = self.actual_velocity.x;
        let new_vy = self.actual_velocity.y - g * dt;

        let new_x = self.actual_position.x + new_vx * dt;
        let new_y = self.actual_position.y + self.actual_velocity.y * dt - 0.5 * g * dt * dt;

        if new_y < 0.0 {
            return None;
        }

        let new_time = self.time + dt;
        fn round_to_decimals(value: f32, decimals: u32) -> f32 {
            let factor = (10_f32).powi(decimals as i32);
            (value * factor).round() / factor
        }

        let new_x = round_to_decimals(new_x, 4);
        let new_y = round_to_decimals(new_y, 4);
        let new_vx = round_to_decimals(new_vx, 4);
        let new_vy = round_to_decimals(new_vy, 4);

        let new_throw = ThrowObject {
            init_position: self.init_position.clone(),
            init_velocity: self.init_velocity.clone(),
            actual_position: Object { x: new_x, y: new_y },
            actual_velocity: Object { x: new_vx, y: new_vy },
            time: new_time,
        };

        *self = new_throw.clone();

        Some(new_throw)
    }
}
