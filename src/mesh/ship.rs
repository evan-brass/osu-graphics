use crate::mesh::mesh::Mesh;

use crate::gl;
use gl::types::*;

pub struct Ship {
	display_list: GLuint
}
impl Ship {
	pub fn new() -> Ship {
		Ship {
			display_list: 0
		}
	}
}
impl Mesh for Ship {
	fn init(&mut self) {
		unsafe {
			self.display_list = gl::GenLists(1);
			gl::NewList(self.display_list, gl::COMPILE);
				let width = 1.0;
				let height = 1.0;
				let bow_lead = 1.0;
				let length = 3.0;
				// Haul:
				gl::Color3f(43.0 / 255.0, 26.0 / 255.0, 31.0 / 255.0);
				gl::Begin(gl::TRIANGLE_STRIP);
					gl::Vertex3f(0.0, 0.0, length);
					gl::Vertex3f(0.0, height, length);
					gl::Vertex3f(width / 2.0, 0.0, length - bow_lead);
					gl::Vertex3f(width / 2.0, height, length - bow_lead);
					gl::Vertex3f(width / 2.0, 0.0, 0.0);
					gl::Vertex3f(width / 2.0, height, 0.0);

					gl::Vertex3f(-width / 2.0, 0.0, 0.0);
					gl::Vertex3f(-width / 2.0, height, 0.0);

					gl::Vertex3f(-width / 2.0, 0.0, length - bow_lead);
					gl::Vertex3f(-width / 2.0, height, length - bow_lead);
					gl::Vertex3f(0.0, height, length);
					gl::Vertex3f(0.0, 0.0, length);
					gl::Vertex3f(-width / 2.0, 0.0, length - bow_lead);
					gl::Vertex3f(width / 2.0, 0.0, length - bow_lead);
					gl::Vertex3f(-width / 2.0, 0.0, 0.0);
					gl::Vertex3f(width / 2.0, 0.0, 0.0);
				gl::End( );
				// Sail
				gl::Color3f(1.0, 1.0, 1.0);
				gl::Begin(gl::QUADS);
					gl::Vertex3f(width / 2.0, height * 2.5, length / 2.0);
					gl::Vertex3f(-width / 2.0, height * 2.5, length / 2.0);
					gl::Vertex3f(-width / 2.0, height * 1.5, length / 2.0);
					gl::Vertex3f(width / 2.0, height * 1.5, length / 2.0);
				gl::End();
				// Mast
				gl::Color3f(99.0 / 255.0, 55.0 / 255.0, 44.0 / 255.0);
				gl::LineWidth(50.0);
				gl::Begin(gl::LINES);
					gl::Vertex3f(0.0, 0.0, length / 2.0);
					gl::Vertex3f(0.0, height * 2.5, length / 2.0);
				gl::End();
			gl::EndList();
		}
	}
	fn draw(&mut self) {
		unsafe {
			gl::CallList(self.display_list);
		}
	}
}