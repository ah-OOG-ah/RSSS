#![cfg_attr(target_arch = "spirv", no_std)]
#![feature(lang_items)]
#![feature(lang_items)]
#![feature(register_attr)]
#![register_attr(spirv)]

use spirv_std::glam::{const_vec3, Vec2, Vec3, Vec4};
use spirv_std::storage_class::{Input, Output, UniformConstant};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

#[allow(unused_attributes)]
#[spirv(fragment)]
pub fn main_fs(
	#[spirv(frag_coord)] frag_coord : Input<Vec4>,
	u_time : UniformConstant<f32>,
	u_resolution : UniformConstant<Vec2>,
	mut output : Output<Vec4>
) {
	let coord = frag_coord.load();
	let res = u_resolution.load();

	let uv = Vec2::new(coord.x / res.x, coord.y / res.y);
	let uv_swizzle = Vec3::new(uv.y, uv.x, uv.y); // does rust-gpu support swizzling?
	
	let color = Vec3::splat(0.5) + 0.5 * (u_time.load().cos() * uv_swizzle);
	
    output.store(color.extend(1.0))
}