use crate::mesh::mesh::Mesh;

use crate::gl;
use gl::types::*;

use rand::prelude::*;

const WATER_SIZE: usize = 20;

pub struct Ocean {
	display_list: GLuint,
	offsets: [[f32; WATER_SIZE]; WATER_SIZE],
	time: f32
}
impl Ocean {
	pub fn new() -> Ocean {
		let mut offsets = [[0 as f32; WATER_SIZE]; WATER_SIZE];
		let mut rng = rand::thread_rng();
		for row in offsets.iter_mut() {
			for item in row.iter_mut() {
				*item = rng.gen_range(0.0, 1000.0);
			}
		}
		Ocean {
			display_list: 0,
			time: 0.0,
			offsets
		}
	}
	pub fn animate(&mut self, time_diff: f32) {
		self.time += time_diff / 500.0;
	}
}
impl Mesh for Ocean {
	fn init(&mut self) {}
	fn draw(&mut self) {
		unsafe {
			gl::Begin(gl::QUADS);
				let total_width = 15.0;
				let total_height = total_width;
				let width = total_width / self.offsets.len() as f32;
				let height = total_height / self.offsets.len() as f32;
				let wave_height = 0.2;
				let red_start = 17.0;
				let green_start = 29.0;
				let blue_start = 74.0;
				let red_cof = (103.0 - red_start) / 2.0;
				let green_cof = (122.0 - green_start) / 2.0;
				let blue_cof = (140.0 - blue_start) / 2.0;
				for r in 0..(self.offsets.len() - 1) {
					for c in 0..(self.offsets.len() - 1) {
						let tl = (self.time + self.offsets[r][c]).sin() + 1.0;
						
						gl::Color3f(
							(red_start + red_cof * tl) / 255.0, 
							(green_start + green_cof * tl) / 255.0, 
							(blue_start + blue_cof * tl) / 255.0
						);
						gl::Vertex3f(
							-total_width / 2.0 + width * r as f32, 
							tl * wave_height, 
							-total_height / 2.0 + height * c as f32
						);
						let tr = (self.time + self.offsets[r][c + 1]).sin() + 1.0;
						gl::Color3f(
							(red_start + red_cof * tr) / 255.0, 
							(green_start + green_cof * tr) / 255.0, 
							(blue_start + blue_cof * tr) / 255.0
						);
						gl::Vertex3f(
							-total_width / 2.0 + width * r as f32, 
							tr * wave_height, 
							-total_height / 2.0 + height * (c + 1) as f32
						);
						let br = (self.time + self.offsets[r + 1][c + 1]).sin() + 1.0;
						gl::Color3f(
							(red_start + red_cof * br) / 255.0, 
							(green_start + green_cof * br) / 255.0, 
							(blue_start + blue_cof * br) / 255.0
						);
						gl::Vertex3f(
							-total_width / 2.0 + width * (r + 1) as f32, 
							br * wave_height, 
							-total_height / 2.0 + height * (c + 1) as f32
						);
						let bl = (self.time + self.offsets[r + 1][c]).sin() + 1.0;
						gl::Color3f(
							(red_start + red_cof * bl) / 255.0, 
							(green_start + green_cof * bl) / 255.0, 
							(blue_start + blue_cof * bl) / 255.0
						);
						gl::Vertex3f(
							-total_width / 2.0 + width * (r + 1) as f32, 
							bl * wave_height, 
							-total_height / 2.0 + height * c as f32
						);
					}
				}
			gl::End();
		}
	}
}