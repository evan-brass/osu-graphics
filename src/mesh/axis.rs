use crate::mesh::mesh::Mesh;

use crate::gl;
use gl::types::*;

pub struct Axis {
	display_list: GLuint
}
impl Axis {
	pub fn new() -> Axis {
		Axis {
			display_list: 0
		}
	}
}
impl Mesh for Axis {
	fn init(&mut self) {
		unsafe {
			// Build Arrow display list
			let axis = gl::GenLists( 1 );
			gl::NewList(axis, gl::COMPILE);
				gl::LineWidth( 1.0 );
				gl::Begin(gl::LINE_STRIP);
				let d = 0.2;
				let l = 2.0;
				gl::Vertex3f(0.0, 0.0, 0.0);
				gl::Vertex3f(0.0, l, 0.0);
				gl::Vertex3f(-d, l - d, 0.0);
				gl::Vertex3f(0.0, l, 0.0);
				gl::Vertex3f(d, l - d, 0.0);
				gl::End();
				gl::LineWidth( 1.0 );
			gl::EndList();

			self.display_list = gl::GenLists( 1 );
			gl::NewList(self.display_list, gl::COMPILE);
				// Y-axis label
				gl::Color3f(0.0, 1.0, 0.0);
				gl::CallList(axis);
				gl::Color3f(1.0, 0.0, 0.0);
				// X-axis label
				gl::Color3f(1.0, 0.0, 0.0);
				gl::PushMatrix();
				gl::Rotatef(-90.0, 0.0, 0.0, 1.0);
				gl::CallList(axis);
				gl::PopMatrix();
				// Z-axis label
				gl::Color3f(0.0, 0.0, 1.0);
				gl::PushMatrix();
				gl::Rotatef(90.0, 1.0, 0.0, 0.0);
				gl::CallList(axis);
				gl::PopMatrix();
			gl::EndList();
		}
	}
	fn draw(&self) {
		unsafe{ 
			gl::CallList(self.display_list);
		}
	}
}