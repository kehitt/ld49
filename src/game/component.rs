use specs::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Transform {
    pub pos: glam::Vec3,
    pub rot: glam::Quat,
}

impl Transform {
    pub fn to_model_mat(&self) -> [[f32; 4]; 4] {
        // @TODO Can be cached
        (glam::Mat4::from_translation(self.pos) * glam::Mat4::from_quat(self.rot))
            .to_cols_array_2d()
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Display {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
