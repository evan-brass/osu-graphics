use crate::mesh::mesh::Mesh;

use crate::gl;
use gl::types::*;

pub struct Propeller {
	display_list: GLuint
}
impl Propeller {
	pub fn new() -> Propeller {
		Propeller {
			display_list: 0
		}
	}
}
impl Mesh for Propeller {
	fn init(&mut self) {
		unsafe {
			self.display_list = gl::GenLists(1);
			// blade parameters:
			const BLADE_RADIUS: f32 = 1.0;
			const BLADE_WIDTH: f32 = 0.4;

			// draw the helicopter blade with radius BLADE_RADIUS and
			//	width BLADE_WIDTH centered at (0.,0.,0.) in the XY plane
			gl::NewList(self.display_list, gl::COMPILE);
				gl::Color3f(1.0, 1.0, 1.0);
				gl::Begin(gl::TRIANGLES);
					gl::Vertex2f(  BLADE_RADIUS,  BLADE_WIDTH/2. );
					gl::Vertex2f(  0., 0. );
					gl::Vertex2f(  BLADE_RADIUS, -BLADE_WIDTH/2. );

					gl::Vertex2f( -BLADE_RADIUS, -BLADE_WIDTH/2. );
					gl::Vertex2f(  0., 0. );
					gl::Vertex2f( -BLADE_RADIUS,  BLADE_WIDTH/2. );
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