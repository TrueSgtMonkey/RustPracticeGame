use bevy::prelude::*;
use std::f32::consts::PI;

const NUM_UNIT_ANGLES: usize = 16;
const Y_AXIS: usize = 8;

// since enums suck in rust
pub const EAST: usize      = 0;
pub const NORTHEAST: usize = 1;
pub const NORTH: usize     = 2;
pub const NORTHWEST: usize = 3;
pub const WEST: usize      = 4;
pub const SOUTHWEST: usize = 5;
pub const SOUTH: usize     = 6;
pub const SOUTHEAST: usize = 7;
pub const ERROR: usize     = 8;

#[derive(Resource)]
pub struct UnitAnglesEights {
    pub num_angles: usize,
    angles: [f32; NUM_UNIT_ANGLES]
}

// TODO: Backport this back into Godot because this is FAR more efficient to
//       just do at the beginning of the game -- probably make a new class for
//       it in Godot with these and put it into a singleton that everyone
//       can access??
impl UnitAnglesEights {
    pub fn new() -> Self {
        Self {
            num_angles: NUM_UNIT_ANGLES,
            angles: [
                // x-axis
                f32::cos( PI         * 0.125f32) * f32::cos(PI         * 0.125f32), //0
                f32::cos( 3f32  * PI * 0.125f32) * f32::cos(3f32  * PI * 0.125f32), //1
                -f32::cos(5f32  * PI * 0.125f32) * f32::cos(5f32  * PI * 0.125f32), //2 - negative x-axis
                -f32::cos(7f32  * PI * 0.125f32) * f32::cos(7f32  * PI * 0.125f32), //3 - negative x-axis
                -f32::cos(9f32  * PI * 0.125f32) * f32::cos(9f32  * PI * 0.125f32), //4 - negative x-axis
                -f32::cos(11f32 * PI * 0.125f32) * f32::cos(11f32 * PI * 0.125f32), //5 - negative x-axis
                f32::cos( 13f32 * PI * 0.125f32) * f32::cos(13f32 * PI * 0.125f32), //6
                f32::cos( 15f32 * PI * 0.125f32) * f32::cos(15f32 * PI * 0.125f32), //7

                //y-axis
                f32::sin( PI         * 0.125f32) * f32::sin(PI         * 0.125f32), //0
                f32::sin( 3f32  * PI * 0.125f32) * f32::sin(3f32  * PI * 0.125f32), //1
                f32::sin( 5f32  * PI * 0.125f32) * f32::sin(5f32  * PI * 0.125f32), //2
                f32::sin( 7f32  * PI * 0.125f32) * f32::sin(7f32  * PI * 0.125f32), //3
                -f32::sin(9f32  * PI * 0.125f32) * f32::sin(9f32  * PI * 0.125f32), //4 - negative y-axis
                -f32::sin(11f32 * PI * 0.125f32) * f32::sin(11f32 * PI * 0.125f32), //5 - negative y-axis
                -f32::sin(13f32 * PI * 0.125f32) * f32::sin(13f32 * PI * 0.125f32), //6 - negative y-axis
                -f32::sin(15f32 * PI * 0.125f32) * f32::sin(15f32 * PI * 0.125f32), //7 - negative y-axis
            ]
        }
    }

    /**
        Assuming that direction is normalized, returns a number (0-7)
        representing the "angle" that a sprite is facing.
    */
    pub fn get_numeric_direction_angle(&self, direction: &Vec2) -> usize {
        if is_between(direction, -0.01, 0.01, -0.01, 0.01) {
            return EAST;
        }

        if is_between(direction, self.angles[0], 1f32, self.angles[Y_AXIS+7], self.angles[Y_AXIS]) {
            return EAST;
        } else if is_between(direction, self.angles[1], self.angles[0], self.angles[Y_AXIS], self.angles[Y_AXIS+1]) {
            return NORTHEAST;
        } else if is_between(direction, self.angles[2], self.angles[1], self.angles[Y_AXIS+1], 1f32) {
            return NORTH;
        } else if is_between(direction, self.angles[3], self.angles[2], self.angles[Y_AXIS+3], self.angles[Y_AXIS+2]) {
            return NORTHWEST;
        } else if is_between(direction, -1f32, self.angles[3], self.angles[Y_AXIS+4], self.angles[Y_AXIS+3]) {
            return WEST;
        } else if is_between(direction, self.angles[4], self.angles[5], self.angles[Y_AXIS+5], self.angles[Y_AXIS+4]) {
            return SOUTHWEST;
        } else if is_between(direction, self.angles[5], self.angles[6], -1f32, self.angles[Y_AXIS+6]) {
            return SOUTH;
        } else if is_between(direction, self.angles[6], self.angles[7], self.angles[Y_AXIS+6], self.angles[Y_AXIS+7]) {
            return SOUTHEAST;
        } else {
            println!("ERROR: pass in normalized vector to crate::components::character::unit_angle::UnitAnglesEights::get_numeric_direction_angle()!");
            return ERROR;
        }
    }
}

fn is_between(direction: &Vec2, low_x: f32, high_x: f32, low_y: f32, high_y: f32) -> bool {
    (direction.x >= low_x && direction.x <= high_x) &&
        (direction.y >= low_y && direction.y <= high_y)
}
