use crate::gl;
use gl::types::*;

fn id_to_enum(id: u32) -> GLenum {
	match id {
		0 => gl::LIGHT0,
		1 => gl::LIGHT1,
		2 => gl::LIGHT2,
		3 => gl::LIGHT3,
		4 => gl::LIGHT4,
		5 => gl::LIGHT5,
		6 => gl::LIGHT6,
		7 => gl::LIGHT7,
		_ => panic!("The light id ({:?}) is out of bounds [0, 7]", id),
	}
}

pub struct Light {
	enabled: bool,
	id: u32,
	position: [f32; 4],
	diffuse: [f32; 4],
	specular: [f32; 4],
	ambient: [f32; 4],
	ca: f32,
	la: f32,
	qa: f32,
}
impl Light {
	pub fn new(id: u32) -> Self {
		Self {
			enabled: true,
			id,
			position: [0.0, 0.0, 0.0, 1.0],
			diffuse: [0.8, 0.8, 0.8, 1.0],
			specular: [0.4, 0.4, 0.4, 1.0],
			ambient: [0.0, 0.0, 0.0, 0.0],
			ca: 0.0,
			la: 1.0,
			qa: 0.0,
		}
	}
	pub fn place(&mut self, x: f32, y: f32, z: f32, w: f32) {
		self.position = [x, y, z, w];
	}
	pub fn diffuse(&mut self, r: f32, g: f32, b: f32) {
		self.diffuse = [r, g, b, 1.0];
	}
	pub fn specular(&mut self, r: f32, g: f32, b: f32) {
		self.specular = [r, g, b, 1.0];
	}
	pub fn ambient(&mut self, r: f32, g: f32, b: f32) {
		self.ambient = [r, g, b, 1.0];
	}
	pub fn attenuate(&mut self, c: f32, l: f32, q: f32) {
		self.ca = c;
		self.la = l;
		self.qa = q;
	}
	pub fn use_ambient(ar: f32, ag: f32, ab: f32) {
		let arr = [ar, ag, ab, 1.0];
		unsafe {
			gl::LightModelfv(gl::LIGHT_MODEL_AMBIENT, arr.as_ptr());
		}
	}
	pub fn call(&self) {
		let light_enum = id_to_enum(self.id);
		// println!("Light Enum is: {:?} ({:X})", light_enum, light_enum);
		unsafe {
			if self.enabled {
				gl::Enable(light_enum);
			} else {
				gl::Disable(light_enum);
			}
			gl::LightModeli(gl::LIGHT_MODEL_TWO_SIDE, gl::TRUE as i32);

			gl::Lightfv(light_enum, gl::POSITION, self.position.as_ptr());
			// Marker of where the point light is:
			gl::Begin(gl::POINTS);
			gl::Color3f(self.diffuse[0], self.diffuse[1], self.diffuse[2]);
			gl::Vertex3f(self.position[0], self.position[1], self.position[2]);
			gl::End();

			gl::Lightfv(light_enum, gl::AMBIENT, self.ambient.as_ptr());
			gl::Lightfv(light_enum, gl::DIFFUSE, self.diffuse.as_ptr());
			gl::Lightfv(light_enum, gl::SPECULAR, self.specular.as_ptr());
			gl::Lightf(light_enum, gl::CONSTANT_ATTENUATION, self.ca);
			gl::Lightf(light_enum, gl::LINEAR_ATTENUATION, self.la);
			gl::Lightf(light_enum, gl::QUADRATIC_ATTENUATION, self.qa);
		}
	}
	pub fn toggleEnable(&mut self) {
		self.enabled = !self.enabled;
	}
}
