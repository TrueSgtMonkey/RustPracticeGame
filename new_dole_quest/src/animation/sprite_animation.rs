use bevy::prelude::*; // need this even in submodules

#[derive(Component)]
pub struct AnimatedEntity {
    pub frame_ready: bool,
    pub frame_num: usize,
    pub curr_ani_start_frame: usize,
    pub total_sprite_frames: usize,
    pub hframes: usize,
    pub vframes: usize,
    pub curr_frame_increase: isize,
    pub angle_increase: usize,
    pub timer: Timer,
}

struct IsoAnimationParams {
    pub curr_frame:     usize,
    pub row:            usize,
    pub angle_increase: usize,
    pub curr_angle:     usize,
    pub start_angle:    usize,
}

impl Default for IsoAnimationParams {
    fn default() -> Self {
        Self {
            curr_frame: 0,
            row: 0,
            angle_increase: 1,
            curr_angle: 0,
            start_angle: 0,
        }
    }
}

impl AnimatedEntity {
    /**
        This function changes the current frame by frame_increase and ensures
        the frame_num is in bounds of the sprite sheet
    */
    pub fn animation(
        &mut self, 
        st_frame: usize, 
        max_frame: usize, 
        frame_increase: isize) -> usize 
    {
        let addition = 0usize;
        let addition: usize = addition.wrapping_add_signed(frame_increase.into());
        if st_frame < max_frame {
            // have to use this stupid bullshit function simply for adding signed values
            self.frame_num += addition;
            if self.frame_num > max_frame {
                self.frame_num = st_frame;
            }
        } else if st_frame > max_frame {
            // play the animation backwards if max_frame is less than st_frame
            self.frame_num -= addition;

            // check if greater than st_frame in case of wrap-around
            if self.frame_num < max_frame || self.frame_num > st_frame {
                self.frame_num = st_frame;
            }
        }

        self.frame_num = min(self.total_sprite_frames, self.frame_num);
        return self.frame_num;
    }
    /**
        This function increases the animation by a specified number of frames
        (default=1) taking into account the angle (or row) passed into the function.

        Passing negative value into frameIncrease plays the animation backwards
    */
    pub fn iso_animation(
        &mut self,
        angle: usize,
        st_angle: usize,
        frame_increase: isize,
        angle_increase: usize,)
    {
        if angle_increase == 0 {
            return;
        }

        let mut iso_params = IsoAnimationParams {
            start_angle: st_angle,
            curr_angle: angle,
            angle_increase: angle_increase,
            ..Default::default()
        };

        self.get_iso_angle_helper(&mut iso_params);

        let mut start_frame: usize = iso_params.row * self.hframes;
        let mut end_frame: usize = (iso_params.row + angle_increase) * self.hframes - 1;
        let is_decreasing = frame_increase < 0;

        let frame_increase: isize = if is_decreasing {
            let temp_frame: usize = start_frame;
            start_frame = end_frame;
            end_frame = temp_frame;
            -frame_increase
        } else { 
            frame_increase
        };

        self.animation(start_frame, end_frame, frame_increase);
    }

    /**
         This function sets the frame based on the current frame and angle. This
        allows us to play the animation from any angle.
    */
    fn get_iso_angle_helper(
        &mut self,
        iso_params: &mut IsoAnimationParams,
    )
    {
        // getting current frame of animation playing -- regardless of angle
        iso_params.curr_frame = self.frame_num % (self.hframes * iso_params.angle_increase);

        // shifting angle -- usually used for playing separate animations
        // angle_increase represents the amount of rows the animation takes up
        iso_params.row = iso_params.start_angle + iso_params.curr_angle * iso_params.angle_increase;

        // finding the correct 
        self.frame_num = iso_params.curr_frame + (iso_params.curr_angle * self.hframes * iso_params.angle_increase);
        self.frame_num += iso_params.start_angle * self.hframes;
    }
}

impl Default for AnimatedEntity {
    fn default() -> Self {
        Self {
            frame_ready: true,
            frame_num: 0,
            curr_ani_start_frame: 0,
            total_sprite_frames: 1,
            hframes: 1,
            vframes: 1,
            curr_frame_increase: 1,
            angle_increase: 1,
            timer: Timer::default(),
        }
    }
}

// fn max(x: usize, y: usize) -> usize {
//     if x < y {
//         return y;
//     }
//     return x;
// }

fn min(x: usize, y: usize) -> usize {
    if x < y {
        return x;
    }
    return y;
}
