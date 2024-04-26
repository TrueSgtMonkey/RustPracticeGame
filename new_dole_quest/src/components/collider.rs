use bevy::prelude::*;

#[derive(Component)]
pub struct SphereCollider {
    pub radius: f32,
    pub position: Vec2,
}

#[derive(Component)]
pub struct BoxCollider {
    pub width: f32,
    pub height: f32,
    pub position: Vec2,
}

impl BoxCollider {
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

    pub fn gigi_collsison_response(&self, velocity: &mut Vec2, position: &Vec2, width: f32, height: f32){
        let x_max: f32 = position.x + width;
        let y_max: f32 = position.y + height;

        if position.x <= self.position.x && velocity.x > 0.0f32 {
            velocity.x = 0.0;
        }

        if x_max >= self.position.x + self.width && velocity.x < 0.0f32 {
            velocity.x = 0.0;
        }

        if position.y <= self.position.y && velocity.y > 0.0f32 {
            velocity.y = 0.0;
        }

        if y_max >= self.position.y + self.height && velocity.y < 0.0f32 {
            velocity.y = 0.0;
        }
    }
}

impl SphereCollider {

}

