use crate::mesh::mesh::Mesh;

use crate::gl;
use gl::types::*;

use std::f32::consts::PI;

pub struct Cone {
	pub height: f32,
	pub radius: f32,
	pub segments: u32
}
impl Cone {
	pub fn new() -> Self {
		Self {
			height: 1.0,
			radius: 0.5,
			segments: 8
		}
	}
}
impl Mesh for Cone {
	fn init(&mut self) {}
	fn draw(&mut self) {
		let segment_angle = 2.0 * PI / self.segments as f32;
		unsafe {
			gl::Color3f(0.0, 255.0, 255.0);
			
			gl::Enable(gl::CULL_FACE);
			gl::Begin(gl::TRIANGLES);
		}
		for i in 0..self.segments {
			// println!("Segment: {:?}", i);
			let angle_1 = (i as f32) * segment_angle;
			let angle_3 = (1 as f32 + 0.5) * segment_angle;
			let angle_2 = (i as f32 + 1.0) * segment_angle;
			let v1_x = self.radius * angle_1.sin();
			let v1_z = self.radius * angle_1.cos();
			let v2_x = self.radius * angle_2.sin();
			let v2_z = self.radius * angle_2.cos();
			unsafe {
				// Point face
				gl::Normal3f(angle_3.sin() * self.height, self.radius, angle_3.cos() * self.height);
				gl::Vertex3f(0.0, self.height, 0.0);
				gl::Normal3f(angle_1.sin() * self.height, self.radius, angle_1.cos() * self.height);
				gl::Vertex3f(v1_x, 0.0, v1_z);
				gl::Normal3f(angle_2.sin() * self.height, self.radius, angle_2.cos() * self.height);
				gl::Vertex3f(v2_x, 0.0, v2_z);
				// Flat face
				gl::Normal3f(0.0, -1.0, 0.0);
				gl::Vertex3f(0.0, 0.0, 0.0);
				gl::Normal3f(0.0, -1.0, 0.0);
				gl::Vertex3f(v2_x, 0.0, v2_z);
				gl::Normal3f(0.0, -1.0, 0.0);
				gl::Vertex3f(v1_x, 0.0, v1_z);
			}
		}
		unsafe {
			gl::End();
			gl::Disable(gl::CULL_FACE);
		}
	}
}