use crate::gl;
use gl::types::*;

pub enum ActiveFace {
	Front,
	Back,
	FrontAndBack
}
pub struct Material {
	pub active: ActiveFace,
	diffuse: [f32; 4],
	specular: [f32; 4],
	emission: [f32; 4],
	ambient: [f32; 4],
	pub shininess: f32
}
impl Material {
	pub fn new(active: ActiveFace) -> Self {
		Self {
			active,
			diffuse: [255.0, 255.0, 255.0, 1.0],
			specular: [255.0, 255.0, 255.0, 1.0],
			emission: [0.0, 0.0, 0.0, 1.0],
			ambient: [0.0, 0.0, 0.0, 1.0],
			shininess: 1.0
		}
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
	fn apply_to_face(&self, face: GLenum) {
		unsafe {
			gl::Materialfv(face, gl::AMBIENT, self.ambient.as_ptr());
			gl::Materialfv(face, gl::DIFFUSE, self.diffuse.as_ptr());
			gl::Materialfv(face, gl::SPECULAR, self.specular.as_ptr());
			gl::Materialf (face, gl::SHININESS, self.shininess);
			gl::Materialfv(face, gl::EMISSION, self.emission.as_ptr());
			// gl::LightModeli (gl::LIGHT_MODEL_TWO_SIDE,  gl::TRUE);
		}
	}
	pub fn call(&self) {
		match self.active {
			ActiveFace::Front => {
				self.apply_to_face(gl::FRONT);
			},
			ActiveFace::Back => {
				self.apply_to_face(gl::BACK);
			},
			ActiveFace::FrontAndBack => {
				self.apply_to_face(gl::FRONT);
				self.apply_to_face(gl::BACK);
			}
		}
	}
}