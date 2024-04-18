use bevy::prelude::*;

#[derive(Component)]
pub struct Collider {
    pub width: f32,
    pub height: f32,
    pub position: Vec2,
}

impl Collider {
    pub fn is_colliding(&self, position: &Vec2, width: f32, height: f32) -> bool {
        let x_max: f32 = position.x + width;
        let y_max: f32 = position.y + height;

        if
            (x_max > self.position.x && position.x < (self.position.x + self.width)) &&
            (y_max > self.position.y && position.y < (self.position.y + self.height))
        {
            return true;
        }
        return false;
    }

    pub fn collision_response(&self, position: &Vec2) -> Vec2 {
        let dist_vec: Vec2 = Vec2 {
            x: position.x - self.position.x,
            y: position.y - self.position.y
        };

        return dist_vec.normalize_or_zero();
    }
}

