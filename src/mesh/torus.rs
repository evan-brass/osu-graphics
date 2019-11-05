use crate::mesh::mesh::Mesh;

use crate::gl;
use gl::types::*;

use std::f32::consts::PI;

pub struct Torus {
	pub major_radius: f32,
	pub major_segments: u32,
	pub minor_radius: f32,
	pub minor_segments: u32
}
impl Torus {
	pub fn new() -> Self {
		Self {
			major_radius: 3.0,
			major_segments: 16,
			minor_radius: 1.0,
			minor_segments: 6
		}
	}
}
impl Mesh for Torus {
	fn init(&mut self) {}
	fn draw(&mut self) {
		let major_angle = 2.0 * PI / self.major_segments as f32;
		let minor_angle = 2.0 * PI / self.minor_segments as f32;
		unsafe {
			gl::Color3f(0.0, 255.0, 255.0);
			// gl::Enable(gl::CULL_FACE);
			gl::Begin(gl::QUADS);
		}
		for a in 0..self.major_segments {
			let angle_1 = (a as f32) * major_angle;
			let angle_2 = (a as f32 + 1.0) * major_angle;
			for i in 0..self.minor_segments {
				let angle_3 = (i as f32) * minor_angle;
				let angle_4 = (i as f32 + 1.0) * minor_angle;
				fn do_single(a1: f32, a2: f32, ar: f32, ir: f32) {
					let length = ar - ir * a2.sin();
					unsafe {
						gl::Normal3f(
							- a1.sin() * a2.sin(),
							a2.cos(),
							- a1.cos() * a2.sin()
						);
						gl::Vertex3f(
							length * a1.sin(),
							ir * a2.cos(),
							length * a1.cos(),
						);
					}
				}
				do_single(angle_1, angle_4, self.major_radius, self.minor_radius);
				do_single(angle_1, angle_3, self.major_radius, self.minor_radius);
				do_single(angle_2, angle_3, self.major_radius, self.minor_radius);
				do_single(angle_2, angle_4, self.major_radius, self.minor_radius);
			}
		}
		unsafe {
			gl::End();
			// gl::Disable(gl::CULL_FACE);
		}
	}
}