use crate::mesh::mesh::Mesh;

use image::{ImageDecoder, jpeg::JPEGDecoder};

use crate::gl;
use gl::types::*;

use rand::prelude::*;

use std::fs::File;
use std::path::Path;

const WATER_SIZE: usize = 20;

pub struct Ocean {
	display_list: GLuint,
	offsets: [[f32; WATER_SIZE]; WATER_SIZE],
	time: f32,
	texture: GLuint,
	morph_on: bool,
	texture_on: bool
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
			offsets,
			texture: 0,
			morph_on: false,
			texture_on: false
		}
	}
	pub fn animate(&mut self, time_diff: f32) {
		self.time += time_diff / 500.0;
	}
	pub fn toggle_morph(&mut self) {
		self.morph_on = !self.morph_on;
	}
	pub fn toggle_texture(&mut self) {
		self.texture_on = !self.texture_on;
	}
}
impl Mesh for Ocean {
	fn init(&mut self) {
		let path = Path::new("./src/img/piex2.jpg");
		println!("Attempting to load image from: {:?}", path);
    	let texture = File::open(&path).unwrap();
		let decoder = JPEGDecoder::new(texture).unwrap();
		let (width, height) = decoder.dimensions();
		println!("Texture Dimensions: {:?}", decoder.dimensions());
		println!("Texture Color Type: {:?}", decoder.colortype());
		let image_data = decoder.read_image_with_progress(|progress| {
			println!("{:?}", progress);
		}).unwrap();
		unsafe {
			gl::GenTextures(1, &mut self.texture);
			gl::BindTexture(gl::TEXTURE_2D, self.texture);
			gl::TexEnvf(gl::TEXTURE_ENV, gl::TEXTURE_ENV_MODE, gl::REPLACE as f32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
			gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
			gl::TexImage2D(gl::TEXTURE_2D, 0, 3, width as i32, height as i32, 0, gl::RGB, gl::UNSIGNED_BYTE, image_data.as_ptr() as *const std::ffi::c_void);
			println!("{:?}", gl::GetError());
		}
		
	}
	fn draw(&mut self) {
		let total_width = 15.0;
		let total_height = total_width;
		let tex_width = 1.0 / self.offsets.len() as f32;
		let rotation_radius = if self.morph_on {
			tex_width * 0.2 // To be certain that it only stretches, this needs to be <= 0.5
		} else {
			0.0
		};
		unsafe {
			if self.texture_on {
				gl::Enable(gl::TEXTURE_2D);
			}
			gl::BindTexture(gl::TEXTURE_2D, self.texture);
			gl::PushMatrix();
			gl::Rotatef(90.0, 1.0, 0.0, 0.0);
			gl::Scalef(total_width / self.offsets.len() as f32, total_height / self.offsets.len() as f32, 1.0);
			gl::Translatef(self.offsets.len() as f32 / -2.0, self.offsets.len() as f32 / -2.0, 0.0);
			gl::Begin(gl::QUADS);
				for r in 0..(self.offsets.len() - 1) {
					for c in 0..(self.offsets.len() - 1) {
						let tl = self.time + self.offsets[r][c];
						gl::TexCoord2f(
							c as f32 * tex_width + rotation_radius * tl.cos(),
							r as f32 * tex_width + rotation_radius * tl.sin()
						);
						gl::Vertex2f(r as f32, c as f32);

						let tr = self.time + self.offsets[r][c + 1];
						gl::TexCoord2f(
							(c + 1) as f32 * tex_width + rotation_radius * tr.cos(),
							r as f32 * tex_width + rotation_radius * tr.sin()
						);
						gl::Vertex2f(r as f32, (c + 1) as f32);

						let br = self.time + self.offsets[r + 1][c + 1];
						gl::TexCoord2f(
							(c + 1) as f32 * tex_width + rotation_radius * br.cos(),
							(r + 1) as f32 * tex_width + rotation_radius * br.sin()
						);
						gl::Vertex2f((r + 1) as f32, (c + 1) as f32);

						let bl = self.time + self.offsets[r + 1][c];
						gl::TexCoord2f(
							c as f32 * tex_width + rotation_radius * bl.cos(),
							(r + 1) as f32 * tex_width + rotation_radius * bl.sin()
						);
						gl::Vertex2f((r + 1) as f32, c as f32);
					}
				}
			gl::End();
			gl::PopMatrix();
			gl::Disable(gl::TEXTURE_2D);
		}
	}
}