use derive_new::new;
use half::f16;

// Maybe need to make these structs for PartialEq, new, hash (for compression over net)

#[repr(C)]
#[derive(Debug, new)]
pub struct Vec4f([f32; 4]);
#[repr(C)]
#[derive(Debug, new)]
pub struct Vec3f([f32; 3]);
#[repr(C)]
#[derive(Debug, new)]
pub struct Vec2f([f32; 2]);

#[repr(C)]
#[derive(Debug, new)]
pub struct Vec4fh([f16; 4]);
#[repr(C)]
#[derive(Debug, new)]
pub struct Vec3fh([f16; 3]);
#[repr(C)]
#[derive(Debug, new)]
pub struct Vec2fh([f16; 2]);

#[repr(C)]
#[derive(Debug, new)]
pub struct Vec4([u32; 4]);
#[repr(C)]
#[derive(Debug, new)]
pub struct Vec3([u32; 3]);
#[repr(C)]
#[derive(Debug, new)]
pub struct Vec2([u32; 2]);

#[repr(C)]
#[derive(Debug, new)]
pub struct Vec4s([u16; 4]);
#[repr(C)]
#[derive(Debug, new)]
pub struct Vec3s([u16; 3]);
#[repr(C)]
#[derive(Debug, new)]
pub struct Vec2s([u16; 2]);

// BYTES

#[repr(C)]
#[derive(Debug, new)]
pub struct Vec4b([u8; 4]);
#[repr(C)]
#[derive(Debug, new)]
pub struct Vec3b([u8; 3]);
#[repr(C)]
#[derive(Debug, new)]
pub struct Vec2b([u8; 2]);
