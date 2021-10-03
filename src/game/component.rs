use specs::{Component, NullStorage, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Transform {
    pub position: glam::Vec2,
    pub rotation: glam::Quat,
    pub scale: glam::Vec2,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: glam::Vec2::ZERO,
            rotation: glam::Quat::IDENTITY,
            scale: glam::Vec2::ONE * 50.0,
        }
    }
}

impl Transform {
    const UP: glam::Vec3 = glam::Vec3::Y;

    pub fn to_model_mat(&self) -> [[f32; 4]; 4] {
        // @TODO Can be cached
        let vec3_pos = glam::Vec3::new(self.position.x, self.position.y, -1.0);
        let vec3_scale = glam::Vec3::new(self.scale.x, self.scale.y, 1.0);
        (glam::Mat4::from_translation(vec3_pos)
            * glam::Mat4::from_scale(vec3_scale)
            * glam::Mat4::from_quat(self.rotation))
        .to_cols_array_2d()
    }

    pub fn get_facing_vector(&self) -> glam::Vec2 {
        let facing = self.rotation * Transform::UP;
        glam::vec2(facing.x, facing.y)
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub direction: glam::Vec2,
    pub speed: f32,
}

impl Default for Velocity {
    fn default() -> Self {
        Self {
            direction: glam::Vec2::Y,
            speed: Default::default(),
        }
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Display {
    pub sprite_idx: u32,
}

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Player;
